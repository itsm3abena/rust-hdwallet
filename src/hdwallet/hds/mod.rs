pub mod bip32;
pub mod bip44;
pub mod bip49;
pub mod bip84;
pub mod bip86;
pub mod ihd;
pub mod algorand;

pub use algorand::AlgorandHd;
pub use bip32::Bip32Hd;
pub use ihd::{Ihd, IndexList};

use crate::hdwallet::exceptions::HDError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HdClass {
    Algorand,
    Bip32,
    Bip44,
    Bip49,
    Bip84,
    Bip86,
    Bip141,
    Cardano,
    ElectrumV1,
    ElectrumV2,
    Monero,
}

impl HdClass {
    pub const ALL: &[HdClass] = &[
        HdClass::Algorand,
        HdClass::Bip32,
        HdClass::Bip44,
        HdClass::Bip49,
        HdClass::Bip84,
        HdClass::Bip86,
        HdClass::Bip141,
        HdClass::Cardano,
        HdClass::ElectrumV1,
        HdClass::ElectrumV2,
        HdClass::Monero,
    ];

    #[must_use]
    pub fn name(self) -> &'static str {
        match self {
            HdClass::Algorand => "Algorand",
            HdClass::Bip32 => "BIP32",
            HdClass::Bip44 => "BIP44",
            HdClass::Bip49 => "BIP49",
            HdClass::Bip84 => "BIP84",
            HdClass::Bip86 => "BIP86",
            HdClass::Bip141 => "BIP141",
            HdClass::Cardano => "Cardano",
            HdClass::ElectrumV1 => "Electrum-V1",
            HdClass::ElectrumV2 => "Electrum-V2",
            HdClass::Monero => "Monero",
        }
    }

    pub fn from_name(name: &str) -> Result<Self, HDError> {
        Self::ALL
            .iter()
            .copied()
            .find(|c| c.name() == name)
            .ok_or_else(|| {
                let expected = Self::expected_names_string();
                HDError::new(
                    "Invalid HD name",
                    None,
                    Some(expected.as_str()),
                    Some(name),
                )
            })
    }

    fn expected_names_string() -> String {
        Self::ALL
            .iter()
            .map(|c| c.name())
            .collect::<Vec<_>>()
            .join(", ")
    }
}

impl std::fmt::Display for HdClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

pub struct Hds;

impl Hds {
    #[must_use]
    pub fn names() -> Vec<&'static str> {
        HdClass::ALL.iter().map(|c| c.name()).collect()
    }

    #[must_use]
    pub fn classes() -> &'static [HdClass] {
        HdClass::ALL
    }

    pub fn hd(name: &str) -> Result<HdClass, HDError> {
        HdClass::from_name(name)
    }

    #[must_use]
    pub fn is_hd(name: &str) -> bool {
        HdClass::from_name(name).is_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hds_names_match_python_registry() {
        assert_eq!(
            Hds::names(),
            vec![
                "Algorand",
                "BIP32",
                "BIP44",
                "BIP49",
                "BIP84",
                "BIP86",
                "BIP141",
                "Cardano",
                "Electrum-V1",
                "Electrum-V2",
                "Monero",
            ]
        );
    }

    #[test]
    fn hd_lookup_and_is_hd() {
        assert_eq!(Hds::hd("BIP44").unwrap(), HdClass::Bip44);
        assert!(Hds::is_hd("Electrum-V2"));
        assert!(!Hds::is_hd("nope"));
        assert!(Hds::hd("nope").is_err());
    }

    #[test]
    fn ihd_default_unsupported_from_seed() {
        struct Stub;
        impl Ihd for Stub {
            fn registry_name(&self) -> &'static str {
                "Stub"
            }
        }
        let mut h = Stub;
        assert!(h.from_seed(b"any").is_err());
        assert_eq!(h.address(), None);
    }
}
