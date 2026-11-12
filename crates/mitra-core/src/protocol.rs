//! Mitra Binary Network Protocol Implementation
//!
//! Mengimplementasikan struktur biner fixed 16-byte `tagDGRAMHEADER` legacy.
//! Sesuai Batch 2A (Protocol Compatibility Correction):
//! - Format wire legacy 16-byte dipertahankan eksak tanpa field ekstra.
//! - Offset 0x0E..0x0F diperlakukan secara konservatif sebagai `reserved_tail` (u16 little-endian).
//! - Integrity checksum internal dipisahkan dan bersifat opsional sehingga paket legacy
//!   yang dikirim dengan tail nol atau format tanpa checksum tidak ditolak secara keliru.

use crate::error::ProtocolError;

pub const MAGIC_SYNC_BYTE: u8 = 0x5A;
pub const HEADER_SIZE: usize = 16;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PacketType {
    Audio = 0x01,
    Control = 0x02,
    Heartbeat = 0x03,
    EmergencyAlarm = 0x04,
}

impl TryFrom<u8> for PacketType {
    type Error = ProtocolError;

    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0x01 => Ok(PacketType::Audio),
            0x02 => Ok(PacketType::Control),
            0x03 => Ok(PacketType::Heartbeat),
            0x04 => Ok(PacketType::EmergencyAlarm),
            other => Err(ProtocolError::InvalidPacketType(other)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum AudioCodecId {
    Pcm = 0x01,
    ImaAdpcm = 0x02,
    Mp3 = 0x03,
    Opus = 0x04,
}

impl TryFrom<u8> for AudioCodecId {
    type Error = ProtocolError;

    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0x01 => Ok(AudioCodecId::Pcm),
            0x02 => Ok(AudioCodecId::ImaAdpcm),
            0x03 => Ok(AudioCodecId::Mp3),
            0x04 => Ok(AudioCodecId::Opus),
            other => Err(ProtocolError::InvalidCodecId(other)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SampleRateCode {
    Hz11025 = 0x01,
    Hz22050 = 0x02,
    Hz44100 = 0x03,
    Hz48000 = 0x04,
}

impl TryFrom<u8> for SampleRateCode {
    type Error = ProtocolError;

    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0x01 => Ok(SampleRateCode::Hz11025),
            0x02 => Ok(SampleRateCode::Hz22050),
            0x03 => Ok(SampleRateCode::Hz44100),
            0x04 => Ok(SampleRateCode::Hz48000),
            other => Err(ProtocolError::InvalidSampleRateCode(other)),
        }
    }
}

impl SampleRateCode {
    pub fn to_hz(self) -> u32 {
        match self {
            SampleRateCode::Hz11025 => 11025,
            SampleRateCode::Hz22050 => 22050,
            SampleRateCode::Hz44100 => 44100,
            SampleRateCode::Hz48000 => 48000,
        }
    }

    pub fn from_hz(hz: u32) -> Result<Self, ProtocolError> {
        match hz {
            11025 => Ok(SampleRateCode::Hz11025),
            22050 => Ok(SampleRateCode::Hz22050),
            44100 => Ok(SampleRateCode::Hz44100),
            48000 => Ok(SampleRateCode::Hz48000),
            _ => Err(ProtocolError::InvalidSampleRateCode(0)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PriorityLevel {
    Bgm = 0x01,
    NormalCall = 0x02,
    EmergencyFireAlarm = 0x03,
}

impl TryFrom<u8> for PriorityLevel {
    type Error = ProtocolError;

    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0x01 => Ok(PriorityLevel::Bgm),
            0x02 => Ok(PriorityLevel::NormalCall),
            0x03 => Ok(PriorityLevel::EmergencyFireAlarm),
            other => Err(ProtocolError::InvalidPriority(other)),
        }
    }
}

/// Struktur biner representasi `tagDGRAMHEADER` tepat 16 byte
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DgramHeader {
    pub sync_byte: u8,
    pub packet_type: PacketType,
    pub channel_id: u16,
    pub sequence_number: u32,
    pub payload_size: u16,
    pub audio_codec: AudioCodecId,
    pub sample_rate_code: SampleRateCode,
    pub priority: PriorityLevel,
    pub reserved_pad: u8,
    pub reserved_tail: u16,
}

impl DgramHeader {
    /// Membuat instance header baru untuk wire protocol legacy 16 byte
    pub fn new(
        packet_type: PacketType,
        channel_id: u16,
        sequence_number: u32,
        payload_size: u16,
        audio_codec: AudioCodecId,
        sample_rate_code: SampleRateCode,
        priority: PriorityLevel,
    ) -> Self {
        Self {
            sync_byte: MAGIC_SYNC_BYTE,
            packet_type,
            channel_id,
            sequence_number,
            payload_size,
            audio_codec,
            sample_rate_code,
            priority,
            reserved_pad: 0x00,
            reserved_tail: 0x0000,
        }
    }

    /// Menghitung additive 16-bit checksum atas 14 byte pertama header (opsional)
    pub fn compute_checksum(&self) -> u16 {
        let mut bytes = [0u8; 14];
        bytes[0] = self.sync_byte;
        bytes[1] = self.packet_type as u8;
        bytes[2..4].copy_from_slice(&self.channel_id.to_le_bytes());
        bytes[4..8].copy_from_slice(&self.sequence_number.to_le_bytes());
        bytes[8..10].copy_from_slice(&self.payload_size.to_le_bytes());
        bytes[10] = self.audio_codec as u8;
        bytes[11] = self.sample_rate_code as u8;
        bytes[12] = self.priority as u8;
        bytes[13] = self.reserved_pad;

        let mut sum: u32 = 0;
        for &b in &bytes {
            sum += b as u32;
        }
        (sum & 0xFFFF) as u16
    }

    /// Menyisipkan checksum opsional ke field reserved_tail
    pub fn with_computed_checksum(mut self) -> Self {
        self.reserved_tail = self.compute_checksum();
        self
    }

    /// Verifikasi apakah reserved_tail cocok dengan checksum yang dihitung
    pub fn is_checksum_valid(&self) -> bool {
        self.reserved_tail == self.compute_checksum()
    }

    /// Serialize header ke dalam buffer 16 byte tepat
    pub fn serialize(&self, out: &mut [u8; HEADER_SIZE]) {
        out[0] = self.sync_byte;
        out[1] = self.packet_type as u8;
        out[2..4].copy_from_slice(&self.channel_id.to_le_bytes());
        out[4..8].copy_from_slice(&self.sequence_number.to_le_bytes());
        out[8..10].copy_from_slice(&self.payload_size.to_le_bytes());
        out[10] = self.audio_codec as u8;
        out[11] = self.sample_rate_code as u8;
        out[12] = self.priority as u8;
        out[13] = self.reserved_pad;
        out[14..16].copy_from_slice(&self.reserved_tail.to_le_bytes());
    }

    /// Parse dan validasi buffer biner header 16 byte
    /// Konservatif: Tidak menolak paket legacy jika reserved_tail tidak berisi checksum
    pub fn parse(buf: &[u8]) -> Result<Self, ProtocolError> {
        if buf.len() < HEADER_SIZE {
            return Err(ProtocolError::HeaderTooShort {
                expected: HEADER_SIZE,
                actual: buf.len(),
            });
        }

        let sync_byte = buf[0];
        if sync_byte != MAGIC_SYNC_BYTE {
            return Err(ProtocolError::InvalidMagicByte {
                expected: MAGIC_SYNC_BYTE,
                actual: sync_byte,
            });
        }

        let packet_type = PacketType::try_from(buf[1])?;
        let channel_id = u16::from_le_bytes([buf[2], buf[3]]);
        let sequence_number = u32::from_le_bytes([buf[4], buf[5], buf[6], buf[7]]);
        let payload_size = u16::from_le_bytes([buf[8], buf[9]]);
        let audio_codec = AudioCodecId::try_from(buf[10])?;
        let sample_rate_code = SampleRateCode::try_from(buf[11])?;
        let priority = PriorityLevel::try_from(buf[12])?;
        let reserved_pad = buf[13];
        let reserved_tail = u16::from_le_bytes([buf[14], buf[15]]);

        Ok(Self {
            sync_byte,
            packet_type,
            channel_id,
            sequence_number,
            payload_size,
            audio_codec,
            sample_rate_code,
            priority,
            reserved_pad,
            reserved_tail,
        })
    }
}

/// Struktur paket lengkap (Header + Audio Payload)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DgramPacket {
    pub header: DgramHeader,
    pub payload: Vec<u8>,
}

impl DgramPacket {
    pub fn new(header: DgramHeader, payload: Vec<u8>) -> Result<Self, ProtocolError> {
        if payload.len() != header.payload_size as usize {
            return Err(ProtocolError::PayloadLengthMismatch {
                expected: header.payload_size as usize,
                actual: payload.len(),
            });
        }
        Ok(Self { header, payload })
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(HEADER_SIZE + self.payload.len());
        let mut header_bytes = [0u8; HEADER_SIZE];
        self.header.serialize(&mut header_bytes);
        bytes.extend_from_slice(&header_bytes);
        bytes.extend_from_slice(&self.payload);
        bytes
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, ProtocolError> {
        let header = DgramHeader::parse(bytes)?;
        let payload_start = HEADER_SIZE;
        let payload_end = payload_start + header.payload_size as usize;

        if bytes.len() < payload_end {
            return Err(ProtocolError::PayloadLengthMismatch {
                expected: header.payload_size as usize,
                actual: bytes.len() - payload_start,
            });
        }

        let payload = bytes[payload_start..payload_end].to_vec();
        Ok(Self { header, payload })
    }
}
