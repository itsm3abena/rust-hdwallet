use crate::hdwallet::cryptocurrencies::sui::SUI_PARAMS;
use crate::hdwallet::eccs::{
    validate_and_get_public_key, IPublicKey, PublicKeyInput, Slip10Ed25519PublicKeyStd,
};
use crate::hdwallet::exceptions::PublicKeyError;
use crate::hdwallet::utils::{blake2b_256, integer_to_bytes_python};

use super::iaddress::{
    AddressSchemeError, DecodeKwargs, EncodeKwargs, EncodePublicKey, IAddress,
};

const PUBKEY_LEN: usize = 32;
const HASH_HEX_LEN: usize = 64;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct SuiAddress;

impl SuiAddress {
    pub const REGISTRY_NAME: &'static str = "Sui";

    fn address_prefix() -> &'static str {
        SUI_PARAMS.address_prefix.unwrap_or("0x")
    }

    fn key_type_bytes() -> Vec<u8> {
        integer_to_bytes_python(u32::from(SUI_PARAMS.key_type.unwrap_or(0)), None)
    }

    fn pubkey_body(pk: &Slip10Ed25519PublicKeyStd) -> Result<[u8; PUBKEY_LEN], AddressSchemeError> {
        let raw = pk.raw_compressed();
        raw[1..]
            .try_into()
            .map_err(|_| AddressSchemeError::Msg("expected 33-byte compressed SLIP10 Ed25519 key".into()))
    }
}

impl IAddress for SuiAddress {
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
        let body = Self::pubkey_body(&pk)?;
        let mut payload = Self::key_type_bytes();
        payload.extend_from_slice(&body);
        let hash = blake2b_256(&payload);
        Ok(format!("{}{}", Self::address_prefix(), hex::encode(hash)))
    }

    fn decode(&self, address: &str, _kwargs: &DecodeKwargs) -> super::iaddress::AddressEncodeResult {
        let prefix = Self::address_prefix();
        let prefix_got = address.get(..prefix.len()).unwrap_or("");
        if prefix_got != prefix {
            return Err(AddressSchemeError::Msg(format!(
                "Invalid address_prefix (expected: {prefix}, got: {prefix_got})"
            )));
        }
        let address_no_prefix = address
            .get(prefix.len()..)
            .ok_or_else(|| AddressSchemeError::Msg("Sui address too short".into()))?;

        if address_no_prefix.len() != HASH_HEX_LEN {
            return Err(AddressSchemeError::Msg(format!(
                "Invalid length (expected: {HASH_HEX_LEN}, got: {})",
                address_no_prefix.len()
            )));
        }
        if !address_no_prefix.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(AddressSchemeError::Msg(
                "Sui address body must be hexadecimal".into(),
            ));
        }

        Ok(address_no_prefix.to_owned())
    }
}
