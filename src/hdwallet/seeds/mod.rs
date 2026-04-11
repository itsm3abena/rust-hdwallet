use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::exceptions::SeedError;

pub mod electrum;
pub mod iseed;
pub mod monero;

pub trait ISeed {
    fn name() -> &'static str
    where
        Self: Sized;
}

use crate::seeds::algorand::AlgorandSeed;
use crate::seeds::bip39::BIP39Seed;
use crate::seeds::cardano::CardanoSeed;
use crate::seeds::electrum::{ElectrumV1Seed, ElectrumV2Seed};
use crate::seeds::monero::MoneroSeed;


pub static SEEDS: Lazy<HashMap<&'static str, fn() -> Box<dyn ISeed>>> = Lazy::new(|| {
    let mut m: HashMap<&'static str, fn() -> Box<dyn ISeed>> = HashMap::new();

    m.insert(AlgorandSeed::name(), || Box::new(AlgorandSeed));
    m.insert(BIP39Seed::name(), || Box::new(BIP39Seed));
    m.insert(CardanoSeed::name(), || Box::new(CardanoSeed));
    m.insert(ElectrumV1Seed::name(), || Box::new(ElectrumV1Seed));
    m.insert(ElectrumV2Seed::name(), || Box::new(ElectrumV2Seed));
    m.insert(MoneroSeed::name(), || Box::new(MoneroSeed));

    m
});


pub fn names() -> Vec<&'static str> {
    SEEDS.keys().cloned().collect()
}

pub fn classes() -> Vec<fn() -> Box<dyn ISeed>> {
    SEEDS.values().cloned().collect()
}

pub fn seed(name: &str) -> Result<Box<dyn ISeed>, SeedError> {
    match SEEDS.get(name) {
        Some(constructor) => Ok(constructor()),
        None => Err(SeedError::new(
            "Invalid seed name",
            Some(format!("{:?}", names())),
            Some(name.to_string()),
        )),
    }
}

pub fn is_seed(name: &str) -> bool {
    SEEDS.contains_key(name)
}