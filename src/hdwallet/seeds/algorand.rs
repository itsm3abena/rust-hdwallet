use crate::hdwallet::exceptions::SeedError;
use crate::hdwallet::mnemonics::algorand::AlgorandMnemonic;
use crate::hdwallet::mnemonics::imnemonic::IMnemonic;
use crate::hdwallet::seeds::iseed::ISeed;

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

    fn from_mnemonic(mnemonic: &dyn IMnemonic) -> Result<String, SeedError> {
        Self::from_mnemonic_str(&mnemonic.mnemonic())
    }
}

impl AlgorandSeed {
    pub fn from_mnemonic_str(mnemonic: &str) -> Result<String, SeedError> {
        let words: Vec<String> = mnemonic
            .split_whitespace()
            .map(|w| w.to_lowercase())
            .collect();

        let entropy = AlgorandMnemonic::decode_to_entropy(&words)
            .map_err(|e| SeedError::new(&e.to_string(), None, None, None))?;

        Ok(hex::encode(entropy))
    }
}
