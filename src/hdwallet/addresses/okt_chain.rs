use crate::hdwallet::cryptocurrencies::oktchain::Mainnet as OktChainMainnet;
use crate::hdwallet::eccs::{PublicKeyInput, Slip10Secp256k1PublicKey};
use crate::hdwallet::libs::segwit_bech32::{base_bech32_decode, bech32_encode, convertbits};

use super::ethereum::EthereumAddress;
use super::iaddress::{
    AddressSchemeError, DecodeKwargs, EncodeKwargs, EncodePublicKey, IAddress,
};

pub(crate) fn encode_bech32_eth_address(
    public_key: PublicKeyInput<'_, Slip10Secp256k1PublicKey>,
    default_hrp: &str,
    kwargs: &EncodeKwargs<'_>,
) -> Result<String, AddressSchemeError> {
    let eth_address = EthereumAddress::encode_public_key(public_key, true)?;
    let hash_bytes = hex::decode(&eth_address[2..]).map_err(|e| AddressSchemeError::Msg(e.to_string()))?;
    let hrp = kwargs.hrp.unwrap_or(default_hrp);
    bech32_encode(hrp, &hash_bytes).ok_or_else(|| {
        AddressSchemeError::Msg(format!("failed to bech32-encode {hrp} address"))
    })
}

pub(crate) fn decode_bech32_eth_address(
    address: &str,
    default_hrp: &str,
    kwargs: &DecodeKwargs,
) -> Result<String, AddressSchemeError> {
    let hrp = kwargs.hrp.unwrap_or(default_hrp);
    let (hrp_got, data) = base_bech32_decode(address).ok_or_else(|| {
        AddressSchemeError::Msg(format!("invalid bech32 address for hrp {hrp}"))
    })?;
    if hrp_got != hrp {
        return Err(AddressSchemeError::Msg(format!(
            "invalid bech32 hrp (expected: {hrp}, got: {hrp_got})"
        )));
    }
    let payload = convertbits(&data, 5, 8, false).ok_or_else(|| {
        AddressSchemeError::Msg("invalid bech32 payload".into())
    })?;
    let eth_address = format!("0x{}", hex::encode(&payload));
    EthereumAddress::decode_address(&eth_address, true)
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct OKTChainAddress;

impl OKTChainAddress {
    pub const REGISTRY_NAME: &'static str = "OKT-Chain";

    fn default_hrp() -> &'static str {
        OktChainMainnet::HRP
    }
}

impl IAddress for OKTChainAddress {
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
    use super::super::iaddress::encode_with_ipublic_key;
    use crate::hdwallet::eccs::IPublicKey;
    use crate::hdwallet::hds::bip44::bip44_path;
    use crate::hdwallet::hds::Bip32Hd;
    use crate::hdwallet::slip44::coin_types;

    fn test_public_key() -> Slip10Secp256k1PublicKey {
        let seed = hex::decode(
            "5eb00bbddcf069084889a8ab9155568165f4c767744d69fbf5d1791c2fe1563254c36a54693640219b24e06f1011116141de699825262eeacc22f8ac8ae26a",
        )
        .expect("seed hex");
        let hd = Bip32Hd::from_seed(Network::Bitcoin, &seed).expect("bip32");
        let path = bip44_path(coin_types::OKT_CHAIN, 0, 0, 0).expect("path");
        let compressed = hd.derive_xpub(&path).expect("xpub").public_key.serialize();
        Slip10Secp256k1PublicKey::from_bytes(&compressed).expect("pk")
    }

    #[test]
    fn roundtrip_default_hrp() {
        let pk = test_public_key();
        let okt = OKTChainAddress;
        let addr = encode_with_ipublic_key(&okt, &pk, &EncodeKwargs::default()).expect("encode");
        assert!(addr.starts_with("ex1"));
        let decoded = okt.decode(&addr, &DecodeKwargs::default()).expect("decode");
        assert_eq!(decoded.len(), 40);
        let eth = EthereumAddress::encode_public_key(PublicKeyInput::Instance(pk), true)
            .expect("eth");
        assert_eq!(decoded, eth[2..]);
    }
}
