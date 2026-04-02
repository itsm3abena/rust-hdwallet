//! Custom BIP-32 path (mirrors `hdwallet/derivations/custom.py` — thin).

use bitcoin::bip32::DerivationPath;
use std::str::FromStr;

use crate::hdwallet::exceptions::{Error, Result};

/// Parsed BIP-32 derivation path.
#[derive(Debug, Clone)]
pub struct CustomDerivation {
    path: DerivationPath,
}

impl CustomDerivation {
    pub fn from_path_str(s: &str) -> Result<Self> {
        let path = DerivationPath::from_str(s.trim())
            .map_err(|e| Error::Derivation(e.to_string()))?;
        Ok(Self { path })
    }

    #[must_use]
    pub fn path(&self) -> &DerivationPath {
        &self.path
    }
}
