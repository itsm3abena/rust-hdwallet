use std::fmt;

use crate::hdwallet::cryptocurrencies::Bitcoin;
use crate::hdwallet::exceptions::DerivationError;

use super::iderivation::{
    index_tuple_to_string, normalize_index, ChangeInput, ChangeValue, DerivationData, IDerivation,
    IndexInput, NormalizedIndex,
};

pub struct Changes;

impl Changes {
    pub const EXTERNAL_CHAIN: &'static str = "external-chain";
    pub const INTERNAL_CHAIN: &'static str = "internal-chain";
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PurposeDerivation {
    purpose: NormalizedIndex,
    data: DerivationData,
    coin_type: NormalizedIndex,
    account: NormalizedIndex,
    change: NormalizedIndex,
    address: NormalizedIndex,
}

impl PurposeDerivation {
    pub fn new(
        purpose: u32,
        coin_type: Option<IndexInput<'_>>,
        account: Option<IndexInput<'_>>,
        change: Option<ChangeInput<'_>>,
        address: Option<IndexInput<'_>>,
    ) -> Result<Self, DerivationError> {
        let purpose = NormalizedIndex::Single {
            index: purpose,
            hardened: true,
        };
        let coin_type = normalize_index(
            coin_type.unwrap_or(IndexInput::Int(Bitcoin::COIN_TYPE)),
            true,
        )?;
        let account = normalize_index(account.unwrap_or(IndexInput::Int(0)), true)?;
        let change_value = Self::get_change_value(
            change.unwrap_or(ChangeInput::Str(Changes::EXTERNAL_CHAIN)),
            false,
        )?;
        let change = normalize_index(
            IndexInput::Int(match change_value {
                ChangeValue::Index(index) => index,
                ChangeValue::Label(_) => unreachable!(),
            }),
            false,
        )?;
        let address = normalize_index(address.unwrap_or(IndexInput::Int(0)), false)?;

        let mut derivation = Self {
            purpose,
            data: DerivationData::default(),
            coin_type,
            account,
            change,
            address,
        };
        derivation.rebuild_path()?;
        Ok(derivation)
    }

    pub fn get_change_value(
        change: ChangeInput<'_>,
        name_only: bool,
    ) -> Result<ChangeValue, DerivationError> {
        let (label, index) = match change {
            ChangeInput::Str(value) => match value {
                Changes::EXTERNAL_CHAIN | "0" => (Changes::EXTERNAL_CHAIN, 0),
                Changes::INTERNAL_CHAIN | "1" => (Changes::INTERNAL_CHAIN, 1),
                other => {
                    let expected = format!(
                        "{}, {}, {}, {}, {}, {}",
                        Changes::EXTERNAL_CHAIN,
                        0,
                        "'0'",
                        Changes::INTERNAL_CHAIN,
                        1,
                        "'1'"
                    );
                    return Err(DerivationError::new(
                        "Bad BIP44 change index",
                        None,
                        Some(&expected),
                        Some(other),
                    ));
                }
            },
            ChangeInput::Int(value) => match value {
                0 => (Changes::EXTERNAL_CHAIN, 0),
                1 => (Changes::INTERNAL_CHAIN, 1),
                other => {
                    let expected = format!(
                        "{}, {}, {}, {}, {}, {}",
                        Changes::EXTERNAL_CHAIN,
                        0,
                        "'0'",
                        Changes::INTERNAL_CHAIN,
                        1,
                        "'1'"
                    );
                    return Err(DerivationError::new(
                        "Bad BIP44 change index",
                        None,
                        Some(&expected),
                        Some(&other.to_string()),
                    ));
                }
            },
        };

        if name_only {
            Ok(ChangeValue::Label(label))
        } else {
            Ok(ChangeValue::Index(index))
        }
    }

    pub fn from_coin_type(
        &mut self,
        coin_type: IndexInput<'_>,
    ) -> Result<(), DerivationError> {
        self.coin_type = normalize_index(coin_type, true)?;
        self.rebuild_path()
    }

    pub fn from_account(&mut self, account: IndexInput<'_>) -> Result<(), DerivationError> {
        self.account = normalize_index(account, true)?;
        self.rebuild_path()
    }

    pub fn from_change(&mut self, change: ChangeInput<'_>) -> Result<(), DerivationError> {
        let change_value = Self::get_change_value(change, false)?;
        let index = match change_value {
            ChangeValue::Index(index) => index,
            ChangeValue::Label(_) => unreachable!(),
        };
        self.change = normalize_index(IndexInput::Int(index), false)?;
        self.rebuild_path()
    }

    pub fn from_address(&mut self, address: IndexInput<'_>) -> Result<(), DerivationError> {
        self.address = normalize_index(address, false)?;
        self.rebuild_path()
    }

