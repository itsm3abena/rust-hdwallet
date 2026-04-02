//! Hierarchical deterministic schemes (mirrors `hdwallet/hds/`).

pub mod bip32;
pub mod bip44;
pub mod bip49;
pub mod bip84;
pub mod bip86;

pub use bip32::Bip32Hd;

/// HD class names (mirrors Python `HDS` registry strings).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HdClass {
    Bip32,
    Bip44,
    Bip49,
    Bip84,
    Bip86,
}

impl HdClass {
    #[must_use]
    pub fn name(self) -> &'static str {
        match self {
            HdClass::Bip32 => "BIP32",
            HdClass::Bip44 => "BIP44",
            HdClass::Bip49 => "BIP49",
            HdClass::Bip84 => "BIP84",
            HdClass::Bip86 => "BIP86",
        }
    }
}
