//! Mitra Core Error System

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MitraError {
    Protocol(ProtocolError),
    Audio(AudioError),
    Priority(PriorityError),
    License(LicenseError),
    Validation(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProtocolError {
    HeaderTooShort { expected: usize, actual: usize },
    InvalidMagicByte { expected: u8, actual: u8 },
    InvalidPacketType(u8),
    InvalidCodecId(u8),
    InvalidSampleRateCode(u8),
    InvalidPriority(u8),
    ChecksumMismatch { calculated: u16, header: u16 },
    PayloadLengthMismatch { expected: usize, actual: usize },
    BufferTooSmall { required: usize, provided: usize },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AudioError {
    UnsupportedSampleRate(u32),
    UnsupportedChannels(u16),
    InvalidCodecForTarget(String),
    BufferUnderflow,
    BufferOverflow,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PriorityError {
    PreemptedByHigherPriority { active: u8, requested: u8 },
    ChannelBusy(u16),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LicenseError {
    InvalidSignature,
    MalformedKey,
    Expired {
        expired_at: u64,
        current_time: u64,
    },
    NotYetValid {
        not_before: u64,
        current_time: u64,
    },
    HardwareMismatch {
        expected_hash: String,
        actual_hash: String,
    },
    ProductMismatch {
        expected: String,
        actual: String,
    },
    DuplicateSerialNumber(String),
    DuplicateLicenseId(String),
    LicenseRevoked(String),
    IncompletePayload(String),
    UnsupportedSchemaVersion(u32),
    InvalidSerialNumberFormat,
    InvalidNativeIdFormat,
    CapacityExceeded {
        max_allowed: u32,
        requested: u32,
    },
    FeatureNotEntitled(String),
    ClockRollbackDetected,
}

impl From<ProtocolError> for MitraError {
    fn from(err: ProtocolError) -> Self {
        MitraError::Protocol(err)
    }
}

impl From<AudioError> for MitraError {
    fn from(err: AudioError) -> Self {
        MitraError::Audio(err)
    }
}

impl From<PriorityError> for MitraError {
    fn from(err: PriorityError) -> Self {
        MitraError::Priority(err)
    }
}

impl From<LicenseError> for MitraError {
    fn from(err: LicenseError) -> Self {
        MitraError::License(err)
    }
}
