use crate::hdwallet::cryptocurrencies::injective::Mainnet as InjectiveMainnet;
use crate::hdwallet::eccs::{
    validate_and_get_public_key, PublicKeyInput, Slip10Secp256k1PublicKey,
};
use crate::hdwallet::exceptions::PublicKeyError;
use crate::hdwallet::libs::segwit_bech32::{base_bech32_decode, bech32_encode, convertbits};

use super::ethereum::EthereumAddress;
use super::iaddress::{
    AddressSchemeError, DecodeKwargs, EncodeKwargs, EncodePublicKey, IAddress,
};

const HASH_LEN: usize = 20;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct InjectiveAddress;

impl InjectiveAddress {
    pub const REGISTRY_NAME: &'static str = "Injective";

    fn default_hrp() -> &'static str {
        InjectiveMainnet::HRP
    }

    fn hrp(kwargs: &EncodeKwargs<'_>) -> &'static str {
        kwargs.hrp.unwrap_or(Self::default_hrp())
    }

    fn hrp_decode(kwargs: &DecodeKwargs) -> &'static str {
        kwargs.hrp.unwrap_or(Self::default_hrp())
    }

    fn eth_hash_bytes(
        public_key: PublicKeyInput<'_, Slip10Secp256k1PublicKey>,
    ) -> Result<[u8; HASH_LEN], AddressSchemeError> {
        let eth_address = EthereumAddress::encode_public_key(public_key, false)?;
        let bytes = hex::decode(&eth_address[2..]).map_err(|e| AddressSchemeError::Msg(e.to_string()))?;
        bytes
            .try_into()
            .map_err(|_| AddressSchemeError::Msg(format!("expected {HASH_LEN}-byte hash")))
    }

    pub fn encode_public_key(
        public_key: PublicKeyInput<'_, Slip10Secp256k1PublicKey>,
        kwargs: &EncodeKwargs<'_>,
    ) -> Result<String, AddressSchemeError> {
        let pk = validate_and_get_public_key(public_key)
            .map_err(|e: PublicKeyError| AddressSchemeError::Msg(e.to_string()))?;
        let hash_bytes = Self::eth_hash_bytes(PublicKeyInput::Instance(pk))?;
        let hrp = Self::hrp(kwargs);
        bech32_encode(hrp, &hash_bytes).ok_or_else(|| {
            AddressSchemeError::Msg(format!("failed to bech32-encode {hrp} address"))
        })
    }

    pub fn decode_address(address: &str, kwargs: &DecodeKwargs) -> Result<String, AddressSchemeError> {
        let hrp = Self::hrp_decode(kwargs);
        let (hrp_got, data) = base_bech32_decode(address).ok_or_else(|| {
            AddressSchemeError::Msg(format!("invalid bech32 address for hrp {hrp}"))
        })?;
        if hrp_got != hrp {
            return Err(AddressSchemeError::Msg(format!(
                "invalid bech32 hrp (expected: {hrp}, got: {hrp_got})"
            )));
        }
        let address_decode = convertbits(&data, 5, 8, false).ok_or_else(|| {
            AddressSchemeError::Msg("invalid bech32 payload".into())
        })?;
        if address_decode.len() != HASH_LEN {
            return Err(AddressSchemeError::Msg(format!(
                "Invalid length (expected: {HASH_LEN}, got: {})",
                address_decode.len()
            )));
        }
        Ok(hex::encode(address_decode))
    }
}

impl IAddress for InjectiveAddress {
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
        let path = bip44_path(coin_types::INJECTIVE, 0, 0, 0).expect("path");
        let compressed = hd.derive_xpub(&path).expect("xpub").public_key.serialize();
        Slip10Secp256k1PublicKey::from_bytes(&compressed).expect("pk")
    }

    #[test]
    fn roundtrip_default_hrp() {
        let pk = test_public_key();
        let inj = InjectiveAddress;
        let addr = encode_with_ipublic_key(&inj, &pk, &EncodeKwargs::default()).expect("encode");
        assert!(addr.starts_with("inj1"));
        let decoded = inj.decode(&addr, &DecodeKwargs::default()).expect("decode");
        assert_eq!(decoded.len(), 40);
        let hash = InjectiveAddress::eth_hash_bytes(crate::hdwallet::eccs::PublicKeyInput::Instance(
            pk,
        ))
        .expect("hash");
        assert_eq!(decoded, hex::encode(hash));
    }
}
