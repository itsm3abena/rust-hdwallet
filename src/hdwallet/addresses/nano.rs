use crate::hdwallet::cryptocurrencies::nano::NANO_PARAMS;
use crate::hdwallet::eccs::{
    validate_and_get_public_key, IPublicKey, PublicKeyInput, Slip10Ed25519Blake2bPublicKey,
};
use crate::hdwallet::exceptions::PublicKeyError;
use crate::hdwallet::libs::base32::{self, Base32Error};
use crate::hdwallet::utils::{blake2b_40, bytes_reverse};

use super::iaddress::{
    AddressSchemeError, DecodeKwargs, EncodeKwargs, EncodePublicKey, IAddress,
};

const PAYLOAD_PADDING_DECODED: [u8; 3] = [0x00, 0x00, 0x00];
const PAYLOAD_PADDING_ENCODED: &str = "1111";
const PUBKEY_LEN: usize = 32;
const CHECKSUM_LEN: usize = 5;
const DECODED_PAYLOAD_LEN: usize =
    PAYLOAD_PADDING_DECODED.len() + PUBKEY_LEN + CHECKSUM_LEN;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct NanoAddress;

impl NanoAddress {
    pub const REGISTRY_NAME: &'static str = "Nano";

    fn address_prefix() -> &'static str {
        NANO_PARAMS.address_prefix.unwrap_or("nano_")
    }

    fn alphabet() -> &'static str {
        NANO_PARAMS
            .alphabet
            .unwrap_or("13456789abcdefghijkmnopqrstuwxyz")
    }

    fn pubkey_body(pk: &Slip10Ed25519Blake2bPublicKey) -> Result<[u8; PUBKEY_LEN], AddressSchemeError> {
        let raw = pk.raw_compressed();
        raw[1..]
            .try_into()
            .map_err(|_| AddressSchemeError::Msg("expected 33-byte compressed SLIP10 Ed25519 key".into()))
    }

    fn compute_checksum(public_key: &[u8; PUBKEY_LEN]) -> [u8; CHECKSUM_LEN] {
        let reversed = bytes_reverse(&blake2b_40(public_key));
        reversed
            .try_into()
            .expect("blake2b_40 digest is 5 bytes")
    }

    fn map_base32(err: Base32Error) -> AddressSchemeError {
        AddressSchemeError::Msg(err.to_string())
    }

    fn encode_payload(body: &[u8; PUBKEY_LEN]) -> Result<String, AddressSchemeError> {
        let checksum = Self::compute_checksum(body);
        let mut payload = Vec::with_capacity(DECODED_PAYLOAD_LEN);
        payload.extend_from_slice(&PAYLOAD_PADDING_DECODED);
        payload.extend_from_slice(body);
        payload.extend_from_slice(&checksum);

        let b32_encoded = base32::encode_no_padding(&hex::encode(payload), Some(Self::alphabet()))
            .map_err(Self::map_base32)?;
        let strip = PAYLOAD_PADDING_ENCODED.len();
        if b32_encoded.len() < strip {
            return Err(AddressSchemeError::Msg("invalid base32 encoding length".into()));
        }
        Ok(format!("{}{}", Self::address_prefix(), &b32_encoded[strip..]))
    }

    fn decode_payload(address: &str) -> Result<[u8; PUBKEY_LEN], AddressSchemeError> {
        let prefix = Self::address_prefix();
        if !address.starts_with(prefix) {
            let got = address.get(..prefix.len()).unwrap_or(address);
            return Err(AddressSchemeError::Msg(format!(
                "Invalid prefix (expected: {prefix}, got: {got})"
            )));
        }
        let address_no_prefix = &address[prefix.len()..];
        let hex_payload = base32::decode(
            &format!("{PAYLOAD_PADDING_ENCODED}{address_no_prefix}"),
            Some(Self::alphabet()),
        )
        .map_err(Self::map_base32)?;
        let address_decode = hex::decode(hex_payload).map_err(|e| {
            AddressSchemeError::Msg(format!("invalid decoded payload hex: {e}"))
        })?;

        if address_decode.len() != DECODED_PAYLOAD_LEN {
            return Err(AddressSchemeError::Msg(format!(
                "Invalid length (expected: {DECODED_PAYLOAD_LEN}, got: {})",
                address_decode.len()
            )));
        }
        if address_decode[..PAYLOAD_PADDING_DECODED.len()] != PAYLOAD_PADDING_DECODED {
            return Err(AddressSchemeError::Msg("invalid payload padding".into()));
        }

        let body_start = PAYLOAD_PADDING_DECODED.len();
        let body_end = body_start + PUBKEY_LEN;
        let checksum: [u8; CHECKSUM_LEN] = address_decode[body_end..]
            .try_into()
            .map_err(|_| AddressSchemeError::Msg("invalid checksum length".into()))?;
        let public_key: [u8; PUBKEY_LEN] = address_decode[body_start..body_end]
            .try_into()
            .map_err(|_| AddressSchemeError::Msg("invalid public key length".into()))?;

        let checksum_got = Self::compute_checksum(&public_key);
        if checksum != checksum_got {
            return Err(AddressSchemeError::Msg(format!(
                "Invalid checksum (expected: {}, got: {})",
                hex::encode(checksum),
                hex::encode(checksum_got)
            )));
        }

        Slip10Ed25519Blake2bPublicKey::from_bytes(&public_key).map_err(|_| {
            AddressSchemeError::Msg(format!(
                "Invalid {} public key {}",
                Slip10Ed25519Blake2bPublicKey::ecc_name(),
                hex::encode(public_key)
            ))
        })?;

        Ok(public_key)
    }
}

impl IAddress for NanoAddress {
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
        let pk = validate_and_get_public_key::<Slip10Ed25519Blake2bPublicKey>(input).map_err(
            |e: PublicKeyError| AddressSchemeError::Msg(e.to_string()),
        )?;
        let body = Self::pubkey_body(&pk)?;
        Self::encode_payload(&body)
    }

    fn decode(
        &self,
        address: &str,
        _kwargs: &DecodeKwargs,
    ) -> super::iaddress::AddressEncodeResult {
        let body = Self::decode_payload(address)?;
        Ok(hex::encode(body))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::iaddress::encode_with_ipublic_key;
    use crate::hdwallet::eccs::{IPrivateKey, Slip10Ed25519Blake2bPrivateKey};

    fn test_public_key() -> Slip10Ed25519Blake2bPublicKey {
        let mut secret = [0u8; 32];
        secret[0] = 9;
        let sk = Slip10Ed25519Blake2bPrivateKey::from_bytes(&secret).expect("sk");
        sk.public_key()
    }

    #[test]
    fn roundtrip_default_prefix() {
        let pk = test_public_key();
        let nano = NanoAddress;
        let addr = encode_with_ipublic_key(&nano, &pk, &EncodeKwargs::default()).expect("encode");
        assert!(addr.starts_with("nano_"));
        let decoded = nano.decode(&addr, &DecodeKwargs::default()).expect("decode");
        assert_eq!(decoded.len(), 64);
        assert_eq!(decoded, hex::encode(NanoAddress::pubkey_body(&pk).unwrap()));
    }
}
