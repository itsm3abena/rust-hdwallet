use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
pub struct MnemonicError {
    pub message: String,
}

impl MnemonicError {
    pub fn new(msg: &str) -> Self {
        Self {
            message: msg.to_string(),
        }
    }
}

impl fmt::Display for MnemonicError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MnemonicError: {}", self.message)
    }
}

impl std::error::Error for MnemonicError {}

pub trait Mnemonic {
    fn name(&self) -> &'static str;
}

pub struct AlgorandMnemonic;
pub struct BIP39Mnemonic;
pub struct ElectrumV1Mnemonic;
pub struct ElectrumV2Mnemonic;
pub struct MoneroMnemonic;

impl Mnemonic for AlgorandMnemonic {
    fn name(&self) -> &'static str {
        "Algorand"
    }
}

impl Mnemonic for BIP39Mnemonic {
    fn name(&self) -> &'static str {
        "BIP39"
    }
}

impl Mnemonic for ElectrumV1Mnemonic {
    fn name(&self) -> &'static str {
        "Electrum-V1"
    }
}

impl Mnemonic for ElectrumV2Mnemonic {
    fn name(&self) -> &'static str {
        "Electrum-V2"
    }
}

impl Mnemonic for MoneroMnemonic {
    fn name(&self) -> &'static str {
        "Monero"
    }
}

type MnemonicFactory = fn() -> Box<dyn Mnemonic>;

pub struct Mnemonics;

impl Mnemonics {
    fn registry() -> HashMap<&'static str, MnemonicFactory> {
        let mut map: HashMap<&'static str, MnemonicFactory> = HashMap::new();

        map.insert("Algorand", || Box::new(AlgorandMnemonic));
        map.insert("BIP39", || Box::new(BIP39Mnemonic));
        map.insert("Electrum-V1", || Box::new(ElectrumV1Mnemonic));
        map.insert("Electrum-V2", || Box::new(ElectrumV2Mnemonic));
        map.insert("Monero", || Box::new(MoneroMnemonic));

        map
    }

    pub fn names() -> Vec<&'static str> {
        Self::registry().keys().cloned().collect()
    }

    pub fn is_mnemonic(name: &str) -> bool {
        Self::registry().contains_key(name)
    }

    pub fn mnemonic(name: &str) -> Result<Box<dyn Mnemonic>, MnemonicError> {
        let registry = Self::registry();

        match registry.get(name) {
            Some(factory) => Ok(factory()),
            None => Err(MnemonicError::new("Invalid mnemonic name")),
        }
    }
}