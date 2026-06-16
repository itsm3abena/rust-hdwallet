use num_bigint::BigUint;

use crate::hdwallet::eccs::iecc::IEllipticCurveCryptography;
use crate::hdwallet::libs::ed25519::COORD_BYTE_LEN;
use crate::hdwallet::eccs::slip10::ed25519::Ed25519Variant;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Slip10Ed25519Blake2bId;

impl Ed25519Variant for Slip10Ed25519Blake2bId {
    const NAME: &'static str = "SLIP10-Ed25519-Blake2b";

    fn seed_hash(seed: &[u8; COORD_BYTE_LEN]) -> Vec<u8> {
        crate::hdwallet::utils::blake2b_512(seed).to_vec()
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Slip10Ed25519Blake2bEcc;

impl Slip10Ed25519Blake2bEcc {
    pub const NAME: &'static str = Slip10Ed25519Blake2bId::NAME;
}

impl IEllipticCurveCryptography for Slip10Ed25519Blake2bEcc {
    const NAME: &'static str = Slip10Ed25519Blake2bId::NAME;

    fn order() -> BigUint {
        crate::hdwallet::libs::ed25519::l().clone()
    }
}
