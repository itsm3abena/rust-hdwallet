use sha2::{Digest, Sha256};

use crate::hdwallet::exceptions::SeedError;
use crate::hdwallet::mnemonics::electrum::ElectrumV1Mnemonic;
use crate::hdwallet::mnemonics::imnemonic::IMnemonic;
use crate::hdwallet::seeds::iseed::ISeed;

pub struct ElectrumV1Seed {
    seed: String,
}

impl ElectrumV1Seed {
    pub const HASH_ITERATION_NUMBER: usize = 100_000;

    pub fn from_mnemonic_phrase(phrase: &str) -> Result<String, SeedError> {
        if !ElectrumV1Mnemonic::is_valid_phrase(phrase, None, None) {
            return Err(SeedError::new(
                &format!("Invalid {} mnemonic words", Self::name()),
                None,
                None,
                None,
            ));
        }

        let entropy_bytes = ElectrumV1Mnemonic::decode_phrase(phrase, None, None).map_err(|e| {
            SeedError::new(&e.to_string(), None, None, None)
        })?;

        let encoded_entropy = entropy_bytes.as_slice();

        let mut entropy_hash = encoded_entropy.to_vec();
        for _ in 0..Self::HASH_ITERATION_NUMBER {
            let mut combined = entropy_hash;
            combined.extend_from_slice(encoded_entropy);
            entropy_hash = Sha256::digest(&combined).to_vec();
        }

        Ok(hex::encode(entropy_hash))
    }
}

impl ISeed for ElectrumV1Seed {
    const LENGTH: usize = 64;

    fn new(seed: String) -> Self {
        Self { seed }
    }

    fn name() -> &'static str {
        "Electrum-V1"
    }

    fn seed(&self) -> &str {
        &self.seed
    }

    fn from_mnemonic(mnemonic: &dyn IMnemonic) -> Result<String, SeedError> {
        Self::from_mnemonic_phrase(&mnemonic.mnemonic())
    }
}
