use data_encoding::BASE64;

use crate::hdwallet::crypto::crc16_ccitt;
use crate::hdwallet::cryptocurrencies::ton::Ton;
use crate::hdwallet::eccs::{
    validate_and_get_public_key, IPublicKey, PublicKeyInput, Slip10Ed25519PublicKeyStd,
};
use crate::hdwallet::exceptions::PublicKeyError;
use crate::hdwallet::libs::ton::wallet::{
    wallet_v4r2_address_hash, wallet_v5r1_address_hash, DEFAULT_WALLET_ID,
    MAINNET_NETWORK_GLOBAL_ID, TESTNET_NETWORK_GLOBAL_ID,
};

use super::iaddress::{
    AddressSchemeError, DecodeKwargs, EncodeKwargs, EncodePublicKey, IAddress,
};

const PUBKEY_LEN: usize = 32;
const FRIENDLY_ADDRESS_LEN: usize = 48;
const ADDRESS_PAYLOAD_LEN: usize = 36;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct TonAddress;

impl TonAddress {
    pub const REGISTRY_NAME: &'static str = "Ton";

    fn pubkey_body(pk: &Slip10Ed25519PublicKeyStd) -> Result<[u8; PUBKEY_LEN], AddressSchemeError> {
        let raw = pk.raw_compressed();
        raw[1..].try_into().map_err(|_| {
            AddressSchemeError::Msg("expected 33-byte compressed SLIP10 Ed25519 key".into())
        })
    }

    fn encode_friendly_address(
        address_hash: &[u8],
        workchain: i8,
        bounceable: bool,
        testnet: bool,
    ) -> Result<String, AddressSchemeError> {
        let mut tag = if bounceable {
            Ton::BOUNCEABLE_TAG
        } else {
            Ton::NON_BOUNCEABLE_TAG
        };
        if testnet {
            tag |= Ton::TEST_FLAG;
        }

        let mut payload = Vec::with_capacity(34);
        payload.push(tag);
        payload.push(workchain as u8);
        payload.extend_from_slice(address_hash);
        let checksum = crc16_ccitt(&payload);
        payload.extend_from_slice(&checksum);

        let encoded = BASE64.encode(&payload);
        Ok(encoded.replace('+', "-").replace('/', "_"))
    }

    fn decode_friendly_address(address: &str) -> Result<(bool, bool, i8, Vec<u8>), AddressSchemeError> {
        if address.len() != FRIENDLY_ADDRESS_LEN {
            return Err(AddressSchemeError::Msg(format!(
                "Invalid length (expected: {FRIENDLY_ADDRESS_LEN}, got: {})",
                address.len()
            )));
        }

        let normalized = address.replace('-', "+").replace('_', "/");
        let data = BASE64
            .decode(normalized.as_bytes())
            .map_err(|e| AddressSchemeError::Msg(format!("Invalid base64 address: {e}")))?;
        if data.len() != ADDRESS_PAYLOAD_LEN {
            return Err(AddressSchemeError::Msg(format!(
                "Invalid address payload length (expected: {ADDRESS_PAYLOAD_LEN}, got: {})",
                data.len()
            )));
        }

        let payload = &data[..34];
        let checksum = &data[34..36];
        let calculated = crc16_ccitt(payload);
        if checksum != calculated.as_slice() {
            return Err(AddressSchemeError::Msg("Invalid address checksum".into()));
        }

        let mut tag = payload[0];
        let is_test_only = (tag & Ton::TEST_FLAG) != 0;
        if is_test_only {
            tag ^= Ton::TEST_FLAG;
        }
        if tag != Ton::BOUNCEABLE_TAG && tag != Ton::NON_BOUNCEABLE_TAG {
            return Err(AddressSchemeError::Msg(format!(
                "Invalid address tag (got: {tag:#x})"
            )));
        }

        let workchain: i8 = if payload[1] == 0xFF {
            -1
        } else {
            payload[1] as i8
        };
        if workchain != 0 && workchain != -1 {
            return Err(AddressSchemeError::Msg(format!(
                "Invalid workchain (got: {workchain})"
            )));
        }

        Ok((
            is_test_only,
            tag == Ton::BOUNCEABLE_TAG,
            workchain,
            payload[2..34].to_vec(),
        ))
    }
}

impl IAddress for TonAddress {
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
        let pk = validate_and_get_public_key::<Slip10Ed25519PublicKeyStd>(input)
            .map_err(|e: PublicKeyError| AddressSchemeError::Msg(e.to_string()))?;

        let bounceable = kwargs.bounceable.unwrap_or(Ton::DEFAULT_BOUNCEABLE);
        let testnet = kwargs.ton_testnet.unwrap_or(false);
        let workchain = kwargs.workchain.unwrap_or(Ton::WORKCHAIN);
        let wallet_version = kwargs
            .wallet_version
            .unwrap_or(Ton::DEFAULT_WALLET_VERSION);
        let public_key_bytes = Self::pubkey_body(&pk)?;

        let address_hash = if wallet_version == "v4r2" {
            let wallet_id = kwargs.wallet_id.unwrap_or(DEFAULT_WALLET_ID);
            wallet_v4r2_address_hash(&public_key_bytes, i32::from(workchain), wallet_id)
                .map_err(AddressSchemeError::Msg)?
        } else if wallet_version == "v5r1" {
            let network_global_id = kwargs.network_global_id.unwrap_or(if testnet {
                TESTNET_NETWORK_GLOBAL_ID
            } else {
                MAINNET_NETWORK_GLOBAL_ID
            });
            wallet_v5r1_address_hash(
                &public_key_bytes,
                i32::from(workchain),
                network_global_id,
                kwargs.subwallet_number.unwrap_or(Ton::SUBWALLET_NUMBER),
                kwargs.wallet_id,
            )
            .map_err(AddressSchemeError::Msg)?
        } else {
            return Err(AddressSchemeError::Msg(format!(
                "Unsupported wallet version (got: {wallet_version})"
            )));
        };

        Self::encode_friendly_address(&address_hash, workchain, bounceable, testnet)
    }

    fn decode(
        &self,
        address: &str,
        _kwargs: &DecodeKwargs,
    ) -> super::iaddress::AddressDecodeResult {
        let (_is_test_only, _is_bounceable, _workchain, hash_part) =
            Self::decode_friendly_address(address)?;
        Ok(hex::encode(hash_part))
    }
}
