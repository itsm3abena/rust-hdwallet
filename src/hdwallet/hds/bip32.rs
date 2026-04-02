use bitcoin::bip32::{DerivationPath, Xpriv, Xpub};
use bitcoin::key::Secp256k1;
use bitcoin::secp256k1::SecretKey;
use bitcoin::Network;

use crate::hdwallet::exceptions::{Error, Result};

#[derive(Debug, Clone)]
pub struct Bip32Hd {
    network: Network,
    master: Xpriv,
    secp: Secp256k1<bitcoin::secp256k1::All>,
}

impl Bip32Hd {
    pub fn from_seed(network: Network, seed: &[u8]) -> Result<Self> {
        let secp = Secp256k1::new();
        let master = Xpriv::new_master(network, seed).map_err(Error::from)?;
        Ok(Self {
            network,
            master,
            secp,
        })
    }

    #[must_use]
    pub fn master_xpriv(&self) -> &Xpriv {
        &self.master
    }

    pub fn derive_xpriv(&self, path: &DerivationPath) -> Result<Xpriv> {
        self.master
            .derive_priv(&self.secp, path)
            .map_err(Error::from)
    }

    pub fn derive_xpub(&self, path: &DerivationPath) -> Result<Xpub> {
        let xk = self.derive_xpriv(path)?;
        Ok(Xpub::from_priv(&self.secp, &xk))
    }

    pub fn secret_key_at(&self, path: &DerivationPath) -> Result<SecretKey> {
        let xk = self.derive_xpriv(path)?;
        Ok(xk.private_key)
    }
}
