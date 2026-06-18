use argon2::password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString};
use argon2::Argon2;

#[derive(Debug, thiserror::Error)]
pub enum PasswordError {
    #[error("hashing failed: {0}")]
    Hash(String),
    #[error("verification failed: {0}")]
    Verify(String),
}

pub fn hash_passphrase(passphrase: &[u8]) -> Result<String, PasswordError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2
        .hash_password(passphrase, &salt)
        .map_err(|e| PasswordError::Hash(e.to_string()))?;
    Ok(hash.to_string())
}

pub fn verify_passphrase(passphrase: &[u8], hash_str: &str) -> Result<bool, PasswordError> {
    let parsed_hash =
        PasswordHash::new(hash_str).map_err(|e| PasswordError::Verify(e.to_string()))?;
    let argon2 = Argon2::default();
    Ok(argon2.verify_password(passphrase, &parsed_hash).is_ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_and_verify() {
        let passphrase = b"my-secret-passphrase";
        let hash = hash_passphrase(passphrase).unwrap();
        assert!(verify_passphrase(passphrase, &hash).unwrap());
    }

    #[test]
    fn test_wrong_passphrase_fails() {
        let passphrase = b"correct-passphrase";
        let wrong = b"wrong-passphrase";
        let hash = hash_passphrase(passphrase).unwrap();
        assert!(!verify_passphrase(wrong, &hash).unwrap());
    }
}
