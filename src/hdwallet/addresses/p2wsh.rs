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

    fn script_hash_from_public_key_bytes(pubkey_bytes: &[u8]) -> [u8; 32] {
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

#[cfg(test)]
mod tests {
    use bitcoin::blockdata::opcodes::all::{OP_CHECKMULTISIG, OP_PUSHNUM_1};
    use bitcoin::blockdata::script::{Builder, PushBytes};
    use bitcoin::Address as BtcAddress;
    use bitcoin::KnownHrp;
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
        let p2wsh = P2WSHAddress;
        let addr = encode_with_ipublic_key(&p2wsh, &pk, &EncodeKwargs::default()).expect("encode");
        assert!(addr.to_ascii_lowercase().starts_with("bc1q"));
        let decoded = p2wsh.decode(&addr, &DecodeKwargs::default()).expect("decode");
        assert_eq!(decoded.len(), 64);
        let script_hash =
            P2WSHAddress::script_hash_from_public_key_bytes(&pk.raw_compressed());
        assert_eq!(decoded, hex::encode(script_hash));
    }

    #[test]
    fn matches_bitcoin_crate_p2wsh() {
        let pk = test_public_key();
        let compressed = pk.raw_compressed();
        let push_bytes: &PushBytes = compressed
            .as_slice()
            .try_into()
            .expect("compressed pubkey push");
        let script = Builder::new()
            .push_opcode(OP_PUSHNUM_1)
            .push_slice(push_bytes)
            .push_opcode(OP_PUSHNUM_1)
            .push_opcode(OP_CHECKMULTISIG)
            .into_script();
        let expected = BtcAddress::p2wsh(&script, KnownHrp::Mainnet).to_string();
        let got = encode_with_ipublic_key(&P2WSHAddress, &pk, &EncodeKwargs::default()).expect("encode");
        assert_eq!(got, expected);
    }
}
