use std::collections::BTreeMap;

use serde_cbor::Value;

use crate::hdwallet::cryptocurrencies::cardano::{
    CARDANO_PARAMS, Mainnet as CardanoMainnet, Testnet as CardanoTestnet,
};
use crate::hdwallet::eccs::{
    validate_and_get_public_key, IPublicKey, KholawEd25519PublicKey, PublicKeyInput,
};
use crate::hdwallet::exceptions::PublicKeyError;
use crate::hdwallet::libs::base58;
use crate::hdwallet::libs::segwit_bech32::{base_bech32_decode, bech32_encode, convertbits};
use crate::hdwallet::utils::{
    blake2b_224, crc32_to_u32, integer_to_bytes_python, sha3_256,
};

use super::iaddress::{
    AddressSchemeError, DecodeKwargs, EncodeKwargs, EncodePublicKey, IAddress,
};

const PAYLOAD_TAG: u64 = 24;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct CardanoAddress;

impl CardanoAddress {
    pub const REGISTRY_NAME: &'static str = "Cardano";

    pub const ENCODE_BYRON_ICARUS: &'static str = "byron-icarus";
    pub const ENCODE_BYRON_LEGACY: &'static str = "byron-legacy";
    pub const ENCODE_PAYMENT: &'static str = "payment";
    pub const ENCODE_STAKING: &'static str = "staking";
    pub const ENCODE_REWARD: &'static str = "reward";

    pub const ADDRESS_PUBLIC_KEY: &'static str = "public-key";
    pub const ADDRESS_REDEMPTION: &'static str = "redemption";

    fn address_type_code(label: &str) -> Result<u64, AddressSchemeError> {
        match label {
            Self::ADDRESS_PUBLIC_KEY => Ok(CARDANO_PARAMS.cardano_public_key_address.unwrap_or(0) as u64),
            Self::ADDRESS_REDEMPTION => Ok(CARDANO_PARAMS.cardano_redemption_address.unwrap_or(2) as u64),
            _ => Err(AddressSchemeError::Msg(format!(
                "Invalid address type (expected: public-key, redemption | got: {label})"
            ))),
        }
    }

    fn network_type(network: &str) -> Result<u32, AddressSchemeError> {
        match network {
            "mainnet" => Ok(CardanoMainnet::TYPE),
            "testnet" => Ok(CardanoTestnet::TYPE),
            other => Err(AddressSchemeError::Msg(format!("Invalid network: {other}"))),
        }
    }

