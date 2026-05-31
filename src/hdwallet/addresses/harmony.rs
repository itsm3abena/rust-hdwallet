use crate::hdwallet::cryptocurrencies::harmony::Mainnet as HarmonyMainnet;
use crate::hdwallet::eccs::PublicKeyInput;

use super::iaddress::{DecodeKwargs, EncodeKwargs, EncodePublicKey, IAddress};
use super::okt_chain::{decode_bech32_eth_address, encode_bech32_eth_address};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct HarmonyAddress;

impl HarmonyAddress {
    pub const REGISTRY_NAME: &'static str = "Harmony";

    fn default_hrp() -> &'static str {
        HarmonyMainnet::HRP
    }
}

impl IAddress for HarmonyAddress {
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
        encode_bech32_eth_address(input, Self::default_hrp(), kwargs)
    }

    fn decode(&self, address: &str, kwargs: &DecodeKwargs) -> super::iaddress::AddressEncodeResult {
        decode_bech32_eth_address(address, Self::default_hrp(), kwargs)
    }
}
