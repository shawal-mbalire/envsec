use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce,
};
use argon2::Argon2;
use rand::RngCore;

const SALT_LEN: usize = 32;
const NONCE_LEN: usize = 12;
const KEY_LEN: usize = 32;
const ARGON2_MEM: u32 = 65536; // 64 MB
const ARGON2_TIME: u32 = 3;
const ARGON2_PARALLELISM: u32 = 4;

#[derive(Debug, thiserror::Error)]
pub enum CryptoError {
    #[error("argon2 key derivation failed: {0}")]
    Argon2(String),
    #[error("encryption failed: {0}")]
    Encrypt(String),
    #[error("decryption failed: {0}")]
    Decrypt(String),
    #[error("invalid vault format: {0}")]
    Format(String),
}

pub fn generate_salt() -> [u8; SALT_LEN] {
    let mut salt = [0u8; SALT_LEN];
    OsRng.fill_bytes(&mut salt);
    salt
}

pub fn derive_key(passphrase: &[u8], salt: &[u8]) -> Result<[u8; KEY_LEN], CryptoError> {
    let mut key = [0u8; KEY_LEN];
    let params = argon2::Params::new(ARGON2_MEM, ARGON2_TIME, ARGON2_PARALLELISM, Some(KEY_LEN))
        .map_err(|e| CryptoError::Argon2(e.to_string()))?;
    let argon2 = Argon2::new(argon2::Algorithm::Argon2id, argon2::Version::V0x13, params);
    argon2
        .hash_password_into(passphrase, salt, &mut key)
        .map_err(|e| CryptoError::Argon2(e.to_string()))?;
    Ok(key)
}

pub fn encrypt(passphrase: &[u8], plaintext: &[u8]) -> Result<Vec<u8>, CryptoError> {
    let salt = generate_salt();
    let key_bytes = derive_key(passphrase, &salt)?;
    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);

    let mut nonce_bytes = [0u8; NONCE_LEN];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, plaintext)
        .map_err(|e| CryptoError::Encrypt(e.to_string()))?;

    // Format: [salt(32)][nonce(12)][ciphertext+tag]
    let mut output = Vec::with_capacity(SALT_LEN + NONCE_LEN + ciphertext.len());
    output.extend_from_slice(&salt);
    output.extend_from_slice(&nonce_bytes);
    output.extend_from_slice(&ciphertext);
    Ok(output)
}

pub fn decrypt(passphrase: &[u8], data: &[u8]) -> Result<Vec<u8>, CryptoError> {
    if data.len() < SALT_LEN + NONCE_LEN + 16 {
        return Err(CryptoError::Format("vault data too short".to_string()));
    }

    let salt = &data[..SALT_LEN];
    let nonce_bytes = &data[SALT_LEN..SALT_LEN + NONCE_LEN];
    let ciphertext = &data[SALT_LEN + NONCE_LEN..];

    let key_bytes = derive_key(passphrase, salt)?;
    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(nonce_bytes);

    cipher
        .decrypt(nonce, ciphertext)
        .map_err(|e| CryptoError::Decrypt(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        let passphrase = b"test-passphrase-123";
        let plaintext = b"Hello, world! This is a secret.";

        let encrypted = encrypt(passphrase, plaintext).unwrap();
        assert_ne!(&encrypted[SALT_LEN + NONCE_LEN..], plaintext);

        let decrypted = decrypt(passphrase, &encrypted).unwrap();
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_wrong_passphrase_fails() {
        let passphrase = b"correct-passphrase";
        let wrong = b"wrong-passphrase";
        let plaintext = b"secret data";

        let encrypted = encrypt(passphrase, plaintext).unwrap();
        let result = decrypt(wrong, &encrypted);
        assert!(result.is_err());
    }

    #[test]
    fn test_different_encryptions_produce_different_ciphertext() {
        let passphrase = b"test-passphrase";
        let plaintext = b"same data";

        let enc1 = encrypt(passphrase, plaintext).unwrap();
        let enc2 = encrypt(passphrase, plaintext).unwrap();

        // Different salt and nonce means different ciphertext
        assert_ne!(enc1, enc2);
    }

    #[test]
    fn test_derive_key_deterministic() {
        let passphrase = b"test-passphrase";
        let salt = [0u8; 32];
        let key1 = derive_key(passphrase, &salt).unwrap();
        let key2 = derive_key(passphrase, &salt).unwrap();
        assert_eq!(key1, key2);
    }

    #[test]
    fn test_short_data_returns_error() {
        let passphrase = b"test";
        let data = [0u8; 10];
        assert!(decrypt(passphrase, &data).is_err());
    }
}
