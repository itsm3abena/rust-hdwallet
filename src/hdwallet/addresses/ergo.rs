use crate::hdwallet::cryptocurrencies::ergo::{
    Mainnet as ErgoMainnet, Testnet as ErgoTestnet, ADDRESS_TYPES, ERGO_PARAMS,
};
use crate::hdwallet::eccs::{
    validate_and_get_public_key, IPublicKey, PublicKeyInput, Slip10Secp256k1PublicKey,
};
use crate::hdwallet::exceptions::PublicKeyError;
use crate::hdwallet::libs::base58;
use crate::hdwallet::utils::{blake2b_256, integer_to_bytes_python};

use super::iaddress::{
    AddressSchemeError, DecodeKwargs, EncodeKwargs, EncodePublicKey, IAddress,
};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct ErgoAddress;

impl ErgoAddress {
    pub const REGISTRY_NAME: &'static str = "Ergo";

    const DEFAULT_ADDRESS_TYPE: &'static str = "p2pkh";

    fn checksum_length() -> usize {
        ERGO_PARAMS.checksum_length.unwrap_or(4) as usize
    }

    fn valid_address_types() -> Vec<&'static str> {
        ADDRESS_TYPES.iter().map(|(_, label)| *label).collect()
    }

    fn address_type_code(label: Option<&str>) -> Result<u32, AddressSchemeError> {
        let label = label.unwrap_or(Self::DEFAULT_ADDRESS_TYPE);
        let norm = label.to_ascii_lowercase();
        if let Some(codes) = ERGO_PARAMS.address_type_codes {
            for (name, code) in codes {
                if name.eq_ignore_ascii_case(label) || name.to_ascii_lowercase() == norm {
                    return Ok(u32::from(*code));
                }
            }
        }
        match norm.as_str() {
            "p2pkh" => Ok(0x01),
            "p2sh" => Ok(0x02),
            _ => {
                let expected = Self::valid_address_types().join(", ");
                Err(AddressSchemeError::Msg(format!(
                    "Invalid Ergo address type (expected: {expected}, got: {label})"
                )))
            }
        }
    }

    fn network_type_code(network: Option<&str>) -> Result<u32, AddressSchemeError> {
        let network = network.ok_or_else(|| {
            AddressSchemeError::Msg("Ergo network type is required".into())
        })?;
        match network {
            "mainnet" => Ok(ErgoMainnet::TYPE),
            "testnet" => Ok(ErgoTestnet::TYPE),
            other => Err(AddressSchemeError::Msg(format!(
                "Invalid Ergo network type (expected: mainnet, testnet, got: {other})"
            ))),
        }
    }

    fn prefix_bytes(address_type: Option<&str>, network: Option<&str>) -> Result<Vec<u8>, AddressSchemeError> {
        let address_type = Self::address_type_code(address_type)?;
        let network_type = Self::network_type_code(network)?;
        Ok(integer_to_bytes_python(address_type + network_type, None))
    }

    fn compute_checksum(payload: &[u8]) -> Vec<u8> {
        blake2b_256(payload)[..Self::checksum_length()].to_vec()
    }

    pub fn encode_public_key(
        public_key: PublicKeyInput<'_, Slip10Secp256k1PublicKey>,
        kwargs: &EncodeKwargs<'_>,
    ) -> Result<String, AddressSchemeError> {
        let pk = validate_and_get_public_key(public_key)
            .map_err(|e: PublicKeyError| AddressSchemeError::Msg(e.to_string()))?;
        let mut address_payload = Self::prefix_bytes(kwargs.address_type, kwargs.network)?;
        address_payload.extend_from_slice(&pk.raw_compressed());
        let checksum = Self::compute_checksum(&address_payload);
        address_payload.extend_from_slice(&checksum);
        Ok(base58::encode(&address_payload, None))
    }

    pub fn decode_address(address: &str, kwargs: &DecodeKwargs) -> Result<String, AddressSchemeError> {
        let prefix = Self::prefix_bytes(kwargs.address_type, kwargs.network)?;
        let address_decode = base58::decode(address.as_bytes(), None)
            .map_err(|e| AddressSchemeError::Msg(e.to_string()))?;
        let checksum_len = Self::checksum_length();
        let expected_length =
            Slip10Secp256k1PublicKey::compressed_len() + checksum_len + prefix.len();
        if address_decode.len() != expected_length {
            return Err(AddressSchemeError::Msg(format!(
                "Invalid length (expected: {expected_length}, got: {})",
                address_decode.len()
            )));
        }
        let checksum = &address_decode[address_decode.len() - checksum_len..];
        let address_with_prefix = &address_decode[..address_decode.len() - checksum_len];
        let checksum_got = Self::compute_checksum(address_with_prefix);
        if checksum != checksum_got.as_slice() {
            return Err(AddressSchemeError::Msg(format!(
                "Invalid checksum (expected: {}, got: {})",
                hex::encode(checksum),
                hex::encode(&checksum_got)
            )));
        }
        let prefix_got = &address_with_prefix[..prefix.len()];
        if prefix != prefix_got {
            return Err(AddressSchemeError::Msg(format!(
                "Invalid prefix (expected: {}, got: {})",
                hex::encode(&prefix),
                hex::encode(prefix_got)
            )));
        }
        let public_key = &address_with_prefix[prefix.len()..];
        if !Slip10Secp256k1PublicKey::is_valid_bytes(public_key) {
            return Err(AddressSchemeError::Msg(format!(
                "Invalid {} public key {}",
                Slip10Secp256k1PublicKey::ecc_name(),
                hex::encode(public_key)
            )));
        }
        Ok(hex::encode(public_key))
    }
}

impl IAddress for ErgoAddress {
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
        Self::encode_public_key(input, kwargs)
    }

    fn decode(&self, address: &str, kwargs: &DecodeKwargs) -> super::iaddress::AddressEncodeResult {
        Self::decode_address(address, kwargs)
    }
}
