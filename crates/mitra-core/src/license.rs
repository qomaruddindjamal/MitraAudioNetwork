//! Mitra Cryptographic Licensing Subsystem
//!
//! Mengimplementasikan model lisensi permanen asimetris Ed25519,
//! pembangkitan Serial Number (SN) terikat Native ID & Product ID,
//! License Authority PoC, serta verifikasi independen di sisi klien.

use crate::error::LicenseError;
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use sha2::{Digest, Sha256};

pub const SCHEMA_VERSION_V1: u32 = 1;
pub const ENTITLEMENT_WEB_MANAGEMENT: &str = "WEB_MANAGEMENT";
pub const ENTITLEMENT_BASIC_AUDIO: &str = "BASIC_AUDIO";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LicenseTier {
    Demo,
    Standard,
    Professional,
    Enterprise,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LicenseClaims {
    pub license_id: String,
    pub customer_name: String,
    pub tier: LicenseTier,
    pub max_terminals: u32,
    pub not_before: u64,
    pub expires_at: u64,
    pub hardware_hash: String,
}

impl LicenseClaims {
    pub fn to_canonical_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(self.license_id.as_bytes());
        bytes.push(b':');
        bytes.extend_from_slice(self.customer_name.as_bytes());
        bytes.push(b':');
        bytes.extend_from_slice(match self.tier {
            LicenseTier::Demo => b"DEMO",
            LicenseTier::Standard => b"STANDARD",
            LicenseTier::Professional => b"PROFESSIONAL",
            LicenseTier::Enterprise => b"ENTERPRISE",
        });
        bytes.push(b':');
        bytes.extend_from_slice(&self.max_terminals.to_le_bytes());
        bytes.push(b':');
        bytes.extend_from_slice(&self.not_before.to_le_bytes());
        bytes.push(b':');
        bytes.extend_from_slice(&self.expires_at.to_le_bytes());
        bytes.push(b':');
        bytes.extend_from_slice(self.hardware_hash.as_bytes());
        bytes
    }
}

/// Struktur Payload Lisensi Terversi Mitra Audio Network System (Permanent)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MitraLicensePayload {
    pub schema_version: u32,
    pub license_id: String,
    pub product_id: String,
    pub native_id: String,
    pub serial_number: String,
    pub issued_at: u64,
    pub license_type: String, // "permanent"
    pub entitlements: Vec<String>,
    pub issuer_id: String,
}

impl MitraLicensePayload {
    /// Serialisasi kanonikal deterministik untuk penandatanganan dan verifikasi Ed25519.
    /// Format: `schema_version|license_id|product_id|native_id|serial_number|issued_at|license_type|entitlement1,entitlement2|issuer_id`
    pub fn to_canonical_bytes(&self) -> Vec<u8> {
        let mut sorted_entitlements = self.entitlements.clone();
        sorted_entitlements.sort();
        let ent_str = sorted_entitlements.join(",");

        let canonical_str = format!(
            "V{}|{}|{}|{}|{}|{}|{}|{}|{}",
            self.schema_version,
            self.license_id,
            self.product_id,
            self.native_id,
            self.serial_number,
            self.issued_at,
            self.license_type,
            ent_str,
            self.issuer_id
        );
        canonical_str.into_bytes()
    }

    /// Validasi kelengkapan bidang payload
    pub fn validate_completeness(&self) -> Result<(), LicenseError> {
        if self.schema_version != SCHEMA_VERSION_V1 {
            return Err(LicenseError::UnsupportedSchemaVersion(self.schema_version));
        }
        if self.license_id.trim().is_empty() {
            return Err(LicenseError::IncompletePayload(
                "license_id is empty".into(),
            ));
        }
        if self.product_id.trim().is_empty() {
            return Err(LicenseError::IncompletePayload(
                "product_id is empty".into(),
            ));
        }
        if self.native_id.trim().is_empty() {
            return Err(LicenseError::IncompletePayload("native_id is empty".into()));
        }
        if self.serial_number.trim().is_empty() {
            return Err(LicenseError::IncompletePayload(
                "serial_number is empty".into(),
            ));
        }
        if self.license_type != "permanent" {
            return Err(LicenseError::IncompletePayload(
                "license_type must be permanent".into(),
            ));
        }
        if self.issuer_id.trim().is_empty() {
            return Err(LicenseError::IncompletePayload("issuer_id is empty".into()));
        }
        Ok(())
    }
}

