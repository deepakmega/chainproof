use hmac::{Hmac, Mac};
use sha2::Sha256;
use base64::{engine::general_purpose::STANDARD, Engine};

type HmacSha256 = Hmac<Sha256>;

/// Sign a JSON string using HMAC-SHA256 and return the base64-encoded signature
pub fn sign(json_str: &str, key: &[u8]) -> String {
    let mut mac = HmacSha256::new_from_slice(key).expect("HMAC can take key of any size");
    mac.update(json_str.as_bytes());
    let signature = mac.finalize();
    STANDARD.encode(signature.into_bytes())
}

/// Verify a base64-encoded signature against a JSON string using HMAC-SHA256
pub fn verify(json_str: &str, signature: &str, key: &[u8]) -> bool {
    // Decode the base64 signature
    let signature_bytes = match STANDARD.decode(signature) {
        Ok(bytes) => bytes,
        Err(_) => return false,
    };

    // Compute the expected HMAC
    let mut mac = HmacSha256::new_from_slice(key).expect("HMAC can take key of any size");
    mac.update(json_str.as_bytes());

    // Compare signatures in constant time
    mac.verify_slice(&signature_bytes).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sign_and_verify() {
        let json_str = r#"{"name":"test"}"#;
        let key = b"secret_key";

        let signature = sign(json_str, key);
        assert!(verify(json_str, &signature, key));
    }

    #[test]
    fn test_verify_invalid_signature() {
        let json_str = r#"{"name":"test"}"#;
        let key = b"secret_key";
        let wrong_signature = "invalid_base64_signature";

        assert!(!verify(json_str, wrong_signature, key));
    }

    #[test]
    fn test_verify_wrong_key() {
        let json_str = r#"{"name":"test"}"#;
        let key1 = b"secret_key_1";
        let key2 = b"secret_key_2";

        let signature = sign(json_str, key1);
        assert!(!verify(json_str, &signature, key2));
    }

    #[test]
    fn test_verify_modified_json() {
        let json_str = r#"{"name":"test"}"#;
        let modified_json = r#"{"name":"test2"}"#;
        let key = b"secret_key";

        let signature = sign(json_str, key);
        assert!(!verify(modified_json, &signature, key));
    }
}
