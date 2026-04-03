use std::collections::HashMap;

pub mod algorand;
pub mod bip39;
pub mod electrum;
pub mod ientropy;
pub mod monero;

pub use bip39::BIP39Entropy;
pub use ientropy::{EntropyError as RawEntropyError, EntropyInstance, IEntropy};

use algorand::AlgorandEntropy;
use electrum::{ElectrumV1Entropy, ElectrumV2Entropy};
use monero::MoneroEntropy;

use crate::hdwallet::exceptions::EntropyError;

pub struct Entropies;

impl Entropies {
    fn dictionary() -> HashMap<&'static str, fn() -> Box<dyn IEntropy>> {
        let mut map: HashMap<&'static str, fn() -> Box<dyn IEntropy>> = HashMap::new();

        map.insert("Algorand", || Box::new(AlgorandEntropy));
        map.insert("BIP39", || Box::new(BIP39Entropy));
        map.insert("Electrum-V1", || Box::new(ElectrumV1Entropy));
        map.insert("Electrum-V2", || Box::new(ElectrumV2Entropy));
        map.insert("Monero", || Box::new(MoneroEntropy));

        map
    }

    pub fn names() -> Vec<&'static str> {
        Self::dictionary().keys().cloned().collect()
    }

    pub fn is_entropy(name: &str) -> bool {
        Self::dictionary().contains_key(name)
    }

    pub fn entropy(name: &str) -> Result<Box<dyn IEntropy>, EntropyError> {
        let dict = Self::dictionary();

        match dict.get(name) {
            Some(constructor) => Ok(constructor()),
            None => {
                let expected = Self::names().join(", ");
                Err(EntropyError::new(
                    "Invalid entropy name",
                    None,
                    Some(&expected),
                    Some(name),
                ))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_contains_bip39() {
        assert!(Entropies::is_entropy("BIP39"));
        let e = Entropies::entropy("BIP39").expect("BIP39");
        assert_eq!(e.name(), "BIP39");
    }

    #[test]
    fn unknown_entropy_errors() {
        let res = Entropies::entropy("Nope");
        assert!(res.is_err());
        let err = res.err().expect("err");
        assert!(err.to_string().contains("Invalid entropy name"));
    }

    #[test]
    fn bip39_generate_round_trip_length() {
        let scheme = BIP39Entropy;
        let hex = scheme.generate(128).expect("generate");
        assert_eq!(hex.len(), 32);
        let inst = EntropyInstance::new::<BIP39Entropy>(&hex).expect("parse");
        assert_eq!(inst.strength_bits(), 128);
    }
}
