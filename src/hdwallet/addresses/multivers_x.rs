use crate::hdwallet::cryptocurrencies::multivers_x::Mainnet as MultiversXMainnet;
use crate::hdwallet::eccs::{
    validate_and_get_public_key, IPublicKey, PublicKeyInput, Slip10Ed25519PublicKeyStd,
};
use crate::hdwallet::exceptions::PublicKeyError;
use crate::hdwallet::libs::bech32::{base_bech32_decode, bech32_encode_raw, convert_bits as convertbits};

use super::iaddress::{
    AddressSchemeError, DecodeKwargs, EncodeKwargs, EncodePublicKey, IAddress,
};

const PUBKEY_BODY_LEN: usize = 32;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct MultiversXAddress;

impl MultiversXAddress {
    pub const REGISTRY_NAME: &'static str = "MultiversX";

    fn hrp(kwargs: &EncodeKwargs<'_>) -> &'static str {
        kwargs.hrp.unwrap_or(MultiversXMainnet::HRP)
    }

    fn hrp_decode(kwargs: &DecodeKwargs) -> &'static str {
        kwargs.hrp.unwrap_or(MultiversXMainnet::HRP)
    }

    fn pubkey_body_from_compressed(compressed: &[u8]) -> Result<[u8; PUBKEY_BODY_LEN], AddressSchemeError> {
        let body = if compressed.len() == PUBKEY_BODY_LEN + 1 && compressed[0] == 0x00 {
            &compressed[1..]
        } else if compressed.len() == PUBKEY_BODY_LEN {
            compressed
        } else {
            return Err(AddressSchemeError::Msg(format!(
                "expected {}-byte Ed25519 public key body (or 33-byte compressed with 0x00 prefix)",
                PUBKEY_BODY_LEN
            )));
        };
        body.try_into()
            .map_err(|_| AddressSchemeError::Msg("invalid Ed25519 public key body length".into()))
    }

    fn pubkey_body(pk: &Slip10Ed25519PublicKeyStd) -> Result<[u8; PUBKEY_BODY_LEN], AddressSchemeError> {
        Self::pubkey_body_from_compressed(&pk.raw_compressed())
    }

    fn encode_bech32(hrp: &str, body: &[u8; PUBKEY_BODY_LEN]) -> Result<String, AddressSchemeError> {
        bech32_encode_raw(hrp, body).ok_or_else(|| {
            AddressSchemeError::Msg(format!("failed to bech32-encode {hrp} address"))
        })
    }

    fn decode_bech32(hrp: &str, address: &str) -> Result<String, AddressSchemeError> {
        let (hrp_got, data) = base_bech32_decode(address).ok_or_else(|| {
            AddressSchemeError::Msg(format!("invalid bech32 address for hrp {hrp}"))
        })?;
        if hrp_got != hrp {
            return Err(AddressSchemeError::Msg(format!(
                "invalid bech32 hrp (expected: {hrp}, got: {hrp_got})"
            )));
        }
        let decoded = convertbits(&data, 5, 8, false).ok_or_else(|| {
            AddressSchemeError::Msg("invalid bech32 payload".into())
        })?;
        if decoded.len() != PUBKEY_BODY_LEN {
            return Err(AddressSchemeError::Msg(format!(
                "Invalid length (expected: {PUBKEY_BODY_LEN}, got: {})",
                decoded.len()
            )));
        }
        Ok(hex::encode(decoded))
    }
}

impl IAddress for MultiversXAddress {
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
        let body = Self::pubkey_body(&pk)?;
        Self::encode_bech32(Self::hrp(kwargs), &body)
    }

    fn decode(&self, address: &str, kwargs: &DecodeKwargs) -> super::iaddress::AddressEncodeResult {
        Self::decode_bech32(Self::hrp_decode(kwargs), address)
    }
}