    fn payment_hrp(network: &str) -> Result<&'static str, AddressSchemeError> {
        match network {
            "mainnet" => Ok(CardanoMainnet::PAYMENT_ADDRESS_HRP),
            "testnet" => Ok(CardanoTestnet::PAYMENT_ADDRESS_HRP),
            other => Err(AddressSchemeError::Msg(format!("Invalid network: {other}"))),
        }
    }

    fn reward_hrp(network: &str) -> Result<&'static str, AddressSchemeError> {
        match network {
            "mainnet" => Ok(CardanoMainnet::REWARD_ADDRESS_HRP),
            "testnet" => Ok(CardanoTestnet::REWARD_ADDRESS_HRP),
            other => Err(AddressSchemeError::Msg(format!("Invalid network: {other}"))),
        }
    }

    fn shelley_prefix(payment: bool, network: &str) -> Result<Vec<u8>, AddressSchemeError> {
        let base = if payment {
            CARDANO_PARAMS.cardano_payment_prefix.unwrap_or(0)
        } else {
            CARDANO_PARAMS.cardano_reward_prefix.unwrap_or(0x0e)
        };
        let net = Self::network_type(network)?;
        Ok(integer_to_bytes_python(u32::from(base) << 4 | net, None))
    }

    fn parse_kholaw_pk(input: PublicKeyInput<'_, KholawEd25519PublicKey>) -> Result<KholawEd25519PublicKey, AddressSchemeError> {
        validate_and_get_public_key(input)
            .map_err(|e: PublicKeyError| AddressSchemeError::Msg(e.to_string()))
    }

    fn pk_body(pk: &KholawEd25519PublicKey) -> &[u8; 32] {
        pk.encoded_point()
    }

    fn cbor_bytes(value: &Value) -> Result<Vec<u8>, AddressSchemeError> {
        serde_cbor::to_vec(value).map_err(|e| AddressSchemeError::Msg(e.to_string()))
    }

    fn cbor_u64(v: u64) -> Value {
        Value::Integer(v.into())
    }

    fn integer_to_cbor_u64(value: &Value) -> Result<u64, AddressSchemeError> {
        match value {
            Value::Integer(i) => {
                if *i < 0 {
                    return Err(AddressSchemeError::Msg("Invalid integer".into()));
                }
                (*i).try_into().map_err(|_| AddressSchemeError::Msg("Invalid integer".into()))
            }
            _ => Err(AddressSchemeError::Msg("Invalid integer".into())),
        }
    }

    fn encode_byron(
        public_key: &KholawEd25519PublicKey,
        chain_code: &[u8],
        address_attributes: BTreeMap<u64, Value>,
        address_type: &str,
    ) -> Result<String, AddressSchemeError> {
        if chain_code.len() != 32 {
            return Err(AddressSchemeError::Msg(format!(
                "Invalid chain code length (expected: 32 | got: {})",
                chain_code.len()
            )));
        }
        let type_code = Self::address_type_code(address_type)?;
        let mut payload = Vec::with_capacity(64);
        payload.extend_from_slice(Self::pk_body(public_key));
        payload.extend_from_slice(chain_code);

        let attrs_map: BTreeMap<Value, Value> = address_attributes
            .iter()
            .map(|(k, v)| (Self::cbor_u64(*k), v.clone()))
            .collect();

        let serialize = Self::cbor_bytes(&Value::Array(vec![
            Self::cbor_u64(type_code),
            Value::Array(vec![Self::cbor_u64(type_code), Value::Bytes(payload.clone())]),
            Value::Map(attrs_map.clone()),
        ]))?;

        let address_root_hash = blake2b_224(&sha3_256(&serialize));
        let address_payload = Self::cbor_bytes(&Value::Array(vec![
            Value::Bytes(address_root_hash.to_vec()),
            Value::Map(attrs_map),
            Self::cbor_u64(type_code),
        ]))?;

        let outer = Self::cbor_bytes(&Value::Array(vec![
            Value::Tag(PAYLOAD_TAG, Box::new(Value::Bytes(address_payload.clone()))),
            Self::cbor_u64(u64::from(crc32_to_u32(&address_payload))),
        ]))?;

        Ok(base58::encode(&outer, None))
    }

    pub fn encode_byron_icarus(
        public_key: PublicKeyInput<'_, KholawEd25519PublicKey>,
        chain_code: &[u8],
        address_type: &str,
    ) -> Result<String, AddressSchemeError> {
        let pk = Self::parse_kholaw_pk(public_key)?;
        Self::encode_byron(&pk, chain_code, BTreeMap::new(), address_type)
    }

    pub fn encode_byron_legacy(
        _public_key: PublicKeyInput<'_, KholawEd25519PublicKey>,
        _path: &str,
        _path_key: &[u8],
        _chain_code: &[u8],
        _address_type: &str,
    ) -> Result<String, AddressSchemeError> {
        Err(AddressSchemeError::Unimplemented {
            scheme: "Cardano Byron legacy (ChaCha20-Poly1305 path encryption not enabled)",
        })
    }

    pub fn encode_shelley_payment(
        public_key: PublicKeyInput<'_, KholawEd25519PublicKey>,
        staking_public_key: PublicKeyInput<'_, KholawEd25519PublicKey>,
        network: &str,
    ) -> Result<String, AddressSchemeError> {
        let pk = Self::parse_kholaw_pk(public_key)?;
        let stake = Self::parse_kholaw_pk(staking_public_key)?;
        let mut payload = Self::shelley_prefix(true, network)?;
        payload.extend_from_slice(&blake2b_224(Self::pk_body(&pk)));
        payload.extend_from_slice(&blake2b_224(Self::pk_body(&stake)));
        Self::bech32_encode_payload(Self::payment_hrp(network)?, &payload)
    }

    pub fn encode_shelley_staking(
        public_key: PublicKeyInput<'_, KholawEd25519PublicKey>,
        network: &str,
    ) -> Result<String, AddressSchemeError> {
        let pk = Self::parse_kholaw_pk(public_key)?;
        let mut payload = Self::shelley_prefix(false, network)?;
        payload.extend_from_slice(&blake2b_224(Self::pk_body(&pk)));
        Self::bech32_encode_payload(Self::reward_hrp(network)?, &payload)
    }

    fn bech32_encode_payload(hrp: &str, payload: &[u8]) -> Result<String, AddressSchemeError> {
        bech32_encode(hrp, payload).ok_or_else(|| {
            AddressSchemeError::Msg(format!("failed to bech32-encode Cardano address for hrp {hrp}"))
        })
    }

    fn bech32_decode_payload(hrp: &str, address: &str) -> Result<Vec<u8>, AddressSchemeError> {
        let (hrp_got, data) = base_bech32_decode(address).ok_or_else(|| {
            AddressSchemeError::Msg(format!("invalid bech32 address for hrp {hrp}"))
        })?;
        if hrp_got != hrp {
            return Err(AddressSchemeError::Msg(format!(
                "invalid bech32 hrp (expected: {hrp}, got: {hrp_got})"
            )));
        }
        convertbits(&data, 5, 8, false).ok_or_else(|| {
            AddressSchemeError::Msg("invalid bech32 payload".into())
        })
    }

    pub fn decode_byron(address: &str, address_type: &str) -> Result<String, AddressSchemeError> {
        let type_code = Self::address_type_code(address_type)?;
        let address_decode = base58::decode(address.as_bytes(), None)
            .map_err(|e| AddressSchemeError::Msg(e.to_string()))?;
        let outer: Value = serde_cbor::from_slice(&address_decode)
            .map_err(|_| AddressSchemeError::Msg("Invalid CBOR encoding".into()))?;
        let Value::Array(items) = outer else {
            return Err(AddressSchemeError::Msg("Invalid address encoding".into()));
        };
        if items.len() != 2 {
            return Err(AddressSchemeError::Msg("Invalid address encoding".into()));
        }
        let payload_bytes = match &items[0] {
            Value::Tag(tag, payload_val) if *tag == PAYLOAD_TAG => {
                let Value::Bytes(b) = payload_val.as_ref() else {
                    return Err(AddressSchemeError::Msg("Invalid address encoding".into()));
                };
                b.clone()
            }
            Value::Bytes(b) => b.clone(),
            _ => return Err(AddressSchemeError::Msg("Invalid address encoding".into())),
        };
        if !matches!(&items[1], Value::Integer(_)) {
            return Err(AddressSchemeError::Msg("Invalid address encoding".into()));
        }
        let crc_expected = crc32_to_u32(&payload_bytes);
        let crc_got = Self::integer_to_cbor_u64(&items[1])?;
        if crc_got != u64::from(crc_expected) {
            return Err(AddressSchemeError::Msg(format!(
                "Invalid CRC (expected: {crc_expected}, got: {crc_got})"
            )));
        }

        let address_payload: Value = serde_cbor::from_slice(&payload_bytes)
            .map_err(|_| AddressSchemeError::Msg("Invalid address payload".into()))?;
        let Value::Array(parts) = address_payload else {
            return Err(AddressSchemeError::Msg("Invalid address payload".into()));
        };
        if parts.len() != 3 {
            return Err(AddressSchemeError::Msg("Invalid address payload".into()));
        }
        let Value::Bytes(root_hash) = &parts[0] else {
            return Err(AddressSchemeError::Msg("Invalid address payload".into()));
        };
        if root_hash.len() != 28 {
            return Err(AddressSchemeError::Msg(format!(
                "Invalid length (expected: 28 | got: {})",
                root_hash.len()
            )));
        }
        let Value::Map(attrs) = &parts[1] else {
            return Err(AddressSchemeError::Msg("Invalid address attributes".into()));
        };
        if attrs.len() > 2
            || (!attrs.is_empty()
                && !attrs.iter().any(|(k, _)| matches!(k, Value::Integer(i) if *i == 1.into()))
                && !attrs.iter().any(|(k, _)| matches!(k, Value::Integer(i) if *i == 2.into())))
        {
            return Err(AddressSchemeError::Msg("Invalid address attributes".into()));
        }
        let got_type = Self::integer_to_cbor_u64(&parts[2])?;
        if got_type != type_code {
            return Err(AddressSchemeError::Msg(format!(
                "Invalid address type (expected: {type_code} | got: {got_type})"
            )));
        }

        let mut out = root_hash.clone();
        if let Some((_, Value::Bytes(enc))) = attrs.iter().find(|(k, _)| matches!(k, Value::Integer(i) if *i == 1.into()))
        {
            out.extend_from_slice(enc);
        }
        Ok(hex::encode(out))
    }

    pub fn decode_shelley_payment(address: &str, network: &str) -> Result<String, AddressSchemeError> {
        let hrp = Self::payment_hrp(network)?;
        let decoded = Self::bech32_decode_payload(hrp, address)?;
        let expected_len = (28 * 2) + 1;
        if decoded.len() != expected_len {
            return Err(AddressSchemeError::Msg(format!(
                "Invalid length (expected: {expected_len} | got: {})",
                decoded.len()
            )));
        }
        let prefix = Self::shelley_prefix(true, network)?;
        if decoded[..prefix.len()] != prefix[..] {
            return Err(AddressSchemeError::Msg(format!(
                "Invalid prefix (expected: {} | got: {})",
                hex::encode(&prefix),
                hex::encode(&decoded[..prefix.len()])
            )));
        }
        Ok(hex::encode(&decoded[prefix.len()..]))
    }

    pub fn decode_shelley_staking(address: &str, network: &str) -> Result<String, AddressSchemeError> {
        let hrp = Self::reward_hrp(network)?;
        let decoded = Self::bech32_decode_payload(hrp, address)?;
        let expected_len = 28 + 1;
        if decoded.len() != expected_len {
            return Err(AddressSchemeError::Msg(format!(
                "Invalid length (expected: {expected_len} | got: {})",
                decoded.len()
            )));
        }
        let prefix = Self::shelley_prefix(false, network)?;
        if decoded[..prefix.len()] != prefix[..] {
            return Err(AddressSchemeError::Msg(format!(
                "Invalid prefix (expected: {} | got: {})",
                hex::encode(&prefix),
                hex::encode(&decoded[..prefix.len()])
            )));
        }
        Ok(hex::encode(&decoded[prefix.len()..]))
    }

    fn encode_type(kwargs: &EncodeKwargs<'_>) -> &'static str {
        kwargs.encode_type.unwrap_or(Self::ENCODE_PAYMENT)
    }

    fn decode_type(kwargs: &DecodeKwargs) -> &'static str {
        kwargs.decode_type.unwrap_or(Self::ENCODE_PAYMENT)
    }

    fn network(kwargs: &EncodeKwargs<'_>) -> &'static str {
        kwargs.network.unwrap_or("mainnet")
    }

    fn network_decode(kwargs: &DecodeKwargs) -> &'static str {
        kwargs.network.unwrap_or("mainnet")
    }

    fn address_type_label(kwargs: &EncodeKwargs<'_>) -> &'static str {
        kwargs.address_type.unwrap_or(Self::ADDRESS_PUBLIC_KEY)
    }

    fn address_type_label_decode(kwargs: &DecodeKwargs) -> &'static str {
        kwargs.address_type.unwrap_or(Self::ADDRESS_PUBLIC_KEY)
    }
}


