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

#[derive(Debug, Default, Clone, Copy)]
#[non_exhaustive]
pub struct EncodeKwargs<'a> {
    pub hrp: Option<&'static str>,
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
}

#[derive(Debug, Default, Clone, Copy)]
#[non_exhaustive]
pub struct DecodeKwargs {
    pub hrp: Option<&'static str>,
    pub address_type: Option<&'static str>,
    pub decode_type: Option<&'static str>,
    pub network: Option<&'static str>,
    pub skip_checksum_encode: Option<bool>,
}

#[derive(Debug, Clone, Copy)]
pub enum EncodePublicKey<'a> {
    Bytes(&'a [u8]),
    Str(&'a str),
}

impl<'a> EncodePublicKey<'a> {
    pub fn as_bytes(self) -> std::result::Result<&'a [u8], AddressSchemeError> {
        match self {
            Self::Bytes(b) => Ok(b),
            Self::Str(_) => Err(AddressSchemeError::Msg(
                "EncodePublicKey::Str must be resolved by the address implementation".into(),
            )),
        }
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

    fn decode(&self, address: &str, kwargs: &DecodeKwargs) -> AddressEncodeResult {
        let _ = (address, kwargs);
        Err(AddressSchemeError::Unimplemented {
            scheme: self.name(),
        })
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Copy)]
    struct Dummy;

    impl IAddress for Dummy {
        fn name(&self) -> &'static str {
            "Dummy"
        }
    }

    #[test]
    fn default_encode_is_unimplemented() {
        let d = Dummy;
        let r = d.encode(EncodePublicKey::Bytes(&[1, 2, 3]), &EncodeKwargs::default());
        assert!(matches!(
            r,
            Err(AddressSchemeError::Unimplemented { scheme: "Dummy" })
        ));
    }
}
