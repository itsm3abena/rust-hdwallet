use sha3::{Digest, Keccak256};

use crate::hdwallet::cryptocurrencies::monero::MONERO_PARAMS;
use crate::hdwallet::cryptocurrencies::NetworkDescriptor;
use crate::hdwallet::eccs::{
    validate_and_get_public_key, IPublicKey, PublicKeyInput, Slip10Ed25519MoneroPublicKey,
};
use crate::hdwallet::exceptions::PublicKeyError;
use crate::hdwallet::libs::base58::{self, Base58Error};
use crate::hdwallet::utils::integer_to_bytes_python;

use super::iaddress::{AddressSchemeError, IAddress};

pub const ADDRESS_TYPE_STANDARD: &str = "standard";
pub const ADDRESS_TYPE_INTEGRATED: &str = "integrated";
pub const ADDRESS_TYPE_SUB_ADDRESS: &str = "sub-address";

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct MoneroAddress;

impl MoneroAddress {
    pub const REGISTRY_NAME: &'static str = "Monero";

    fn checksum_length() -> usize {
        MONERO_PARAMS.checksum_length.unwrap_or(4) as usize
    }

    fn payment_id_length() -> usize {
        MONERO_PARAMS.payment_id_length.unwrap_or(8) as usize
    }

    fn network_descriptor(network: &str) -> Result<&'static NetworkDescriptor, AddressSchemeError> {
        match network {
            "mainnet" => Ok(&crate::hdwallet::cryptocurrencies::monero::MAINNET),
            "stagenet" => Ok(&crate::hdwallet::cryptocurrencies::monero::STAGENET),
            "testnet" => Ok(&crate::hdwallet::cryptocurrencies::monero::TESTNET),
            other => Err(AddressSchemeError::Msg(format!("Invalid network: {other}"))),
        }
    }

    fn address_type_code(
        network: &str,
        address_type: &str,
    ) -> Result<u32, AddressSchemeError> {
        let net = Self::network_descriptor(network)?;
        let code = match address_type {
            ADDRESS_TYPE_STANDARD => net.standard,
            ADDRESS_TYPE_INTEGRATED => net.integrated,
            ADDRESS_TYPE_SUB_ADDRESS => net.sub_address,
            other => {
                return Err(AddressSchemeError::Msg(format!(
                    "Invalid address type (expected: standard, integrated, sub-address | got: {other})"
                )));
            }
        };
        code.ok_or_else(|| {
            AddressSchemeError::Msg(format!("Missing address type code for {address_type}"))
        })
    }

    fn version_bytes(network: &str, address_type: &str) -> Result<Vec<u8>, AddressSchemeError> {
        let code = Self::address_type_code(network, address_type)?;
        Ok(integer_to_bytes_python(code, None))
    }

    fn keccak256(data: &[u8]) -> [u8; 32] {
        let mut hasher = Keccak256::new();
        hasher.update(data);
        hasher.finalize().into()
    }

    fn compute_checksum(payload: &[u8]) -> Vec<u8> {
        Self::keccak256(payload)[..Self::checksum_length()].to_vec()
    }

    fn parse_public_key(
        input: PublicKeyInput<'_, Slip10Ed25519MoneroPublicKey>,
    ) -> Result<Slip10Ed25519MoneroPublicKey, AddressSchemeError> {
        validate_and_get_public_key(input)
            .map_err(|e: PublicKeyError| AddressSchemeError::Msg(e.to_string()))
    }

    pub fn encode(
        spend_public_key: PublicKeyInput<'_, Slip10Ed25519MoneroPublicKey>,
        view_public_key: PublicKeyInput<'_, Slip10Ed25519MoneroPublicKey>,
        network: &str,
        address_type: &str,
        payment_id: Option<&[u8]>,
    ) -> Result<String, AddressSchemeError> {
        let spend = Self::parse_public_key(spend_public_key)?;
        let view = Self::parse_public_key(view_public_key)?;

        if let Some(pid) = payment_id {
            if pid.len() != Self::payment_id_length() {
                return Err(AddressSchemeError::Msg("Invalid payment ID length".into()));
            }
        }

        let version = Self::version_bytes(network, address_type)?;
        let mut payload = version;
        payload.extend_from_slice(&spend.raw_compressed());
        payload.extend_from_slice(&view.raw_compressed());
        if let Some(pid) = payment_id {
            payload.extend_from_slice(pid);
        }

        let mut with_checksum = payload;
        with_checksum.extend_from_slice(&Self::compute_checksum(&with_checksum));
        Ok(base58::encode_monero(&with_checksum, None))
    }

    pub fn decode(
        address: &str,
        network: &str,
        address_type: &str,
        payment_id: Option<&[u8]>,
    ) -> Result<(String, String), AddressSchemeError> {
        let address_decode = base58::decode_monero(address, None)
            .map_err(|e: Base58Error| AddressSchemeError::Msg(e.to_string()))?;

        let checksum_len = Self::checksum_length();
        if address_decode.len() < checksum_len {
            return Err(AddressSchemeError::Msg("Invalid Monero address length".into()));
        }
        let checksum = &address_decode[address_decode.len() - checksum_len..];
        let payload_with_prefix = &address_decode[..address_decode.len() - checksum_len];

        let checksum_got = Self::compute_checksum(payload_with_prefix);
        if checksum != checksum_got.as_slice() {
            return Err(AddressSchemeError::Msg(format!(
                "Invalid checksum (expected: {}, got: {})",
                hex::encode(checksum),
                hex::encode(&checksum_got)
            )));
        }

        let version = Self::version_bytes(network, address_type)?;
        if payload_with_prefix.len() < version.len() {
            return Err(AddressSchemeError::Msg("Invalid Monero address payload".into()));
        }
        if payload_with_prefix[..version.len()] != version[..] {
            return Err(AddressSchemeError::Msg(format!(
                "Invalid version (expected: {}, got: {})",
                hex::encode(&version),
                hex::encode(&payload_with_prefix[..version.len()])
            )));
        }

        let payload_without_prefix = &payload_with_prefix[version.len()..];
        let key_len = Slip10Ed25519MoneroPublicKey::compressed_len();
        let expected_length = key_len * 2;

        let (spend_public_key, view_public_key) = if payload_without_prefix.len() == expected_length {
            (
                &payload_without_prefix[..key_len],
                &payload_without_prefix[key_len..key_len * 2],
            )
        } else if payload_without_prefix.len() == expected_length + Self::payment_id_length() {
            if payment_id.is_none() || payment_id.unwrap().len() != Self::payment_id_length() {
                return Err(AddressSchemeError::Msg("Invalid payment ID".into()));
            }
            let pid_got = &payload_without_prefix[expected_length..];
            if payment_id.unwrap() != pid_got {
                return Err(AddressSchemeError::Msg(format!(
                    "Invalid payment ID (expected: {}, got: {})",
                    hex::encode(payment_id.unwrap()),
                    hex::encode(pid_got)
                )));
            }
            (
                &payload_without_prefix[..key_len],
                &payload_without_prefix[key_len..key_len * 2],
            )
        } else {
            return Err(AddressSchemeError::Msg(format!(
                "Invalid length (expected: {expected_length} or {} | got: {})",
                expected_length + Self::payment_id_length(),
                payload_without_prefix.len()
            )));
        };

        if !Slip10Ed25519MoneroPublicKey::is_valid_bytes(spend_public_key) {
            return Err(AddressSchemeError::Msg(format!(
                "Invalid {} public key {}",
                Slip10Ed25519MoneroPublicKey::ecc_name(),
                hex::encode(spend_public_key)
            )));
        }
        if !Slip10Ed25519MoneroPublicKey::is_valid_bytes(view_public_key) {
            return Err(AddressSchemeError::Msg(format!(
                "Invalid {} public key {}",
                Slip10Ed25519MoneroPublicKey::ecc_name(),
                hex::encode(view_public_key)
            )));
        }

        Ok((hex::encode(spend_public_key), hex::encode(view_public_key)))
    }
}

