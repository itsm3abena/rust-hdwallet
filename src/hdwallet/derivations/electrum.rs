use std::fmt;

use crate::hdwallet::exceptions::DerivationError;

use super::iderivation::{
    index_tuple_to_string, normalize_index, ChangeValue, DerivationData, IDerivation, IndexInput,
    NormalizedIndex,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ElectrumDerivation {
    data: DerivationData,
    change: NormalizedIndex,
    address: NormalizedIndex,
}

impl ElectrumDerivation {
    pub fn new(
        change: Option<IndexInput<'_>>,
        address: Option<IndexInput<'_>>,
    ) -> Result<Self, DerivationError> {
        let change = normalize_index(change.unwrap_or(IndexInput::Int(0)), false)?;
        let address = normalize_index(address.unwrap_or(IndexInput::Int(0)), false)?;

        let mut derivation = Self {
            data: DerivationData::default(),
            change,
            address,
        };
        derivation.rebuild_path()?;
        Ok(derivation)
    }

    pub fn from_change<'a>(
        &mut self,
        change: impl Into<IndexInput<'a>>,
    ) -> Result<&mut Self, DerivationError> {
        self.change = normalize_index(change.into(), false)?;
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

    pub fn change_value(&self) -> u32 {
        self.change.display_value()
    }

    pub fn address_value(&self) -> u32 {
        self.address.display_value()
    }

    fn rebuild_path(&mut self) -> Result<(), DerivationError> {
        let path = format!(
            "m/{}/{}",
            index_tuple_to_string(self.change),
            index_tuple_to_string(self.address),
        );
        self.data = DerivationData::new(Some(&path), None)?;
        Ok(())
    }

    fn do_clean(&mut self) -> Result<(), DerivationError> {
        self.change = normalize_index(IndexInput::Int(0), false)?;
        self.address = normalize_index(IndexInput::Int(0), false)?;
        self.rebuild_path()
    }
}

impl IDerivation for ElectrumDerivation {
    fn name() -> &'static str {
        "Electrum"
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

    fn change(&self) -> Option<ChangeValue> {
        Some(ChangeValue::Index(self.change_value()))
    }

    fn address(&self) -> Option<u32> {
        Some(self.address_value())
    }
}

impl fmt::Display for ElectrumDerivation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.data.fmt(f)
    }
}