/// Struktur Lisensi Ditandatangani (Signed License Package)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignedMitraLicense {
    pub payload: MitraLicensePayload,
    pub signature_bytes: [u8; 64],
}

/// Generator Serial Number Terikat Native ID dan Product ID
pub struct SerialNumberGenerator;

impl SerialNumberGenerator {
    /// Format SN yang diterbitkan Mitra Audio: 4 segmen alfanumerik dipisahkan strip `-` (contoh: `XXXX-YYYY-ZZZZ-WWWW`).
    /// Ketentuan khusus:
    /// - Mengandung salah satu penanda khusus Mitra Audio: `CIMT` atau `MITG`.
    /// - Posisi penanda dipilih secara acak (segmen 0, 1, 2, atau 3).
    /// - Segmen lain dibangkitkan secara acak menggunakan CSPRNG dan diformat alfanumerik.
    /// - Hubungan deterministik dengan Native ID dipertahankan pada tingkat verifikasi lisensi Ed25519.
    pub fn generate_sn_with_csprng<R: mitra_pal::SecureRandomProvider>(
        product_id: &str,
        native_id: &str,
        _secret_salt: &str,
        rng: &R,
    ) -> Result<String, LicenseError> {
        if product_id.trim().is_empty() {
            return Err(LicenseError::ProductMismatch {
                expected: "non-empty product_id".into(),
                actual: product_id.to_string(),
            });
        }
        if native_id.trim().is_empty() {
            return Err(LicenseError::InvalidNativeIdFormat);
        }

        // 1. Ambil 16 byte acak dari CSPRNG
        let mut random_bytes = [0u8; 16];
        rng.fill_random_bytes(&mut random_bytes)
            .map_err(|e| LicenseError::IncompletePayload(format!("CSPRNG failure: {:?}", e)))?;

        // 2. Pilih tag khusus antara CIMT atau MITG
        let tag = if (random_bytes[0] & 1) == 0 {
            "CIMT"
        } else {
            "MITG"
        };

        // 3. Tentukan posisi acak untuk tag (0, 1, 2, atau 3)
        let tag_pos = ((random_bytes[1] % 4) as usize) % 4;

        // 4. Bangkitkan 3 segmen acak alfanumerik (masing-masing 4 karakter hex/alfanumerik)
        let seg_a = format!("{:02X}{:02X}", random_bytes[2], random_bytes[3]);
        let seg_b = format!("{:02X}{:02X}", random_bytes[4], random_bytes[5]);
        let seg_c = format!("{:02X}{:02X}", random_bytes[6], random_bytes[7]);

        let mut segments = vec![seg_a, seg_b, seg_c];
        segments.insert(tag_pos, tag.to_string());

        let sn = segments.join("-");
        Ok(sn)
    }

    /// Metode kompatibilitas / default generator: jika tidak diberikan provider CSPRNG eksternal,
    /// menggunakan hash SHA-256 terstruktur dengan penempatan tag CIMT/MITG secara dinamis.
    pub fn generate_sn(
        product_id: &str,
        native_id: &str,
        secret_salt: &str,
    ) -> Result<String, LicenseError> {
        if product_id.trim().is_empty() {
            return Err(LicenseError::ProductMismatch {
                expected: "non-empty product_id".into(),
                actual: product_id.to_string(),
            });
        }
        if native_id.trim().is_empty() {
            return Err(LicenseError::InvalidNativeIdFormat);
        }

        let mut hasher = Sha256::new();
        hasher.update(product_id.as_bytes());
        hasher.update(b":");
        hasher.update(native_id.as_bytes());
        hasher.update(b":");
        hasher.update(secret_salt.as_bytes());
        let hash = hasher.finalize();

        let tag = if (hash[0] & 1) == 0 { "CIMT" } else { "MITG" };
        let tag_pos = ((hash[1] % 4) as usize) % 4;

        let seg_a = format!("{:02X}{:02X}", hash[2], hash[3]);
        let seg_b = format!("{:02X}{:02X}", hash[4], hash[5]);
        let seg_c = format!("{:02X}{:02X}", hash[6], hash[7]);

        let mut segments = vec![seg_a, seg_b, seg_c];
        segments.insert(tag_pos, tag.to_string());

        let sn = segments.join("-");
        Ok(sn)
    }

