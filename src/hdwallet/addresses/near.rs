use crate::hdwallet::eccs::{
    validate_and_get_public_key, IPublicKey, PublicKeyInput, Slip10Ed25519PublicKeyStd,
};
use crate::hdwallet::exceptions::PublicKeyError;

use super::iaddress::{
    AddressSchemeError, DecodeKwargs, EncodeKwargs, EncodePublicKey, IAddress,
};

const PUBKEY_LEN: usize = 32;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct NearAddress;

impl NearAddress {
    pub const REGISTRY_NAME: &'static str = "Near";

    fn pubkey_body(pk: &Slip10Ed25519PublicKeyStd) -> Result<[u8; PUBKEY_LEN], AddressSchemeError> {
        let raw = pk.raw_compressed();
        raw[1..]
            .try_into()
            .map_err(|_| AddressSchemeError::Msg("expected 33-byte compressed SLIP10 Ed25519 key".into()))
    }

    fn encode_hex_body(body: &[u8; PUBKEY_LEN]) -> String {
        hex::encode(body)
    }

    fn decode_address_bytes(address: &str) -> Result<Vec<u8>, AddressSchemeError> {
        let bytes = hex::decode(address).map_err(|e| {
            AddressSchemeError::Msg(format!("invalid Near address hex: {e}"))
        })?;
        if bytes.len() != PUBKEY_LEN {
            return Err(AddressSchemeError::Msg(format!(
                "Invalid length (expected: {PUBKEY_LEN}, got: {})",
                bytes.len()
            )));
        }
        Ok(bytes)
    }
}

impl IAddress for NearAddress {
    fn name(&self) -> &'static str {
        Self::REGISTRY_NAME
    }

    fn encode(
        &self,
        public_key: EncodePublicKey<'_>,
        _kwargs: &EncodeKwargs<'_>,
    ) -> super::iaddress::AddressEncodeResult {
        let input = match public_key {
            EncodePublicKey::Bytes(b) => PublicKeyInput::Bytes(b),
            EncodePublicKey::Str(s) => PublicKeyInput::Hex(s),
        };
        let pk = validate_and_get_public_key::<Slip10Ed25519PublicKeyStd>(input).map_err(
            |e: PublicKeyError| AddressSchemeError::Msg(e.to_string()),
        )?;
        Ok(Self::encode_hex_body(&Self::pubkey_body(&pk)?))
    }

    fn decode(
        &self,
        address: &str,
        _kwargs: &DecodeKwargs,
    ) -> super::iaddress::AddressEncodeResult {
        let bytes = Self::decode_address_bytes(address)?;
        validate_and_get_public_key::<Slip10Ed25519PublicKeyStd>(PublicKeyInput::Bytes(&bytes))
            .map_err(|e: PublicKeyError| AddressSchemeError::Msg(e.to_string()))?;
        Ok(address.to_owned())
    }
}
