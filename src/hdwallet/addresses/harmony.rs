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

#[cfg(test)]
mod tests {
    use bitcoin::Network;

    use super::*;
    use super::super::ethereum::EthereumAddress;
    use super::super::iaddress::encode_with_ipublic_key;
    use crate::hdwallet::eccs::{IPublicKey, PublicKeyInput, Slip10Secp256k1PublicKey};
    use crate::hdwallet::hds::bip44::bip44_path;
    use crate::hdwallet::hds::Bip32Hd;
    use crate::hdwallet::slip44::coin_types;

    fn test_public_key() -> Slip10Secp256k1PublicKey {
        let seed = hex::decode(
            "5eb00bbddcf069084889a8ab9155568165f4c767744d69fbf5d1791c2fe1563254c36a54693640219b24e06f1011116141de699825262eeacc22f8ac8ae26a",
        )
        .expect("seed hex");
        let hd = Bip32Hd::from_seed(Network::Bitcoin, &seed).expect("bip32");
        let path = bip44_path(coin_types::HARMONY, 0, 0, 0).expect("path");
        let compressed = hd.derive_xpub(&path).expect("xpub").public_key.serialize();
        Slip10Secp256k1PublicKey::from_bytes(&compressed).expect("pk")
    }

    #[test]
    fn roundtrip_default_hrp() {
        let pk = test_public_key();
        let harmony = HarmonyAddress;
        let addr = encode_with_ipublic_key(&harmony, &pk, &EncodeKwargs::default()).expect("encode");
        assert!(addr.starts_with("one1"));
        let decoded = harmony
            .decode(&addr, &DecodeKwargs::default())
            .expect("decode");
        assert_eq!(decoded.len(), 40);
        let eth = EthereumAddress::encode_public_key(
            crate::hdwallet::eccs::PublicKeyInput::Instance(pk),
            true,
        )
        .expect("eth");
        assert_eq!(decoded, eth[2..]);
    }
}
