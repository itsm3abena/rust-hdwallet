use crate::hdwallet::cryptocurrencies::ergo::{
    Mainnet as ErgoMainnet, Testnet as ErgoTestnet, ADDRESS_TYPES, ERGO_PARAMS,
};
use crate::hdwallet::eccs::{
    validate_and_get_public_key, IPublicKey, PublicKeyInput, Slip10Secp256k1PublicKey,
};
use crate::hdwallet::exceptions::PublicKeyError;
use crate::hdwallet::libs::base58;
use crate::hdwallet::utils::{blake2b_256, integer_to_bytes_python};

use super::iaddress::{
    AddressSchemeError, DecodeKwargs, EncodeKwargs, EncodePublicKey, IAddress,
};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct ErgoAddress;

impl ErgoAddress {
    pub const REGISTRY_NAME: &'static str = "Ergo";

    const DEFAULT_ADDRESS_TYPE: &'static str = "p2pkh";

    fn checksum_length() -> usize {
        ERGO_PARAMS.checksum_length.unwrap_or(4) as usize
    }

    fn valid_address_types() -> Vec<&'static str> {
        ADDRESS_TYPES.iter().map(|(_, label)| *label).collect()
    }

    fn address_type_code(label: Option<&str>) -> Result<u32, AddressSchemeError> {
        let label = label.unwrap_or(Self::DEFAULT_ADDRESS_TYPE);
        let norm = label.to_ascii_lowercase();
        if let Some(codes) = ERGO_PARAMS.address_type_codes {
            for (name, code) in codes {
                if name.eq_ignore_ascii_case(label) || name.to_ascii_lowercase() == norm {
                    return Ok(u32::from(*code));
                }
            }
        }
        match norm.as_str() {
            "p2pkh" => Ok(0x01),
            "p2sh" => Ok(0x02),
            _ => {
                let expected = Self::valid_address_types().join(", ");
                Err(AddressSchemeError::Msg(format!(
                    "Invalid Ergo address type (expected: {expected}, got: {label})"
                )))
            }
        }
    }

    fn network_type_code(network: Option<&str>) -> Result<u32, AddressSchemeError> {
        let network = network.ok_or_else(|| {
            AddressSchemeError::Msg("Ergo network type is required".into())
        })?;
        match network {
            "mainnet" => Ok(ErgoMainnet::TYPE),
            "testnet" => Ok(ErgoTestnet::TYPE),
            other => Err(AddressSchemeError::Msg(format!(
                "Invalid Ergo network type (expected: mainnet, testnet, got: {other})"
            ))),
        }
    }

    fn prefix_bytes(address_type: Option<&str>, network: Option<&str>) -> Result<Vec<u8>, AddressSchemeError> {
        let address_type = Self::address_type_code(address_type)?;
        let network_type = Self::network_type_code(network)?;
        Ok(integer_to_bytes_python(address_type + network_type, None))
    }

    fn compute_checksum(payload: &[u8]) -> Vec<u8> {
        blake2b_256(payload)[..Self::checksum_length()].to_vec()
    }

    pub fn encode_public_key(
        public_key: PublicKeyInput<'_, Slip10Secp256k1PublicKey>,
        kwargs: &EncodeKwargs<'_>,
    ) -> Result<String, AddressSchemeError> {
        let pk = validate_and_get_public_key(public_key)
            .map_err(|e: PublicKeyError| AddressSchemeError::Msg(e.to_string()))?;
        let mut address_payload = Self::prefix_bytes(kwargs.address_type, kwargs.network)?;
        address_payload.extend_from_slice(&pk.raw_compressed());
        let checksum = Self::compute_checksum(&address_payload);
        address_payload.extend_from_slice(&checksum);
        Ok(base58::encode(&address_payload, None))
    }

    pub fn decode_address(address: &str, kwargs: &DecodeKwargs) -> Result<String, AddressSchemeError> {
        let prefix = Self::prefix_bytes(kwargs.address_type, kwargs.network)?;
        let address_decode = base58::decode(address.as_bytes(), None)
            .map_err(|e| AddressSchemeError::Msg(e.to_string()))?;
        let checksum_len = Self::checksum_length();
        let expected_length =
            Slip10Secp256k1PublicKey::compressed_len() + checksum_len + prefix.len();
        if address_decode.len() != expected_length {
            return Err(AddressSchemeError::Msg(format!(
                "Invalid length (expected: {expected_length}, got: {})",
                address_decode.len()
            )));
        }
        let checksum = &address_decode[address_decode.len() - checksum_len..];
        let address_with_prefix = &address_decode[..address_decode.len() - checksum_len];
        let checksum_got = Self::compute_checksum(address_with_prefix);
        if checksum != checksum_got.as_slice() {
            return Err(AddressSchemeError::Msg(format!(
                "Invalid checksum (expected: {}, got: {})",
                hex::encode(checksum),
                hex::encode(&checksum_got)
            )));
        }
        let prefix_got = &address_with_prefix[..prefix.len()];
        if prefix != prefix_got {
            return Err(AddressSchemeError::Msg(format!(
                "Invalid prefix (expected: {}, got: {})",
                hex::encode(&prefix),
                hex::encode(prefix_got)
            )));
        }
        let public_key = &address_with_prefix[prefix.len()..];
        if !Slip10Secp256k1PublicKey::is_valid_bytes(public_key) {
            return Err(AddressSchemeError::Msg(format!(
                "Invalid {} public key {}",
                Slip10Secp256k1PublicKey::ecc_name(),
                hex::encode(public_key)
            )));
        }
        Ok(hex::encode(public_key))
    }
}

