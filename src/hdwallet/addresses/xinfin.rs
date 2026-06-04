use crate::hdwallet::cryptocurrencies::xinfin::XINFIN_PARAMS;
use crate::hdwallet::eccs::PublicKeyInput;

use super::ethereum::EthereumAddress;
use super::iaddress::{DecodeKwargs, EncodeKwargs, EncodePublicKey, IAddress};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct XinFinAddress;

impl XinFinAddress {
    pub const REGISTRY_NAME: &'static str = "XinFin";

    fn address_prefix() -> &'static str {
        EthereumAddress::prefix_from_params(XINFIN_PARAMS.address_prefix)
    }
}

impl IAddress for XinFinAddress {
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
        EthereumAddress::encode_public_key_with_prefix(
            input,
            kwargs.skip_checksum_encode.unwrap_or(false),
            Self::address_prefix(),
        )
    }

    fn decode(&self, address: &str, kwargs: &DecodeKwargs) -> super::iaddress::AddressEncodeResult {
        EthereumAddress::decode_address_with_prefix(
            address,
            kwargs.skip_checksum_encode.unwrap_or(false),
            Self::address_prefix(),
        )
    }
}
