mod point;
mod private_key;
mod public_key;

use num_bigint::BigUint;

pub use point::Slip10Secp256k1Point;
pub use private_key::Slip10Secp256k1PrivateKey;
pub use public_key::Slip10Secp256k1PublicKey;

use crate::hdwallet::eccs::iecc::IEllipticCurveCryptography;
use crate::hdwallet::libs::ecc;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Slip10Secp256k1Ecc;

impl Slip10Secp256k1Ecc {
    pub const NAME: &'static str = "SLIP10-Secp256k1";
}

impl IEllipticCurveCryptography for Slip10Secp256k1Ecc {
    const NAME: &'static str = "SLIP10-Secp256k1";

    fn order() -> BigUint {
        ecc::n().clone()
    }
}
