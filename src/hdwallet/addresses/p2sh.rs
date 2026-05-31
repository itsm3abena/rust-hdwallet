use crate::hdwallet::consts::PublicKeyTypes;
use crate::hdwallet::cryptocurrencies::bitcoin::{
    Mainnet as BitcoinMainnet, Testnet as BitcoinTestnet, BITCOIN_PARAMS,
};
use crate::hdwallet::eccs::{
    validate_and_get_public_key, PublicKeyInput, Slip10Secp256k1PublicKey,
};
use crate::hdwallet::exceptions::PublicKeyError;
use crate::hdwallet::libs::base58::{self, Base58Error};
use crate::hdwallet::libs::ecc::hash160;
use crate::hdwallet::utils::integer_to_bytes_python;

use super::iaddress::{
    AddressSchemeError, DecodeKwargs, EncodeKwargs, EncodePublicKey, IAddress,
};
use super::p2pkh::P2PKHAddress;

const HASH_LEN: usize = 20;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct P2SHAddress;

impl P2SHAddress {
    pub const REGISTRY_NAME: &'static str = "P2SH";

    fn default_script_address_prefix() -> u32 {
        BitcoinMainnet::SCRIPT_ADDRESS_PREFIX
    }

    pub(crate) fn script_address_prefix_encode(kwargs: &EncodeKwargs<'_>) -> u32 {
        kwargs
            .script_address_prefix
            .unwrap_or_else(|| Self::script_address_prefix_from_network(kwargs.network))
    }

    pub(crate) fn script_address_prefix_decode(kwargs: &DecodeKwargs) -> u32 {
        kwargs
            .script_address_prefix
            .unwrap_or_else(|| Self::script_address_prefix_from_network(kwargs.network))
    }

    fn script_address_prefix_from_network(network: Option<&str>) -> u32 {
        match network {
            Some("testnet") | Some("regtest") => BitcoinTestnet::SCRIPT_ADDRESS_PREFIX,
            _ => Self::default_script_address_prefix(),
        }
    }

    fn prefix_bytes(prefix: u32) -> Vec<u8> {
        integer_to_bytes_python(prefix, None)
    }

    fn alphabet(kwargs_alphabet: Option<&'static str>) -> &'static str {
        kwargs_alphabet.unwrap_or_else(|| {
            BITCOIN_PARAMS
                .alphabet
                .unwrap_or(base58::DEFAULT_ALPHABET)
        })
    }

    pub(crate) fn use_compressed_public_key(kwargs: &EncodeKwargs<'_>) -> bool {
        !matches!(
            kwargs.public_key_type,
            Some(PublicKeyTypes::UNCOMPRESSED)
        )
    }

    fn script_hash_from_public_key_hash(pubkey_hash: &[u8; HASH_LEN]) -> [u8; HASH_LEN] {
        let script_hex = format!("76a914{}88ac", hex::encode(pubkey_hash));
        let script_bytes = hex::decode(&script_hex).expect("valid P2PKH script hex");
        hash160(&script_bytes)
    }

    pub(crate) fn script_hash_from_nested_p2wpkh(pubkey_hash: &[u8; HASH_LEN]) -> [u8; HASH_LEN] {
        let script_hex = format!("0014{}", hex::encode(pubkey_hash));
        let script_bytes = hex::decode(&script_hex).expect("valid nested P2WPKH script hex");
        hash160(&script_bytes)
    }

    pub(crate) fn script_hash_from_nested_p2wsh(pubkey_bytes: &[u8]) -> [u8; HASH_LEN] {
        let witness_program =
            super::p2wsh::P2WSHAddress::script_hash_from_public_key_bytes(pubkey_bytes);
        let script_hex = format!("0020{}", hex::encode(witness_program));
        let script_bytes = hex::decode(&script_hex).expect("valid nested P2WSH script hex");
        hash160(&script_bytes)
    }

    pub(crate) fn encode_script_hash(
        script_hash: &[u8; HASH_LEN],
        prefix: u32,
        alphabet: Option<&'static str>,
    ) -> String {
        let mut raw = Self::prefix_bytes(prefix);
        raw.extend_from_slice(script_hash);
        base58::check_encode(&raw, Some(Self::alphabet(alphabet)))
    }

    fn map_base58(err: Base58Error) -> AddressSchemeError {
        AddressSchemeError::Msg(err.to_string())
    }

    pub(crate) fn encode_public_key(
        pk: &Slip10Secp256k1PublicKey,
        prefix: u32,
        compressed: bool,
        alphabet: Option<&'static str>,
    ) -> String {
        let pubkey_hash = P2PKHAddress::public_key_hash(pk, compressed);
        let script_hash = Self::script_hash_from_public_key_hash(&pubkey_hash);
        Self::encode_script_hash(&script_hash, prefix, alphabet)
    }

    pub(crate) fn decode_address(
        address: &str,
        prefix: u32,
        alphabet: Option<&'static str>,
    ) -> Result<String, AddressSchemeError> {
        let address_decode =
            base58::check_decode(address.as_bytes(), Some(Self::alphabet(alphabet)))
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

impl IAddress for P2SHAddress {
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
            Self::script_address_prefix_encode(kwargs),
            Self::use_compressed_public_key(kwargs),
            kwargs.alphabet,
        ))
    }

    fn decode(&self, address: &str, kwargs: &DecodeKwargs) -> super::iaddress::AddressEncodeResult {
        Self::decode_address(
            address,
            Self::script_address_prefix_decode(kwargs),
            kwargs.alphabet,
        )
    }
}
