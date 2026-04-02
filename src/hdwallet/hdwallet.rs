use crate::hd::traits::Derivation;
use crate::cryptocurrencies::traits::Cryptocurrency;
use crate::mnemonics::bip39::mnemonic_to_seed;
use crate::utils::errors::WalletError;

pub struct HDWallet<D, C>
where
    D: Derivation,
    C: Cryptocurrency,
{
    seed: Vec<u8>,
    derivation: D,
    coin: C,
}

impl<D, C> HDWallet<D, C>
where
    D: Derivation,
    C: Cryptocurrency,
{
    pub fn from_mnemonic(
        mnemonic: &str,
        passphrase: &str,
        derivation: D,
        coin: C,
    ) -> Result<Self, WalletError> {
        let seed = mnemonic_to_seed(mnemonic, passphrase)?;

        Ok(Self {
            seed,
            derivation,
            coin,
        })
    }

    pub fn from_seed(
        seed: Vec<u8>,
        derivation: D,
        coin: C,
    ) -> Self {
        Self {
            seed,
            derivation,
            coin,
        }
    }

    pub fn derive_private_key(&self, path: &str) -> Result<Vec<u8>, WalletError> {
        self.derivation.derive_private_key(&self.seed, path)
    }

    pub fn derive_public_key(&self, path: &str) -> Result<Vec<u8>, WalletError> {
        self.derivation.derive_public_key(&self.seed, path)
    }

    pub fn derive_address(&self, path: &str) -> Result<String, WalletError> {
        let public_key = self.derive_public_key(path)?;
        Ok(self.coin.derive_address(&public_key)?)
    }

    pub fn seed(&self) -> &[u8] {
        &self.seed
    }
}