use crate::hdwallet::cryptocurrencies::algorand::ALGORAND_PARAMS;
use crate::hdwallet::eccs::{
    validate_and_get_public_key, IPublicKey, PublicKeyInput, Slip10Ed25519PublicKeyStd,
};
use crate::hdwallet::exceptions::PublicKeyError;
use crate::hdwallet::exceptions::Error as CoreError;
use crate::hdwallet::hds::algorand::{decode_address as decode_algo_payload, encode_address};
use crate::hdwallet::utils::sha512_256;

use super::iaddress::{
    AddressSchemeError, DecodeKwargs, EncodeKwargs, EncodePublicKey, IAddress,
};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct AlgorandAddress;

impl AlgorandAddress {
    pub const REGISTRY_NAME: &'static str = "Algorand";

    #[inline]
    pub fn checksum_length() -> usize {
        ALGORAND_PARAMS.checksum_length.unwrap_or(4) as usize
    }

    pub fn compute_checksum(pubkey_body: &[u8]) -> [u8; 4] {
        let h = sha512_256(pubkey_body);
        h[28..32].try_into().expect("sha512-256 is 32 bytes")
    }

    fn body_32_from_slip(pk: &Slip10Ed25519PublicKeyStd) -> Result<[u8; 32], AddressSchemeError> {
        let raw = pk.raw_compressed();
        raw[1..]
            .try_into()
            .map_err(|_| AddressSchemeError::Msg("expected 33-byte compressed SLIP10 Ed25519 key".into()))
    }

    fn map_core(err: CoreError) -> AddressSchemeError {
        AddressSchemeError::Msg(err.to_string())
    }
}

impl IAddress for AlgorandAddress {
    fn name(&self) -> &'static str {
        Self::REGISTRY_NAME
    }

    fn encode(
        &self,
        public_key: EncodePublicKey<'_>,
        _kwargs: &EncodeKwargs<'_>,
    ) -> super::iaddress::AddressEncodeResult {
        let input = match public_key {
            EncodePublicKey::Bytes(b) => PublicKeyInput::Bytes(b),
            EncodePublicKey::Str(s) => PublicKeyInput::Hex(s),
        };
        let pk = validate_and_get_public_key::<Slip10Ed25519PublicKeyStd>(input).map_err(
            |e: PublicKeyError| AddressSchemeError::Msg(e.to_string()),
        )?;
        let body = Self::body_32_from_slip(&pk)?;
        encode_address(&body).map_err(Self::map_core)
    }

    fn decode(&self, address: &str, _kwargs: &DecodeKwargs) -> super::iaddress::AddressEncodeResult {
        decode_algo_payload(address).map_err(Self::map_core)
    }
}
