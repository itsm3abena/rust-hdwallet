use sha3::{Digest, Keccak256};

use crate::hdwallet::cryptocurrencies::tron::{Mainnet as TronMainnet, TRON_PARAMS};
use crate::hdwallet::eccs::{
    validate_and_get_public_key, IPublicKey, PublicKeyInput, Slip10Secp256k1PublicKey,
};
use crate::hdwallet::exceptions::PublicKeyError;
use crate::hdwallet::libs::base58::{self, Base58Error};
use crate::hdwallet::utils::integer_to_bytes_python;

use super::iaddress::{
    AddressSchemeError, DecodeKwargs, EncodeKwargs, EncodePublicKey, IAddress,
};

const HASH_LEN: usize = 20;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct TronAddress;

impl TronAddress {
    pub const REGISTRY_NAME: &'static str = "Tron";

    fn alphabet(kwargs_alphabet: Option<&'static str>) -> Option<&'static str> {
        kwargs_alphabet.or(TRON_PARAMS.alphabet)
    }

    fn public_key_address_prefix_encode(kwargs: &EncodeKwargs<'_>) -> u32 {
        kwargs
            .public_key_address_prefix
            .unwrap_or(TronMainnet::PUBLIC_KEY_ADDRESS_PREFIX)
    }

    fn public_key_address_prefix_decode(kwargs: &DecodeKwargs) -> u32 {
        kwargs
            .public_key_address_prefix
            .unwrap_or(TronMainnet::PUBLIC_KEY_ADDRESS_PREFIX)
    }

    fn prefix_bytes(prefix: u32) -> Vec<u8> {
        integer_to_bytes_python(prefix, None)
    }

    fn keccak256(data: &[u8]) -> [u8; 32] {
        let mut hasher = Keccak256::new();
        hasher.update(data);
        hasher.finalize().into()
    }

    fn address_hash(pk: &Slip10Secp256k1PublicKey) -> [u8; HASH_LEN] {
        let uncompressed = pk.raw_uncompressed();
        let hash = Self::keccak256(&uncompressed[1..]);
        hash[12..]
            .try_into()
            .expect("keccak256 output is 32 bytes")
    }

    fn map_base58(err: Base58Error) -> AddressSchemeError {
        AddressSchemeError::Msg(err.to_string())
    }

    fn encode_public_key(
        pk: &Slip10Secp256k1PublicKey,
        prefix: u32,
        alphabet: Option<&'static str>,
    ) -> String {
        let prefix_bytes = Self::prefix_bytes(prefix);
        let hash = Self::address_hash(pk);
        let mut raw = prefix_bytes;
        raw.extend_from_slice(&hash);
        base58::check_encode(&raw, Self::alphabet(alphabet))
    }

    fn decode_address(
        address: &str,
        prefix: u32,
        alphabet: Option<&'static str>,
    ) -> Result<String, AddressSchemeError> {
        let address_decode =
            base58::check_decode(address.as_bytes(), Self::alphabet(alphabet))
                .map_err(Self::map_base58)?;

        let prefix_bytes = Self::prefix_bytes(prefix);
        let expected_length = HASH_LEN + prefix_bytes.len();
        if address_decode.len() != expected_length {
            return Err(AddressSchemeError::Msg(format!(
                "Invalid length (expected: {expected_length}, got: {})",
                address_decode.len()
            )));
        }

        let prefix_got = &address_decode[..prefix_bytes.len()];
        if prefix_got != prefix_bytes.as_slice() {
            return Err(AddressSchemeError::Msg(format!(
                "Invalid prefix (expected: {}, got: {})",
                hex::encode(&prefix_bytes),
                hex::encode(prefix_got)
            )));
        }

        Ok(hex::encode(&address_decode[prefix_bytes.len()..]))
    }
}

impl IAddress for TronAddress {
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
        Ok(Self::encode_public_key(
            &pk,
            Self::public_key_address_prefix_encode(kwargs),
            kwargs.alphabet,
        ))
    }

    fn decode(&self, address: &str, kwargs: &DecodeKwargs) -> super::iaddress::AddressEncodeResult {
        Self::decode_address(
            address,
            Self::public_key_address_prefix_decode(kwargs),
            kwargs.alphabet,
        )
    }
}
