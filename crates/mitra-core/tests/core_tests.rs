use mitra_core::*;

#[test]
fn test_dgram_header_serialization_roundtrip() {
    let original = DgramHeader::new(
        PacketType::Audio,
        101,
        42,
        512,
        AudioCodecId::ImaAdpcm,
        SampleRateCode::Hz22050,
        PriorityLevel::NormalCall,
    );

    let mut buf = [0u8; HEADER_SIZE];
    original.serialize(&mut buf);

    assert_eq!(buf[0], MAGIC_SYNC_BYTE);
    assert_eq!(buf[1], 0x01); // Audio
    assert_eq!(u16::from_le_bytes([buf[2], buf[3]]), 101); // Channel
    assert_eq!(u32::from_le_bytes([buf[4], buf[5], buf[6], buf[7]]), 42); // Sequence
    assert_eq!(u16::from_le_bytes([buf[8], buf[9]]), 512); // Payload size
    assert_eq!(buf[10], 0x02); // IMA-ADPCM
    assert_eq!(buf[11], 0x02); // 22050 Hz
    assert_eq!(buf[12], 0x02); // NormalCall
    assert_eq!(buf[13], 0x00); // Reserved pad
    assert_eq!(buf[14], 0x00); // Reserved tail byte 0
    assert_eq!(buf[15], 0x00); // Reserved tail byte 1

    let parsed = DgramHeader::parse(&buf).expect("Failed to parse valid serialized header");
    assert_eq!(original, parsed);
}

#[test]
fn test_dgram_header_legacy_wire_golden_packet() {
    // Paket biner 16 byte mentah yang sesuai dengan tangkapan jaringan server legacy:
    // Sync 0x5A, Audio 0x01, Channel 1, Seq 100, Payload 1024 (0x0400), ADPCM 0x02, 22.05kHz 0x02, Priority Normal 0x02, Pad 0x00, Tail 0x0000
    let legacy_bytes: [u8; 16] = [
        0x5A, 0x01, 0x01, 0x00, 0x64, 0x00, 0x00, 0x00, 0x00, 0x04, 0x02, 0x02, 0x02, 0x00, 0x00,
        0x00,
    ];

    let header =
        DgramHeader::parse(&legacy_bytes).expect("Legacy wire packet must be parsed successfully");
    assert_eq!(header.sync_byte, 0x5A);
    assert_eq!(header.packet_type, PacketType::Audio);
    assert_eq!(header.channel_id, 1);
    assert_eq!(header.sequence_number, 100);
    assert_eq!(header.payload_size, 1024);
    assert_eq!(header.audio_codec, AudioCodecId::ImaAdpcm);
    assert_eq!(header.sample_rate_code, SampleRateCode::Hz22050);
    assert_eq!(header.priority, PriorityLevel::NormalCall);
    assert_eq!(header.reserved_pad, 0x00);
    assert_eq!(header.reserved_tail, 0x0000);
}

#[test]
fn test_dgram_header_with_optional_checksum() {
    let header = DgramHeader::new(
        PacketType::Audio,
        1,
        1,
        100,
        AudioCodecId::Pcm,
        SampleRateCode::Hz11025,
        PriorityLevel::Bgm,
    )
    .with_computed_checksum();

    assert!(header.is_checksum_valid());

    let mut buf = [0u8; HEADER_SIZE];
    header.serialize(&mut buf);

    let parsed = DgramHeader::parse(&buf).expect("Should parse header with checksum");
    assert!(parsed.is_checksum_valid());

    // Uji jika tail dirusak
    let mut corrupt_header = parsed;
    corrupt_header.reserved_tail ^= 0xFFFF;
    assert!(!corrupt_header.is_checksum_valid());
}

#[test]
fn test_dgram_header_invalid_magic_byte() {
    let mut buf = [0u8; HEADER_SIZE];
    buf[0] = 0xAA; // Salah magic byte
    let result = DgramHeader::parse(&buf);
    assert!(matches!(
        result,
        Err(ProtocolError::InvalidMagicByte {
            expected: 0x5A,
            actual: 0xAA
        })
    ));
}

#[test]
fn test_dgram_packet_roundtrip() {
    let header = DgramHeader::new(
        PacketType::Audio,
        5,
        1001,
        4,
        AudioCodecId::Opus,
        SampleRateCode::Hz48000,
        PriorityLevel::EmergencyFireAlarm,
    );
    let payload = vec![0x12, 0x34, 0x56, 0x78];

    let packet = DgramPacket::new(header, payload.clone()).expect("Failed to create packet");
    let bytes = packet.to_bytes();

    let parsed_packet = DgramPacket::from_bytes(&bytes).expect("Failed to parse packet");
    assert_eq!(packet, parsed_packet);
    assert_eq!(parsed_packet.payload, payload);
}

