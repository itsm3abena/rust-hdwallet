use crate::hdwallet::cryptocurrencies::eos::EOS_PARAMS;
use crate::hdwallet::eccs::{
    validate_and_get_public_key, IPublicKey, PublicKeyInput, Slip10Secp256k1PublicKey,
};
use crate::hdwallet::exceptions::PublicKeyError;
use crate::hdwallet::libs::base58;
use crate::hdwallet::libs::ripemd160::ripemd160;

use super::iaddress::{
    AddressSchemeError, DecodeKwargs, EncodeKwargs, EncodePublicKey, IAddress,
};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct EOSAddress;

impl EOSAddress {
    pub const REGISTRY_NAME: &'static str = "EOS";

    fn address_prefix() -> &'static str {
        EOS_PARAMS.address_prefix.unwrap_or("EOS")
    }

    fn checksum_length() -> usize {
        EOS_PARAMS.checksum_length.unwrap_or(4) as usize
    }

    fn compute_checksum(pub_key_bytes: &[u8]) -> Vec<u8> {
        ripemd160(pub_key_bytes)[..Self::checksum_length()].to_vec()
    }

    pub fn encode_public_key(
        public_key: PublicKeyInput<'_, Slip10Secp256k1PublicKey>,
    ) -> Result<String, AddressSchemeError> {
        let pk = validate_and_get_public_key(public_key)
            .map_err(|e: PublicKeyError| AddressSchemeError::Msg(e.to_string()))?;
        let compressed = pk.raw_compressed();
        let checksum = Self::compute_checksum(&compressed);
        let mut payload = compressed;
        payload.extend_from_slice(&checksum);
        Ok(format!(
            "{}{}",
            Self::address_prefix(),
            base58::encode(&payload, None)
        ))
    }

    pub fn decode_address(address: &str) -> Result<String, AddressSchemeError> {
        let prefix = Self::address_prefix();
        let prefix_got = address.get(..prefix.len()).unwrap_or("");
        if prefix_got != prefix {
            return Err(AddressSchemeError::Msg(format!(
                "Invalid prefix (expected: {prefix}, got: {prefix_got})"
            )));
        }
        let address_no_prefix = address
            .get(prefix.len()..)
            .ok_or_else(|| AddressSchemeError::Msg("EOS address too short".into()))?;
        let address_decode = base58::decode(address_no_prefix.as_bytes(), None)
            .map_err(|e| AddressSchemeError::Msg(e.to_string()))?;
        let checksum_len = Self::checksum_length();
        let expected_length = Slip10Secp256k1PublicKey::compressed_len() + checksum_len;
        if address_decode.len() != expected_length {
            return Err(AddressSchemeError::Msg(format!(
                "Invalid length (expected: {expected_length}, got: {})",
                address_decode.len()
            )));
        }
        let checksum = &address_decode[address_decode.len() - checksum_len..];
        let public_key = &address_decode[..address_decode.len() - checksum_len];
        let checksum_got = Self::compute_checksum(public_key);
        if checksum != checksum_got.as_slice() {
            return Err(AddressSchemeError::Msg(format!(
                "Invalid checksum (expected: {}, got: {})",
                hex::encode(checksum),
                hex::encode(&checksum_got)
            )));
        }
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

impl IAddress for EOSAddress {
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
        Self::encode_public_key(input)
    }

    fn decode(&self, address: &str, _kwargs: &DecodeKwargs) -> super::iaddress::AddressEncodeResult {
        Self::decode_address(address)
    }
}
