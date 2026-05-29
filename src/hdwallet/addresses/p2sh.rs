use crate::hdwallet::consts::PublicKeyTypes;
use crate::hdwallet::cryptocurrencies::bitcoin::{
    Mainnet as BitcoinMainnet, Testnet as BitcoinTestnet, BITCOIN_PARAMS,
};
use crate::hdwallet::eccs::{
    validate_and_get_public_key, PublicKeyInput, Slip10Secp256k1PublicKey,
};
use crate::hdwallet::exceptions::PublicKeyError;
use crate::hdwallet::libs::base58::{self, Base58Error};
use crate::hdwallet::libs::ecc::hash160;
use crate::hdwallet::utils::integer_to_bytes_python;

use super::iaddress::{
    AddressSchemeError, DecodeKwargs, EncodeKwargs, EncodePublicKey, IAddress,
};
use super::p2pkh::P2PKHAddress;

const HASH_LEN: usize = 20;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct P2SHAddress;

impl P2SHAddress {
    pub const REGISTRY_NAME: &'static str = "P2SH";

    fn default_script_address_prefix() -> u32 {
        BitcoinMainnet::SCRIPT_ADDRESS_PREFIX
    }

    fn script_address_prefix_encode(kwargs: &EncodeKwargs<'_>) -> u32 {
        kwargs
            .script_address_prefix
            .unwrap_or_else(|| Self::script_address_prefix_from_network(kwargs.network))
    }

    fn script_address_prefix_decode(kwargs: &DecodeKwargs) -> u32 {
        kwargs
            .script_address_prefix
            .unwrap_or_else(|| Self::script_address_prefix_from_network(kwargs.network))
    }

    fn script_address_prefix_from_network(network: Option<&str>) -> u32 {
        match network {
            Some("testnet") | Some("regtest") => BitcoinTestnet::SCRIPT_ADDRESS_PREFIX,
            _ => Self::default_script_address_prefix(),
        }
    }

    fn prefix_bytes(prefix: u32) -> Vec<u8> {
        integer_to_bytes_python(prefix, None)
    }

    fn alphabet(kwargs_alphabet: Option<&'static str>) -> &'static str {
        kwargs_alphabet.unwrap_or_else(|| {
            BITCOIN_PARAMS
                .alphabet
                .unwrap_or(base58::DEFAULT_ALPHABET)
        })
    }

    fn use_compressed_public_key(kwargs: &EncodeKwargs<'_>) -> bool {
        !matches!(
            kwargs.public_key_type,
            Some(PublicKeyTypes::UNCOMPRESSED)
        )
    }

    fn script_hash_from_public_key_hash(pubkey_hash: &[u8; HASH_LEN]) -> [u8; HASH_LEN] {
        let script_hex = format!("76a914{}88ac", hex::encode(pubkey_hash));
        let script_bytes = hex::decode(&script_hex).expect("valid P2PKH script hex");
        hash160(&script_bytes)
    }

    fn map_base58(err: Base58Error) -> AddressSchemeError {
        AddressSchemeError::Msg(err.to_string())
    }

    pub(crate) fn encode_public_key(
        pk: &Slip10Secp256k1PublicKey,
        prefix: u32,
        compressed: bool,
        alphabet: Option<&'static str>,
    ) -> String {
        let pubkey_hash = P2PKHAddress::public_key_hash(pk, compressed);
        let script_hash = Self::script_hash_from_public_key_hash(&pubkey_hash);
        let mut raw = Self::prefix_bytes(prefix);
        raw.extend_from_slice(&script_hash);
        base58::check_encode(&raw, Some(Self::alphabet(alphabet)))
    }

    pub(crate) fn decode_address(
        address: &str,
        prefix: u32,
        alphabet: Option<&'static str>,
    ) -> Result<String, AddressSchemeError> {
        let address_decode =
            base58::check_decode(address.as_bytes(), Some(Self::alphabet(alphabet)))
                .map_err(Self::map_base58)?;

        let prefix_bytes = Self::prefix_bytes(prefix);
        let expected_length = HASH_LEN + prefix_bytes.len();
        if address_decode.len() != expected_length {
            return Err(AddressSchemeError::Msg(format!(
                "Invalid length (expected: {expected_length}, got: {})",
                address_decode.len()
            )));
        }

        let prefix_got = &address_decode[..prefix_bytes.len()];
        if prefix_got != prefix_bytes.as_slice() {
            return Err(AddressSchemeError::Msg(format!(
                "Invalid prefix (expected: {}, got: {})",
                hex::encode(&prefix_bytes),
                hex::encode(prefix_got)
            )));
        }

        Ok(hex::encode(&address_decode[prefix_bytes.len()..]))
    }
}

impl IAddress for P2SHAddress {
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
            Self::script_address_prefix_encode(kwargs),
            Self::use_compressed_public_key(kwargs),
            kwargs.alphabet,
        ))
    }

    fn decode(&self, address: &str, kwargs: &DecodeKwargs) -> super::iaddress::AddressEncodeResult {
        Self::decode_address(
            address,
            Self::script_address_prefix_decode(kwargs),
            kwargs.alphabet,
        )
    }
}

#[cfg(test)]
mod tests {
    use bitcoin::hashes::Hash;
    use bitcoin::Address as BtcAddress;
    use bitcoin::Network;
    use bitcoin::PubkeyHash;
    use bitcoin::PublicKey as BtcPublicKey;
    use bitcoin::ScriptBuf;

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
    fn roundtrip_default_mainnet_prefix() {
        let pk = test_public_key();
        let p2sh = P2SHAddress;
        let addr = encode_with_ipublic_key(&p2sh, &pk, &EncodeKwargs::default()).expect("encode");
        assert!(addr.starts_with('3'));
        let decoded = p2sh.decode(&addr, &DecodeKwargs::default()).expect("decode");
        assert_eq!(decoded.len(), 40);
        let pubkey_hash = P2PKHAddress::public_key_hash(&pk, true);
        let script_hash = P2SHAddress::script_hash_from_public_key_hash(&pubkey_hash);
        assert_eq!(decoded, hex::encode(script_hash));
    }

    #[test]
    fn matches_bitcoin_crate_p2sh() {
        let pk = test_public_key();
        let btc_pk = BtcPublicKey::from(*pk.inner());
        let pk_hash = PubkeyHash::hash(&btc_pk.to_bytes());
        let script = ScriptBuf::new_p2pkh(&pk_hash);
        let expected = BtcAddress::p2sh(&script, Network::Bitcoin)
            .expect("p2sh")
            .to_string();
        let got = encode_with_ipublic_key(&P2SHAddress, &pk, &EncodeKwargs::default()).expect("encode");
        assert_eq!(got, expected);
    }
}