    /// Validasi format SN Mitra Audio:
    /// - Terdiri atas 4 segmen dipisahkan tanda strip `-`
    /// - Setiap segmen panjangnya 4 karakter alfanumerik (Total 19 karakter: `XXXX-XXXX-XXXX-XXXX`)
    /// - Mengandung salah satu token penanda khusus Mitra Audio: `CIMT` atau `MITG` di salah satu segmennya
    pub fn validate_sn_format(sn: &str) -> bool {
        let parts: Vec<&str> = sn.split('-').collect();
        // Terima format baru (4 segmen) atau format legacy PoC (5 segmen MANS-...)
        if parts.len() == 4 {
            let all_len_4 = parts
                .iter()
                .all(|p| p.len() == 4 && p.chars().all(|c| c.is_ascii_alphanumeric()));
            let contains_tag = parts.iter().any(|&p| p == "CIMT" || p == "MITG");
            return all_len_4 && contains_tag;
        }

        // Backward compatibility untuk format lama MANS-...
        if parts.len() == 5 && parts[0] == "MANS" {
            return parts[1].len() == 4
                && parts[2].len() == 4
                && parts[3].len() == 4
                && parts[4].len() == 4;
        }

        false
    }
}

impl SignedMitraLicense {
    /// Serialisasi SignedMitraLicense ke format teks berkas `.lic` yang aman dan portabel.
    pub fn to_license_file_content(&self) -> String {
        let canonical_str = String::from_utf8_lossy(&self.payload.to_canonical_bytes()).to_string();
        let sig_hex: String = self
            .signature_bytes
            .iter()
            .map(|b| format!("{:02X}", b))
            .collect();
        format!(
            "MITRA_AUDIO_LICENSE_V1\nPAYLOAD={}\nSIGNATURE={}\n",
            canonical_str, sig_hex
        )
    }

    /// Parsing berkas lisensi `.lic` menjadi struktur `SignedMitraLicense`.
    pub fn from_license_file_content(content: &str) -> Result<Self, LicenseError> {
        let mut payload_str = None;
        let mut sig_hex_str = None;

        for line in content.lines() {
            let line = line.trim();
            if let Some(val) = line.strip_prefix("PAYLOAD=") {
                payload_str = Some(val.to_string());
            } else if let Some(val) = line.strip_prefix("SIGNATURE=") {
                sig_hex_str = Some(val.to_string());
            }
        }

        let p_str = payload_str.ok_or_else(|| {
            LicenseError::IncompletePayload("Missing PAYLOAD in license file".into())
        })?;
        let s_str = sig_hex_str.ok_or_else(|| {
            LicenseError::IncompletePayload("Missing SIGNATURE in license file".into())
        })?;

        // Parse canonical payload format: V<ver>|id|prod|native|sn|issued|type|entitlements|issuer
        let parts: Vec<&str> = p_str.split('|').collect();
        if parts.len() != 9 {
            return Err(LicenseError::IncompletePayload(
                "Malformed canonical payload structure".into(),
            ));
        }

        let schema_ver: u32 = parts[0]
            .strip_prefix('V')
            .and_then(|v| v.parse().ok())
            .ok_or(LicenseError::UnsupportedSchemaVersion(0))?;

        let license_id = parts[1].to_string();
        let product_id = parts[2].to_string();
        let native_id = parts[3].to_string();
        let serial_number = parts[4].to_string();
        let issued_at: u64 = parts[5]
            .parse()
            .map_err(|_| LicenseError::IncompletePayload("Invalid issued_at timestamp".into()))?;
        let license_type = parts[6].to_string();
        let entitlements: Vec<String> = if parts[7].is_empty() {
            Vec::new()
        } else {
            parts[7].split(',').map(|s| s.to_string()).collect()
        };
        let issuer_id = parts[8].to_string();

        let payload = MitraLicensePayload {
            schema_version: schema_ver,
            license_id,
            product_id,
            native_id,
            serial_number,
            issued_at,
            license_type,
            entitlements,
            issuer_id,
        };

        // Parse signature bytes from hex
        if s_str.len() != 128 {
            return Err(LicenseError::InvalidSignature);
        }
        let mut sig_bytes = [0u8; 64];
        for i in 0..64 {
            let byte_hex = &s_str[i * 2..i * 2 + 2];
            sig_bytes[i] =
                u8::from_str_radix(byte_hex, 16).map_err(|_| LicenseError::InvalidSignature)?;
        }

        Ok(SignedMitraLicense {
            payload,
            signature_bytes: sig_bytes,
        })
    }
}

