use crate::hdwallet::eccs::{
    validate_and_get_public_key, IPublicKey, PublicKeyInput, Slip10Ed25519PublicKeyStd,
};
use crate::hdwallet::exceptions::PublicKeyError;

use super::iaddress::{
    AddressSchemeError, DecodeKwargs, EncodeKwargs, EncodePublicKey, IAddress,
};

const PUBKEY_LEN: usize = 32;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct NearAddress;

impl NearAddress {
    pub const REGISTRY_NAME: &'static str = "Near";

    fn pubkey_body(pk: &Slip10Ed25519PublicKeyStd) -> Result<[u8; PUBKEY_LEN], AddressSchemeError> {
        let raw = pk.raw_compressed();
        raw[1..]
            .try_into()
            .map_err(|_| AddressSchemeError::Msg("expected 33-byte compressed SLIP10 Ed25519 key".into()))
    }

    fn encode_hex_body(body: &[u8; PUBKEY_LEN]) -> String {
        hex::encode(body)
    }

    fn decode_address_bytes(address: &str) -> Result<Vec<u8>, AddressSchemeError> {
        let bytes = hex::decode(address).map_err(|e| {
            AddressSchemeError::Msg(format!("invalid Near address hex: {e}"))
        })?;
        if bytes.len() != PUBKEY_LEN {
            return Err(AddressSchemeError::Msg(format!(
                "Invalid length (expected: {PUBKEY_LEN}, got: {})",
                bytes.len()
            )));
        }
        Ok(bytes)
    }
}

impl IAddress for NearAddress {
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
        let pk = validate_and_get_public_key::<Slip10Ed25519PublicKeyStd>(input).map_err(
            |e: PublicKeyError| AddressSchemeError::Msg(e.to_string()),
        )?;
        Ok(Self::encode_hex_body(&Self::pubkey_body(&pk)?))
    }

    fn decode(
        &self,
        address: &str,
        _kwargs: &DecodeKwargs,
    ) -> super::iaddress::AddressEncodeResult {
        let bytes = Self::decode_address_bytes(address)?;
        validate_and_get_public_key::<Slip10Ed25519PublicKeyStd>(PublicKeyInput::Bytes(&bytes))
            .map_err(|e: PublicKeyError| AddressSchemeError::Msg(e.to_string()))?;
        Ok(address.to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::iaddress::encode_with_ipublic_key;
    use crate::hdwallet::eccs::{IPrivateKey, Slip10Ed25519PrivateKeyStd};

    fn test_public_key() -> Slip10Ed25519PublicKeyStd {
        let mut secret = [0u8; 32];
        secret[0] = 9;
        let sk = Slip10Ed25519PrivateKeyStd::from_bytes(&secret).expect("sk");
        sk.public_key()
    }

    #[test]
    fn roundtrip_hex_body() {
        let pk = test_public_key();
        let near = NearAddress;
        let addr = encode_with_ipublic_key(&near, &pk, &EncodeKwargs::default()).expect("encode");
        assert_eq!(addr.len(), 64);
        assert!(!addr.starts_with("00"));
        let decoded = near.decode(&addr, &DecodeKwargs::default()).expect("decode");
        assert_eq!(decoded, addr);
        assert_eq!(decoded, NearAddress::encode_hex_body(&NearAddress::pubkey_body(&pk).unwrap()));
    }

    #[test]
    fn encode_strips_compressed_prefix_hex() {
        let pk = test_public_key();
        let full = hex::encode(pk.raw_compressed());
        let body = NearAddress::encode_hex_body(&NearAddress::pubkey_body(&pk).unwrap());
        assert_eq!(body, full[2..]);
    }
}
