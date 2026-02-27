use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Nonce,
};
use hkdf::Hkdf;
use rand::RngCore;
use sha2::Sha256;

/// Compile-time secret mixed into key derivation.
/// Not a standalone secret — combined with device fingerprint via HKDF.
/// Changing this invalidates all existing local state (users must re-import license).
const COMPILE_TIME_SECRET: &[u8] = b"pb_lic_v1_9f4a2c7e8b1d3f6a0e5c9d2b7a4f8e1c3d6b0a5e9f2c7d4b8a1e6f3c0d5b9a";

/// Derive a 256-bit AES key from device fingerprint hash + compile-time secret using HKDF-SHA256.
pub fn derive_state_key(fingerprint_hash: &str) -> [u8; 32] {
    let hk = Hkdf::<Sha256>::new(Some(COMPILE_TIME_SECRET), fingerprint_hash.as_bytes());
    let mut key = [0u8; 32];
    hk.expand(b"probook-security-state-v1", &mut key)
        .expect("HKDF expand should not fail for 32-byte output");
    key
}

/// Encrypt plaintext using AES-256-GCM.
/// Returns: nonce (12 bytes) || ciphertext (variable length with 16-byte auth tag).
pub fn encrypt_state(key: &[u8; 32], plaintext: &[u8]) -> Result<Vec<u8>, String> {
    let cipher =
        Aes256Gcm::new_from_slice(key).map_err(|e| format!("Failed to create cipher: {}", e))?;

    let mut nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, plaintext)
        .map_err(|e| format!("Encryption failed: {}", e))?;

    let mut result = Vec::with_capacity(12 + ciphertext.len());
    result.extend_from_slice(&nonce_bytes);
    result.extend_from_slice(&ciphertext);
    Ok(result)
}

/// Decrypt data encrypted by `encrypt_state`.
/// Input format: nonce (12 bytes) || ciphertext.
pub fn decrypt_state(key: &[u8; 32], data: &[u8]) -> Result<Vec<u8>, String> {
    if data.len() < 12 {
        return Err("Data too short to contain nonce".to_string());
    }

    let cipher =
        Aes256Gcm::new_from_slice(key).map_err(|e| format!("Failed to create cipher: {}", e))?;

    let nonce = Nonce::from_slice(&data[..12]);
    let ciphertext = &data[12..];

    cipher
        .decrypt(nonce, ciphertext)
        .map_err(|e| format!("Decryption failed: {}", e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_derive_state_key_deterministic() {
        let k1 = derive_state_key("test-fingerprint");
        let k2 = derive_state_key("test-fingerprint");
        assert_eq!(k1, k2);
    }

    #[test]
    fn test_derive_state_key_different_inputs() {
        let k1 = derive_state_key("fingerprint-a");
        let k2 = derive_state_key("fingerprint-b");
        assert_ne!(k1, k2);
    }

    #[test]
    fn test_derive_state_key_length() {
        let key = derive_state_key("any-input");
        assert_eq!(key.len(), 32);
    }

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        let key = derive_state_key("roundtrip-test");
        let plaintext = b"hello licensing engine";
        let encrypted = encrypt_state(&key, plaintext).unwrap();
        let decrypted = decrypt_state(&key, &encrypted).unwrap();
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_encrypt_decrypt_empty() {
        let key = derive_state_key("empty-test");
        let plaintext = b"";
        let encrypted = encrypt_state(&key, plaintext).unwrap();
        let decrypted = decrypt_state(&key, &encrypted).unwrap();
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_encrypt_produces_nonce_prefix() {
        let key = derive_state_key("nonce-test");
        let encrypted = encrypt_state(&key, b"data").unwrap();
        // 12 bytes nonce + at least 16 bytes auth tag + ciphertext
        assert!(encrypted.len() >= 12 + 16);
    }

    #[test]
    fn test_decrypt_wrong_key_fails() {
        let key1 = derive_state_key("key-one");
        let key2 = derive_state_key("key-two");
        let encrypted = encrypt_state(&key1, b"secret").unwrap();
        let result = decrypt_state(&key2, &encrypted);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Decryption failed"));
    }

    #[test]
    fn test_decrypt_too_short_fails() {
        let key = derive_state_key("short-test");
        let result = decrypt_state(&key, &[0u8; 5]);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("too short"));
    }

    #[test]
    fn test_decrypt_tampered_ciphertext_fails() {
        let key = derive_state_key("tamper-test");
        let mut encrypted = encrypt_state(&key, b"original data").unwrap();
        // Flip a byte in the ciphertext (after the 12-byte nonce)
        let last = encrypted.len() - 1;
        encrypted[last] ^= 0xFF;
        let result = decrypt_state(&key, &encrypted);
        assert!(result.is_err());
    }

    #[test]
    fn test_encrypt_nondeterministic() {
        let key = derive_state_key("nonce-unique");
        let e1 = encrypt_state(&key, b"same data").unwrap();
        let e2 = encrypt_state(&key, b"same data").unwrap();
        // Different random nonces → different ciphertext
        assert_ne!(e1, e2);
        // But both decrypt to the same value
        assert_eq!(decrypt_state(&key, &e1).unwrap(), decrypt_state(&key, &e2).unwrap());
    }
}
