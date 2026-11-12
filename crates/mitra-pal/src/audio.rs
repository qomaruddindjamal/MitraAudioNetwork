//! Trait Abstraksi Perangkat Audio

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AudioDeviceInfo {
    pub id: String,
    pub name: String,
    pub is_default_input: bool,
    pub is_default_output: bool,
    pub max_input_channels: u16,
    pub max_output_channels: u16,
    pub supported_sample_rates: Vec<u32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AudioStreamConfig {
    pub sample_rate: u32,
    pub channels: u16,
    pub buffer_size_frames: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AudioPalError {
    DeviceNotFound(String),
    FormatNotSupported,
    DeviceInUse,
    BackendUnavailable(String),
    StreamError(String),
}

pub trait AudioStreamHandle: Send + Sync {
    fn is_active(&self) -> bool;
    fn stop(&mut self) -> Result<(), AudioPalError>;
}

pub type AudioInputCallback = Box<dyn FnMut(&[f32]) + Send + 'static>;
pub type AudioOutputCallback = Box<dyn FnMut(&mut [f32]) + Send + 'static>;

pub trait AudioBackend: Send + Sync {
    fn name(&self) -> &'static str;
    fn enumerate_devices(&self) -> Result<Vec<AudioDeviceInfo>, AudioPalError>;
    fn open_input_stream(
        &mut self,
        device_id: Option<&str>,
        config: &AudioStreamConfig,
        callback: AudioInputCallback,
    ) -> Result<Box<dyn AudioStreamHandle>, AudioPalError>;
    fn open_output_stream(
        &mut self,
        device_id: Option<&str>,
        config: &AudioStreamConfig,
        callback: AudioOutputCallback,
    ) -> Result<Box<dyn AudioStreamHandle>, AudioPalError>;
}