impl IAddress for CardanoAddress {
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
        match Self::encode_type(kwargs) {
            Self::ENCODE_BYRON_LEGACY => {
                let path = kwargs.path.ok_or_else(|| {
                    AddressSchemeError::Msg("Byron legacy encode requires path".into())
                })?;
                let path_key = kwargs.path_key.ok_or_else(|| {
                    AddressSchemeError::Msg("Byron legacy encode requires path_key".into())
                })?;
                let chain_code = kwargs.chain_code.ok_or_else(|| {
                    AddressSchemeError::Msg("Byron encode requires chain_code".into())
                })?;
                Self::encode_byron_legacy(
                    input,
                    path,
                    path_key,
                    chain_code,
                    Self::address_type_label(kwargs),
                )
            }
            Self::ENCODE_BYRON_ICARUS => {
                let chain_code = kwargs.chain_code.ok_or_else(|| {
                    AddressSchemeError::Msg("Byron encode requires chain_code".into())
                })?;
                Self::encode_byron_icarus(input, chain_code, Self::address_type_label(kwargs))
            }
            Self::ENCODE_PAYMENT => {
                let staking = kwargs.staking_public_key.ok_or_else(|| {
                    AddressSchemeError::Msg("Shelley payment encode requires staking_public_key".into())
                })?;
                let stake_input = PublicKeyInput::Bytes(staking);
                Self::encode_shelley_payment(input, stake_input, Self::network(kwargs))
            }
            Self::ENCODE_STAKING | Self::ENCODE_REWARD => {
                Self::encode_shelley_staking(input, Self::network(kwargs))
            }
            other => Err(AddressSchemeError::Msg(format!(
                "Invalid encode type = type (expected: byron-icarus, byron-legacy, payment, staking, reward | got: {other})"
            ))),
        }
    }

    fn decode(
        &self,
        address: &str,
        kwargs: &DecodeKwargs,
    ) -> super::iaddress::AddressEncodeResult {
        match Self::decode_type(kwargs) {
            Self::ENCODE_BYRON_LEGACY | Self::ENCODE_BYRON_ICARUS => {
                Self::decode_byron(address, Self::address_type_label_decode(kwargs))
            }
            Self::ENCODE_PAYMENT => {
                Self::decode_shelley_payment(address, Self::network_decode(kwargs))
            }
            Self::ENCODE_STAKING | Self::ENCODE_REWARD => {
                Self::decode_shelley_staking(address, Self::network_decode(kwargs))
            }
            other => Err(AddressSchemeError::Msg(format!(
                "Invalid decode type (expected: byron-icarus, byron-legacy, payment, staking, reward | got: {other})"
            ))),
        }
    }
}
