pub mod crypto;
pub mod store;
pub mod types;

use std::path::{Path, PathBuf};

use store::StoreError;
use types::VaultData;

pub struct Vault {
    path: PathBuf,
    passphrase: Vec<u8>,
    data: VaultData,
}

#[derive(Debug, thiserror::Error)]
pub enum VaultError {
    #[error("store error: {0}")]
    Store(#[from] StoreError),
    #[error("vault not initialized")]
    NotInitialized,
}

impl Vault {
    pub fn load(vault_path: &Path, passphrase: &[u8]) -> Result<Self, VaultError> {
        let data = store::load_vault(vault_path, passphrase)?;
        Ok(Self {
            path: vault_path.to_path_buf(),
            passphrase: passphrase.to_vec(),
            data,
        })
    }

    pub fn data(&self) -> &VaultData {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut VaultData {
        &mut self.data
    }

    pub fn save(&self) -> Result<(), VaultError> {
        store::save_vault(&self.path, &self.passphrase, &self.data)?;
        Ok(())
    }

    pub fn vault_path(&self) -> &Path {
        &self.path
    }
}
