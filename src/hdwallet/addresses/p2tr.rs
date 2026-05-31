use num_bigint::BigUint;
use num_traits::{One, Zero};

use crate::hdwallet::cryptocurrencies::bitcoin::{
    Mainnet as BitcoinMainnet, Regtest as BitcoinRegtest, Testnet as BitcoinTestnet,
    BITCOIN_PARAMS,
};
use crate::hdwallet::eccs::{
    validate_and_get_public_key, IPoint, IPublicKey, PublicKeyInput, Slip10Secp256k1Point,
    Slip10Secp256k1PublicKey,
};
use crate::hdwallet::exceptions::PublicKeyError;
use crate::hdwallet::libs::ecc::{self, sha256};
use crate::hdwallet::libs::segwit_bech32::{segwit_decode, segwit_encode};

use super::iaddress::{
    AddressSchemeError, DecodeKwargs, EncodeKwargs, EncodePublicKey, IAddress,
};

const OUTPUT_KEY_LEN: usize = 32;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct P2TRAddress;

impl P2TRAddress {
    pub const REGISTRY_NAME: &'static str = "P2TR";

    const DEFAULT_WITNESS_VERSION: u8 = 0x01;

    fn default_hrp() -> &'static str {
        BitcoinMainnet::HRP
    }

    fn hrp_encode(kwargs: &EncodeKwargs<'_>) -> &'static str {
        kwargs.hrp.unwrap_or_else(|| Self::hrp_from_network(kwargs.network))
    }

    fn hrp_decode(kwargs: &DecodeKwargs) -> &'static str {
        kwargs.hrp.unwrap_or_else(|| Self::hrp_from_network(kwargs.network))
    }

    fn hrp_from_network(network: Option<&str>) -> &'static str {
        match network {
            Some("testnet") => BitcoinTestnet::HRP,
            Some("regtest") => BitcoinRegtest::HRP,
            _ => Self::default_hrp(),
        }
    }

    fn witness_version_encode(kwargs: &EncodeKwargs<'_>) -> u8 {
        kwargs
            .witness_version
            .unwrap_or(Self::DEFAULT_WITNESS_VERSION)
    }

    fn witness_version_decode(kwargs: &DecodeKwargs) -> u8 {
        kwargs
            .witness_version
            .unwrap_or(Self::DEFAULT_WITNESS_VERSION)
    }

    fn tap_tweak_tag() -> Result<[u8; 32], AddressSchemeError> {
        let hex_tag = BITCOIN_PARAMS.tap_tweak_sha256.ok_or_else(|| {
            AddressSchemeError::Msg("missing tap_tweak_sha256 parameter".into())
        })?;
        let bytes = hex::decode(hex_tag).map_err(|e| {
            AddressSchemeError::Msg(format!("invalid tap_tweak_sha256 hex: {e}"))
        })?;
        bytes
            .try_into()
            .map_err(|_| AddressSchemeError::Msg("tap_tweak_sha256 must be 32 bytes".into()))
    }

    fn tagged_hash(tag: &[u8; 32], data: &[u8]) -> [u8; 32] {
        let mut preimage = Vec::with_capacity(64 + data.len());
        preimage.extend_from_slice(tag);
        preimage.extend_from_slice(tag);
        preimage.extend_from_slice(data);
        sha256(&preimage)
    }

    fn integer_to_bytes_minimal(value: &BigUint) -> Vec<u8> {
        let be = value.to_bytes_be();
        if be.is_empty() {
            vec![0]
        } else {
            be
        }
    }

    fn map_eccs(err: crate::hdwallet::eccs::EccsError) -> AddressSchemeError {
        AddressSchemeError::Eccs(err)
    }

    fn hash_tap_tweak(pk: &Slip10Secp256k1PublicKey) -> Result<[u8; 32], AddressSchemeError> {
        let tag = Self::tap_tweak_tag()?;
        let x_bytes = Self::integer_to_bytes_minimal(&pk.point().x());
        Ok(Self::tagged_hash(&tag, &x_bytes))
    }

    fn lift_x(pk: &Slip10Secp256k1PublicKey) -> Result<Slip10Secp256k1Point, AddressSchemeError> {
        let p = ecc::p();
        let x = pk.point().x();
        if x >= *p {
            return Err(AddressSchemeError::Msg("Unable to compute LiftX point".into()));
        }
        let seven = BigUint::from(7u32);
        let c = (x.modpow(&BigUint::from(3u32), p) + seven) % p;
        let exp = (p + BigUint::one()) / BigUint::from(4u32);
        let y = c.modpow(&exp, p);
        if y.modpow(&BigUint::from(2u32), p) != c {
            return Err(AddressSchemeError::Msg("Unable to compute LiftX point".into()));
        }
        let y_even = if &y % BigUint::from(2u32) == BigUint::zero() {
            y
        } else {
            p - &y
        };
        Slip10Secp256k1Point::from_coordinates(&x, &y_even).map_err(Self::map_eccs)
    }

    pub(crate) fn tweak_public_key(pk: &Slip10Secp256k1PublicKey) -> Result<Vec<u8>, AddressSchemeError> {
        let h = Self::hash_tap_tweak(pk)?;
        let h_int = BigUint::from_bytes_be(&h);
        let lifted = Self::lift_x(pk)?;
        let generator = Slip10Secp256k1Point::generator();
        let tweak_point = generator
            .mul_scalar(&h_int)
            .map_err(Self::map_eccs)?;
        let out_point = lifted.add(&tweak_point).map_err(Self::map_eccs)?;
        Ok(Self::integer_to_bytes_minimal(&out_point.x()))
    }

    pub(crate) fn encode_public_key(
        pk: &Slip10Secp256k1PublicKey,
        hrp: &str,
        witness_version: u8,
    ) -> Result<String, AddressSchemeError> {
        let output_key = Self::tweak_public_key(pk)?;
        segwit_encode(hrp, witness_version, &output_key).ok_or_else(|| {
            AddressSchemeError::Msg(format!("failed to segwit-encode {hrp} P2TR address"))
        })
    }

    pub(crate) fn decode_address(
        address: &str,
        hrp: &str,
        witness_version: u8,
    ) -> Result<String, AddressSchemeError> {
        let (witver, address_decode) = segwit_decode(hrp, address).ok_or_else(|| {
            AddressSchemeError::Msg(format!("invalid segwit address for hrp {hrp}"))
        })?;
        if address_decode.len() != OUTPUT_KEY_LEN {
            return Err(AddressSchemeError::Msg(format!(
                "Invalid length (expected: {OUTPUT_KEY_LEN}, got: {})",
                address_decode.len()
            )));
        }
        if witver != witness_version {
            return Err(AddressSchemeError::Msg(format!(
                "Invalid witness version (expected: {witness_version}, got: {witver})"
            )));
        }
        Ok(hex::encode(address_decode))
    }
}

impl IAddress for P2TRAddress {
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
        let pk = validate_and_get_public_key::<Slip10Secp256k1PublicKey>(input).map_err(
            |e: PublicKeyError| AddressSchemeError::Msg(e.to_string()),
        )?;
        Self::encode_public_key(
            &pk,
            Self::hrp_encode(kwargs),
            Self::witness_version_encode(kwargs),
        )
    }

    fn decode(&self, address: &str, kwargs: &DecodeKwargs) -> super::iaddress::AddressEncodeResult {
        Self::decode_address(
            address,
            Self::hrp_decode(kwargs),
            Self::witness_version_decode(kwargs),
        )
    }
}
