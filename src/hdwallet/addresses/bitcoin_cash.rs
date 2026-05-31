use crate::hdwallet::consts::PublicKeyTypes;
use crate::hdwallet::cryptocurrencies::bitcoin_cash::Mainnet as BitcoinCashMainnet;
use crate::hdwallet::eccs::{
    validate_and_get_public_key, IPublicKey, PublicKeyInput, Slip10Secp256k1PublicKey,
};
use crate::hdwallet::exceptions::PublicKeyError;
use crate::hdwallet::libs::cashaddr::{cashaddr_decode, cashaddr_encode};
use crate::hdwallet::libs::ecc::hash160;

use super::iaddress::{
    AddressSchemeError, DecodeKwargs, EncodeKwargs, EncodePublicKey, IAddress,
};

const P2PKH_VERSION_BYTE: u8 = 0x00;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct BitcoinCashAddress;

impl BitcoinCashAddress {
    pub const REGISTRY_NAME: &'static str = "BitcoinCash";

    fn hrp(kwargs: &EncodeKwargs<'_>) -> &'static str {
        kwargs.hrp.unwrap_or(BitcoinCashMainnet::HRP)
    }

    fn hrp_decode(kwargs: &DecodeKwargs) -> &'static str {
        kwargs.hrp.unwrap_or(BitcoinCashMainnet::HRP)
    }

    fn use_compressed(kwargs: &EncodeKwargs<'_>) -> bool {
        match kwargs.public_key_type {
            None | Some(PublicKeyTypes::COMPRESSED) => true,
            Some(PublicKeyTypes::UNCOMPRESSED) => false,
            Some(_) => true,
        }
    }

    fn public_key_hash(pk: &Slip10Secp256k1PublicKey, compressed: bool) -> [u8; 20] {
        let raw = if compressed {
            pk.raw_compressed()
        } else {
            pk.raw_uncompressed()
        };
        hash160(&raw)
    }

    fn encode_payload(hrp: &str, hash: &[u8; 20]) -> Result<String, AddressSchemeError> {
        let mut payload = Vec::with_capacity(21);
        payload.push(P2PKH_VERSION_BYTE);
        payload.extend_from_slice(hash);
        cashaddr_encode(hrp, &payload).ok_or_else(|| {
            AddressSchemeError::Msg(format!("failed to CashAddr-encode for hrp {hrp}"))
        })
    }

    fn decode_payload(hrp: &str, address: &str) -> Result<String, AddressSchemeError> {
        let decoded = cashaddr_decode(hrp, address).ok_or_else(|| {
            AddressSchemeError::Msg("invalid CashAddr".into())
        })?;
        if decoded.len() < 21 {
            return Err(AddressSchemeError::Msg("invalid CashAddr data".into()));
        }
        Ok(hex::encode(&decoded[1..21]))
    }
}

impl IAddress for BitcoinCashAddress {
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
        let hash = Self::public_key_hash(&pk, Self::use_compressed(kwargs));
        Self::encode_payload(Self::hrp(kwargs), &hash)
    }

    fn decode(
        &self,
        address: &str,
        kwargs: &DecodeKwargs,
    ) -> super::iaddress::AddressEncodeResult {
        if let Some((hrp_got, _)) = address.split_once(':') {
            let expected = Self::hrp_decode(kwargs);
            if hrp_got != expected {
                return Err(AddressSchemeError::Msg(format!(
                    "Invalid HRP (expected: {expected}, got: {hrp_got})"
                )));
            }
        }
        Self::decode_payload(Self::hrp_decode(kwargs), address)
    }
}
