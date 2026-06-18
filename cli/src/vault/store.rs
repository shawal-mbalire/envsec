use std::fs;
use std::path::Path;

use super::crypto;
use super::types::VaultData;

#[derive(Debug, thiserror::Error)]
pub enum StoreError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("crypto error: {0}")]
    Crypto(#[from] crypto::CryptoError),
    #[error("serialization error: {0}")]
    Serde(#[from] serde_json::Error),
}

pub fn load_vault(vault_path: &Path, passphrase: &[u8]) -> Result<VaultData, StoreError> {
    if !vault_path.exists() {
        return Ok(VaultData::default());
    }
    let encrypted = fs::read(vault_path)?;
    if encrypted.is_empty() {
        return Ok(VaultData::default());
    }
    let decrypted = crypto::decrypt(passphrase, &encrypted)?;
    let data: VaultData = serde_json::from_slice(&decrypted)?;
    Ok(data)
}

pub fn save_vault(
    vault_path: &Path,
    passphrase: &[u8],
    data: &VaultData,
) -> Result<(), StoreError> {
    let plaintext = serde_json::to_vec(data)?;
    let encrypted = crypto::encrypt(passphrase, &plaintext)?;

    if let Some(parent) = vault_path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(vault_path, &encrypted)?;

    // Set permissions to 600 on Unix
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(vault_path, fs::Permissions::from_mode(0o600))?;
    }

    Ok(())
}
