use chrono::{DateTime, Utc};
use ed25519_dalek::{Signature, Verifier, VerifyingKey, PUBLIC_KEY_LENGTH, SIGNATURE_LENGTH};
use serde::{Deserialize, Serialize};

/// Ed25519 public key for license verification (32 bytes, compiled in).
/// To update: replace public_key.bin in this directory with the real key, then rebuild.
const PUBLIC_KEY_BYTES: [u8; PUBLIC_KEY_LENGTH] = *include_bytes!("public_key.bin");

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LicenseType {
    Trial,
    Annual,
    Lifetime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicensePayload {
    pub license_id: String,
    pub customer_name: String,
    pub license_type: LicenseType,
    pub issued_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub device_hash: Option<String>,
    pub max_devices: u8,
}

/// License file format:
/// - payload_length: 4 bytes (little-endian u32)
/// - payload_json: `payload_length` bytes (UTF-8 JSON)
/// - signature: 64 bytes (Ed25519 signature over payload_json bytes)
const HEADER_SIZE: usize = 4;

/// Parse and verify a .probook license file.
/// Returns the verified payload if signature is valid.
pub fn verify_license(file_bytes: &[u8]) -> Result<LicensePayload, String> {
    if file_bytes.len() < HEADER_SIZE + SIGNATURE_LENGTH {
        return Err("License file is too small".to_string());
    }

    // Read payload length
    let payload_len =
        u32::from_le_bytes(file_bytes[..4].try_into().unwrap()) as usize;

    let expected_total = HEADER_SIZE + payload_len + SIGNATURE_LENGTH;
    if file_bytes.len() < expected_total {
        return Err("License file is corrupted (truncated)".to_string());
    }

    let payload_bytes = &file_bytes[HEADER_SIZE..HEADER_SIZE + payload_len];
    let sig_bytes =
        &file_bytes[HEADER_SIZE + payload_len..HEADER_SIZE + payload_len + SIGNATURE_LENGTH];

    // Verify Ed25519 signature
    let verifying_key = VerifyingKey::from_bytes(&PUBLIC_KEY_BYTES)
        .map_err(|e| format!("Invalid embedded public key: {}", e))?;

    let signature = Signature::from_bytes(
        sig_bytes
            .try_into()
            .map_err(|_| "Invalid signature length".to_string())?,
    );

    verifying_key
        .verify(payload_bytes, &signature)
        .map_err(|_| "License signature verification failed — file may be tampered".to_string())?;

    // Deserialize payload
    let payload: LicensePayload = serde_json::from_slice(payload_bytes)
        .map_err(|e| format!("Failed to parse license payload: {}", e))?;

    Ok(payload)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_license_too_small() {
        let result = verify_license(&[0u8; 3]);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("too small"));
    }

    #[test]
    fn test_verify_license_empty() {
        let result = verify_license(&[]);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("too small"));
    }

    #[test]
    fn test_verify_license_truncated() {
        // Header says payload is 100 bytes but we only provide header + 10 bytes + 64 sig
        let mut data = Vec::new();
        data.extend_from_slice(&100u32.to_le_bytes()); // payload_len = 100
        data.extend_from_slice(&[0u8; 10]); // only 10 bytes of payload
        data.extend_from_slice(&[0u8; SIGNATURE_LENGTH]); // signature
        // Total is 4 + 10 + 64 = 78, but expected is 4 + 100 + 64 = 168
        let result = verify_license(&data);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("truncated"));
    }

    #[test]
    fn test_verify_license_invalid_signature() {
        let payload = b"{}";
        let payload_len = payload.len() as u32;
        let mut data = Vec::new();
        data.extend_from_slice(&payload_len.to_le_bytes());
        data.extend_from_slice(payload);
        data.extend_from_slice(&[0u8; SIGNATURE_LENGTH]);
        let result = verify_license(&data);
        assert!(result.is_err());
        // Could be "Invalid embedded public key" or "signature verification failed"
        // depending on whether the placeholder key is valid
        let err = result.unwrap_err();
        assert!(
            err.contains("signature verification failed") || err.contains("public key"),
            "Unexpected error: {}",
            err
        );
    }

    #[test]
    fn test_verify_license_header_only_with_signature() {
        // Minimum valid-looking file: header + 0-length payload + signature
        let mut data = Vec::new();
        data.extend_from_slice(&0u32.to_le_bytes()); // payload_len = 0
        data.extend_from_slice(&[0u8; SIGNATURE_LENGTH]); // signature
        let result = verify_license(&data);
        // Should fail on signature or public key validation, not on size checks
        assert!(result.is_err());
    }
}
