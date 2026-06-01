use crate::hdwallet::cryptocurrencies::solana::SOLANA_PARAMS;
use crate::hdwallet::eccs::{
    validate_and_get_public_key, IPublicKey, PublicKeyInput, Slip10Ed25519PublicKeyStd,
};
use crate::hdwallet::exceptions::PublicKeyError;
use crate::hdwallet::libs::base58::{self, Base58Error};

use super::iaddress::{
    AddressSchemeError, DecodeKwargs, EncodeKwargs, EncodePublicKey, IAddress,
};

const PUBKEY_LEN: usize = 32;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct SolanaAddress;

impl SolanaAddress {
    pub const REGISTRY_NAME: &'static str = "Solana";

    fn alphabet(kwargs_alphabet: Option<&'static str>) -> Option<&'static str> {
        kwargs_alphabet.or(SOLANA_PARAMS.alphabet)
    }

    fn map_base58(err: Base58Error) -> AddressSchemeError {
        AddressSchemeError::Msg(err.to_string())
    }

    fn pubkey_body(pk: &Slip10Ed25519PublicKeyStd) -> Result<[u8; PUBKEY_LEN], AddressSchemeError> {
        let raw = pk.raw_compressed();
        raw[1..]
            .try_into()
            .map_err(|_| AddressSchemeError::Msg("expected 33-byte compressed SLIP10 Ed25519 key".into()))
    }

    fn encode_body(body: &[u8; PUBKEY_LEN], alphabet: Option<&'static str>) -> String {
        base58::encode(body, Self::alphabet(alphabet))
    }

    fn decode_address_bytes(
        address: &str,
        alphabet: Option<&'static str>,
    ) -> Result<Vec<u8>, AddressSchemeError> {
        base58::decode(address.as_bytes(), Self::alphabet(alphabet)).map_err(Self::map_base58)
    }
}

impl IAddress for SolanaAddress {
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
        Ok(Self::encode_body(&Self::pubkey_body(&pk)?, kwargs.alphabet))
    }

    fn decode(&self, address: &str, kwargs: &DecodeKwargs) -> super::iaddress::AddressEncodeResult {
        let public_key = Self::decode_address_bytes(address, kwargs.alphabet)?;

        let expected_length = Slip10Ed25519PublicKeyStd::compressed_len() - 1;
        if public_key.len() != expected_length {
            return Err(AddressSchemeError::Msg(format!(
                "Invalid length (expected: {expected_length}, got: {})",
                public_key.len()
            )));
        }

        if !Slip10Ed25519PublicKeyStd::is_valid_bytes(&public_key) {
            return Err(AddressSchemeError::Msg(format!(
                "Invalid {} public key {}",
                Slip10Ed25519PublicKeyStd::ecc_name(),
                hex::encode(&public_key)
            )));
        }

        Ok(hex::encode(public_key))
    }
}
