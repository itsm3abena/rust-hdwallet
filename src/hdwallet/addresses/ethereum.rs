use bitcoin::secp256k1::PublicKey;
use sha3::{Digest, Keccak256};

use crate::hdwallet::cryptocurrencies::ethereum::ETHEREUM_PARAMS;
use crate::hdwallet::eccs::{
    validate_and_get_public_key, IPublicKey, PublicKeyInput, Slip10Secp256k1PublicKey,
};
use crate::hdwallet::exceptions::{Error, PublicKeyError};

use super::iaddress::{
    AddressSchemeError, DecodeKwargs, EncodeKwargs, EncodePublicKey, IAddress,
};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct EthereumAddress;

impl EthereumAddress {
    pub const REGISTRY_NAME: &'static str = "Ethereum";

    fn address_prefix() -> &'static str {
        ETHEREUM_PARAMS.address_prefix.unwrap_or("0x")
    }

    fn keccak256(data: &[u8]) -> [u8; 32] {
        let mut hasher = Keccak256::new();
        hasher.update(data);
        hasher.finalize().into()
    }

    fn pubkey_hash_hex(pk: &Slip10Secp256k1PublicKey) -> String {
        let uncompressed = pk.raw_uncompressed();
        let hash = Self::keccak256(&uncompressed[1..]);
        hex::encode(&hash[12..])
    }

    pub fn checksum_encode(address: &str) -> String {
        let address_lower = address.to_ascii_lowercase();
        let address_hash = hex::encode(Self::keccak256(address_lower.as_bytes()));
        address
            .chars()
            .zip(address_hash.chars())
            .map(|(c, h)| {
                if h.to_digit(16).unwrap_or(0) >= 8 {
                    c.to_ascii_uppercase()
                } else {
                    c.to_ascii_lowercase()
                }
            })
            .collect()
    }

    pub fn encode_public_key(
        public_key: PublicKeyInput<'_, Slip10Secp256k1PublicKey>,
        skip_checksum_encode: bool,
    ) -> std::result::Result<String, AddressSchemeError> {
        let pk = validate_and_get_public_key(public_key)
            .map_err(|e: PublicKeyError| AddressSchemeError::Msg(e.to_string()))?;
        let address = Self::pubkey_hash_hex(&pk);
        let body = if skip_checksum_encode {
            address
        } else {
            Self::checksum_encode(&address)
        };
        Ok(format!("{}{body}", Self::address_prefix()))
    }

    pub fn decode_address(
        address: &str,
        skip_checksum_encode: bool,
    ) -> std::result::Result<String, AddressSchemeError> {
        let prefix = Self::address_prefix();
        let prefix_got = address.get(..prefix.len()).unwrap_or("");
        if prefix_got != prefix {
            return Err(AddressSchemeError::Msg(format!(
                "Invalid address_prefix (expected: {prefix}, got: {prefix_got})"
            )));
        }
        let address_no_prefix = address
            .get(prefix.len()..)
            .ok_or_else(|| AddressSchemeError::Msg("Ethereum address too short".into()))?;
        if address_no_prefix.len() != 40 {
            return Err(AddressSchemeError::Msg(format!(
                "Invalid length (expected: 40, got: {})",
                address_no_prefix.len()
            )));
        }
        if !skip_checksum_encode
            && address_no_prefix != Self::checksum_encode(address_no_prefix)
        {
            return Err(AddressSchemeError::Msg("Invalid checksum encode".into()));
        }
        Ok(address_no_prefix.to_ascii_lowercase())
    }
}

impl IAddress for EthereumAddress {
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
        Self::encode_public_key(input, kwargs.skip_checksum_encode.unwrap_or(false))
    }

    fn decode(&self, address: &str, kwargs: &DecodeKwargs) -> super::iaddress::AddressEncodeResult {
        Self::decode_address(address, kwargs.skip_checksum_encode.unwrap_or(false))
    }
}

