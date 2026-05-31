use bitcoin::secp256k1::PublicKey;
use sha3::{Digest, Keccak256};

use crate::hdwallet::cryptocurrencies::ethereum::ETHEREUM_PARAMS;
use crate::hdwallet::eccs::{
    validate_and_get_public_key, IPublicKey, PublicKeyInput, Slip10Secp256k1PublicKey,
};
use crate::hdwallet::exceptions::{Error, PublicKeyError};

use super::iaddress::{
    AddressSchemeError, DecodeKwargs, EncodeKwargs, EncodePublicKey, IAddress,
};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct EthereumAddress;

impl EthereumAddress {
    pub const REGISTRY_NAME: &'static str = "Ethereum";

    fn address_prefix() -> &'static str {
        ETHEREUM_PARAMS.address_prefix.unwrap_or("0x")
    }

    fn keccak256(data: &[u8]) -> [u8; 32] {
        let mut hasher = Keccak256::new();
        hasher.update(data);
        hasher.finalize().into()
    }

    fn pubkey_hash_hex(pk: &Slip10Secp256k1PublicKey) -> String {
        let uncompressed = pk.raw_uncompressed();
        let hash = Self::keccak256(&uncompressed[1..]);
        hex::encode(&hash[12..])
    }

    pub fn checksum_encode(address: &str) -> String {
        let address_lower = address.to_ascii_lowercase();
        let address_hash = hex::encode(Self::keccak256(address_lower.as_bytes()));
        address
            .chars()
            .zip(address_hash.chars())
            .map(|(c, h)| {
                if h.to_digit(16).unwrap_or(0) >= 8 {
                    c.to_ascii_uppercase()
                } else {
                    c.to_ascii_lowercase()
                }
            })
            .collect()
    }

    pub fn encode_public_key(
        public_key: PublicKeyInput<'_, Slip10Secp256k1PublicKey>,
        skip_checksum_encode: bool,
    ) -> std::result::Result<String, AddressSchemeError> {
        let pk = validate_and_get_public_key(public_key)
            .map_err(|e: PublicKeyError| AddressSchemeError::Msg(e.to_string()))?;
        let address = Self::pubkey_hash_hex(&pk);
        let body = if skip_checksum_encode {
            address
        } else {
            Self::checksum_encode(&address)
        };
        Ok(format!("{}{body}", Self::address_prefix()))
    }

    pub fn decode_address(
        address: &str,
        skip_checksum_encode: bool,
    ) -> std::result::Result<String, AddressSchemeError> {
        let prefix = Self::address_prefix();
        let prefix_got = address.get(..prefix.len()).unwrap_or("");
        if prefix_got != prefix {
            return Err(AddressSchemeError::Msg(format!(
                "Invalid address_prefix (expected: {prefix}, got: {prefix_got})"
            )));
        }
        let address_no_prefix = address
            .get(prefix.len()..)
            .ok_or_else(|| AddressSchemeError::Msg("Ethereum address too short".into()))?;
        if address_no_prefix.len() != 40 {
            return Err(AddressSchemeError::Msg(format!(
                "Invalid length (expected: 40, got: {})",
                address_no_prefix.len()
            )));
        }
        if !skip_checksum_encode
            && address_no_prefix != Self::checksum_encode(address_no_prefix)
        {
            return Err(AddressSchemeError::Msg("Invalid checksum encode".into()));
        }
        Ok(address_no_prefix.to_ascii_lowercase())
    }
}

impl IAddress for EthereumAddress {
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
        Self::encode_public_key(input, kwargs.skip_checksum_encode.unwrap_or(false))
    }

    fn decode(&self, address: &str, kwargs: &DecodeKwargs) -> super::iaddress::AddressEncodeResult {
        Self::decode_address(address, kwargs.skip_checksum_encode.unwrap_or(false))
    }
}

/// Lowercase `0x` address without EIP-55 checksum (legacy helper).
pub fn ethereum_address_from_pubkey(pk: &PublicKey) -> std::result::Result<String, Error> {
    let slip = Slip10Secp256k1PublicKey(*pk);
    let u = slip.raw_uncompressed();
    if u.len() != 65 || u[0] != 0x04 {
        return Err(Error::new(
            "expected uncompressed public key",
            None,
            None,
            None,
        ));
    }
    let hash = EthereumAddress::keccak256(&u[1..]);
    Ok(format!("0x{}", hex::encode(&hash[12..])))
}
