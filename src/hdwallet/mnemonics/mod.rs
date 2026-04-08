pub mod algorand;
pub mod bip39;
pub mod electrum;
pub mod imnemonic;
pub mod monero;

pub use algorand::{AlgorandMnemonic, AlgorandMnemonicLanguages, AlgorandMnemonicWords};
pub use bip39::{BIP39Mnemonic, BIP39MnemonicLanguages, BIP39MnemonicWords};
pub use electrum::{
    ElectrumV1Mnemonic, ElectrumV2Mnemonic, ElectrumV1MnemonicLanguages,
    ElectrumV1MnemonicWordCounts, ElectrumV2MnemonicLanguages, ElectrumV2MnemonicTypes,
    ElectrumV2MnemonicWordCounts,
};
pub use imnemonic::IMnemonic;
pub use monero::{
    MoneroMnemonic, MoneroMnemonicLanguages, MoneroMnemonicWordCounts,
};

use crate::hdwallet::exceptions::MnemonicError;

pub struct Mnemonics;

impl Mnemonics {
    pub fn names() -> Vec<&'static str> {
        vec![
            BIP39Mnemonic::name(),
            MoneroMnemonic::name(),
            ElectrumV1Mnemonic::name(),
            ElectrumV2Mnemonic::name(),
            AlgorandMnemonic::name(),
        ]
    }

    pub fn is_mnemonic(name: &str) -> bool {
        matches!(
            name,
            "BIP39" | "Monero" | "Electrum-V1" | "Electrum-V2" | "Algorand"
        )
    }

    pub fn ensure(name: &str) -> Result<(), MnemonicError> {
        if Self::is_mnemonic(name) {
            Ok(())
        } else {
            Err(MnemonicError::new(
                "Invalid mnemonic name",
                None,
                Some(&Self::names().join(", ")),
                Some(name),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use imnemonic::IMnemonic;

    #[test]
    fn registry_lists_all() {
        let n = Mnemonics::names();
        assert!(n.contains(&"BIP39"));
        assert!(n.contains(&"Monero"));
        Mnemonics::ensure("Electrum-V1").unwrap();
    }

    #[test]
    fn bip39_twelve_words() {
        let s = BIP39Mnemonic::from_words(12, BIP39MnemonicLanguages::ENGLISH).unwrap();
        assert_eq!(s.split_whitespace().count(), 12);
    }

    #[test]
    fn electrum_v1_twelve() {
        let s = ElectrumV1Mnemonic::from_words(12, ElectrumV1MnemonicLanguages::ENGLISH).unwrap();
        assert_eq!(s.split_whitespace().count(), 12);
    }

    #[test]
    fn algorand_twenty_five() {
        let s = AlgorandMnemonic::from_words(25, AlgorandMnemonicLanguages::ENGLISH).unwrap();
        assert_eq!(s.split_whitespace().count(), 25);
    }

    #[test]
    fn monero_thirteen_english() {
        let s = MoneroMnemonic::from_words(13, MoneroMnemonicLanguages::ENGLISH).unwrap();
        assert_eq!(s.split_whitespace().count(), 13);
    }

    #[test]
    fn electrum_v2_twelve_standard() {
        let m = ElectrumV2Mnemonic::generate_random(
            12,
            ElectrumV2MnemonicLanguages::ENGLISH,
            ElectrumV2MnemonicTypes::STANDARD,
        )
        .unwrap();
        assert_eq!(m.phrase().split_whitespace().count(), 12);
    }
}
