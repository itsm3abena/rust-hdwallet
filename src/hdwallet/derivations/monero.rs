use std::fmt;

use crate::hdwallet::exceptions::DerivationError;

use super::iderivation::{
    index_tuple_to_string, normalize_index, DerivationData, IDerivation, IndexInput, NormalizedIndex,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MoneroDerivation {
    data: DerivationData,
    minor: NormalizedIndex,
    major: NormalizedIndex,
}

impl MoneroDerivation {
    pub fn new(
        minor: Option<IndexInput<'_>>,
        major: Option<IndexInput<'_>>,
    ) -> Result<Self, DerivationError> {
        let minor = normalize_index(minor.unwrap_or(IndexInput::Int(1)), false)?;
        let major = normalize_index(major.unwrap_or(IndexInput::Int(0)), false)?;

        let mut derivation = Self {
            data: DerivationData::default(),
            minor,
            major,
        };
        derivation.rebuild_path()?;
        Ok(derivation)
    }

    pub fn from_minor<'a>(
        &mut self,
        minor: impl Into<IndexInput<'a>>,
    ) -> Result<&mut Self, DerivationError> {
        self.minor = normalize_index(minor.into(), false)?;
        self.rebuild_path()?;
        Ok(self)
    }

    pub fn from_major<'a>(
        &mut self,
        major: impl Into<IndexInput<'a>>,
    ) -> Result<&mut Self, DerivationError> {
        self.major = normalize_index(major.into(), false)?;
        self.rebuild_path()?;
        Ok(self)
    }

    pub fn clean(&mut self) -> Result<&mut Self, DerivationError> {
        self.do_clean()?;
        Ok(self)
    }

    pub fn minor_value(&self) -> u32 {
        self.minor.display_value()
    }

    pub fn major_value(&self) -> u32 {
        self.major.display_value()
    }

    fn rebuild_path(&mut self) -> Result<(), DerivationError> {
        let path = format!(
            "m/{}/{}",
            index_tuple_to_string(self.minor),
            index_tuple_to_string(self.major),
        );
        self.data = DerivationData::new(Some(&path), None)?;
        Ok(())
    }

    fn do_clean(&mut self) -> Result<(), DerivationError> {
        self.minor = normalize_index(IndexInput::Int(1), false)?;
        self.major = normalize_index(IndexInput::Int(0), false)?;
        self.rebuild_path()
    }
}

impl IDerivation for MoneroDerivation {
    fn name() -> &'static str {
        "Monero"
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

    fn minor(&self) -> Option<u32> {
        Some(self.minor_value())
    }

    fn major(&self) -> Option<u32> {
        Some(self.major_value())
    }
}

impl fmt::Display for MoneroDerivation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.data.fmt(f)
    }
}
