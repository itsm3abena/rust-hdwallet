use num_bigint::BigUint;

use crate::hdwallet::eccs::iecc::IEllipticCurveCryptography;
use crate::hdwallet::libs::ed25519::COORD_BYTE_LEN;
use crate::hdwallet::eccs::slip10::ed25519::Ed25519Variant;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Slip10Ed25519MoneroId;

impl Ed25519Variant for Slip10Ed25519MoneroId {
    const NAME: &'static str = "SLIP10-Ed25519-Monero";

    fn seed_hash(seed: &[u8; COORD_BYTE_LEN]) -> Vec<u8> {
        crate::hdwallet::utils::sha512(seed)
    }

    fn public_key_prefix() -> Option<u8> {
        None
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Slip10Ed25519MoneroEcc;

impl Slip10Ed25519MoneroEcc {
    pub const NAME: &'static str = Slip10Ed25519MoneroId::NAME;
}

impl IEllipticCurveCryptography for Slip10Ed25519MoneroEcc {
    const NAME: &'static str = Slip10Ed25519MoneroId::NAME;

    fn order() -> BigUint {
        crate::hdwallet::libs::ed25519::l().clone()
    }
}