/// Entitas Penerbit Lisensi (License Issuer / Authority)
pub struct LicenseIssuer {
    issuer_id: String,
    signing_key: SigningKey,
    authority_salt: String,
}

impl LicenseIssuer {
    pub fn new(issuer_id: &str, signing_key: SigningKey, salt: &str) -> Self {
        Self {
            issuer_id: issuer_id.to_string(),
            signing_key,
            authority_salt: salt.to_string(),
        }
    }

    pub fn verifying_key(&self) -> VerifyingKey {
        self.signing_key.verifying_key()
    }

    pub fn issue_license(
        &self,
        license_id: &str,
        product_id: &str,
        native_id: &str,
        issued_at: u64,
        entitlements: Vec<String>,
    ) -> Result<SignedMitraLicense, LicenseError> {
        let sn = SerialNumberGenerator::generate_sn(product_id, native_id, &self.authority_salt)?;

        let payload = MitraLicensePayload {
            schema_version: SCHEMA_VERSION_V1,
            license_id: license_id.to_string(),
            product_id: product_id.to_string(),
            native_id: native_id.to_string(),
            serial_number: sn,
            issued_at,
            license_type: "permanent".to_string(),
            entitlements,
            issuer_id: self.issuer_id.clone(),
        };

        payload.validate_completeness()?;

        let canonical_bytes = payload.to_canonical_bytes();
        let signature = self.signing_key.sign(&canonical_bytes);

        Ok(SignedMitraLicense {
            payload,
            signature_bytes: signature.to_bytes(),
        })
    }
}

/// Mesin Verifikasi Lisensi di Sisi Klien / Aplikasi
pub struct LicenseVerifier {
    public_key: VerifyingKey,
}

impl LicenseVerifier {
    pub fn from_public_key_bytes(bytes: &[u8; 32]) -> Result<Self, LicenseError> {
        let public_key = VerifyingKey::from_bytes(bytes).map_err(|_| LicenseError::MalformedKey)?;
        Ok(Self { public_key })
    }

    pub fn verify_signature(
        &self,
        message: &[u8],
        signature_bytes: &[u8; 64],
    ) -> Result<(), LicenseError> {
        let signature = Signature::from_bytes(signature_bytes);
        self.public_key
            .verify(message, &signature)
            .map_err(|_| LicenseError::InvalidSignature)
    }

    pub fn validate_claims(
        &self,
        claims: &LicenseClaims,
        signature_bytes: &[u8; 64],
        current_time: u64,
        actual_hardware_hash: &str,
    ) -> Result<(), LicenseError> {
        let canonical_bytes = claims.to_canonical_bytes();
        self.verify_signature(&canonical_bytes, signature_bytes)?;

        if current_time < claims.not_before {
            return Err(LicenseError::NotYetValid {
                not_before: claims.not_before,
                current_time,
            });
        }

        if claims.expires_at > 0 && current_time > claims.expires_at {
            return Err(LicenseError::Expired {
                expired_at: claims.expires_at,
                current_time,
            });
        }

        if !claims.hardware_hash.is_empty() && claims.hardware_hash != actual_hardware_hash {
            return Err(LicenseError::HardwareMismatch {
                expected_hash: claims.hardware_hash.clone(),
                actual_hash: actual_hardware_hash.to_string(),
            });
        }

        Ok(())
    }

    /// Verifikasi menyeluruh SignedMitraLicense:
    /// 1. Validasi skema & kelengkapan payload
    /// 2. Verifikasi tanda tangan digital asimetris Ed25519 atas canonical payload
    /// 3. Validasi kesesuaian Product ID aplikasi
    /// 4. Validasi kesesuaian Native ID perangkat lokal
    /// 5. Validasi status pencabutan (revocation check)
    pub fn verify_mitra_license(
        &self,
        license: &SignedMitraLicense,
        expected_product_id: &str,
        actual_native_id: &str,
        revoked_license_ids: &[&str],
    ) -> Result<(), LicenseError> {
        // 1. Validasi kelengkapan payload & skema versi
        license.payload.validate_completeness()?;

        // 2. Verifikasi status pencabutan (Revocation check)
        if revoked_license_ids.contains(&license.payload.license_id.as_str()) {
            return Err(LicenseError::LicenseRevoked(
                license.payload.license_id.clone(),
            ));
        }

        // 3. Verifikasi tanda tangan kriptografi Ed25519
        let canonical_bytes = license.payload.to_canonical_bytes();
        self.verify_signature(&canonical_bytes, &license.signature_bytes)?;

        // 4. Verifikasi Product ID binding
        if license.payload.product_id != expected_product_id {
            return Err(LicenseError::ProductMismatch {
                expected: expected_product_id.to_string(),
                actual: license.payload.product_id.clone(),
            });
        }

        // 5. Verifikasi Native ID binding
        if license.payload.native_id != actual_native_id {
            return Err(LicenseError::HardwareMismatch {
                expected_hash: license.payload.native_id.clone(),
                actual_hash: actual_native_id.to_string(),
            });
        }

        Ok(())
    }

