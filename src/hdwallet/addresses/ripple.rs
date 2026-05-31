use crate::hdwallet::consts::PublicKeyTypes;
use crate::hdwallet::cryptocurrencies::ripple::{Mainnet as RippleMainnet, RIPPLE_PARAMS};
use crate::hdwallet::eccs::{
    validate_and_get_public_key, PublicKeyInput, Slip10Secp256k1PublicKey,
};
use crate::hdwallet::exceptions::PublicKeyError;

use super::iaddress::{
    AddressSchemeError, DecodeKwargs, EncodeKwargs, EncodePublicKey, IAddress,
};
use super::p2pkh::P2PKHAddress;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct RippleAddress;

impl RippleAddress {
    pub const REGISTRY_NAME: &'static str = "Ripple";

    fn alphabet(kwargs_alphabet: Option<&'static str>) -> Option<&'static str> {
        kwargs_alphabet.or(RIPPLE_PARAMS.alphabet)
    }

    fn public_key_address_prefix_encode(kwargs: &EncodeKwargs<'_>) -> u32 {
        kwargs
            .public_key_address_prefix
            .unwrap_or(RippleMainnet::PUBLIC_KEY_ADDRESS_PREFIX)
    }

    fn public_key_address_prefix_decode(kwargs: &DecodeKwargs) -> u32 {
        kwargs
            .public_key_address_prefix
            .unwrap_or(RippleMainnet::PUBLIC_KEY_ADDRESS_PREFIX)
    }

    fn use_compressed_public_key(kwargs: &EncodeKwargs<'_>) -> bool {
        !matches!(
            kwargs.public_key_type,
            Some(PublicKeyTypes::UNCOMPRESSED)
        )
    }
}

impl IAddress for RippleAddress {
    fn name(&self) -> &'static str {
        Self::REGISTRY_NAME
    }

    fn encode(
        &self,
        public_key: EncodePublicKey<'_>,
        kwargs: &EncodeKwargs<'_>,
    ) -> super::iaddress::AddressEncodeResult {
        let input = match public_key {
            EncodePublicKey::Bytes(b) => PublicKeyInput::Bytes(b),
            EncodePublicKey::Str(s) => PublicKeyInput::Hex(s),
        };
        let pk = validate_and_get_public_key::<Slip10Secp256k1PublicKey>(input).map_err(
            |e: PublicKeyError| AddressSchemeError::Msg(e.to_string()),
        )?;
        Ok(P2PKHAddress::encode_public_key(
            &pk,
            Self::public_key_address_prefix_encode(kwargs),
            Self::use_compressed_public_key(kwargs),
            Self::alphabet(kwargs.alphabet),
        ))
    }

    fn decode(&self, address: &str, kwargs: &DecodeKwargs) -> super::iaddress::AddressEncodeResult {
        P2PKHAddress::decode_address(
            address,
            Self::public_key_address_prefix_decode(kwargs),
            Self::alphabet(kwargs.alphabet),
        )
    }
}
