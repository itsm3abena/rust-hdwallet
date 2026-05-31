use crate::hdwallet::eccs::{
    validate_and_get_public_key, IPublicKey, PublicKeyInput, Slip10Secp256k1PublicKey,
};
use crate::hdwallet::exceptions::PublicKeyError;
use crate::hdwallet::libs::ecc::sha256;
use crate::hdwallet::libs::segwit_bech32::segwit_encode;

use super::iaddress::{
    AddressSchemeError, DecodeKwargs, EncodeKwargs, EncodePublicKey, IAddress,
};
use super::p2wpkh::P2WPKHAddress;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct P2WSHAddress;

impl P2WSHAddress {
    pub const REGISTRY_NAME: &'static str = "P2WSH";

    const DEFAULT_WITNESS_VERSION: u8 = 0x00;

    fn witness_version_encode(kwargs: &EncodeKwargs<'_>) -> u8 {
        kwargs
            .witness_version
            .unwrap_or(Self::DEFAULT_WITNESS_VERSION)
    }

    pub(crate) fn script_hash_from_public_key_bytes(pubkey_bytes: &[u8]) -> [u8; 32] {
        let script_hex = format!("5121{}51ae", hex::encode(pubkey_bytes));
        let script_bytes = hex::decode(&script_hex).expect("valid P2WSH witness script hex");
        sha256(&script_bytes)
    }

    pub(crate) fn encode_public_key(
        pk: &Slip10Secp256k1PublicKey,
        hrp: &str,
        witness_version: u8,
        compressed: bool,
    ) -> Result<String, AddressSchemeError> {
        let pubkey_bytes = if compressed {
            pk.raw_compressed()
        } else {
            pk.raw_uncompressed()
        };
        let script_hash = Self::script_hash_from_public_key_bytes(&pubkey_bytes);
        segwit_encode(hrp, witness_version, &script_hash).ok_or_else(|| {
            AddressSchemeError::Msg(format!("failed to segwit-encode {hrp} P2WSH address"))
        })
    }
}

impl IAddress for P2WSHAddress {
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
        Self::encode_public_key(
            &pk,
            P2WPKHAddress::hrp_encode(kwargs),
            Self::witness_version_encode(kwargs),
            P2WPKHAddress::use_compressed_public_key(kwargs),
        )
    }

    fn decode(&self, address: &str, kwargs: &DecodeKwargs) -> super::iaddress::AddressEncodeResult {
        P2WPKHAddress::decode_address(address, P2WPKHAddress::hrp_decode(kwargs))
    }
}
