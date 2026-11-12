use mitra_adapter_windows::hw_windows::WindowsSecureRandomProvider;
use mitra_core::license::SerialNumberGenerator;

#[test]
fn test_mitra_audio_sn_csprng_random_tag_position() {
    let rng = WindowsSecureRandomProvider::new();
    let product_id = "MITRA-AUDIO-SERVER";
    let native_id = "WIN-HWID-V1:MB-TEST-CSPRNG:CPU-I9:DISK-SSD";

    let mut found_cimt = false;
    let mut found_mitg = false;
    let mut tag_positions = std::collections::HashSet::new();

    for _ in 0..50 {
        let sn = SerialNumberGenerator::generate_sn_with_csprng(
            product_id,
            native_id,
            "TEST_SALT",
            &rng,
        )
        .expect("SN generation with CSPRNG");

        assert!(SerialNumberGenerator::validate_sn_format(&sn));

        let parts: Vec<&str> = sn.split('-').collect();
        assert_eq!(parts.len(), 4);

        if sn.contains("CIMT") {
            found_cimt = true;
            let pos = parts.iter().position(|&p| p == "CIMT").unwrap();
            tag_positions.insert(pos);
        }
        if sn.contains("MITG") {
            found_mitg = true;
            let pos = parts.iter().position(|&p| p == "MITG").unwrap();
            tag_positions.insert(pos);
        }
    }

    assert!(found_cimt, "Must generate CIMT in sample");
    assert!(found_mitg, "Must generate MITG in sample");
    // Assert that tag appears in more than 1 position across 50 trials
    assert!(
        tag_positions.len() >= 2,
        "Tag position must be dynamic across 4 segments"
    );
}
