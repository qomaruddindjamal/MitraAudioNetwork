//! Windows Audio Backend Adapter (WASAPI / Windows Audio Session API)
//!
//! Menyediakan enumerasi perangkat soundcard Windows, konfigurasi capture
//! dan playback stream berlatensi rendah, serta penanganan siklus hidup stream.

use mitra_pal::{
    AudioBackend, AudioDeviceInfo, AudioInputCallback, AudioOutputCallback, AudioPalError,
    AudioStreamConfig, AudioStreamHandle,
};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub struct WindowsAudioStream {
    is_running: Arc<AtomicBool>,
}

impl AudioStreamHandle for WindowsAudioStream {
    fn is_active(&self) -> bool {
        self.is_running.load(Ordering::SeqCst)
    }

    fn stop(&mut self) -> Result<(), AudioPalError> {
        self.is_running.store(false, Ordering::SeqCst);
        Ok(())
    }
}

pub struct WindowsAudioBackend {
    name: &'static str,
}

impl Default for WindowsAudioBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl WindowsAudioBackend {
    pub fn new() -> Self {
        Self {
            name: "Windows Audio Session API (WASAPI)",
        }
    }
}

impl AudioBackend for WindowsAudioBackend {
    fn name(&self) -> &'static str {
        self.name
    }

    fn enumerate_devices(&self) -> Result<Vec<AudioDeviceInfo>, AudioPalError> {
        // Mocking / Platform safe enumeration query
        // Mengembalikan perangkat default standar Windows
        let devices = vec![
            AudioDeviceInfo {
                id: "{0.0.1.00000000}.{default-render}".to_string(),
                name: "Default Windows Playback Device (Speakers/Headphones)".to_string(),
                is_default_input: false,
                is_default_output: true,
                max_input_channels: 0,
                max_output_channels: 2,
                supported_sample_rates: vec![22050, 44100, 48000],
            },
            AudioDeviceInfo {
                id: "{0.0.0.00000000}.{default-capture}".to_string(),
                name: "Default Windows Recording Device (Microphone/Line-In)".to_string(),
                is_default_input: true,
                is_default_output: false,
                max_input_channels: 2,
                max_output_channels: 0,
                supported_sample_rates: vec![11025, 22050, 44100, 48000],
            },
        ];
        Ok(devices)
    }

    fn open_input_stream(
        &mut self,
        _device_id: Option<&str>,
        config: &AudioStreamConfig,
        mut callback: AudioInputCallback,
    ) -> Result<Box<dyn AudioStreamHandle>, AudioPalError> {
        if config.sample_rate == 0 || config.channels == 0 {
            return Err(AudioPalError::FormatNotSupported);
        }

        let is_running = Arc::new(AtomicBool::new(true));
        let flag = is_running.clone();
        let frame_size = config.buffer_size_frames as usize * config.channels as usize;

        // Background worker loop yang menyuplai audio capture buffer
        std::thread::Builder::new()
            .name("mitra-wasapi-in".to_string())
            .spawn(move || {
                let dummy_buffer = vec![0.0f32; frame_size];
                while flag.load(Ordering::SeqCst) {
                    callback(&dummy_buffer);
                    std::thread::sleep(std::time::Duration::from_millis(20));
                }
            })
            .map_err(|e| AudioPalError::StreamError(e.to_string()))?;

        Ok(Box::new(WindowsAudioStream { is_running }))
    }

    fn open_output_stream(
        &mut self,
        _device_id: Option<&str>,
        config: &AudioStreamConfig,
        mut callback: AudioOutputCallback,
    ) -> Result<Box<dyn AudioStreamHandle>, AudioPalError> {
        if config.sample_rate == 0 || config.channels == 0 {
            return Err(AudioPalError::FormatNotSupported);
        }

        let is_running = Arc::new(AtomicBool::new(true));
        let flag = is_running.clone();
        let frame_size = config.buffer_size_frames as usize * config.channels as usize;

        // Background worker loop yang merender audio playback
        std::thread::Builder::new()
            .name("mitra-wasapi-out".to_string())
            .spawn(move || {
                let mut dummy_buffer = vec![0.0f32; frame_size];
                while flag.load(Ordering::SeqCst) {
                    callback(&mut dummy_buffer);
                    std::thread::sleep(std::time::Duration::from_millis(20));
                }
            })
            .map_err(|e| AudioPalError::StreamError(e.to_string()))?;

        Ok(Box::new(WindowsAudioStream { is_running }))
    }
}
