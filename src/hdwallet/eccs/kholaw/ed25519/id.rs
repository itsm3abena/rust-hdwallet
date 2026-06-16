use num_bigint::BigUint;

use crate::hdwallet::libs::ed25519::COORD_BYTE_LEN;
use crate::hdwallet::eccs::iecc::IEllipticCurveCryptography;
use crate::hdwallet::eccs::slip10::ed25519::Ed25519Variant;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct KholawEd25519Id;

impl Ed25519Variant for KholawEd25519Id {
    const NAME: &'static str = "Kholaw-Ed25519";

    fn seed_hash(seed: &[u8; COORD_BYTE_LEN]) -> Vec<u8> {
        crate::hdwallet::utils::sha512(seed)
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct KholawEd25519Ecc;

impl KholawEd25519Ecc {
    pub const NAME: &'static str = KholawEd25519Id::NAME;

    pub fn order() -> BigUint {
        crate::hdwallet::libs::ed25519::l().clone()
    }
}

impl IEllipticCurveCryptography for KholawEd25519Ecc {
    const NAME: &'static str = KholawEd25519Id::NAME;

    fn order() -> BigUint {
        KholawEd25519Ecc::order()
    }
}