    /// Pemeriksaan Hak Fitur / Entitlement (Backend Enforcement)
    pub fn check_entitlement(
        &self,
        license: &SignedMitraLicense,
        feature_name: &str,
    ) -> Result<(), LicenseError> {
        // Enforce: Demo and Trial licenses cannot access restricted permanent entitlements
        let lic_type_lower = license.payload.license_type.to_ascii_lowercase();
        if (lic_type_lower == "demo" || lic_type_lower == "trial")
            && feature_name == ENTITLEMENT_WEB_MANAGEMENT
        {
            return Err(LicenseError::FeatureNotEntitled(format!(
                "{}: demo/trial licenses are restricted from permanent web management",
                feature_name
            )));
        }

        if !license
            .payload
            .entitlements
            .iter()
            .any(|e| e == feature_name)
        {
            return Err(LicenseError::FeatureNotEntitled(feature_name.to_string()));
        }
        Ok(())
    }
}

/// Record Database License Authority untuk PoC (Tracking Pembelian & Dukungan 1 Tahun)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LicenseAuthorityRecord {
    pub license_id: String,
    pub product_id: String,
    pub native_id: String,
    pub serial_number: String,
    pub purchase_date: u64,
    pub support_until: u64, // purchase_date + 1 tahun (Layanan penggantian SN)
    pub issued_at: u64,
    pub license_status: String, // "ACTIVE", "REVOKED", "TRANSFERRED"
    pub entitlements: Vec<String>,
}

/// Storage In-Memory / SQLite Registry PoC untuk License Authority
#[derive(Default)]
pub struct LicenseAuthorityStorage {
    records: Vec<LicenseAuthorityRecord>,
    audit_logs: Vec<String>,
}

impl LicenseAuthorityStorage {
    pub fn new() -> Self {
        Self {
            records: Vec::new(),
            audit_logs: Vec::new(),
        }
    }

    pub fn insert_record(&mut self, record: LicenseAuthorityRecord) -> Result<(), LicenseError> {
        // Cek duplikasi License ID
        if self
            .records
            .iter()
            .any(|r| r.license_id == record.license_id)
        {
            return Err(LicenseError::DuplicateLicenseId(record.license_id.clone()));
        }
        // Cek duplikasi Serial Number
        if self
            .records
            .iter()
            .any(|r| r.serial_number == record.serial_number)
        {
            return Err(LicenseError::DuplicateSerialNumber(
                record.serial_number.clone(),
            ));
        }

        let log_entry = format!(
            "AUDIT [INSERT]: LicID={} ProdID={} SN={} NatID={} SupportUntil={}",
            record.license_id,
            record.product_id,
            record.serial_number,
            record.native_id,
            record.support_until
        );
        self.audit_logs.push(log_entry);
        self.records.push(record);
        Ok(())
    }

    pub fn get_record_by_sn(&self, sn: &str) -> Option<&LicenseAuthorityRecord> {
        self.records.iter().find(|r| r.serial_number == sn)
    }

    pub fn get_record_by_native_id(&self, native_id: &str) -> Option<&LicenseAuthorityRecord> {
        self.records.iter().find(|r| r.native_id == native_id)
    }

    pub fn get_all_records(&self) -> &[LicenseAuthorityRecord] {
        &self.records
    }

    pub fn revoke_license(&mut self, license_id: &str, reason: &str) -> Result<(), LicenseError> {
        if let Some(record) = self.records.iter_mut().find(|r| r.license_id == license_id) {
            record.license_status = "REVOKED".to_string();
            let log_entry = format!("AUDIT [REVOKE]: LicID={} Reason={}", license_id, reason);
            self.audit_logs.push(log_entry);
            Ok(())
        } else {
            Err(LicenseError::IncompletePayload("License not found".into()))
        }
    }

    pub fn audit_logs(&self) -> &[String] {
        &self.audit_logs
    }
}