impl IAddress for MoneroAddress {
    fn name(&self) -> &'static str {
        Self::REGISTRY_NAME
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hdwallet::eccs::{IPrivateKey, Slip10Ed25519MoneroPrivateKey};

    fn sample_keypair(
        spend_seed: u8,
        view_seed: u8,
    ) -> (Slip10Ed25519MoneroPublicKey, Slip10Ed25519MoneroPublicKey) {
        let mut spend_secret = [0u8; 32];
        spend_secret[0] = spend_seed;
        let mut view_secret = [0u8; 32];
        view_secret[0] = view_seed;
        let spend_sk = Slip10Ed25519MoneroPrivateKey::from_bytes(&spend_secret).expect("spend sk");
        let view_sk = Slip10Ed25519MoneroPrivateKey::from_bytes(&view_secret).expect("view sk");
        (spend_sk.public_key(), view_sk.public_key())
    }

    #[test]
    fn standard_roundtrip_mainnet() {
        let (spend, view) = sample_keypair(9, 10);
        let addr = MoneroAddress::encode(
            PublicKeyInput::Instance(spend),
            PublicKeyInput::Instance(view),
            "mainnet",
            ADDRESS_TYPE_STANDARD,
            None,
        )
        .expect("encode");
        assert!(addr.starts_with('4'));
        let (spend_hex, view_hex) = MoneroAddress::decode(
            &addr,
            "mainnet",
            ADDRESS_TYPE_STANDARD,
            None,
        )
        .expect("decode");
        assert_eq!(spend_hex.len(), 66);
        assert_eq!(view_hex.len(), 66);
    }

    #[test]
    fn integrated_roundtrip_with_payment_id() {
        let (spend, view) = sample_keypair(3, 4);
        let payment_id = [0x12u8; 8];
        let addr = MoneroAddress::encode(
            PublicKeyInput::Instance(spend),
            PublicKeyInput::Instance(view),
            "mainnet",
            ADDRESS_TYPE_INTEGRATED,
            Some(&payment_id),
        )
        .expect("encode");
        let (spend_hex, view_hex) = MoneroAddress::decode(
            &addr,
            "mainnet",
            ADDRESS_TYPE_INTEGRATED,
            Some(&payment_id),
        )
        .expect("decode");
        assert_eq!(spend_hex.len(), 66);
        assert_eq!(view_hex.len(), 66);
    }

    #[test]
    fn version_constants_match_mainnet() {
        use crate::hdwallet::cryptocurrencies::monero::{
            Mainnet as MoneroMainnet, Stagenet as MoneroStagenet, Testnet as MoneroTestnet,
        };
        assert_eq!(MoneroMainnet::STANDARD, 0x12);
        assert_eq!(MoneroMainnet::INTEGRATED, 0x13);
        assert_eq!(MoneroMainnet::SUB_ADDRESS, 0x2a);
        assert_eq!(MoneroTestnet::STANDARD, 0x35);
        assert_eq!(MoneroStagenet::STANDARD, 0x18);
    }
}
