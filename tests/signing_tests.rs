use chainproof::signing;

#[test]
fn test_sign_and_verify_roundtrip() {
    let json_str = r#"{"name":"test"}"#;
    let key = b"secret_key";

    let signature = signing::sign(json_str, key);
    assert!(signing::verify(json_str, &signature, key), "Signature should verify");
}

#[test]
fn test_verify_fails_with_wrong_key() {
    let json_str = r#"{"name":"test"}"#;
    let key1 = b"secret_key_1";
    let key2 = b"secret_key_2";

    let signature = signing::sign(json_str, key1);
    assert!(!signing::verify(json_str, &signature, key2), "Signature should fail with wrong key");
}

#[test]
fn test_verify_fails_with_tampered_payload() {
    let json_str = r#"{"name":"test"}"#;
    let tampered = r#"{"name":"tampered"}"#;
    let key = b"secret_key";

    let signature = signing::sign(json_str, key);
    assert!(!signing::verify(tampered, &signature, key), "Signature should fail with tampered payload");
}
