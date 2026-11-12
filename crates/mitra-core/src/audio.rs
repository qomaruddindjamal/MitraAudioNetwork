//! Mitra Audio Format Model & Codec Abstraction

use crate::error::AudioError;
use crate::protocol::{AudioCodecId, SampleRateCode};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChannelLayout {
    Mono = 1,
    Stereo = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AudioFormat {
    pub codec: AudioCodecId,
    pub sample_rate_hz: u32,
    pub channels: ChannelLayout,
    pub bits_per_sample: u16,
}

impl AudioFormat {
    pub fn pcm_default() -> Self {
        Self {
            codec: AudioCodecId::Pcm,
            sample_rate_hz: 22050,
            channels: ChannelLayout::Mono,
            bits_per_sample: 16,
        }
    }

    pub fn adpcm_legacy() -> Self {
        Self {
            codec: AudioCodecId::ImaAdpcm,
            sample_rate_hz: 22050,
            channels: ChannelLayout::Mono,
            bits_per_sample: 4,
        }
    }

    pub fn opus_modern() -> Self {
        Self {
            codec: AudioCodecId::Opus,
            sample_rate_hz: 48000,
            channels: ChannelLayout::Stereo,
            bits_per_sample: 16,
        }
    }

    pub fn validate(&self) -> Result<(), AudioError> {
        match self.sample_rate_hz {
            11025 | 22050 | 44100 | 48000 => (),
            hz => return Err(AudioError::UnsupportedSampleRate(hz)),
        }

        match self.channels {
            ChannelLayout::Mono | ChannelLayout::Stereo => (),
        }

        Ok(())
    }

    pub fn sample_rate_code(&self) -> Result<SampleRateCode, AudioError> {
        SampleRateCode::from_hz(self.sample_rate_hz)
            .map_err(|_| AudioError::UnsupportedSampleRate(self.sample_rate_hz))
    }
}