impl IAddress for ErgoAddress {
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
        Self::encode_public_key(input, kwargs)
    }

    fn decode(&self, address: &str, kwargs: &DecodeKwargs) -> super::iaddress::AddressEncodeResult {
        Self::decode_address(address, kwargs)
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
        let path = bip44_path(coin_types::ERGO, 0, 0, 0).expect("path");
        let compressed = hd.derive_xpub(&path).expect("xpub").public_key.serialize();
        Slip10Secp256k1PublicKey::from_bytes(&compressed).expect("pk")
    }

    fn mainnet_p2pkh_kwargs() -> EncodeKwargs<'static> {
        EncodeKwargs {
            address_type: Some("p2pkh"),
            network: Some("mainnet"),
            ..EncodeKwargs::default()
        }
    }

    fn mainnet_p2pkh_decode_kwargs() -> DecodeKwargs {
        DecodeKwargs {
            address_type: Some("p2pkh"),
            network: Some("mainnet"),
            ..DecodeKwargs::default()
        }
    }

    #[test]
    fn roundtrip_mainnet_p2pkh() {
        let pk = test_public_key();
        let ergo = ErgoAddress;
        let kwargs = mainnet_p2pkh_kwargs();
        let addr = encode_with_ipublic_key(&ergo, &pk, &kwargs).expect("encode");
        let decoded = ergo
            .decode(&addr, &mainnet_p2pkh_decode_kwargs())
            .expect("decode");
        assert_eq!(decoded, hex::encode(pk.raw_compressed()));
    }

    #[test]
    fn roundtrip_testnet_p2sh() {
        let pk = test_public_key();
        let encode_kwargs = EncodeKwargs {
            address_type: Some("p2sh"),
            network: Some("testnet"),
            ..EncodeKwargs::default()
        };
        let decode_kwargs = DecodeKwargs {
            address_type: Some("p2sh"),
            network: Some("testnet"),
            ..DecodeKwargs::default()
        };
        let addr = encode_with_ipublic_key(&ErgoAddress, &pk, &encode_kwargs).expect("encode");
        let decoded = ErgoAddress
            .decode(&addr, &decode_kwargs)
            .expect("decode");
        assert_eq!(decoded, hex::encode(pk.raw_compressed()));
    }

    #[test]
    fn missing_network_errors() {
        let pk = test_public_key();
        let kwargs = EncodeKwargs {
            address_type: Some("p2pkh"),
            ..EncodeKwargs::default()
        };
        assert!(encode_with_ipublic_key(&ErgoAddress, &pk, &kwargs).is_err());
    }

    #[test]
    fn invalid_address_type_errors() {
        let pk = test_public_key();
        let kwargs = EncodeKwargs {
            address_type: Some("p2wpkh"),
            network: Some("mainnet"),
            ..EncodeKwargs::default()
        };
        assert!(encode_with_ipublic_key(&ErgoAddress, &pk, &kwargs).is_err());
    }
}
