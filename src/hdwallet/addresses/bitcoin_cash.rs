use crate::hdwallet::consts::PublicKeyTypes;
use crate::hdwallet::cryptocurrencies::bitcoin_cash::Mainnet as BitcoinCashMainnet;
use crate::hdwallet::eccs::{
    validate_and_get_public_key, IPublicKey, PublicKeyInput, Slip10Secp256k1PublicKey,
};
use crate::hdwallet::exceptions::PublicKeyError;
use crate::hdwallet::libs::cashaddr::{cashaddr_decode, cashaddr_encode};
use crate::hdwallet::libs::ecc::hash160;

use super::iaddress::{
    AddressSchemeError, DecodeKwargs, EncodeKwargs, EncodePublicKey, IAddress,
};

const P2PKH_VERSION_BYTE: u8 = 0x00;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct BitcoinCashAddress;

impl BitcoinCashAddress {
    pub const REGISTRY_NAME: &'static str = "BitcoinCash";

    fn hrp(kwargs: &EncodeKwargs) -> &'static str {
        kwargs.hrp.unwrap_or(BitcoinCashMainnet::HRP)
    }

    fn hrp_decode(kwargs: &DecodeKwargs) -> &'static str {
        kwargs.hrp.unwrap_or(BitcoinCashMainnet::HRP)
    }

    fn use_compressed(kwargs: &EncodeKwargs) -> bool {
        match kwargs.public_key_type {
            None | Some(PublicKeyTypes::COMPRESSED) => true,
            Some(PublicKeyTypes::UNCOMPRESSED) => false,
            Some(_) => true,
        }
    }

    fn public_key_hash(pk: &Slip10Secp256k1PublicKey, compressed: bool) -> [u8; 20] {
        let raw = if compressed {
            pk.raw_compressed()
        } else {
            pk.raw_uncompressed()
        };
        hash160(&raw)
    }

    fn encode_payload(hrp: &str, hash: &[u8; 20]) -> Result<String, AddressSchemeError> {
        let mut payload = Vec::with_capacity(21);
        payload.push(P2PKH_VERSION_BYTE);
        payload.extend_from_slice(hash);
        cashaddr_encode(hrp, &payload).ok_or_else(|| {
            AddressSchemeError::Msg(format!("failed to CashAddr-encode for hrp {hrp}"))
        })
    }

    fn decode_payload(hrp: &str, address: &str) -> Result<String, AddressSchemeError> {
        let decoded = cashaddr_decode(hrp, address).ok_or_else(|| {
            AddressSchemeError::Msg("invalid CashAddr".into())
        })?;
        if decoded.len() < 21 {
            return Err(AddressSchemeError::Msg("invalid CashAddr data".into()));
        }
        Ok(hex::encode(&decoded[1..21]))
    }
}

impl IAddress for BitcoinCashAddress {
    fn name(&self) -> &'static str {
        Self::REGISTRY_NAME
    }

    fn encode(
        &self,
        public_key: EncodePublicKey<'_>,
        kwargs: &EncodeKwargs,
    ) -> super::iaddress::AddressEncodeResult {
        let input = match public_key {
            EncodePublicKey::Bytes(b) => PublicKeyInput::Bytes(b),
            EncodePublicKey::Str(s) => PublicKeyInput::Hex(s),
        };
        let pk = validate_and_get_public_key::<Slip10Secp256k1PublicKey>(input).map_err(
            |e: PublicKeyError| AddressSchemeError::Msg(e.to_string()),
        )?;
        let hash = Self::public_key_hash(&pk, Self::use_compressed(kwargs));
        Self::encode_payload(Self::hrp(kwargs), &hash)
    }

    fn decode(
        &self,
        address: &str,
        kwargs: &DecodeKwargs,
    ) -> super::iaddress::AddressEncodeResult {
        if let Some((hrp_got, _)) = address.split_once(':') {
            let expected = Self::hrp_decode(kwargs);
            if hrp_got != expected {
                return Err(AddressSchemeError::Msg(format!(
                    "Invalid HRP (expected: {expected}, got: {hrp_got})"
                )));
            }
        }
        Self::decode_payload(Self::hrp_decode(kwargs), address)
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
        let path = bip44_path(coin_types::BITCOIN_CASH, 0, 0, 0).expect("path");
        let compressed = hd.derive_xpub(&path).expect("xpub").public_key.serialize();
        Slip10Secp256k1PublicKey::from_bytes(&compressed).expect("pk")
    }

    #[test]
    fn roundtrip_mainnet() {
        let pk = test_public_key();
        let bch = BitcoinCashAddress;
        let addr = encode_with_ipublic_key(&bch, &pk, &EncodeKwargs::default()).expect("encode");
        assert!(addr.starts_with("bitcoincash:q"));
        let decoded = bch
            .decode(&addr, &DecodeKwargs::default())
            .expect("decode");
        let hash = BitcoinCashAddress::public_key_hash(&pk, true);
        assert_eq!(decoded, hex::encode(hash));
    }

    #[test]
    fn wrong_hrp_errors() {
        let pk = test_public_key();
        let bch = BitcoinCashAddress;
        let addr = encode_with_ipublic_key(&bch, &pk, &EncodeKwargs::default()).expect("encode");
        let err = bch.decode(
            &addr,
            &DecodeKwargs {
                hrp: Some("bchtest"),
                ..DecodeKwargs::default()
            },
        );
        assert!(err.is_err());
    }

    #[test]
    fn cashaddr_has_colon_prefix() {
        let pk = test_public_key();
        let bch = BitcoinCashAddress;
        let addr = encode_with_ipublic_key(&bch, &pk, &EncodeKwargs::default()).expect("encode");
        assert!(addr.contains(':'));
        let (hrp, _) = addr.split_once(':').expect("colon");
        assert_eq!(hrp, BitcoinCashMainnet::HRP);
    }
}
