use std::fmt;

use crate::hdwallet::exceptions::DerivationError;

use super::bip44::PurposeDerivation;
use super::iderivation::{
    ChangeInput, ChangeValue, DerivationData, IDerivation, IndexInput,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bip49Derivation(pub(crate) PurposeDerivation);

impl Bip49Derivation {
    pub fn new(
        coin_type: Option<IndexInput<'_>>,
        account: Option<IndexInput<'_>>,
        change: Option<ChangeInput<'_>>,
        address: Option<IndexInput<'_>>,
    ) -> Result<Self, DerivationError> {
        Ok(Self(PurposeDerivation::new(
            49, coin_type, account, change, address,
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

impl IDerivation for Bip49Derivation {
    fn name() -> &'static str {
        "BIP49"
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

impl fmt::Display for Bip49Derivation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.data().fmt(f)
    }
}
