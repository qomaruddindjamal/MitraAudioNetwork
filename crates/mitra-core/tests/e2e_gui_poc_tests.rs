use ed25519_dalek::SigningKey;
use mitra_core::license::{
    LicenseAuthorityRecord, LicenseAuthorityStorage, LicenseIssuer, LicenseVerifier,
    SerialNumberGenerator, SignedMitraLicense, ENTITLEMENT_BASIC_AUDIO, ENTITLEMENT_WEB_MANAGEMENT,
};

#[test]
fn test_gui_poc_end_to_end_registration_lifecycle() {
    // 1. Setup Test Authority Environment (Vendor Side)
    let raw_key = [0x55u8; 32];
    let signing_key = SigningKey::from_bytes(&raw_key);
    let authority_salt = "MITRA_AUDIO_TEST_AUTHORITY_SALT_2026";
    let issuer = LicenseIssuer::new("MITRA-AUTHORITY-ROOT", signing_key, authority_salt);
    let mut storage = LicenseAuthorityStorage::new();

    // 2. Setup Customer Machine State (Customer Side)
    let customer_native_id = "WIN-HWID-V1:MB-TEST-DELL-OPTIPLEX:CPU-I7-12700:DISK-NVME-SAMSUNG";
    let product_id = "MITRA-AUDIO-SERVER";
    let now = 1731380000u64;

    // 3. Step A: Generate Serial Number in SN Generator
    let generated_sn =
        SerialNumberGenerator::generate_sn(product_id, customer_native_id, authority_salt)
            .expect("SN generation should succeed");
    assert!(SerialNumberGenerator::validate_sn_format(&generated_sn));
    assert!(generated_sn.contains("CIMT") || generated_sn.contains("MITG"));

    // 4. Step B: Issue Signed Permanent License
    let license_id = "LIC-E2E-2026-9901";
    let entitlements = vec![
        ENTITLEMENT_BASIC_AUDIO.to_string(),
        ENTITLEMENT_WEB_MANAGEMENT.to_string(),
    ];
    let signed_license = issuer
        .issue_license(
            license_id,
            product_id,
            customer_native_id,
            now,
            entitlements.clone(),
        )
        .expect("License issuance should succeed");
    assert_eq!(signed_license.payload.serial_number, generated_sn);
    assert_eq!(signed_license.payload.license_type, "permanent");

    // 5. Step C: Record in Authority Storage
    let authority_record = LicenseAuthorityRecord {
        license_id: license_id.to_string(),
        product_id: product_id.to_string(),
        native_id: customer_native_id.to_string(),
        serial_number: generated_sn.clone(),
        purchase_date: now,
        support_until: now + (365 * 86400),
        issued_at: now,
        license_status: "ACTIVE".to_string(),
        entitlements: entitlements.clone(),
    };
    storage
        .insert_record(authority_record)
        .expect("Recording in storage should succeed");

    // 6. Step D: Export License File (.lic content)
    let lic_file_content = signed_license.to_license_file_content();
    assert!(lic_file_content.contains("MITRA_AUDIO_LICENSE_V1"));
    assert!(lic_file_content.contains("PAYLOAD="));
    assert!(lic_file_content.contains("SIGNATURE="));

    // 7. Step E: Customer Application Verification (Using ONLY Public Key)
    let public_key_bytes = issuer.verifying_key().to_bytes();
    let client_verifier = LicenseVerifier::from_public_key_bytes(&public_key_bytes)
        .expect("Client verifier creation should succeed");

    // Parsing exported .lic content on client side
    let parsed_client_license = SignedMitraLicense::from_license_file_content(&lic_file_content)
        .expect("Client should successfully parse .lic file");

    let revoked_list = [];
    let verification_result = client_verifier.verify_mitra_license(
        &parsed_client_license,
        product_id,
        customer_native_id,
        &revoked_list,
    );
    assert!(
        verification_result.is_ok(),
        "Client verification should PASS for matched Native ID"
    );

    // Feature entitlement check
    assert!(client_verifier
        .check_entitlement(&parsed_client_license, ENTITLEMENT_WEB_MANAGEMENT)
        .is_ok());

    // 8. Negative Tests: Reject Tampering & Mismatch
    // Case 1: Wrong Native ID (License cloning attempt)
    let wrong_native_id = "WIN-HWID-V1:MB-PIRATE-BOARD:CPU-AMD:DISK-CLONE";
    let clone_result = client_verifier.verify_mitra_license(
        &parsed_client_license,
        product_id,
        wrong_native_id,
        &revoked_list,
    );
    assert!(
        clone_result.is_err(),
        "Cloned license on wrong Native ID must be rejected"
    );

    // Case 2: Wrong Product ID
    let wrong_product = "MITRA-AUDIO-TERMINAL-CLIENT";
    let prod_result = client_verifier.verify_mitra_license(
        &parsed_client_license,
        wrong_product,
        customer_native_id,
        &revoked_list,
    );
    assert!(
        prod_result.is_err(),
        "License for wrong product must be rejected"
    );

    // Case 3: Tampered Payload
    let mut tampered_license = parsed_client_license.clone();
    tampered_license.payload.license_id = "LIC-HACKED-ID".to_string();
    let tamper_result = client_verifier.verify_mitra_license(
        &tampered_license,
        product_id,
        customer_native_id,
        &revoked_list,
    );
    assert!(
        tamper_result.is_err(),
        "Tampered payload must fail cryptographic verification"
    );

    // Case 4: Revoked License
    let revoked_ids = [license_id];
    let revoke_result = client_verifier.verify_mitra_license(
        &parsed_client_license,
        product_id,
        customer_native_id,
        &revoked_ids,
    );
    assert!(revoke_result.is_err(), "Revoked license must be rejected");

    // Case 5: Duplicate Issuance Prevention
    let dup_res = storage.insert_record(LicenseAuthorityRecord {
        license_id: license_id.to_string(),
        product_id: product_id.to_string(),
        native_id: customer_native_id.to_string(),
        serial_number: generated_sn.clone(),
        purchase_date: now,
        support_until: now + (365 * 86400),
        issued_at: now,
        license_status: "ACTIVE".to_string(),
        entitlements: entitlements.clone(),
    });
    assert!(
        dup_res.is_err(),
        "Duplicate license ID must be rejected by authority storage"
    );

    // Case 6: Support Period Expiry Does Not Expire Permanent License
    let two_years_later = now + (2 * 365 * 86400);
    let record_lookup = storage
        .get_record_by_sn(&generated_sn)
        .expect("Record should exist");
    assert!(
        two_years_later > record_lookup.support_until,
        "Support period ended"
    );
    // But permanent license verification on client remains 100% valid!
    assert!(client_verifier
        .verify_mitra_license(
            &parsed_client_license,
            product_id,
            customer_native_id,
            &revoked_list
        )
        .is_ok());
}
