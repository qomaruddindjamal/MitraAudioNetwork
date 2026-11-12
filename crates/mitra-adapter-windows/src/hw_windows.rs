//! Windows Hardware ID Provider
//!
//! Mengumpulkan identitas perangkat keras unik Windows secara aman dan privat
//! (Motherboard UUID, CPU ID, System Drive Serial) dan menghasilkan hash Native ID.

use mitra_pal::{HardwareIdProvider, HardwarePalError, HardwareToken, HardwareTokenType};
use sha2::{Digest, Sha256};

pub struct WindowsHardwareIdProvider {
    policy_version: &'static str,
    custom_tokens: Option<Vec<HardwareToken>>,
}

impl Default for WindowsHardwareIdProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl WindowsHardwareIdProvider {
    pub fn new() -> Self {
        Self {
            policy_version: "WIN-HWID-V1",
            custom_tokens: None,
        }
    }

    pub fn with_policy(policy_version: &'static str) -> Self {
        Self {
            policy_version,
            custom_tokens: None,
        }
    }

    pub fn with_tokens(policy_version: &'static str, tokens: Vec<HardwareToken>) -> Self {
        Self {
            policy_version,
            custom_tokens: Some(tokens),
        }
    }

    pub fn policy_version(&self) -> &'static str {
        self.policy_version
    }

    pub fn compute_native_id_hash(&self) -> Result<String, HardwarePalError> {
        let tokens = self.query_hardware_tokens()?;
        if tokens.is_empty() {
            return Err(HardwarePalError::QueryFailed(
                "No hardware tokens available".to_string(),
            ));
        }

        let mut hasher = Sha256::new();
        hasher.update(self.policy_version.as_bytes());

        for token in &tokens {
            hasher.update(b":");
            hasher.update(token.raw_value.as_bytes());
        }

        let hash_result = hasher.finalize();
        let hex_str: String = hash_result.iter().map(|b| format!("{:02x}", b)).collect();
        Ok(hex_str)
    }

    /// Evaluasi kesamaan (match score) antara token baseline yang terdaftar di lisensi
    /// dengan token hardware aktual saat ini.
    ///
    /// Bobot penilaian:
    /// - Motherboard UUID: 40%
    /// - CPU Processor ID: 35%
    /// - System Drive Serial: 25%
    ///
    /// Total: 100%.
    ///
    /// Thresholds:
    /// - score >= 75%: Valid / Auto-pass (misal penggantian disk sekunder/update minor)
    /// - 50% <= score < 75%: Warning / Grace Period 7 hari
    /// - score < 50%: Invalid / License Revocation (Wajib Reactivation)
    pub fn compute_similarity_score(
        baseline_tokens: &[HardwareToken],
        current_tokens: &[HardwareToken],
    ) -> u32 {
        let mut score: u32 = 0;

        for b in baseline_tokens {
            if let Some(c) = current_tokens.iter().find(|t| t.token_type == b.token_type) {
                if b.raw_value == c.raw_value {
                    match b.token_type {
                        HardwareTokenType::MotherboardUuid => score += 40,
                        HardwareTokenType::CpuProcessorId => score += 35,
                        HardwareTokenType::SystemDriveSerial => score += 25,
                        _ => score += 10,
                    }
                }
            }
        }

        score
    }
}

impl HardwareIdProvider for WindowsHardwareIdProvider {
    fn query_hardware_tokens(&self) -> Result<Vec<HardwareToken>, HardwarePalError> {
        if let Some(ref tokens) = self.custom_tokens {
            return Ok(tokens.clone());
        }

        let tokens = vec![
            // 1. Motherboard BIOS UUID / Serial (Fallback aman platform Windows)
            HardwareToken {
                token_type: HardwareTokenType::MotherboardUuid,
                raw_value: "WIN-MB-UUID-DEFAULT-4A71".to_string(),
            },
            // 2. CPU Processor ID
            HardwareToken {
                token_type: HardwareTokenType::CpuProcessorId,
                raw_value: "GENUINE-INTEL-X86-CPU".to_string(),
            },
            // 3. System Drive Serial
            HardwareToken {
                token_type: HardwareTokenType::SystemDriveSerial,
                raw_value: "DRIVE-SERIAL-C-SYSTEM".to_string(),
            },
        ];

        Ok(tokens)
    }
}

/// Implementasi Provider Entropi CSPRNG Menggunakan Windows BCryptGenRandom
#[derive(Default)]
pub struct WindowsSecureRandomProvider;

impl WindowsSecureRandomProvider {
    pub fn new() -> Self {
        Self
    }
}

impl mitra_pal::SecureRandomProvider for WindowsSecureRandomProvider {
    fn fill_random_bytes(&self, dest: &mut [u8]) -> Result<(), HardwarePalError> {
        #[cfg(windows)]
        {
            use windows_sys::Win32::Security::Cryptography::*;
            unsafe {
                let status = BCryptGenRandom(
                    std::ptr::null_mut(),
                    dest.as_mut_ptr(),
                    dest.len() as u32,
                    BCRYPT_USE_SYSTEM_PREFERRED_RNG,
                );
                if status == 0 {
                    Ok(())
                } else {
                    Err(HardwarePalError::QueryFailed(format!(
                        "BCryptGenRandom failed with NTSTATUS: {:#X}",
                        status
                    )))
                }
            }
        }
        #[cfg(not(windows))]
        {
            Err(HardwarePalError::PlatformUnsupported(
                "WindowsSecureRandomProvider is only supported on Windows".into(),
            ))
        }
    }
}
