use crate::crypto::pbkdf2_hmac_sha512;
use crate::exceptions::{MnemonicError, SeedError};
use crate::mnemonics::{IMnemonic, bip39::BIP39Mnemonic};
use crate::utils::bytes_to_string;
use crate::seeds::iseed::ISeed;

use unicode_normalization::UnicodeNormalization;

pub struct BIP39Seed {
    seed: String,
}

impl ISeed for BIP39Seed {
    const LENGTH: usize = 128;

    fn new(seed: String) -> Self {
        Self { seed }
    }

    fn name() -> &'static str {
        "BIP39"
    }

    fn seed(&self) -> &str {
        &self.seed
    }

    fn from_mnemonic(
        mnemonic: &dyn IMnemonic,
    ) -> Result<String, SeedError> {
        Self::from_mnemonic_with_passphrase(mnemonic.mnemonic(), None)
    }
}

impl BIP39Seed {
    const SEED_SALT_MODIFIER: &'static str = "mnemonic";
    const SEED_PBKDF2_ROUNDS: u32 = 2048;

    pub fn from_mnemonic_with_passphrase(
        mnemonic: &str,
        passphrase: Option<&str>,
    ) -> Result<String, SeedError> {
        // Validate mnemonic
        if !BIP39Mnemonic::is_valid(mnemonic) {
            return Err(SeedError::from(MnemonicError::new(
                &format!("Invalid {} mnemonic words", Self::name()),
                None,
                None,
            )));
        }

        let salt_raw = match passphrase {
            Some(p) => format!("{}{}", Self::SEED_SALT_MODIFIER, p),
            None => Self::SEED_SALT_MODIFIER.to_string(),
        };

        let salt: String = salt_raw.nfkd().collect();

        let normalized_mnemonic: String = mnemonic.nfkd().collect();

        let seed_bytes = pbkdf2_hmac_sha512(
            normalized_mnemonic.as_bytes(),
            salt.as_bytes(),
            Self::SEED_PBKDF2_ROUNDS,
        );

        Ok(bytes_to_string(&seed_bytes))
    }
}