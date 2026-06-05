use std::fmt;

use crate::hdwallet::exceptions::DerivationError;

use super::iderivation::{DerivationData, IDerivation};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CustomDerivation {
    data: DerivationData,
}

impl CustomDerivation {
    pub fn new(path: Option<&str>, indexes: Option<&[u32]>) -> Result<Self, DerivationError> {
        Ok(Self {
            data: DerivationData::new(path, indexes)?,
        })
    }

    pub fn from_path(&mut self, path: &str) -> Result<&mut Self, DerivationError> {
        if !path.starts_with("m/") {
            return Err(DerivationError::new(
                "Bad path format",
                None,
                Some("like this type of path \"m/0'/0\""),
                Some(path),
            ));
        }
        self.data = DerivationData::new(Some(path), None)?;
        Ok(self)
    }

    pub fn from_indexes(&mut self, indexes: &[u32]) -> Result<&mut Self, DerivationError> {
        self.data = DerivationData::new(None, Some(indexes))?;
        Ok(self)
    }

    pub fn from_index(&mut self, index: u32, hardened: bool) -> Result<&mut Self, DerivationError> {
        let segment = if hardened {
            format!("{index}'")
        } else {
            index.to_string()
        };
        let path = if self.data.path() == "m/" {
            format!("m/{segment}")
        } else {
            format!("{}/{}", self.data.path(), segment)
        };
        self.from_path(&path)
    }

    pub fn clean(&mut self) -> Result<&mut Self, DerivationError> {
        self.data = DerivationData::new(None, None)?;
        Ok(self)
    }
}

impl IDerivation for CustomDerivation {
    fn name() -> &'static str {
        "Custom"
    }

    fn data(&self) -> &DerivationData {
        &self.data
    }

    fn data_mut(&mut self) -> &mut DerivationData {
        &mut self.data
    }

    fn clean(&mut self) -> Result<(), DerivationError> {
        self.data = DerivationData::new(None, None)?;
        Ok(())
    }
}

impl fmt::Display for CustomDerivation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.data.fmt(f)
    }
}
