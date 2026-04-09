use crate::exceptions::{MnemonicError, SeedError};
use crate::mnemonics::{IMnemonic, algorand::AlgorandMnemonic};
use crate::seeds::iseed::ISeed;

pub struct AlgorandSeed {
    seed: String,
}

impl ISeed for AlgorandSeed {
    const LENGTH: usize = 64;

    fn new(seed: String) -> Self {
        Self { seed }
    }

    fn name() -> &'static str {
        "Algorand"
    }

    fn seed(&self) -> &str {
        &self.seed
    }

    fn from_mnemonic(
        mnemonic: &dyn IMnemonic,
    ) -> Result<String, SeedError> {
        let mnemonic_str = mnemonic.mnemonic();

        if !AlgorandMnemonic::is_valid(mnemonic_str) {
            return Err(SeedError::from(MnemonicError::new(
                &format!("Invalid {} mnemonic words", Self::name()),
                None,
                None,
            )));
        }

        Ok(AlgorandMnemonic::decode(mnemonic_str))
    }
}

impl AlgorandSeed {
    pub fn from_mnemonic_str(
        mnemonic: &str,
    ) -> Result<String, SeedError> {
        if !AlgorandMnemonic::is_valid(mnemonic) {
            return Err(SeedError::from(MnemonicError::new(
                &format!("Invalid {} mnemonic words", Self::name()),
                None,
                None,
            )));
        }

        Ok(AlgorandMnemonic::decode(mnemonic))
    }
}