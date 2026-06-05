mod bip44;
mod bip49;
mod bip84;
mod bip86;
mod cip1852;
mod custom;
mod electrum;
mod hdw;
mod iderivation;
mod monero;

pub use bip44::{Bip44Derivation, Changes as CHANGES};
pub use bip49::Bip49Derivation;
pub use bip84::Bip84Derivation;
pub use bip86::Bip86Derivation;
pub use cip1852::{Cip1852Derivation, RoleInput, RoleValue, Roles as ROLES};
pub use custom::CustomDerivation;
pub use electrum::ElectrumDerivation;
pub use hdw::{EccInput, EccValue, HdwDerivation};
pub use iderivation::{
    index_tuple_to_string, normalize_derivation, normalize_index, ChangeInput, ChangeValue,
    DerivationData, DerivationStep, IDerivation, IndexInput, NormalizedIndex,
};
pub use monero::MoneroDerivation;

use std::fmt;

use crate::hdwallet::exceptions::DerivationError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DerivationKind {
    Bip44,
    Bip49,
    Bip84,
    Bip86,
    Cip1852,
    Custom,
    Electrum,
    Monero,
    Hdw,
}

impl DerivationKind {
    pub const ALL: &[DerivationKind] = &[
        DerivationKind::Bip44,
        DerivationKind::Bip49,
        DerivationKind::Bip84,
        DerivationKind::Bip86,
        DerivationKind::Cip1852,
        DerivationKind::Custom,
        DerivationKind::Electrum,
        DerivationKind::Monero,
        DerivationKind::Hdw,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Self::Bip44 => Bip44Derivation::name(),
            Self::Bip49 => Bip49Derivation::name(),
            Self::Bip84 => Bip84Derivation::name(),
            Self::Bip86 => Bip86Derivation::name(),
            Self::Cip1852 => Cip1852Derivation::name(),
            Self::Custom => CustomDerivation::name(),
            Self::Electrum => ElectrumDerivation::name(),
            Self::Monero => MoneroDerivation::name(),
            Self::Hdw => HdwDerivation::name(),
        }
    }

    pub fn from_name(name: &str) -> Result<Self, DerivationError> {
        Self::ALL
            .iter()
            .copied()
            .find(|kind| kind.name() == name)
            .ok_or_else(|| {
                DerivationError::new(
                    "Invalid derivation name",
                    None,
                    Some(&Self::expected_names()),
                    Some(name),
                )
            })
    }

    fn expected_names() -> String {
        Self::ALL
            .iter()
            .map(|kind| kind.name())
            .collect::<Vec<_>>()
            .join(", ")
    }
}

impl fmt::Display for DerivationKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Derivations;

pub type DERIVATIONS = Derivations;

impl Derivations {
    pub fn names() -> Vec<&'static str> {
        DerivationKind::ALL.iter().map(|kind| kind.name()).collect()
    }

    pub fn classes() -> &'static [DerivationKind] {
        DerivationKind::ALL
    }

    pub fn derivation(name: &str) -> Result<DerivationKind, DerivationError> {
        DerivationKind::from_name(name)
    }

    pub fn is_derivation(name: &str) -> bool {
        DerivationKind::ALL
            .iter()
            .any(|kind| kind.name() == name)
    }
}