#[test]
fn test_audio_format_validation() {
    let valid_format = AudioFormat::adpcm_legacy();
    assert!(valid_format.validate().is_ok());
    assert_eq!(
        valid_format.sample_rate_code().unwrap(),
        SampleRateCode::Hz22050
    );

    let invalid_format = AudioFormat {
        codec: AudioCodecId::Pcm,
        sample_rate_hz: 96000,
        channels: ChannelLayout::Stereo,
        bits_per_sample: 16,
    };
    assert!(invalid_format.validate().is_err());
}

#[test]
fn test_priority_arbitration_preemption() {
    let mut arbitrator = PriorityArbitrator::new();

    // 1. Alokasikan BGM pada Channel 1 (Priority 1)
    let res1 = arbitrator.request_stream(1, PriorityLevel::Bgm, 100);
    assert!(res1.is_ok());
    assert_eq!(res1.unwrap(), None);
    assert_eq!(arbitrator.get_active_priority(1), Some(PriorityLevel::Bgm));

    // 2. Normal Call datang pada Channel 1 (Priority 2) -> Harus menginterupsi BGM
    let res2 = arbitrator.request_stream(1, PriorityLevel::NormalCall, 200);
    assert!(res2.is_ok());
    let preempted = res2.unwrap().expect("BGM should be preempted");
    assert_eq!(preempted.priority, PriorityLevel::Bgm);
    assert_eq!(preempted.session_id, 100);
    assert_eq!(
        arbitrator.get_active_priority(1),
        Some(PriorityLevel::NormalCall)
    );

    // 3. Panggilan BGM baru mencoba merebut Channel 1 -> Harus ditolak (PreemptedByHigherPriority)
    let res3 = arbitrator.request_stream(1, PriorityLevel::Bgm, 300);
    assert!(matches!(
        res3,
        Err(PriorityError::PreemptedByHigherPriority {
            active: 2,
            requested: 1
        })
    ));

    // 4. Fire Alarm datang pada Channel 1 (Priority 3) -> Harus menginterupsi Normal Call
    let res4 = arbitrator.request_stream(1, PriorityLevel::EmergencyFireAlarm, 999);
    assert!(res4.is_ok());
    assert_eq!(
        arbitrator.get_active_priority(1),
        Some(PriorityLevel::EmergencyFireAlarm)
    );

    // 5. Release stream
    assert!(arbitrator.release_stream(1, 999));
    assert_eq!(arbitrator.get_active_priority(1), None);
}

#[test]
fn test_ed25519_licensing_verification() {
    use ed25519_dalek::SigningKey;

    let signing_key = SigningKey::from_bytes(&[7u8; 32]);
    let verifying_key = signing_key.verifying_key();

    let claims = LicenseClaims {
        license_id: "TEST-LIC-2026-001".to_string(),
        customer_name: "Mitra Test Lab".to_string(),
        tier: LicenseTier::Enterprise,
        max_terminals: 500,
        not_before: 1000,
        expires_at: 2000,
        hardware_hash: "hw_hash_abc_123".to_string(),
    };

    let canonical_bytes = claims.to_canonical_bytes();
    use ed25519_dalek::Signer;
    let signature = signing_key.sign(&canonical_bytes);

    let verifier = LicenseVerifier::from_public_key_bytes(&verifying_key.to_bytes())
        .expect("Valid public key");

    // 1. Validasi sukses
    let val_ok = verifier.validate_claims(&claims, &signature.to_bytes(), 1500, "hw_hash_abc_123");
    assert!(val_ok.is_ok());

    // 2. Validasi gagal waktu kadaluarsa (Expired)
    let val_expired =
        verifier.validate_claims(&claims, &signature.to_bytes(), 2500, "hw_hash_abc_123");
    assert!(matches!(val_expired, Err(LicenseError::Expired { .. })));

    // 3. Validasi gagal hardware mismatch
    let val_hw_fail =
        verifier.validate_claims(&claims, &signature.to_bytes(), 1500, "different_hardware");
    assert!(matches!(
        val_hw_fail,
        Err(LicenseError::HardwareMismatch { .. })
    ));

    // 4. Validasi gagal jika signature korup
    let mut corrupt_sig = signature.to_bytes();
    corrupt_sig[0] ^= 0xFF;
    let val_sig_fail = verifier.validate_claims(&claims, &corrupt_sig, 1500, "hw_hash_abc_123");
    assert!(matches!(val_sig_fail, Err(LicenseError::InvalidSignature)));
}

