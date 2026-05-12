use crate::hdwallet::cryptocurrencies::aptos::APTOS_PARAMS;
use crate::hdwallet::eccs::{
    validate_and_get_public_key, IPublicKey, PublicKeyInput, Slip10Ed25519PublicKeyStd,
};
use crate::hdwallet::exceptions::PublicKeyError;
use crate::hdwallet::utils::sha3_256;

use super::iaddress::{
    AddressSchemeError, DecodeKwargs, EncodeKwargs, EncodePublicKey, IAddress,
};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct AptosAddress;

fn integer_to_bytes_python_style(value: u32) -> Vec<u8> {
    let bit_len = if value == 0 { 1 } else { 32 - value.leading_zeros() };
    let bytes_num = ((bit_len + 7) / 8) as usize;
    value.to_be_bytes()[4 - bytes_num..].to_vec()
}

impl AptosAddress {
    pub const REGISTRY_NAME: &'static str = "Aptos";

    pub fn address_prefix() -> &'static str {
        APTOS_PARAMS.address_prefix.unwrap_or("0x")
    }

    pub fn suffix_bytes() -> Vec<u8> {
        integer_to_bytes_python_style(APTOS_PARAMS.suffix.unwrap_or(0) as u32)
    }

    fn body_32_from_slip(pk: &Slip10Ed25519PublicKeyStd) -> Result<[u8; 32], AddressSchemeError> {
        let raw = pk.raw_compressed();
        raw[1..]
            .try_into()
            .map_err(|_| AddressSchemeError::Msg("expected 33-byte compressed SLIP10 Ed25519 key".into()))
    }
}

impl IAddress for AptosAddress {
    fn name(&self) -> &'static str {
        Self::REGISTRY_NAME
    }

    fn encode(
        &self,
        public_key: EncodePublicKey<'_>,
        _kwargs: &EncodeKwargs,
    ) -> super::iaddress::AddressEncodeResult {
        let input = match public_key {
            EncodePublicKey::Bytes(b) => PublicKeyInput::Bytes(b),
            EncodePublicKey::Str(s) => PublicKeyInput::Hex(s),
        };
        let pk = validate_and_get_public_key::<Slip10Ed25519PublicKeyStd>(input).map_err(
            |e: PublicKeyError| AddressSchemeError::Msg(e.to_string()),
        )?;
        let body = Self::body_32_from_slip(&pk)?;
        let mut payload = Vec::with_capacity(32 + 4);
        payload.extend_from_slice(&body);
        payload.extend_from_slice(&Self::suffix_bytes());
        let hash = sha3_256(&payload);
        let mut hex_str = hex::encode(hash);
        hex_str = hex_str.trim_start_matches('0').to_string();
        Ok(format!("{}{}", Self::address_prefix(), hex_str))
    }

    fn decode(&self, address: &str, _kwargs: &DecodeKwargs) -> super::iaddress::AddressEncodeResult {
        let prefix = Self::address_prefix();
        let prefix_got = address.get(..prefix.len()).unwrap_or("");
        if prefix_got != prefix {
            return Err(AddressSchemeError::Msg(format!(
                "Invalid address_prefix (expected: {prefix}, got: {prefix_got})"
            )));
        }
        let rest = address
            .get(prefix.len()..)
            .ok_or_else(|| AddressSchemeError::Msg("Aptos address too short".into()))?;
        if rest.len() > 64 {
            return Err(AddressSchemeError::Msg(format!(
                "Invalid Aptos address hex length (expected at most 64, got: {})",
                rest.len()
            )));
        }
        if !rest.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(AddressSchemeError::Msg(
                "Aptos address body must be hexadecimal".into(),
            ));
        }
        let address_no_prefix = format!("{}{}", "0".repeat(64 - rest.len()), rest);
        Ok(address_no_prefix)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hdwallet::addresses::iaddress::encode_with_ipublic_key;
    use crate::hdwallet::hds::algorand::AlgorandHd;

    #[test]
    fn encode_decode_roundtrip_restores_hash_hex() {
        let seed = [9u8; 64];
        let hd = AlgorandHd::from_seed_bytes(&seed).expect("seed");

        let mut prefixed = [0u8; 33];
        prefixed[0] = Slip10Ed25519PublicKeyStd::PREFIX;
        prefixed[1..].copy_from_slice(hd.pub_key_bytes());
        let pk = Slip10Ed25519PublicKeyStd::from_bytes(&prefixed).expect("pk");

        let body: [u8; 32] = *hd.pub_key_bytes();
        let mut payload = Vec::new();
        payload.extend_from_slice(&body);
        payload.extend_from_slice(&AptosAddress::suffix_bytes());
        let expected_hash_hex = hex::encode(sha3_256(&payload));

        let aptos = AptosAddress;
        let addr = encode_with_ipublic_key(&aptos, &pk, &EncodeKwargs::default()).expect("encode");
        let decoded = aptos.decode(&addr, &DecodeKwargs::default()).expect("decode");
        assert_eq!(decoded, expected_hash_hex);
    }

    #[test]
    fn decode_rejects_bad_prefix() {
        let aptos = AptosAddress;
        assert!(aptos.decode("1xabc", &DecodeKwargs::default()).is_err());
    }
}