    pub fn do_clean(&mut self) -> Result<(), DerivationError> {
        self.account = normalize_index(IndexInput::Int(0), true)?;
        let change_value =
            Self::get_change_value(ChangeInput::Str(Changes::EXTERNAL_CHAIN), false)?;
        let index = match change_value {
            ChangeValue::Index(index) => index,
            ChangeValue::Label(_) => unreachable!(),
        };
        self.change = normalize_index(IndexInput::Int(index), false)?;
        self.address = normalize_index(IndexInput::Int(0), false)?;
        self.rebuild_path()
    }

    pub fn purpose_value(&self) -> u32 {
        self.purpose.primary()
    }

    pub fn coin_type_value(&self) -> u32 {
        self.coin_type.display_value()
    }

    pub fn account_value(&self) -> u32 {
        self.account.display_value()
    }

    pub fn change_value(&self, name_only: bool) -> Result<ChangeValue, DerivationError> {
        Self::get_change_value(ChangeInput::Int(self.change.primary()), name_only)
    }

    pub fn address_value(&self) -> u32 {
        self.address.display_value()
    }

    pub fn data(&self) -> &DerivationData {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut DerivationData {
        &mut self.data
    }

    fn rebuild_path(&mut self) -> Result<(), DerivationError> {
        let path = format!(
            "m/{}/{}/{}/{}/{}",
            index_tuple_to_string(self.purpose),
            index_tuple_to_string(self.coin_type),
            index_tuple_to_string(self.account),
            index_tuple_to_string(self.change),
            index_tuple_to_string(self.address),
        );
        self.data = DerivationData::new(Some(&path), None)?;
        Ok(())
    }
}

macro_rules! purpose_derivation {
    ($wrapper:ident, $name:literal, $purpose:expr) => {
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct $wrapper(pub(crate) PurposeDerivation);

        impl $wrapper {
            pub fn new(
                coin_type: Option<IndexInput<'_>>,
                account: Option<IndexInput<'_>>,
                change: Option<ChangeInput<'_>>,
                address: Option<IndexInput<'_>>,
            ) -> Result<Self, DerivationError> {
                Ok(Self(PurposeDerivation::new(
                    $purpose, coin_type, account, change, address,
                )?))
            }

            pub fn get_change_value(
                change: ChangeInput<'_>,
                name_only: bool,
            ) -> Result<ChangeValue, DerivationError> {
                PurposeDerivation::get_change_value(change, name_only)
            }

            pub fn from_coin_type<'a>(
                &mut self,
                coin_type: impl Into<IndexInput<'a>>,
            ) -> Result<&mut Self, DerivationError> {
                self.0.from_coin_type(coin_type.into())?;
                Ok(self)
            }

            pub fn from_account<'a>(
                &mut self,
                account: impl Into<IndexInput<'a>>,
            ) -> Result<&mut Self, DerivationError> {
                self.0.from_account(account.into())?;
                Ok(self)
            }

            pub fn from_change<'a>(
                &mut self,
                change: impl Into<ChangeInput<'a>>,
            ) -> Result<&mut Self, DerivationError> {
                self.0.from_change(change.into())?;
                Ok(self)
            }

            pub fn from_address<'a>(
                &mut self,
                address: impl Into<IndexInput<'a>>,
            ) -> Result<&mut Self, DerivationError> {
                self.0.from_address(address.into())?;
                Ok(self)
            }

            pub fn clean(&mut self) -> Result<&mut Self, DerivationError> {
                self.0.do_clean()?;
                Ok(self)
            }

            pub fn purpose_value(&self) -> u32 {
                self.0.purpose_value()
            }

            pub fn coin_type_value(&self) -> u32 {
                self.0.coin_type_value()
            }

            pub fn account_value(&self) -> u32 {
                self.0.account_value()
            }

            pub fn change_value(&self, name_only: bool) -> Result<ChangeValue, DerivationError> {
                self.0.change_value(name_only)
            }

            pub fn address_value(&self) -> u32 {
                self.0.address_value()
            }
        }

        impl IDerivation for $wrapper {
            fn name() -> &'static str {
                $name
            }

            fn data(&self) -> &DerivationData {
                self.0.data()
            }

            fn data_mut(&mut self) -> &mut DerivationData {
                self.0.data_mut()
            }

            fn clean(&mut self) -> Result<(), DerivationError> {
                self.0.do_clean()
            }

            fn purpose(&self) -> Option<u32> {
                Some(self.purpose_value())
            }

            fn coin_type(&self) -> Option<u32> {
                Some(self.coin_type_value())
            }

            fn account(&self) -> Option<u32> {
                Some(self.account_value())
            }

            fn change(&self) -> Option<ChangeValue> {
                self.change_value(true).ok()
            }

            fn address(&self) -> Option<u32> {
                Some(self.address_value())
            }
        }

        impl fmt::Display for $wrapper {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.data().fmt(f)
            }
        }
    };
}

purpose_derivation!(Bip44Derivation, "BIP44", 44);
