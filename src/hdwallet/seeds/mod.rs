pub mod algorand;
pub mod bip39;
pub mod cardano;
pub mod electrum;
pub mod iseed;
pub mod monero;

pub use algorand::AlgorandSeed;
pub use bip39::BIP39Seed;
pub use cardano::{CardanoSeed, CardanoTypes};
pub use electrum::{ElectrumV1Seed, ElectrumV2Seed};
pub use iseed::ISeed;
pub use monero::MoneroSeed;

use crate::hdwallet::exceptions::SeedError;

pub struct Seeds;

impl Seeds {
    pub fn names() -> Vec<&'static str> {
        vec![
            AlgorandSeed::name(),
            BIP39Seed::name(),
            CardanoSeed::name(),
            ElectrumV1Seed::name(),
            ElectrumV2Seed::name(),
            MoneroSeed::name(),
        ]
    }

    pub fn is_seed(name: &str) -> bool {
        Self::names().contains(&name)
    }

    pub fn seed_from_mnemonic(
        client: &str,
        mnemonic: &str,
        passphrase: Option<&str>,
        cardano_type: &str,
        mnemonic_type: &str,
    ) -> Result<String, SeedError> {
        if !Self::is_seed(client) {
            return Err(SeedError::new(
                "Wrong seed client",
                None,
                Some(&Self::names().join(", ")),
                Some(client),
            ));
        }

        match client {
            "BIP39" => BIP39Seed::from_mnemonic_with_passphrase(mnemonic, passphrase),
            "Cardano" => CardanoSeed::from_mnemonic_with_type(mnemonic, passphrase, cardano_type)
                .map_err(|error| SeedError::new(&error.to_string(), None, None, None)),
            "Electrum-V2" => {
                ElectrumV2Seed::from_mnemonic_phrase(mnemonic, passphrase, mnemonic_type)
            }
            "Electrum-V1" => ElectrumV1Seed::from_mnemonic_phrase(mnemonic),
            "Monero" => MoneroSeed::from_mnemonic_phrase(mnemonic),
            "Algorand" => AlgorandSeed::from_mnemonic_str(mnemonic),
            other => Err(SeedError::new(
                "Wrong seed client",
                None,
                Some(&Self::names().join(", ")),
                Some(other),
            )),
        }
    }

    pub fn seed_from_hex(client: &str, seed: &str) -> Result<String, SeedError> {
        if !Self::is_seed(client) {
            return Err(SeedError::new(
                "Wrong seed client",
                None,
                Some(&Self::names().join(", ")),
                Some(client),
            ));
        }
        if !BIP39Seed::is_valid(seed) && client == "BIP39" {
            return Err(SeedError::new("Invalid seed hex", None, None, None));
        }
        Ok(seed.to_string())
    }
}
