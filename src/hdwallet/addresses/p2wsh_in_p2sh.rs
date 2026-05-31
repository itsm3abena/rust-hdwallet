use crate::hdwallet::eccs::{
    validate_and_get_public_key, IPublicKey, PublicKeyInput, Slip10Secp256k1PublicKey,
};
use crate::hdwallet::exceptions::PublicKeyError;

use super::iaddress::{
    AddressSchemeError, DecodeKwargs, EncodeKwargs, EncodePublicKey, IAddress,
};
use super::p2sh::P2SHAddress;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct P2WSHInP2SHAddress;

impl P2WSHInP2SHAddress {
    pub const REGISTRY_NAME: &'static str = "P2WSH-In-P2SH";

    pub(crate) fn encode_public_key(
        pk: &Slip10Secp256k1PublicKey,
        prefix: u32,
        compressed: bool,
        alphabet: Option<&'static str>,
    ) -> String {
        let pubkey_bytes = if compressed {
            pk.raw_compressed()
        } else {
            pk.raw_uncompressed()
        };
        let script_hash = P2SHAddress::script_hash_from_nested_p2wsh(&pubkey_bytes);
        P2SHAddress::encode_script_hash(&script_hash, prefix, alphabet)
    }
}

impl IAddress for P2WSHInP2SHAddress {
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
            P2SHAddress::script_address_prefix_encode(kwargs),
            P2SHAddress::use_compressed_public_key(kwargs),
            kwargs.alphabet,
        ))
    }

    fn decode(&self, address: &str, kwargs: &DecodeKwargs) -> super::iaddress::AddressEncodeResult {
        P2SHAddress::decode_address(
            address,
            P2SHAddress::script_address_prefix_decode(kwargs),
            kwargs.alphabet,
        )
    }
}
