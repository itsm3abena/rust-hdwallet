use crc::{Crc, CRC_16_XMODEM};

use crate::hdwallet::cryptocurrencies::stellar::STELLAR_PARAMS;
use crate::hdwallet::eccs::{
    validate_and_get_public_key, IPublicKey, PublicKeyInput, Slip10Ed25519PublicKeyStd,
};
use crate::hdwallet::exceptions::PublicKeyError;
use crate::hdwallet::libs::base32::{self, Base32Error};
use crate::hdwallet::utils::{bytes_reverse, integer_to_bytes_python};

use super::iaddress::{
    AddressSchemeError, DecodeKwargs, EncodeKwargs, EncodePublicKey, IAddress,
};

const DEFAULT_ADDRESS_TYPE: &str = "private_key";
const ADDRESS_TYPE_PRIVATE_KEY: u8 = 18 << 3;
const ADDRESS_TYPE_PUBLIC_KEY: u8 = 6 << 3;
const PUBKEY_LEN: usize = 32;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct StellarAddress;

impl StellarAddress {
    pub const REGISTRY_NAME: &'static str = "Stellar";

    fn checksum_length() -> usize {
        STELLAR_PARAMS.checksum_length.unwrap_or(2) as usize
    }

    fn map_base32(err: Base32Error) -> AddressSchemeError {
        AddressSchemeError::Msg(err.to_string())
    }

    fn resolve_address_type(kind: Option<&str>) -> Result<u8, AddressSchemeError> {
        match kind.unwrap_or(DEFAULT_ADDRESS_TYPE) {
            "private_key" | "PRIVATE_KEY" => Ok(ADDRESS_TYPE_PRIVATE_KEY),
            "public_key" | "PUBLIC_KEY" => Ok(ADDRESS_TYPE_PUBLIC_KEY),
            other => Err(AddressSchemeError::Msg(format!(
                "Invalid Stellar address type (expected: private_key, public_key, got: {other})"
            ))),
        }
    }

    fn compute_checksum(payload: &[u8]) -> Vec<u8> {
        let crc = Crc::<u16>::new(&CRC_16_XMODEM);
        let sum = crc.checksum(payload);
        bytes_reverse(&sum.to_be_bytes())
    }

    fn pubkey_body(pk: &Slip10Ed25519PublicKeyStd) -> Result<[u8; PUBKEY_LEN], AddressSchemeError> {
        let raw = pk.raw_compressed();
        raw[1..]
            .try_into()
            .map_err(|_| AddressSchemeError::Msg("expected 33-byte compressed SLIP10 Ed25519 key".into()))
    }

    fn build_payload(address_type: u8, body: &[u8; PUBKEY_LEN]) -> Vec<u8> {
        let mut payload = integer_to_bytes_python(u32::from(address_type), None);
        payload.extend_from_slice(body);
        payload
    }

    fn encode_payload(payload: &[u8]) -> Result<String, AddressSchemeError> {
        let checksum = Self::compute_checksum(payload);
        let mut raw = payload.to_vec();
        raw.extend_from_slice(&checksum);
        base32::encode_no_padding(&hex::encode(raw), None).map_err(Self::map_base32)
    }

    fn decode_address_bytes(address: &str) -> Result<Vec<u8>, AddressSchemeError> {
        let hex_payload = base32::decode(address, None).map_err(Self::map_base32)?;
        hex::decode(hex_payload).map_err(|e| {
            AddressSchemeError::Msg(format!("invalid decoded payload hex: {e}"))
        })
    }
}

impl IAddress for StellarAddress {
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
        let pk = validate_and_get_public_key::<Slip10Ed25519PublicKeyStd>(input).map_err(
            |e: PublicKeyError| AddressSchemeError::Msg(e.to_string()),
        )?;
        let address_type = Self::resolve_address_type(kwargs.address_type)?;
        let body = Self::pubkey_body(&pk)?;
        let payload = Self::build_payload(address_type, &body);
        Self::encode_payload(&payload)
    }

    fn decode(&self, address: &str, kwargs: &DecodeKwargs) -> super::iaddress::AddressEncodeResult {
        let address_type = Self::resolve_address_type(kwargs.address_type)?;
        let checksum_len = Self::checksum_length();
        let address_decode = Self::decode_address_bytes(address)?;

        let expected_length = Slip10Ed25519PublicKeyStd::compressed_len() + checksum_len;
        if address_decode.len() != expected_length {
            return Err(AddressSchemeError::Msg(format!(
                "Invalid length (expected: {expected_length}, got: {})",
                address_decode.len()
            )));
        }

        let checksum = &address_decode[address_decode.len() - checksum_len..];
        let payload = &address_decode[..address_decode.len() - checksum_len];

        let address_type_got = payload[0];
        if address_type != address_type_got {
            return Err(AddressSchemeError::Msg(format!(
                "Invalid address type (expected: {address_type}, got: {address_type_got})"
            )));
        }

        let checksum_got = Self::compute_checksum(payload);
        if checksum != checksum_got.as_slice() {
            return Err(AddressSchemeError::Msg(format!(
                "Invalid checksum (expected: {}, got: {})",
                hex::encode(checksum),
                hex::encode(&checksum_got)
            )));
        }

        let public_key = &payload[1..];
        if !Slip10Ed25519PublicKeyStd::is_valid_bytes(public_key) {
            return Err(AddressSchemeError::Msg(format!(
                "Invalid {} public key {}",
                Slip10Ed25519PublicKeyStd::ecc_name(),
                hex::encode(public_key)
            )));
        }

        Ok(hex::encode(public_key))
    }
}
