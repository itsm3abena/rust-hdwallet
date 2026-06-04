use crate::hdwallet::cryptocurrencies::tezos::TEZOS_PARAMS;
use crate::hdwallet::eccs::{
    validate_and_get_public_key, IPublicKey, PublicKeyInput, Slip10Ed25519PublicKeyStd,
};
use crate::hdwallet::exceptions::PublicKeyError;
use crate::hdwallet::libs::base58::{self, Base58Error};
use crate::hdwallet::utils::blake2b_160;

use super::iaddress::{
    AddressSchemeError, DecodeKwargs, EncodeKwargs, EncodePublicKey, IAddress,
};

const DEFAULT_ADDRESS_PREFIX: &str = "tz1";
const HASH_LEN: usize = 20;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct TezosAddress;

impl TezosAddress {
    pub const REGISTRY_NAME: &'static str = "Tezos";

    fn valid_address_prefixes() -> &'static [&'static str] {
        &["tz1", "tz2", "tz3"]
    }

    fn map_base58(err: Base58Error) -> AddressSchemeError {
        AddressSchemeError::Msg(err.to_string())
    }

    fn resolve_prefix_bytes(prefix: Option<&str>) -> Result<&'static [u8], AddressSchemeError> {
        let label = prefix.unwrap_or(DEFAULT_ADDRESS_PREFIX);
        let chains = TEZOS_PARAMS.chain_prefixes_by_address_type.unwrap_or(&[]);
        let codes = TEZOS_PARAMS.address_type_prefix_bytes.unwrap_or(&[]);

        for (code_label, code_bytes) in codes {
            if let Some((_, chain)) = chains
                .iter()
                .find(|(l, _)| l.eq_ignore_ascii_case(code_label))
            {
                if label == *chain || code_label.eq_ignore_ascii_case(label) {
                    return Ok(*code_bytes);
                }
            }
        }

        let expected = Self::valid_address_prefixes().join(", ");
        Err(AddressSchemeError::Msg(format!(
            "Invalid Tezos address prefix (expected: {expected}, got: {label})"
        )))
    }

    fn pubkey_body(pk: &Slip10Ed25519PublicKeyStd) -> Result<[u8; 32], AddressSchemeError> {
        let raw = pk.raw_compressed();
        raw[1..]
            .try_into()
            .map_err(|_| AddressSchemeError::Msg("expected 33-byte compressed SLIP10 Ed25519 key".into()))
    }

    fn encode_with_prefix(
        body: &[u8; 32],
        prefix_bytes: &[u8],
        alphabet: Option<&'static str>,
    ) -> String {
        let payload = blake2b_160(body);
        let mut raw = prefix_bytes.to_vec();
        raw.extend_from_slice(&payload);
        base58::check_encode(&raw, alphabet)
    }

    fn decode_address(
        address: &str,
        prefix_bytes: &[u8],
        alphabet: Option<&'static str>,
    ) -> Result<String, AddressSchemeError> {
        let address_decode =
            base58::check_decode(address.as_bytes(), alphabet).map_err(Self::map_base58)?;

        let expected_length = prefix_bytes.len() + HASH_LEN;
        if address_decode.len() != expected_length {
            return Err(AddressSchemeError::Msg(format!(
                "Invalid length (expected: {expected_length}, got: {})",
                address_decode.len()
            )));
        }

        let prefix_got = &address_decode[..prefix_bytes.len()];
        if prefix_got != prefix_bytes {
            return Err(AddressSchemeError::Msg(format!(
                "Invalid prefix (expected: {}, got: {})",
                hex::encode(prefix_bytes),
                hex::encode(prefix_got)
            )));
        }

        Ok(hex::encode(&address_decode[prefix_bytes.len()..]))
    }
}

impl IAddress for TezosAddress {
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
        let prefix = Self::resolve_prefix_bytes(kwargs.address_prefix)?;
        let body = Self::pubkey_body(&pk)?;
        Ok(Self::encode_with_prefix(&body, prefix, kwargs.alphabet))
    }

    fn decode(&self, address: &str, kwargs: &DecodeKwargs) -> super::iaddress::AddressEncodeResult {
        let prefix = Self::resolve_prefix_bytes(kwargs.address_prefix)?;
        Self::decode_address(address, prefix, kwargs.alphabet)
    }
}
