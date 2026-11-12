//! Trait Abstraksi Identitas Perangkat Keras

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HardwareTokenType {
    MotherboardUuid,
    CpuProcessorId,
    SystemDriveSerial,
    MacAddress,
    MachineIdFile,
    AslKernelMachineGuid,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HardwareToken {
    pub token_type: HardwareTokenType,
    pub raw_value: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HardwarePalError {
    PermissionDenied,
    QueryFailed(String),
    PlatformUnsupported(String),
}

pub trait HardwareIdProvider: Send + Sync {
    fn query_hardware_tokens(&self) -> Result<Vec<HardwareToken>, HardwarePalError>;
}

/// Kontrak Abstraksi Cryptographically Secure Pseudo-Random Number Generator (CSPRNG)
pub trait SecureRandomProvider: Send + Sync {
    /// Mengisi buffer byte acak dari sumber entropi kriptografis sistem operasi
    fn fill_random_bytes(&self, dest: &mut [u8]) -> Result<(), HardwarePalError>;
}
