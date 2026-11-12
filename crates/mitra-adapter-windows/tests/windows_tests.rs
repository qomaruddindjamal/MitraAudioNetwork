use mitra_adapter_windows::*;
use mitra_pal::*;

#[test]
fn test_windows_audio_backend_enumeration() {
    let backend = WindowsAudioBackend::new();
    assert_eq!(backend.name(), "Windows Audio Session API (WASAPI)");

    let devices = backend
        .enumerate_devices()
        .expect("Enumeration should succeed");
    assert!(!devices.is_empty());
    assert!(devices.iter().any(|d| d.is_default_output));
    assert!(devices.iter().any(|d| d.is_default_input));
}

#[test]
fn test_windows_audio_stream_lifecycle() {
    let mut backend = WindowsAudioBackend::new();
    let config = AudioStreamConfig {
        sample_rate: 44100,
        channels: 2,
        buffer_size_frames: 441, // ~10ms
    };

    let mut stream = backend
        .open_output_stream(
            None,
            &config,
            Box::new(|buf| {
                for sample in buf.iter_mut() {
                    *sample = 0.0;
                }
            }),
        )
        .expect("Stream should open");

    assert!(stream.is_active());
    stream.stop().expect("Stream should stop");
    assert!(!stream.is_active());
}

#[test]
fn test_windows_hardware_id_hash() {
    let provider = WindowsHardwareIdProvider::new();
    let tokens = provider
        .query_hardware_tokens()
        .expect("Tokens query should succeed");
    assert_eq!(tokens.len(), 3);

    let hash1 = provider
        .compute_native_id_hash()
        .expect("Hash generation 1");
    let hash2 = provider
        .compute_native_id_hash()
        .expect("Hash generation 2");
    assert_eq!(hash1, hash2); // Harus deterministik dan stabil
    assert_eq!(hash1.len(), 64); // SHA-256 hex string
}

#[test]
fn test_windows_native_id_stability_and_similarity_scoring() {
    let baseline_tokens = vec![
        HardwareToken {
            token_type: HardwareTokenType::MotherboardUuid,
            raw_value: "WIN-MB-UUID-DEFAULT-4A71".to_string(),
        },
        HardwareToken {
            token_type: HardwareTokenType::CpuProcessorId,
            raw_value: "GENUINE-INTEL-X86-CPU".to_string(),
        },
        HardwareToken {
            token_type: HardwareTokenType::SystemDriveSerial,
            raw_value: "DRIVE-SERIAL-C-SYSTEM".to_string(),
        },
    ];

    // Skenario 1: 100% Identik (Reboot / Restart / Normal state)
    let score_exact =
        WindowsHardwareIdProvider::compute_similarity_score(&baseline_tokens, &baseline_tokens);
    assert_eq!(score_exact, 100);

    // Skenario 2: System Drive Serial berubah (Penggantian SSD C:\)
    // Motherboard (40) + CPU (35) = 75%
    let replaced_drive_tokens = vec![
        HardwareToken {
            token_type: HardwareTokenType::MotherboardUuid,
            raw_value: "WIN-MB-UUID-DEFAULT-4A71".to_string(),
        },
        HardwareToken {
            token_type: HardwareTokenType::CpuProcessorId,
            raw_value: "GENUINE-INTEL-X86-CPU".to_string(),
        },
        HardwareToken {
            token_type: HardwareTokenType::SystemDriveSerial,
            raw_value: "DRIVE-SERIAL-NEW-SAMSUNG-SSD".to_string(),
        },
    ];
    let score_drive_changed = WindowsHardwareIdProvider::compute_similarity_score(
        &baseline_tokens,
        &replaced_drive_tokens,
    );
    assert_eq!(score_drive_changed, 75);
    assert!(
        score_drive_changed >= 75,
        "Single disk replacement should remain within valid tolerance (>= 75%)"
    );

    // Skenario 3: CPU Diganti (Motherboard tetap)
    // Motherboard (40) + Drive (25) = 65% (Grace period)
    let replaced_cpu_tokens = vec![
        HardwareToken {
            token_type: HardwareTokenType::MotherboardUuid,
            raw_value: "WIN-MB-UUID-DEFAULT-4A71".to_string(),
        },
        HardwareToken {
            token_type: HardwareTokenType::CpuProcessorId,
            raw_value: "UPGRADED-INTEL-I9-14900K".to_string(),
        },
        HardwareToken {
            token_type: HardwareTokenType::SystemDriveSerial,
            raw_value: "DRIVE-SERIAL-C-SYSTEM".to_string(),
        },
    ];
    let score_cpu_changed =
        WindowsHardwareIdProvider::compute_similarity_score(&baseline_tokens, &replaced_cpu_tokens);
    assert_eq!(score_cpu_changed, 65);
    assert!(
        (50..75).contains(&score_cpu_changed),
        "Major upgrade enters 7-day grace period"
    );

    // Skenario 4: Motherboard diganti (dan CPU diganti, kloning SSD ke PC baru)
    // Drive (25) saja yang cocok
    let pirated_clone_tokens = vec![
        HardwareToken {
            token_type: HardwareTokenType::MotherboardUuid,
            raw_value: "COMPLETELY-DIFFERENT-MB".to_string(),
        },
        HardwareToken {
            token_type: HardwareTokenType::CpuProcessorId,
            raw_value: "COMPLETELY-DIFFERENT-CPU".to_string(),
        },
        HardwareToken {
            token_type: HardwareTokenType::SystemDriveSerial,
            raw_value: "DRIVE-SERIAL-C-SYSTEM".to_string(),
        },
    ];
    let score_clone = WindowsHardwareIdProvider::compute_similarity_score(
        &baseline_tokens,
        &pirated_clone_tokens,
    );
    assert_eq!(score_clone, 25);
    assert!(
        score_clone < 50,
        "Disk cloning to different machine must trigger license revocation"
    );

    // Skenario 5: Policy Version Hash Isolation (V1 vs V2)
    let provider_v1 = WindowsHardwareIdProvider::with_policy("WIN-HWID-V1");
    let provider_v2 = WindowsHardwareIdProvider::with_policy("WIN-HWID-V2");
    let hash_v1 = provider_v1.compute_native_id_hash().unwrap();
    let hash_v2 = provider_v2.compute_native_id_hash().unwrap();
    assert_ne!(
        hash_v1, hash_v2,
        "Policy version change must yield distinct cryptographic hash"
    );
}

#[test]
fn test_windows_wasapi_latency_measurement_and_buffer_metrics() {
    let mut backend = WindowsAudioBackend::new();

    // Uji berbagai konfigurasi sample rate dan buffer frame
    let configs = [
        (44100, 441, 10.0),  // 44.1 kHz, 441 frame = 10.0 ms
        (48000, 480, 10.0),  // 48.0 kHz, 480 frame = 10.0 ms
        (44100, 220, 4.988), // ~5.0 ms
        (48000, 240, 5.0),   // 5.0 ms
    ];

    for (sr, frames, expected_ms) in configs {
        let config = AudioStreamConfig {
            sample_rate: sr,
            channels: 2,
            buffer_size_frames: frames,
        };

        let calculated_ms = (config.buffer_size_frames as f64 / config.sample_rate as f64) * 1000.0;
        let diff = (calculated_ms - expected_ms).abs();
        assert!(
            diff < 0.05,
            "Calculated buffer duration should match expected duration"
        );

        let stream = backend
            .open_output_stream(None, &config, Box::new(|_buf| {}))
            .expect("Stream should open successfully");
        assert!(stream.is_active());
    }
}
