use std::fmt;

use crate::hdwallet::cryptocurrencies::Cardano;
use crate::hdwallet::exceptions::DerivationError;

use super::iderivation::{
    index_tuple_to_string, normalize_index, DerivationData, IDerivation, IndexInput,
    NormalizedIndex,
};

pub struct Roles;

impl Roles {
    pub const EXTERNAL_CHAIN: &'static str = "external-chain";
    pub const INTERNAL_CHAIN: &'static str = "internal-chain";
    pub const STAKING_KEY: &'static str = "staking-key";
}

#[derive(Debug, Clone, Copy)]
pub enum RoleInput<'a> {
    Str(&'a str),
    Int(u32),
}

impl<'a> From<&'a str> for RoleInput<'a> {
    fn from(value: &'a str) -> Self {
        Self::Str(value)
    }
}

impl From<u32> for RoleInput<'_> {
    fn from(value: u32) -> Self {
        Self::Int(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoleValue {
    Index(u32),
    Label(&'static str),
}

const PURPOSE: NormalizedIndex = NormalizedIndex::Single {
    index: 1852,
    hardened: true,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cip1852Derivation {
    data: DerivationData,
    coin_type: NormalizedIndex,
    account: NormalizedIndex,
    role: NormalizedIndex,
    address: NormalizedIndex,
}

impl Cip1852Derivation {
    pub fn new(
        coin_type: Option<IndexInput<'_>>,
        account: Option<IndexInput<'_>>,
        role: Option<RoleInput<'_>>,
        address: Option<IndexInput<'_>>,
    ) -> Result<Self, DerivationError> {
        let coin_type = normalize_index(
            coin_type.unwrap_or(IndexInput::Int(Cardano::COIN_TYPE)),
            true,
        )?;
        let account = normalize_index(account.unwrap_or(IndexInput::Int(0)), true)?;
        let role_value = Self::get_role_value(
            role.unwrap_or(RoleInput::Str(Roles::EXTERNAL_CHAIN)),
            false,
        )?;
        let role = normalize_index(
            IndexInput::Int(match role_value {
                RoleValue::Index(index) => index,
                RoleValue::Label(_) => unreachable!(),
            }),
            false,
        )?;
        let address = normalize_index(address.unwrap_or(IndexInput::Int(0)), false)?;

        let mut derivation = Self {
            data: DerivationData::default(),
            coin_type,
            account,
            role,
            address,
        };
        derivation.rebuild_path()?;
        Ok(derivation)
    }

    pub fn get_role_value(role: RoleInput<'_>, name_only: bool) -> Result<RoleValue, DerivationError> {
        let (label, index) = match role {
            RoleInput::Str(value) => match value {
                Roles::EXTERNAL_CHAIN | "0" => (Roles::EXTERNAL_CHAIN, 0),
                Roles::INTERNAL_CHAIN | "1" => (Roles::INTERNAL_CHAIN, 1),
                Roles::STAKING_KEY | "2" => (Roles::STAKING_KEY, 2),
                other => {
                    let expected = format!(
                        "{}, {}, {}, {}, {}, {}, {}, {}, {}",
                        Roles::EXTERNAL_CHAIN,
                        0,
                        "'0'",
                        Roles::INTERNAL_CHAIN,
                        1,
                        "'1'",
                        Roles::STAKING_KEY,
                        2,
                        "'2'"
                    );
                    return Err(DerivationError::new(
                        "Bad CIP1852 role index",
                        None,
                        Some(&expected),
                        Some(other),
                    ));
                }
            },
            RoleInput::Int(value) => match value {
                0 => (Roles::EXTERNAL_CHAIN, 0),
                1 => (Roles::INTERNAL_CHAIN, 1),
                2 => (Roles::STAKING_KEY, 2),
                other => {
                    let expected = format!(
                        "{}, {}, {}, {}, {}, {}, {}, {}, {}",
                        Roles::EXTERNAL_CHAIN,
                        0,
                        "'0'",
                        Roles::INTERNAL_CHAIN,
                        1,
                        "'1'",
                        Roles::STAKING_KEY,
                        2,
                        "'2'"
                    );
                    return Err(DerivationError::new(
                        "Bad CIP1852 role index",
                        None,
                        Some(&expected),
                        Some(&other.to_string()),
                    ));
                }
            },
        };

        if name_only {
            Ok(RoleValue::Label(label))
        } else {
            Ok(RoleValue::Index(index))
        }
    }

    pub fn from_coin_type<'a>(
        &mut self,
        coin_type: impl Into<IndexInput<'a>>,
    ) -> Result<&mut Self, DerivationError> {
        self.coin_type = normalize_index(coin_type.into(), true)?;
        self.rebuild_path()?;
        Ok(self)
    }

    pub fn from_account<'a>(
        &mut self,
        account: impl Into<IndexInput<'a>>,
    ) -> Result<&mut Self, DerivationError> {
        self.account = normalize_index(account.into(), true)?;
        self.rebuild_path()?;
        Ok(self)
    }

    pub fn from_role<'a>(
        &mut self,
        role: impl Into<RoleInput<'a>>,
    ) -> Result<&mut Self, DerivationError> {
        let role_value = Self::get_role_value(role.into(), false)?;
        let index = match role_value {
            RoleValue::Index(index) => index,
            RoleValue::Label(_) => unreachable!(),
        };
        self.role = normalize_index(IndexInput::Int(index), false)?;
        self.rebuild_path()?;
        Ok(self)
    }

    pub fn from_address<'a>(
        &mut self,
        address: impl Into<IndexInput<'a>>,
    ) -> Result<&mut Self, DerivationError> {
        self.address = normalize_index(address.into(), false)?;
        self.rebuild_path()?;
        Ok(self)
    }

    pub fn clean(&mut self) -> Result<&mut Self, DerivationError> {
        self.do_clean()?;
        Ok(self)
    }

    pub fn purpose_value(&self) -> u32 {
        PURPOSE.primary()
    }

    pub fn coin_type_value(&self) -> u32 {
        self.coin_type.display_value()
    }

    pub fn account_value(&self) -> u32 {
        self.account.display_value()
    }

    pub fn role_value(&self, name_only: bool) -> Result<RoleValue, DerivationError> {
        Self::get_role_value(RoleInput::Int(self.role.primary()), name_only)
    }

    pub fn address_value(&self) -> u32 {
        self.address.display_value()
    }

    fn rebuild_path(&mut self) -> Result<(), DerivationError> {
        let path = format!(
            "m/{}/{}/{}/{}/{}",
            index_tuple_to_string(PURPOSE),
            index_tuple_to_string(self.coin_type),
            index_tuple_to_string(self.account),
            index_tuple_to_string(self.role),
            index_tuple_to_string(self.address),
        );
        self.data = DerivationData::new(Some(&path), None)?;
        Ok(())
    }

    fn do_clean(&mut self) -> Result<(), DerivationError> {
        self.account = normalize_index(IndexInput::Int(0), true)?;
        let role_value = Self::get_role_value(RoleInput::Str(Roles::EXTERNAL_CHAIN), false)?;
        let index = match role_value {
            RoleValue::Index(index) => index,
            RoleValue::Label(_) => unreachable!(),
        };
        self.role = normalize_index(IndexInput::Int(index), false)?;
        self.address = normalize_index(IndexInput::Int(0), false)?;
        self.rebuild_path()
    }
}

impl IDerivation for Cip1852Derivation {
    fn name() -> &'static str {
        "CIP1852"
    }

    fn data(&self) -> &DerivationData {
        &self.data
    }

    fn data_mut(&mut self) -> &mut DerivationData {
        &mut self.data
    }

    fn clean(&mut self) -> Result<(), DerivationError> {
        self.do_clean()
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

    fn role(&self) -> Option<&str> {
        match self.role_value(true).ok()? {
            RoleValue::Label(label) => Some(label),
            RoleValue::Index(_) => None,
        }
    }

    fn address(&self) -> Option<u32> {
        Some(self.address_value())
    }
}

impl fmt::Display for Cip1852Derivation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.data.fmt(f)
    }
}
