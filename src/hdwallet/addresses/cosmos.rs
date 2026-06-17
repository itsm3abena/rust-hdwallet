use crate::hdwallet::cryptocurrencies::cosmos::Mainnet as CosmosMainnet;
use crate::hdwallet::eccs::{
    validate_and_get_public_key, IPublicKey, PublicKeyInput, Slip10Secp256k1PublicKey,
};
use crate::hdwallet::exceptions::PublicKeyError;
use crate::hdwallet::libs::ecc::sha256;
use crate::hdwallet::libs::ripemd160::ripemd160;
use crate::hdwallet::libs::bech32::{base_bech32_decode, bech32_encode_raw, convert_bits as convertbits};

use super::iaddress::{
    AddressSchemeError, DecodeKwargs, EncodeKwargs, EncodePublicKey, IAddress,
};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct CosmosAddress;

impl CosmosAddress {
    pub const REGISTRY_NAME: &'static str = "Cosmos";

    pub fn hrp(kwargs: &EncodeKwargs<'_>) -> &'static str {
        kwargs.hrp.unwrap_or(CosmosMainnet::HRP)
    }

    pub fn hrp_decode(kwargs: &DecodeKwargs) -> &'static str {
        kwargs.hrp.unwrap_or(CosmosMainnet::HRP)
    }

    pub(crate) fn public_key_hash(compressed: &[u8]) -> [u8; 20] {
        ripemd160(&sha256(compressed))
    }

    fn encode_bech32(hrp: &str, public_key_hash: &[u8; 20]) -> Result<String, AddressSchemeError> {
        bech32_encode_raw(hrp, public_key_hash).ok_or_else(|| {
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
        Ok(hex::encode(decoded))
    }
}

impl IAddress for CosmosAddress {
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
        let hash = Self::public_key_hash(&pk.raw_compressed());
        Self::encode_bech32(Self::hrp(kwargs), &hash)
    }

    fn decode(
        &self,
        address: &str,
        kwargs: &DecodeKwargs,
    ) -> super::iaddress::AddressEncodeResult {
        Self::decode_bech32(Self::hrp_decode(kwargs), address)
    }
}
