use std::fmt;

use thiserror::Error;

use crate::hdwallet::eccs::{EccsError, IPublicKey};

#[derive(Debug, Error)]
pub enum AddressSchemeError {
    #[error("{scheme}: encode/decode not implemented")]
    Unimplemented { scheme: &'static str },
    #[error(transparent)]
    Eccs(#[from] EccsError),
    #[error("{0}")]
    Msg(String),
}

pub type AddressEncodeResult = std::result::Result<String, AddressSchemeError>;

pub type AddressDecodeResult = AddressEncodeResult;

#[derive(Debug, Default, Clone, Copy)]
#[non_exhaustive]
pub struct EncodeKwargs<'a> {
    pub hrp: Option<&'static str>,
    pub address_prefix: Option<&'static str>,
    pub address_type: Option<&'static str>,
    pub public_key_type: Option<&'static str>,
    pub encode_type: Option<&'static str>,
    pub decode_type: Option<&'static str>,
    pub network: Option<&'static str>,
    pub path: Option<&'a str>,
    pub chain_code: Option<&'a [u8]>,
    pub path_key: Option<&'a [u8]>,
    pub staking_public_key: Option<&'a [u8]>,
    pub skip_checksum_encode: Option<bool>,
    pub public_key_address_prefix: Option<u32>,
    pub script_address_prefix: Option<u32>,
    pub witness_version: Option<u8>,
    pub alphabet: Option<&'static str>,
    pub bounceable: Option<bool>,
    pub ton_testnet: Option<bool>,
    pub workchain: Option<i8>,
    pub wallet_version: Option<&'static str>,
    pub wallet_id: Option<i64>,
    pub network_global_id: Option<i64>,
    pub subwallet_number: Option<u32>,
}

#[derive(Debug, Default, Clone, Copy)]
#[non_exhaustive]
pub struct DecodeKwargs {
    pub hrp: Option<&'static str>,
    pub address_prefix: Option<&'static str>,
    pub address_type: Option<&'static str>,
    pub decode_type: Option<&'static str>,
    pub network: Option<&'static str>,
    pub skip_checksum_encode: Option<bool>,
    pub public_key_address_prefix: Option<u32>,
    pub script_address_prefix: Option<u32>,
    pub witness_version: Option<u8>,
    pub alphabet: Option<&'static str>,
}

#[derive(Debug, Clone, Copy)]
pub enum EncodePublicKey<'a> {
    Bytes(&'a [u8]),
    Str(&'a str),
}

pub type AddressPublicKeyInput<'a> = EncodePublicKey<'a>;

impl<'a> EncodePublicKey<'a> {
    pub fn as_bytes(self) -> std::result::Result<&'a [u8], AddressSchemeError> {
        match self {
            Self::Bytes(b) => Ok(b),
            Self::Str(_) => Err(AddressSchemeError::Msg(
                "hex public key must be resolved by the address implementation".into(),
            )),
        }
    }
}

impl<'a> From<&'a [u8]> for EncodePublicKey<'a> {
    fn from(value: &'a [u8]) -> Self {
        Self::Bytes(value)
    }
}

impl<'a, const N: usize> From<&'a [u8; N]> for EncodePublicKey<'a> {
    fn from(value: &'a [u8; N]) -> Self {
        Self::Bytes(value.as_slice())
    }
}

impl<'a> From<&'a str> for EncodePublicKey<'a> {
    fn from(value: &'a str) -> Self {
        Self::Str(value)
    }
}

pub trait IAddress: Send + Sync + 'static {
    fn name(&self) -> &'static str;

    fn encode(
        &self,
        public_key: EncodePublicKey<'_>,
        kwargs: &EncodeKwargs<'_>,
    ) -> AddressEncodeResult {
        let _ = (public_key, kwargs);
        Err(AddressSchemeError::Unimplemented {
            scheme: self.name(),
        })
    }

    fn decode(&self, address: &str, kwargs: &DecodeKwargs) -> AddressDecodeResult {
        let _ = (address, kwargs);
        Err(AddressSchemeError::Unimplemented {
            scheme: self.name(),
        })
    }
}

pub fn encode_address<'a>(
    scheme: &dyn IAddress,
    public_key: impl Into<EncodePublicKey<'a>>,
    kwargs: &EncodeKwargs<'_>,
) -> AddressEncodeResult {
    scheme.encode(public_key.into(), kwargs)
}

pub fn decode_address(
    scheme: &dyn IAddress,
    address: &str,
    kwargs: &DecodeKwargs,
) -> AddressDecodeResult {
    scheme.decode(address, kwargs)
}

pub fn encode_with_ipublic_key<P: IPublicKey>(
    scheme: &dyn IAddress,
    pk: &P,
    kwargs: &EncodeKwargs<'_>,
) -> AddressEncodeResult {
    let bytes = pk.raw_compressed();
    scheme.encode(EncodePublicKey::Bytes(&bytes), kwargs)
}

impl fmt::Display for EncodePublicKey<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EncodePublicKey::Bytes(b) => write!(f, "Bytes(len={})", b.len()),
            EncodePublicKey::Str(s) => write!(f, "Str(len={})", s.len()),
        }
    }
}
