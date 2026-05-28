use crate::hdwallet::cryptocurrencies::icon::ICON_PARAMS;
use crate::hdwallet::eccs::{
    validate_and_get_public_key, IPublicKey, PublicKeyInput, Slip10Secp256k1PublicKey,
};
use crate::hdwallet::exceptions::PublicKeyError;
use crate::hdwallet::utils::sha3_256;

use super::iaddress::{
    AddressSchemeError, DecodeKwargs, EncodeKwargs, EncodePublicKey, IAddress,
};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct IconAddress;

impl IconAddress {
    pub const REGISTRY_NAME: &'static str = "Icon";

    pub const KEY_HASH_LENGTH: usize = 20;

    fn address_prefix() -> &'static str {
        ICON_PARAMS.address_prefix.unwrap_or("hx")
    }

    fn key_hash_length() -> usize {
        Self::KEY_HASH_LENGTH
    }

    pub fn public_key_hash(pk: &Slip10Secp256k1PublicKey) -> [u8; 20] {
        let uncompressed = pk.raw_uncompressed();
        let hash = sha3_256(&uncompressed[1..]);
        let len = Self::key_hash_length();
        hash[hash.len() - len..]
            .try_into()
            .expect("key hash length")
    }

    pub fn encode_public_key(
        public_key: PublicKeyInput<'_, Slip10Secp256k1PublicKey>,
    ) -> Result<String, AddressSchemeError> {
        let pk = validate_and_get_public_key(public_key)
            .map_err(|e: PublicKeyError| AddressSchemeError::Msg(e.to_string()))?;
        let hash = Self::public_key_hash(&pk);
        Ok(format!("{}{}", Self::address_prefix(), hex::encode(hash)))
    }

    pub fn decode_address(address: &str) -> Result<String, AddressSchemeError> {
        let prefix = Self::address_prefix();
        let prefix_got = address.get(..prefix.len()).unwrap_or("");
        if prefix_got != prefix {
            return Err(AddressSchemeError::Msg(format!(
                "Invalid prefix (expected: {prefix}, got: {prefix_got})"
            )));
        }
        let address_no_prefix = address
            .get(prefix.len()..)
            .ok_or_else(|| AddressSchemeError::Msg("Icon address too short".into()))?;
        let public_key_hash = hex::decode(address_no_prefix)
            .map_err(|e| AddressSchemeError::Msg(e.to_string()))?;
        let expected_len = Self::key_hash_length();
        if public_key_hash.len() != expected_len {
            return Err(AddressSchemeError::Msg(format!(
                "Invalid length (expected: {expected_len}, got: {})",
                public_key_hash.len()
            )));
        }
        Ok(hex::encode(public_key_hash))
    }
}

impl IAddress for IconAddress {
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
        Self::encode_public_key(input)
    }

    fn decode(&self, address: &str, _kwargs: &DecodeKwargs) -> super::iaddress::AddressEncodeResult {
        Self::decode_address(address)
    }
}

#[cfg(test)]
mod tests {
    use bitcoin::Network;

    use super::*;
    use super::super::iaddress::encode_with_ipublic_key;
    use crate::hdwallet::hds::bip44::bip44_path;
    use crate::hdwallet::hds::Bip32Hd;
    use crate::hdwallet::slip44::coin_types;

    fn test_public_key() -> Slip10Secp256k1PublicKey {
        let seed = hex::decode(
            "5eb00bbddcf069084889a8ab9155568165f4c767744d69fbf5d1791c2fe1563254c36a54693640219b24e06f1011116141de699825262eeacc22f8ac8ae26a",
        )
        .expect("seed hex");
        let hd = Bip32Hd::from_seed(Network::Bitcoin, &seed).expect("bip32");
        let path = bip44_path(coin_types::ICON, 0, 0, 0).expect("path");
        let compressed = hd.derive_xpub(&path).expect("xpub").public_key.serialize();
        Slip10Secp256k1PublicKey::from_bytes(&compressed).expect("pk")
    }

    #[test]
    fn roundtrip() {
        let pk = test_public_key();
        let icon = IconAddress;
        let addr = encode_with_ipublic_key(&icon, &pk, &EncodeKwargs::default()).expect("encode");
        assert!(addr.starts_with("hx"));
        assert_eq!(addr.len(), 2 + 40);
        let decoded = icon.decode(&addr, &DecodeKwargs::default()).expect("decode");
        assert_eq!(decoded, hex::encode(IconAddress::public_key_hash(&pk)));
    }

    #[test]
    fn wrong_prefix_errors() {
        let pk = test_public_key();
        let addr = encode_with_ipublic_key(&IconAddress, &pk, &EncodeKwargs::default()).expect("encode");
        let bad = format!("xx{}", &addr[2..]);
        assert!(IconAddress.decode(&bad, &DecodeKwargs::default()).is_err());
    }
}
