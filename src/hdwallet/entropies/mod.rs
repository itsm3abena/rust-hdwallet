use std::collections::HashMap;

pub mod algorand;
pub mod bip39;
pub mod electrum_v1;
pub mod electrum_v2;
pub mod monero;

use algorand::AlgorandEntropy;
use bip39::BIP39Entropy;
use electrum_v1::ElectrumV1Entropy;
use electrum_v2::ElectrumV2Entropy;
use monero::MoneroEntropy;

use crate::exceptions::EntropyError;

pub trait IEntropy {
    fn name() -> &'static str
    where
        Self: Sized;
}

pub struct Entropies;

impl Entropies {
    fn dictionary() -> HashMap<&'static str, fn() -> Box<dyn IEntropy>> {
        let mut map: HashMap<&'static str, fn() -> Box<dyn IEntropy>> = HashMap::new();

        map.insert(AlgorandEntropy::name(), || Box::new(AlgorandEntropy));
        map.insert(BIP39Entropy::name(), || Box::new(BIP39Entropy));
        map.insert(ElectrumV1Entropy::name(), || Box::new(ElectrumV1Entropy));
        map.insert(ElectrumV2Entropy::name(), || Box::new(ElectrumV2Entropy));
        map.insert(MoneroEntropy::name(), || Box::new(MoneroEntropy));

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
            None => Err(EntropyError::new(
                "Invalid entropy name",
                Self::names(),
                name.to_string(),
            )),
        }
    }
}