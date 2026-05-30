use crate::hdwallet::eccs::{
    validate_and_get_public_key, PublicKeyInput, Slip10Secp256k1PublicKey,
};
use crate::hdwallet::exceptions::PublicKeyError;

use super::iaddress::{
    AddressSchemeError, DecodeKwargs, EncodeKwargs, EncodePublicKey, IAddress,
};
use super::p2pkh::P2PKHAddress;
use super::p2sh::P2SHAddress;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct P2WPKHInP2SHAddress;

impl P2WPKHInP2SHAddress {
    pub const REGISTRY_NAME: &'static str = "P2WPKH-In-P2SH";

    pub(crate) fn encode_public_key(
        pk: &Slip10Secp256k1PublicKey,
        prefix: u32,
        compressed: bool,
        alphabet: Option<&'static str>,
    ) -> String {
        let pubkey_hash = P2PKHAddress::public_key_hash(pk, compressed);
        let script_hash = P2SHAddress::script_hash_from_nested_p2wpkh(&pubkey_hash);
        P2SHAddress::encode_script_hash(&script_hash, prefix, alphabet)
    }
}

impl IAddress for P2WPKHInP2SHAddress {
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
        let nested = P2WPKHInP2SHAddress;
        let addr = encode_with_ipublic_key(&nested, &pk, &EncodeKwargs::default()).expect("encode");
        assert!(addr.starts_with('3'));
        let decoded = nested.decode(&addr, &DecodeKwargs::default()).expect("decode");
        assert_eq!(decoded.len(), 40);
        let pubkey_hash = P2PKHAddress::public_key_hash(&pk, true);
        let script_hash = P2SHAddress::script_hash_from_nested_p2wpkh(&pubkey_hash);
        assert_eq!(decoded, hex::encode(script_hash));
    }

    #[test]
    fn matches_bitcoin_crate_p2shwpkh() {
        let pk = test_public_key();
        let cpk = CompressedPublicKey::from_slice(&pk.raw_compressed()).expect("compressed");
        let expected = BtcAddress::p2shwpkh(&cpk, Network::Bitcoin).to_string();
        let got =
            encode_with_ipublic_key(&P2WPKHInP2SHAddress, &pk, &EncodeKwargs::default()).expect("encode");
        assert_eq!(got, expected);
    }
}