/// Lowercase `0x` address without EIP-55 checksum (legacy helper).
pub fn ethereum_address_from_pubkey(pk: &PublicKey) -> std::result::Result<String, Error> {
    let slip = Slip10Secp256k1PublicKey(*pk);
    let u = slip.raw_uncompressed();
    if u.len() != 65 || u[0] != 0x04 {
        return Err(Error::new(
            "expected uncompressed public key",
            None,
            None,
            None,
        ));
    }
    let hash = EthereumAddress::keccak256(&u[1..]);
    Ok(format!("0x{}", hex::encode(&hash[12..])))
}

#[cfg(test)]
mod tests {
    use bitcoin::Network;

    use super::*;
    use crate::hdwallet::hds::bip44::bip44_path;
    use crate::hdwallet::hds::Bip32Hd;
    use crate::hdwallet::slip44::coin_types;

    fn test_public_key() -> Slip10Secp256k1PublicKey {
        let seed = hex::decode(
            "5eb00bbddcf069084889a8ab9155568165f4c767744d69fbf5d1791c2fe1563254c36a54693640219b24e06f1011116141de699825262eeacc22f8ac8ae26a",
        )
        .expect("seed hex");
        let hd = Bip32Hd::from_seed(Network::Bitcoin, &seed).expect("bip32");
        let path = bip44_path(coin_types::ETHEREUM, 0, 0, 0).expect("path");
        let compressed = hd.derive_xpub(&path).expect("xpub").public_key.serialize();
        Slip10Secp256k1PublicKey::from_bytes(&compressed).expect("pk")
    }

    #[test]
    fn roundtrip_with_checksum() {
        let pk = test_public_key();
        let hash = EthereumAddress::pubkey_hash_hex(&pk);
        let addr = EthereumAddress::encode_public_key(
            PublicKeyInput::Instance(pk),
            false,
        )
        .expect("encode");
        assert!(addr.starts_with("0x"));
        assert_eq!(addr.len(), 42);
        let decoded = EthereumAddress::decode_address(&addr, false).expect("decode");
        assert_eq!(decoded, hash);
    }

    #[test]
    fn skip_checksum_roundtrip() {
        let pk = test_public_key();
        let kwargs = EncodeKwargs {
            skip_checksum_encode: Some(true),
            ..EncodeKwargs::default()
        };
        let decode_kwargs = DecodeKwargs {
            skip_checksum_encode: Some(true),
            ..DecodeKwargs::default()
        };
        let addr = EthereumAddress
            .encode(
                EncodePublicKey::Bytes(&pk.raw_compressed()),
                &kwargs,
            )
            .expect("encode");
        assert_eq!(addr, format!("0x{}", EthereumAddress::pubkey_hash_hex(&pk)));
        let decoded = EthereumAddress
            .decode(&addr, &decode_kwargs)
            .expect("decode");
        assert_eq!(decoded, EthereumAddress::pubkey_hash_hex(&pk));
    }

    #[test]
    fn invalid_checksum_errors() {
        let pk = test_public_key();
        let addr = EthereumAddress::encode_public_key(PublicKeyInput::Instance(pk), false)
            .expect("encode");
        let body = &addr[2..];
        let mut corrupted = body.to_string();
        if let Some(i) = corrupted.find(|c| matches!(c, 'a'..='f' | 'A'..='F')) {
            let c = corrupted.as_bytes()[i] as char;
            corrupted.replace_range(
                i..=i,
                &if c.is_ascii_uppercase() {
                    c.to_ascii_lowercase().to_string()
                } else {
                    c.to_ascii_uppercase().to_string()
                },
            );
        } else {
            corrupted = corrupted.replacen('0', "1", 1);
        }
        let bad = format!("0x{corrupted}");
        assert!(EthereumAddress::decode_address(&bad, false).is_err());
    }

    #[test]
    fn checksum_encode_known_vector() {
        let addr = "5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed";
        assert_eq!(
            EthereumAddress::checksum_encode("5aaeb6053f3e94c9b9a09f33669435e7ef1beaed"),
            addr
        );
    }
}
