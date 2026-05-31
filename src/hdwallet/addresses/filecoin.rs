use crate::hdwallet::cryptocurrencies::filecoin::{ADDRESS_TYPES, FILECOIN_PARAMS};
use crate::hdwallet::eccs::{
    validate_and_get_public_key, IPublicKey, PublicKeyInput, Slip10Secp256k1PublicKey,
};
use crate::hdwallet::exceptions::PublicKeyError;
use crate::hdwallet::libs::base32::{self, Base32Error};
use crate::hdwallet::utils::{blake2b_160, blake2b_256, integer_to_bytes_python};

use super::iaddress::{
    AddressSchemeError, DecodeKwargs, EncodeKwargs, EncodePublicKey, IAddress,
};

const CHECKSUM_LEN: usize = 4;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct FilecoinAddress;

impl FilecoinAddress {
    pub const REGISTRY_NAME: &'static str = "Filecoin";

    const DEFAULT_ADDRESS_TYPE: &'static str = "secp256k1";

    fn alphabet() -> &'static str {
        FILECOIN_PARAMS.alphabet.unwrap_or("abcdefghijklmnopqrstuvwxyz234567")
    }

    fn address_prefix() -> &'static str {
        FILECOIN_PARAMS.address_prefix.unwrap_or("f")
    }

    fn valid_address_types() -> Vec<&'static str> {
        ADDRESS_TYPES.iter().map(|(_, label)| *label).collect()
    }

    fn address_type_code(label: Option<&str>) -> Result<u8, AddressSchemeError> {
        let label = label.unwrap_or(Self::DEFAULT_ADDRESS_TYPE);
        let norm = label.to_ascii_lowercase();
        if let Some(codes) = FILECOIN_PARAMS.address_type_codes {
            for (name, code) in codes {
                if name.eq_ignore_ascii_case(label) || name.to_ascii_lowercase() == norm {
                    return Ok(*code);
                }
            }
        }
        match norm.as_str() {
            "secp256k1" => Ok(1),
            "bls" => Ok(3),
            _ => {
                let expected = Self::valid_address_types().join(", ");
                Err(AddressSchemeError::Msg(format!(
                    "Invalid Filecoin address type (expected: {expected}, got: {label})"
                )))
            }
        }
    }

    fn public_key_hash(pk: &Slip10Secp256k1PublicKey) -> [u8; 20] {
        blake2b_160(&pk.raw_uncompressed())
    }

    fn compute_checksum(public_key_hash: &[u8], address_type: u8) -> [u8; CHECKSUM_LEN] {
        let mut payload = integer_to_bytes_python(u32::from(address_type), None);
        payload.extend_from_slice(public_key_hash);
        blake2b_256(&payload)[..CHECKSUM_LEN]
            .try_into()
            .expect("checksum length")
    }

    fn type_char(address_type: u8) -> Result<char, AddressSchemeError> {
        char::from_u32(u32::from(address_type) + u32::from(b'0'))
            .ok_or_else(|| AddressSchemeError::Msg(format!("Invalid address type: {address_type}")))
    }

    fn type_from_char(c: char) -> Result<u8, AddressSchemeError> {
        let code = c as u32;
        if code < u32::from(b'0') {
            return Err(AddressSchemeError::Msg(format!("Invalid address type char: {c}")));
        }
        Ok((code - u32::from(b'0')) as u8)
    }

    pub fn encode_public_key(
        public_key: PublicKeyInput<'_, Slip10Secp256k1PublicKey>,
        address_type: Option<&str>,
    ) -> Result<String, AddressSchemeError> {
        let pk = validate_and_get_public_key(public_key)
            .map_err(|e: PublicKeyError| AddressSchemeError::Msg(e.to_string()))?;
        let public_key_hash = Self::public_key_hash(&pk);
        let address_type = Self::address_type_code(address_type)?;
        let checksum = Self::compute_checksum(&public_key_hash, address_type);
        let mut payload = public_key_hash.to_vec();
        payload.extend_from_slice(&checksum);
        let base32_encode = base32::encode_no_padding(&hex::encode(&payload), Some(Self::alphabet()))
            .map_err(|e: Base32Error| AddressSchemeError::Msg(e.to_string()))?;
        Ok(format!(
            "{}{}{}",
            Self::address_prefix(),
            Self::type_char(address_type)?,
            base32_encode
        ))
    }

    pub fn decode_address(
        address: &str,
        address_type: Option<&str>,
    ) -> Result<String, AddressSchemeError> {
        let prefix = Self::address_prefix();
        let prefix_got = address.get(..prefix.len()).unwrap_or("");
        if prefix_got != prefix {
            return Err(AddressSchemeError::Msg(format!(
                "Invalid prefix (expected: {prefix}, got: {prefix_got})"
            )));
        }
        let address_no_prefix = address
            .get(prefix.len()..)
            .ok_or_else(|| AddressSchemeError::Msg("Filecoin address too short".into()))?;
        if address_no_prefix.is_empty() {
            return Err(AddressSchemeError::Msg("Filecoin address too short".into()));
        }
        let address_type = Self::address_type_code(address_type)?;
        let address_type_got = Self::type_from_char(
            address_no_prefix
                .chars()
                .next()
                .ok_or_else(|| AddressSchemeError::Msg("Filecoin address too short".into()))?,
        )?;
        if address_type != address_type_got {
            return Err(AddressSchemeError::Msg(format!(
                "Invalid address type (expected: {address_type}, got: {address_type_got})"
            )));
        }
        let base32_part = &address_no_prefix[1..];
        let address_decode = hex::decode(
            base32::decode(base32_part, Some(Self::alphabet()))
                .map_err(|e: Base32Error| AddressSchemeError::Msg(e.to_string()))?,
        )
        .map_err(|e| AddressSchemeError::Msg(e.to_string()))?;
        if address_decode.len() != 24 {
            return Err(AddressSchemeError::Msg(format!(
                "Invalid length (expected: 24, got: {})",
                address_decode.len()
            )));
        }
        let checksum = &address_decode[address_decode.len() - CHECKSUM_LEN..];
        let public_key_hash = &address_decode[..address_decode.len() - CHECKSUM_LEN];
        let checksum_got = Self::compute_checksum(public_key_hash, address_type);
        if checksum != checksum_got {
            return Err(AddressSchemeError::Msg(format!(
                "Invalid checksum (expected: {}, got: {})",
                hex::encode(checksum),
                hex::encode(checksum_got)
            )));
        }
        Ok(hex::encode(public_key_hash))
    }
}

impl IAddress for FilecoinAddress {
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
        Self::encode_public_key(input, kwargs.address_type)
    }

    fn decode(&self, address: &str, kwargs: &DecodeKwargs) -> super::iaddress::AddressEncodeResult {
        Self::decode_address(address, kwargs.address_type)
    }
}
