use crate::hdwallet::exceptions::{ECCError, PublicKeyError};
use crate::hdwallet::utils;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EccsError {
    #[error("invalid secp256k1 private key")]
    InvalidSecp256k1Secret,
    #[error("invalid secp256k1 public key or point")]
    InvalidSecp256k1Public,
    #[error("invalid Ed25519 encoding")]
    InvalidEd25519,
    #[error("invalid NIST P-256 key or point")]
    InvalidNist256p1,
    #[error("invalid scalar for curve operation")]
    InvalidScalar,
}

mod iecc;
mod ipoint;
mod iprivate_key;
mod ipublic_key;
mod kholaw;
mod slip10;

pub use iecc::IEllipticCurveCryptography;
pub use ipoint::IPoint;
pub use iprivate_key::IPrivateKey;
pub use ipublic_key::IPublicKey;

pub use kholaw::ed25519::{
    KholawEd25519Ecc, KholawEd25519Id, KholawEd25519Point, KholawEd25519PrivateKey, KholawEd25519PublicKey,
};
pub use slip10::ed25519::blake2b::{
    Slip10Ed25519Blake2bEcc, Slip10Ed25519Blake2bId, Slip10Ed25519Blake2bPoint, Slip10Ed25519Blake2bPrivateKey,
    Slip10Ed25519Blake2bPublicKey,
};
pub use slip10::ed25519::monero::{
    Slip10Ed25519MoneroEcc, Slip10Ed25519MoneroId, Slip10Ed25519MoneroPoint, Slip10Ed25519MoneroPrivateKey,
    Slip10Ed25519MoneroPublicKey,
};
pub use slip10::ed25519::{
    Ed25519Variant, Slip10Ed25519Ecc, Slip10Ed25519Id, Slip10Ed25519Point, Slip10Ed25519PrivateKey,
    Slip10Ed25519PrivateKeyStd, Slip10Ed25519PublicKey, Slip10Ed25519PublicKeyStd, Slip10Ed25519PointStd,
};
pub use slip10::nist256p1::{
    Slip10Nist256p1Ecc, Slip10Nist256p1Point, Slip10Nist256p1PrivateKey, Slip10Nist256p1PublicKey,
};
pub use slip10::secp256k1::{
    Slip10Secp256k1Ecc, Slip10Secp256k1Point, Slip10Secp256k1PrivateKey, Slip10Secp256k1PublicKey,
};


pub const ECC_NAMES: &[&str] = &[
    kholaw::ed25519::KholawEd25519Ecc::NAME,
    slip10::ed25519::Slip10Ed25519Ecc::NAME,
    slip10::ed25519::blake2b::Slip10Ed25519Blake2bEcc::NAME,
    slip10::ed25519::monero::Slip10Ed25519MoneroEcc::NAME,
    slip10::nist256p1::Slip10Nist256p1Ecc::NAME,
    slip10::secp256k1::Slip10Secp256k1Ecc::NAME,
];

pub fn display_ecc_registry_name(registry_key: &str) -> &str {
    match registry_key {
        "SLIP10Secp256k1ECC" => Slip10Secp256k1Ecc::NAME,
        "SLIP10Ed25519ECC" => Slip10Ed25519Ecc::NAME,
        "KholawEd25519ECC" => KholawEd25519Ecc::NAME,
        "SLIP10Ed25519Blake2bECC" => Slip10Ed25519Blake2bEcc::NAME,
        "SLIP10Ed25519MoneroECC" => Slip10Ed25519MoneroEcc::NAME,
        "SLIP10Nist256p1ECC" => Slip10Nist256p1Ecc::NAME,
        other => other,
    }
}

pub fn is_ecc(name: &str) -> bool {
    ECC_NAMES.iter().any(|&n| n == name)
}

pub fn ecc_name(name: &str) -> Result<&'static str, ECCError> {
    if !is_ecc(name) {
        return Err(ECCError::new(
            "Invalid ECC name",
            None,
            Some(&ECC_NAMES.join(", ")),
            Some(name),
        ));
    }
    Ok(match name {
        n if n == kholaw::ed25519::KholawEd25519Ecc::NAME => kholaw::ed25519::KholawEd25519Ecc::NAME,
        n if n == slip10::ed25519::Slip10Ed25519Ecc::NAME => slip10::ed25519::Slip10Ed25519Ecc::NAME,
        n if n == slip10::ed25519::blake2b::Slip10Ed25519Blake2bEcc::NAME => {
            slip10::ed25519::blake2b::Slip10Ed25519Blake2bEcc::NAME
        }
        n if n == slip10::ed25519::monero::Slip10Ed25519MoneroEcc::NAME => {
            slip10::ed25519::monero::Slip10Ed25519MoneroEcc::NAME
        }
        n if n == slip10::nist256p1::Slip10Nist256p1Ecc::NAME => slip10::nist256p1::Slip10Nist256p1Ecc::NAME,
        n if n == slip10::secp256k1::Slip10Secp256k1Ecc::NAME => slip10::secp256k1::Slip10Secp256k1Ecc::NAME,
        _ => unreachable!(),
    })
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Eccs;

impl Eccs {
    pub fn names() -> &'static [&'static str] {
        ECC_NAMES
    }

    pub fn is_ecc(name: &str) -> bool {
        is_ecc(name)
    }

    pub fn ecc(name: &str) -> Result<&'static str, ECCError> {
        ecc_name(name)
    }
}

pub enum PublicKeyInput<'a, P> {
    Bytes(&'a [u8]),
    Hex(&'a str),
    Instance(P),
}

pub fn validate_and_get_public_key<P: IPublicKey + Clone>(
    key: PublicKeyInput<'_, P>,
) -> Result<P, PublicKeyError> {
    match key {
        PublicKeyInput::Instance(p) => Ok(p),
        PublicKeyInput::Bytes(b) => P::from_bytes(b).map_err(|_| {
            PublicKeyError::new("Invalid public key data", None, None, None)
        }),
        PublicKeyInput::Hex(s) => {
            let bytes = utils::hex_to_bytes(s.trim().trim_start_matches("0x")).map_err(|_| {
                PublicKeyError::new("Invalid public key data", None, None, None)
            })?;
            P::from_bytes(&bytes).map_err(|_| {
                PublicKeyError::new("Invalid public key data", None, None, None)
            })
        }
    }
}

impl From<EccsError> for PublicKeyError {
    fn from(_: EccsError) -> Self {
        PublicKeyError::new("Invalid public key data", None, None, None)
    }
}
