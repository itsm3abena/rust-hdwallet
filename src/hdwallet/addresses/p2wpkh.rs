use crate::hdwallet::consts::PublicKeyTypes;
use crate::hdwallet::cryptocurrencies::bitcoin::{
    Mainnet as BitcoinMainnet, Regtest as BitcoinRegtest, Testnet as BitcoinTestnet,
};
use crate::hdwallet::eccs::{
    validate_and_get_public_key, PublicKeyInput, Slip10Secp256k1PublicKey,
};
use crate::hdwallet::exceptions::PublicKeyError;
use crate::hdwallet::libs::segwit_bech32::{segwit_decode, segwit_encode};

use super::iaddress::{
    AddressSchemeError, DecodeKwargs, EncodeKwargs, EncodePublicKey, IAddress,
};
use super::p2pkh::P2PKHAddress;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct P2WPKHAddress;

impl P2WPKHAddress {
    pub const REGISTRY_NAME: &'static str = "P2WPKH";

    const DEFAULT_WITNESS_VERSION: u8 = 0x00;

    fn default_hrp() -> &'static str {
        BitcoinMainnet::HRP
    }

    pub(crate) fn hrp_encode(kwargs: &EncodeKwargs<'_>) -> &'static str {
        kwargs.hrp.unwrap_or_else(|| Self::hrp_from_network(kwargs.network))
    }

    pub(crate) fn hrp_decode(kwargs: &DecodeKwargs) -> &'static str {
        kwargs.hrp.unwrap_or_else(|| Self::hrp_from_network(kwargs.network))
    }

    fn hrp_from_network(network: Option<&str>) -> &'static str {
        match network {
            Some("testnet") => BitcoinTestnet::HRP,
            Some("regtest") => BitcoinRegtest::HRP,
            _ => Self::default_hrp(),
        }
    }

    fn witness_version_encode(kwargs: &EncodeKwargs<'_>) -> u8 {
        kwargs
            .witness_version
            .unwrap_or(Self::DEFAULT_WITNESS_VERSION)
    }

    pub(crate) fn use_compressed_public_key(kwargs: &EncodeKwargs<'_>) -> bool {
        !matches!(
            kwargs.public_key_type,
            Some(PublicKeyTypes::UNCOMPRESSED)
        )
    }

    pub(crate) fn encode_public_key(
        pk: &Slip10Secp256k1PublicKey,
        hrp: &str,
        witness_version: u8,
        compressed: bool,
    ) -> Result<String, AddressSchemeError> {
        let hash = P2PKHAddress::public_key_hash(pk, compressed);
        segwit_encode(hrp, witness_version, &hash).ok_or_else(|| {
            AddressSchemeError::Msg(format!("failed to segwit-encode {hrp} P2WPKH address"))
        })
    }

    pub(crate) fn decode_address(address: &str, hrp: &str) -> Result<String, AddressSchemeError> {
        let (_witver, address_decode) = segwit_decode(hrp, address).ok_or_else(|| {
            AddressSchemeError::Msg(format!("invalid segwit address for hrp {hrp}"))
        })?;
        Ok(hex::encode(address_decode))
    }
}

impl IAddress for P2WPKHAddress {
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
            Self::hrp_encode(kwargs),
            Self::witness_version_encode(kwargs),
            Self::use_compressed_public_key(kwargs),
        )
    }

    fn decode(&self, address: &str, kwargs: &DecodeKwargs) -> super::iaddress::AddressEncodeResult {
        Self::decode_address(address, Self::hrp_decode(kwargs))
    }
}

#[cfg(test)]
mod tests {
    use bitcoin::Address as BtcAddress;
    use bitcoin::CompressedPublicKey;
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
        let path = bip44_path(coin_types::BITCOIN, 0, 0, 0).expect("path");
        let compressed = hd.derive_xpub(&path).expect("xpub").public_key.serialize();
        Slip10Secp256k1PublicKey::from_bytes(&compressed).expect("pk")
    }

    #[test]
    fn roundtrip_default_mainnet() {
        let pk = test_public_key();
        let p2wpkh = P2WPKHAddress;
        let addr = encode_with_ipublic_key(&p2wpkh, &pk, &EncodeKwargs::default()).expect("encode");
        assert!(addr.to_ascii_lowercase().starts_with("bc1q"));
        let decoded = p2wpkh.decode(&addr, &DecodeKwargs::default()).expect("decode");
        assert_eq!(decoded.len(), 40);
        assert_eq!(
            decoded,
            hex::encode(P2PKHAddress::public_key_hash(&pk, true))
        );
    }

    #[test]
    fn matches_bitcoin_crate_p2wpkh() {
        let pk = test_public_key();
        let cpk = CompressedPublicKey::from_slice(&pk.raw_compressed()).expect("compressed");
        let expected = BtcAddress::p2wpkh(&cpk, bitcoin::KnownHrp::Mainnet).to_string();
        let got = encode_with_ipublic_key(&P2WPKHAddress, &pk, &EncodeKwargs::default()).expect("encode");
        assert_eq!(got, expected);
    }
}
