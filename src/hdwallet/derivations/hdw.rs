use std::fmt;

use crate::hdwallet::eccs::{
    KholawEd25519Ecc, Slip10Ed25519Blake2bEcc, Slip10Ed25519Ecc, Slip10Ed25519MoneroEcc,
    Slip10Nist256p1Ecc, Slip10Secp256k1Ecc,
};
use crate::hdwallet::exceptions::DerivationError;

use super::iderivation::{
    index_tuple_to_string, normalize_index, DerivationData, IDerivation, IndexInput, NormalizedIndex,
};

#[derive(Debug, Clone, Copy)]
pub enum EccInput<'a> {
    Name(&'static str),
    Str(&'a str),
    Int(u32),
}

impl From<u32> for EccInput<'_> {
    fn from(value: u32) -> Self {
        Self::Int(value)
    }
}

impl<'a> From<&'a str> for EccInput<'a> {
    fn from(value: &'a str) -> Self {
        Self::Str(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EccValue {
    Index(u32),
    Name(&'static str),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HdwDerivation {
    data: DerivationData,
    account: NormalizedIndex,
    ecc: NormalizedIndex,
    address: NormalizedIndex,
}

impl HdwDerivation {
    pub fn new(
        account: Option<IndexInput<'_>>,
        ecc: Option<EccInput<'_>>,
        address: Option<IndexInput<'_>>,
    ) -> Result<Self, DerivationError> {
        let account = normalize_index(account.unwrap_or(IndexInput::Int(0)), true)?;
        let ecc_value = Self::get_ecc_value(
            ecc.unwrap_or(EccInput::Name(Slip10Secp256k1Ecc::NAME)),
            false,
        )?;
        let ecc = normalize_index(
            IndexInput::Int(match ecc_value {
                EccValue::Index(index) => index,
                EccValue::Name(_) => unreachable!(),
            }),
            false,
        )?;
        let address = normalize_index(address.unwrap_or(IndexInput::Int(0)), false)?;

        let mut derivation = Self {
            data: DerivationData::default(),
            account,
            ecc,
            address,
        };
        derivation.rebuild_path()?;
        Ok(derivation)
    }

    pub fn get_ecc_value(ecc: EccInput<'_>, name_only: bool) -> Result<EccValue, DerivationError> {
        let curve = match ecc {
            EccInput::Name(name) => name,
            EccInput::Str(value) => value,
            EccInput::Int(value) => match value {
                0 => Slip10Secp256k1Ecc::NAME,
                1 => Slip10Ed25519Ecc::NAME,
                2 => Slip10Nist256p1Ecc::NAME,
                3 => KholawEd25519Ecc::NAME,
                4 => Slip10Ed25519Blake2bEcc::NAME,
                5 => Slip10Ed25519MoneroEcc::NAME,
                other => {
                    let expected = Self::expected_ecc_list();
                    return Err(DerivationError::new(
                        "Bad HDW ECC index",
                        None,
                        Some(&expected),
                        Some(&other.to_string()),
                    ));
                }
            },
        };

        let (name, index) = match curve {
            name if name == Slip10Secp256k1Ecc::NAME || name == "0" => {
                (Slip10Secp256k1Ecc::NAME, 0)
            }
            name if name == Slip10Ed25519Ecc::NAME || name == "1" => (Slip10Ed25519Ecc::NAME, 1),
            name if name == Slip10Nist256p1Ecc::NAME || name == "2" => {
                (Slip10Nist256p1Ecc::NAME, 2)
            }
            name if name == KholawEd25519Ecc::NAME || name == "3" => (KholawEd25519Ecc::NAME, 3),
            name if name == Slip10Ed25519Blake2bEcc::NAME || name == "4" => {
                (Slip10Ed25519Blake2bEcc::NAME, 4)
            }
            name if name == Slip10Ed25519MoneroEcc::NAME || name == "5" => {
                (Slip10Ed25519MoneroEcc::NAME, 5)
            }
            other => {
                let expected = Self::expected_ecc_list();
                return Err(DerivationError::new(
                    "Bad HDW ECC index",
                    None,
                    Some(&expected),
                    Some(other),
                ));
            }
        };

        if name_only {
            Ok(EccValue::Name(name))
        } else {
            Ok(EccValue::Index(index))
        }
    }

    pub fn from_account<'a>(
        &mut self,
        account: impl Into<IndexInput<'a>>,
    ) -> Result<&mut Self, DerivationError> {
        self.account = normalize_index(account.into(), true)?;
        self.rebuild_path()?;
        Ok(self)
    }

    pub fn from_ecc<'a>(
        &mut self,
        ecc: impl Into<EccInput<'a>>,
    ) -> Result<&mut Self, DerivationError> {
        let ecc_value = Self::get_ecc_value(ecc.into(), false)?;
        let index = match ecc_value {
            EccValue::Index(index) => index,
            EccValue::Name(_) => unreachable!(),
        };
        self.ecc = normalize_index(IndexInput::Int(index), false)?;
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

    pub fn account_value(&self) -> u32 {
        self.account.display_value()
    }

    pub fn ecc_value(&self, name_only: bool) -> Result<EccValue, DerivationError> {
        Self::get_ecc_value(EccInput::Int(self.ecc.primary()), name_only)
    }

    pub fn address_value(&self) -> u32 {
        self.address.display_value()
    }

    fn rebuild_path(&mut self) -> Result<(), DerivationError> {
        let path = format!(
            "m/{}/{}/{}",
            index_tuple_to_string(self.account),
            index_tuple_to_string(self.ecc),
            index_tuple_to_string(self.address),
        );
        self.data = DerivationData::new(Some(&path), None)?;
        Ok(())
    }

    fn do_clean(&mut self) -> Result<(), DerivationError> {
        self.account = normalize_index(IndexInput::Int(0), true)?;
        self.address = normalize_index(IndexInput::Int(0), false)?;
        self.rebuild_path()
    }

    fn expected_ecc_list() -> String {
        format!(
            "{}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}",
            Slip10Secp256k1Ecc::NAME,
            0,
            "'0'",
            Slip10Ed25519Ecc::NAME,
            1,
            "'1'",
            Slip10Nist256p1Ecc::NAME,
            2,
            "'2'",
            KholawEd25519Ecc::NAME,
            3,
            "'3'",
            Slip10Ed25519Blake2bEcc::NAME,
            4,
            "'4'",
            Slip10Ed25519MoneroEcc::NAME,
            5,
            "'5'"
        )
    }
}

impl IDerivation for HdwDerivation {
    fn name() -> &'static str {
        "HDW"
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

    fn account(&self) -> Option<u32> {
        Some(self.account_value())
    }

    fn address(&self) -> Option<u32> {
        Some(self.address_value())
    }
}

impl fmt::Display for HdwDerivation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.data.fmt(f)
    }
}