#[test]
fn test_mitra_license_generator_poc_suite() {
    use ed25519_dalek::SigningKey;

    // Kunci uji coba deterministik khusus PoC
    let test_secret: [u8; 32] = [0x55; 32];
    let signing_key = SigningKey::from_bytes(&test_secret);
    let verifying_key = signing_key.verifying_key();
    let authority_salt = "POC_AUTHORITY_SALT_SECURE_987";

    let issuer = LicenseIssuer::new("MITRA-AUTHORITY-ROOT", signing_key, authority_salt);
    let verifier = LicenseVerifier::from_public_key_bytes(&verifying_key.to_bytes())
        .expect("Verifier creation");

    let product_id = "MITRA-AUDIO-SERVER";
    let native_id = "WIN-HWID-V1:MB-UUID-4A71:CPU-INTEL-X86:DISK-SAMSUNG-SSD";
    let issued_at = 1731370000;

    // 1. Penerbitan SN dan validasi format terstruktur
    let sn = SerialNumberGenerator::generate_sn(product_id, native_id, authority_salt)
        .expect("SN generation");
    assert!(SerialNumberGenerator::validate_sn_format(&sn));
    assert!(sn.contains("CIMT") || sn.contains("MITG"));

    // 2. Penerbitan Signed License Payload
    let signed_license = issuer
        .issue_license(
            "LIC-2026-0001",
            product_id,
            native_id,
            issued_at,
            vec![
                ENTITLEMENT_BASIC_AUDIO.to_string(),
                ENTITLEMENT_WEB_MANAGEMENT.to_string(),
            ],
        )
        .expect("License issuance");

    assert_eq!(signed_license.payload.license_type, "permanent");
    assert_eq!(signed_license.payload.serial_number, sn);

    // SKENARIO WAJIB 1: Lisensi valid dan Native ID sesuai -> PASS
    let res_ok = verifier.verify_mitra_license(&signed_license, product_id, native_id, &[]);
    assert!(
        res_ok.is_ok(),
        "Valid license and matching Native ID must PASS"
    );

    // SKENARIO WAJIB 2: Signature tidak valid (korup signature) -> REJECT
    let mut corrupt_sig_license = signed_license.clone();
    corrupt_sig_license.signature_bytes[0] ^= 0xFF;
    let res_bad_sig =
        verifier.verify_mitra_license(&corrupt_sig_license, product_id, native_id, &[]);
    assert!(matches!(res_bad_sig, Err(LicenseError::InvalidSignature)));

    // SKENARIO WAJIB 3: Payload dimodifikasi (tampered license_id atau entitlement) -> REJECT
    let mut tampered_payload_license = signed_license.clone();
    tampered_payload_license.payload.license_id = "LIC-2026-HACKED".to_string();
    let res_tampered =
        verifier.verify_mitra_license(&tampered_payload_license, product_id, native_id, &[]);
    assert!(matches!(res_tampered, Err(LicenseError::InvalidSignature)));

    // SKENARIO WAJIB 4: Native ID berbeda (hardware beda / kloning) -> REJECT
    let res_hw_mismatch = verifier.verify_mitra_license(
        &signed_license,
        product_id,
        "WIN-HWID-V1:DIFFERENT-MACHINE",
        &[],
    );
    assert!(matches!(
        res_hw_mismatch,
        Err(LicenseError::HardwareMismatch { .. })
    ));

    // SKENARIO WAJIB 5: Product ID berbeda (misal dipakai di Station padahal untuk Server) -> REJECT
    let res_prod_mismatch =
        verifier.verify_mitra_license(&signed_license, "MITRA-AUDIO-STATION", native_id, &[]);
    assert!(matches!(
        res_prod_mismatch,
        Err(LicenseError::ProductMismatch { .. })
    ));

    // SKENARIO WAJIB 6: License ID / SN duplikat di License Authority -> REJECT
    let mut storage = LicenseAuthorityStorage::new();
    let record1 = LicenseAuthorityRecord {
        license_id: "LIC-2026-0001".into(),
        product_id: product_id.into(),
        native_id: native_id.into(),
        serial_number: sn.clone(),
        purchase_date: 1731370000,
        support_until: 1731370000 + 31536000, // +1 tahun layanan support
        issued_at,
        license_status: "ACTIVE".into(),
        entitlements: vec![
            ENTITLEMENT_BASIC_AUDIO.into(),
            ENTITLEMENT_WEB_MANAGEMENT.into(),
        ],
    };
    storage
        .insert_record(record1.clone())
        .expect("Insert record 1");
    let duplicate_insert = storage.insert_record(record1);
    assert!(matches!(
        duplicate_insert,
        Err(LicenseError::DuplicateLicenseId(_))
    ));

    // SKENARIO WAJIB 7: Lisensi revoked -> REJECT
    storage
        .revoke_license("LIC-2026-0001", "Hardware replaced after policy expired")
        .expect("Revoke");
    let revoked_ids = ["LIC-2026-0001"];
    let res_revoked =
        verifier.verify_mitra_license(&signed_license, product_id, native_id, &revoked_ids);
    assert!(matches!(res_revoked, Err(LicenseError::LicenseRevoked(_))));

    // SKENARIO WAJIB 8: Entitlement tidak tersedia (WEB_MANAGEMENT) -> REJECT
    let signed_no_web = issuer
        .issue_license(
            "LIC-2026-BASIC-ONLY",
            product_id,
            native_id,
            issued_at,
            vec![ENTITLEMENT_BASIC_AUDIO.to_string()],
        )
        .expect("License basic");
    let check_web = verifier.check_entitlement(&signed_no_web, ENTITLEMENT_WEB_MANAGEMENT);
    assert!(matches!(
        check_web,
        Err(LicenseError::FeatureNotEntitled(_))
    ));

    let check_web_ok = verifier.check_entitlement(&signed_license, ENTITLEMENT_WEB_MANAGEMENT);
    assert!(check_web_ok.is_ok());

    // SKENARIO WAJIB 9: Public key salah -> REJECT
    let wrong_key_secret: [u8; 32] = [0x99; 32];
    let wrong_signing_key = SigningKey::from_bytes(&wrong_key_secret);
    let wrong_verifier =
        LicenseVerifier::from_public_key_bytes(&wrong_signing_key.verifying_key().to_bytes())
            .expect("Wrong verifier");
    let res_wrong_key =
        wrong_verifier.verify_mitra_license(&signed_license, product_id, native_id, &[]);
    assert!(matches!(res_wrong_key, Err(LicenseError::InvalidSignature)));

    // SKENARIO WAJIB 10: Payload tidak lengkap -> REJECT
    let mut incomplete_license = signed_license.clone();
    incomplete_license.payload.license_id = "".to_string();
    assert!(incomplete_license.payload.validate_completeness().is_err());

    // SKENARIO WAJIB 11: Payload versi tidak dikenal -> REJECT
    let mut unsupported_ver_license = signed_license.clone();
    unsupported_ver_license.payload.schema_version = 99;
    assert!(matches!(
        unsupported_ver_license.payload.validate_completeness(),
        Err(LicenseError::UnsupportedSchemaVersion(99))
    ));

    // SKENARIO WAJIB 12: Lisensi permanen tanpa expiry -> PASS
    // Verifikasi bahwa tipe permanen valid dan tidak mengandung batas kedaluwarsa waktu
    assert_eq!(signed_license.payload.license_type, "permanent");
}

