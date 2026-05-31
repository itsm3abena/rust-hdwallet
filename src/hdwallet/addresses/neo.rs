use crate::hdwallet::cryptocurrencies::neo::NEO_PARAMS;
use crate::hdwallet::eccs::{
    validate_and_get_public_key, IPublicKey, PublicKeyInput, Slip10Nist256p1PublicKey,
};
use crate::hdwallet::exceptions::PublicKeyError;
use crate::hdwallet::libs::base58::{self, Base58Error};
use crate::hdwallet::libs::ecc::hash160;
use crate::hdwallet::utils::integer_to_bytes_python;

use super::iaddress::{
    AddressSchemeError, DecodeKwargs, EncodeKwargs, EncodePublicKey, IAddress,
};

const HASH_LEN: usize = 20;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct NeoAddress;

impl NeoAddress {
    pub const REGISTRY_NAME: &'static str = "Neo";

    fn alphabet() -> &'static str {
        NEO_PARAMS
            .alphabet
            .unwrap_or("123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz")
    }

    fn address_prefix_bytes() -> Vec<u8> {
        integer_to_bytes_python(
            u32::from(NEO_PARAMS.address_prefix_byte.unwrap_or(0x21)),
            None,
        )
    }

    fn address_suffix_bytes() -> Vec<u8> {
        integer_to_bytes_python(u32::from(NEO_PARAMS.suffix.unwrap_or(0xAC)), None)
    }

    fn address_version_bytes() -> Vec<u8> {
        integer_to_bytes_python(
            u32::from(NEO_PARAMS.address_version_byte.unwrap_or(0x17)),
            None,
        )
    }

    fn payload_hash(compressed: &[u8]) -> [u8; HASH_LEN] {
        let mut payload = Self::address_prefix_bytes();
        payload.extend_from_slice(compressed);
        payload.extend_from_slice(&Self::address_suffix_bytes());
        hash160(&payload)
    }

    fn map_base58(err: Base58Error) -> AddressSchemeError {
        AddressSchemeError::Msg(err.to_string())
    }

    fn encode_from_public_key(pk: &Slip10Nist256p1PublicKey) -> String {
        let payload_hash = Self::payload_hash(&pk.raw_compressed());
        let mut raw = Self::address_version_bytes();
        raw.extend_from_slice(&payload_hash);
        base58::check_encode(&raw, Some(Self::alphabet()))
    }

    fn decode_address(address: &str) -> Result<String, AddressSchemeError> {
        let address_decode = base58::check_decode(address.as_bytes(), Some(Self::alphabet()))
            .map_err(Self::map_base58)?;

        let address_version = Self::address_version_bytes();
        let expected_length = HASH_LEN + address_version.len();
        if address_decode.len() != expected_length {
            return Err(AddressSchemeError::Msg(format!(
                "Invalid length (expected: {expected_length}, got: {})",
                address_decode.len()
            )));
        }

        let version_got = integer_to_bytes_python(u32::from(address_decode[0]), None);
        if version_got != address_version {
            return Err(AddressSchemeError::Msg(format!(
                "Invalid address version (expected: {}, got: {})",
                hex::encode(&address_version),
                hex::encode(&version_got)
            )));
        }

        Ok(hex::encode(&address_decode[address_version.len()..]))
    }
}

impl IAddress for NeoAddress {
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
        let pk = validate_and_get_public_key::<Slip10Nist256p1PublicKey>(input).map_err(
            |e: PublicKeyError| AddressSchemeError::Msg(e.to_string()),
        )?;
        Ok(Self::encode_from_public_key(&pk))
    }

    fn decode(
        &self,
        address: &str,
        _kwargs: &DecodeKwargs,
    ) -> super::iaddress::AddressEncodeResult {
        Self::decode_address(address)
    }
}
