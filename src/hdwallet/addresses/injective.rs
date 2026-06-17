use crate::hdwallet::cryptocurrencies::injective::Mainnet as InjectiveMainnet;
use crate::hdwallet::eccs::{
    validate_and_get_public_key, PublicKeyInput, Slip10Secp256k1PublicKey,
};
use crate::hdwallet::exceptions::PublicKeyError;
use crate::hdwallet::libs::bech32::{base_bech32_decode, bech32_encode_raw, convert_bits as convertbits};

use super::ethereum::EthereumAddress;
use super::iaddress::{
    AddressSchemeError, DecodeKwargs, EncodeKwargs, EncodePublicKey, IAddress,
};

const HASH_LEN: usize = 20;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct InjectiveAddress;

impl InjectiveAddress {
    pub const REGISTRY_NAME: &'static str = "Injective";

    fn default_hrp() -> &'static str {
        InjectiveMainnet::HRP
    }

    fn hrp(kwargs: &EncodeKwargs<'_>) -> &'static str {
        kwargs.hrp.unwrap_or(Self::default_hrp())
    }

    fn hrp_decode(kwargs: &DecodeKwargs) -> &'static str {
        kwargs.hrp.unwrap_or(Self::default_hrp())
    }

    fn eth_hash_bytes(
        public_key: PublicKeyInput<'_, Slip10Secp256k1PublicKey>,
    ) -> Result<[u8; HASH_LEN], AddressSchemeError> {
        let eth_address = EthereumAddress::encode_public_key(public_key, false)?;
        let bytes = hex::decode(&eth_address[2..]).map_err(|e| AddressSchemeError::Msg(e.to_string()))?;
        bytes
            .try_into()
            .map_err(|_| AddressSchemeError::Msg(format!("expected {HASH_LEN}-byte hash")))
    }

    pub fn encode_public_key(
        public_key: PublicKeyInput<'_, Slip10Secp256k1PublicKey>,
        kwargs: &EncodeKwargs<'_>,
    ) -> Result<String, AddressSchemeError> {
        let pk = validate_and_get_public_key(public_key)
            .map_err(|e: PublicKeyError| AddressSchemeError::Msg(e.to_string()))?;
        let hash_bytes = Self::eth_hash_bytes(PublicKeyInput::Instance(pk))?;
        let hrp = Self::hrp(kwargs);
        bech32_encode_raw(hrp, &hash_bytes).ok_or_else(|| {
            AddressSchemeError::Msg(format!("failed to bech32-encode {hrp} address"))
        })
    }

    pub fn decode_address(address: &str, kwargs: &DecodeKwargs) -> Result<String, AddressSchemeError> {
        let hrp = Self::hrp_decode(kwargs);
        let (hrp_got, data) = base_bech32_decode(address).ok_or_else(|| {
            AddressSchemeError::Msg(format!("invalid bech32 address for hrp {hrp}"))
        })?;
        if hrp_got != hrp {
            return Err(AddressSchemeError::Msg(format!(
                "invalid bech32 hrp (expected: {hrp}, got: {hrp_got})"
            )));
        }
        let address_decode = convertbits(&data, 5, 8, false).ok_or_else(|| {
            AddressSchemeError::Msg("invalid bech32 payload".into())
        })?;
        if address_decode.len() != HASH_LEN {
            return Err(AddressSchemeError::Msg(format!(
                "Invalid length (expected: {HASH_LEN}, got: {})",
                address_decode.len()
            )));
        }
        Ok(hex::encode(address_decode))
    }
}

impl IAddress for InjectiveAddress {
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