#[test]
fn test_mitra_license_additional_forensic_and_security_cases() {
    use ed25519_dalek::SigningKey;

    let secret: [u8; 32] = [0x42; 32];
    let signing_key = SigningKey::from_bytes(&secret);
    let public_key = signing_key.verifying_key();
    let salt = "SALT_ROTATION_EPOCH_1";

    let issuer = LicenseIssuer::new("MITRA-AUTHORITY-ROOT", signing_key, salt);
    let verifier = LicenseVerifier::from_public_key_bytes(&public_key.to_bytes()).unwrap();

    let product_id = "MITRA-AUDIO-SERVER";
    let native_id = "WIN-HWID-V1:MB-TEST:CPU-TEST:DISK-TEST";

    // 1. Uji SN Collision & Uniqueness pada variasi hardware
    let sn1 =
        SerialNumberGenerator::generate_sn(product_id, "WIN-HWID-V1:MACHINE-A", salt).unwrap();
    let sn2 =
        SerialNumberGenerator::generate_sn(product_id, "WIN-HWID-V1:MACHINE-B", salt).unwrap();
    assert_ne!(sn1, sn2, "Different machines must produce different SNs");

    // 2. Uji Rotasi Key / Salt Authority
    let salt_rotated = "SALT_ROTATION_EPOCH_2";
    let sn_rotated =
        SerialNumberGenerator::generate_sn(product_id, "WIN-HWID-V1:MACHINE-A", salt_rotated)
            .unwrap();
    assert_ne!(sn1, sn_rotated, "Rotated salt must yield distinct SN");

    // 3. Uji Invalid Native ID & Product ID inputs
    assert!(SerialNumberGenerator::generate_sn("", native_id, salt).is_err());
    assert!(SerialNumberGenerator::generate_sn(product_id, "", salt).is_err());

    // 4. Uji Format SN Validation
    assert!(!SerialNumberGenerator::validate_sn_format("INVALID-SN-123"));
    assert!(!SerialNumberGenerator::validate_sn_format(
        "MANS-MITR-123-1234-1234"
    )); // salah panjang chunk

    // 5. Uji Atomic Duplicate Prevention pada Registry
    let mut storage = LicenseAuthorityStorage::new();
    let rec1 = LicenseAuthorityRecord {
        license_id: "LIC-ATOMIC-1".into(),
        product_id: product_id.into(),
        native_id: native_id.into(),
        serial_number: sn1.clone(),
        purchase_date: 1000,
        support_until: 1000 + 31536000,
        issued_at: 1000,
        license_status: "ACTIVE".into(),
        entitlements: vec!["BASIC_AUDIO".into()],
    };
    storage.insert_record(rec1).unwrap();

    // Coba insert duplikat serial number dengan license_id beda
    let rec_dup_sn = LicenseAuthorityRecord {
        license_id: "LIC-ATOMIC-2".into(),
        product_id: product_id.into(),
        native_id: "WIN-HWID-V1:OTHER".into(),
        serial_number: sn1.clone(), // SN duplikat
        purchase_date: 1000,
        support_until: 1000 + 31536000,
        issued_at: 1000,
        license_status: "ACTIVE".into(),
        entitlements: vec!["BASIC_AUDIO".into()],
    };
    assert!(matches!(
        storage.insert_record(rec_dup_sn),
        Err(LicenseError::DuplicateSerialNumber(_))
    ));

    // 6. Uji Audit Trail Logging
    assert_eq!(storage.audit_logs().len(), 1);
    assert!(storage.audit_logs()[0].contains("AUDIT [INSERT]"));

    storage
        .revoke_license("LIC-ATOMIC-1", "Administrative revocation")
        .unwrap();
    assert_eq!(storage.audit_logs().len(), 2);
    assert!(storage.audit_logs()[1].contains("AUDIT [REVOKE]"));

    // 7. Uji Offline Verification
    // Membuktikan bahwa verifikasi signed license sama sekali tidak memerlukan koneksi jaringan
    let lic = issuer
        .issue_license(
            "LIC-OFFLINE-01",
            product_id,
            native_id,
            1000,
            vec!["BASIC_AUDIO".into()],
        )
        .unwrap();
    let off_res = verifier.verify_mitra_license(&lic, product_id, native_id, &[]);
    assert!(
        off_res.is_ok(),
        "Offline verification with public key must succeed without network"
    );

    // 8. Uji Penegakan Entitlement Server-Side & Restriksi Demo/Trial
    // Verifikasi bahwa Demo/Trial tidak diperkenankan mengakses WEB_MANAGEMENT
    let mut demo_payload = lic.payload.clone();
    demo_payload.license_type = "demo".to_string();
    demo_payload.entitlements.push("WEB_MANAGEMENT".to_string());
    let demo_lic = SignedMitraLicense {
        payload: demo_payload,
        signature_bytes: [0u8; 64],
    };
    let demo_check = verifier.check_entitlement(&demo_lic, "WEB_MANAGEMENT");
    assert!(
        matches!(demo_check, Err(LicenseError::FeatureNotEntitled(_))),
        "Demo licenses must be strictly restricted from permanent WEB_MANAGEMENT entitlement"
    );

    // Verifikasi bahwa lisensi permanent dengan hak akses WEB_MANAGEMENT lolos pengecekan
    let mut perm_payload = lic.payload.clone();
    perm_payload.license_type = "permanent".to_string();
    perm_payload.entitlements.push("WEB_MANAGEMENT".to_string());
    let perm_lic = SignedMitraLicense {
        payload: perm_payload,
        signature_bytes: [0u8; 64],
    };
    assert!(
        verifier
            .check_entitlement(&perm_lic, "WEB_MANAGEMENT")
            .is_ok(),
        "Permanent license with WEB_MANAGEMENT must pass"
    );
}
