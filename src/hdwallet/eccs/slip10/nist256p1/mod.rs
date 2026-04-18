mod point;
mod private_key;
mod public_key;

use num_bigint::BigUint;

pub use point::Slip10Nist256p1Point;
pub use private_key::Slip10Nist256p1PrivateKey;
pub use public_key::Slip10Nist256p1PublicKey;

use crate::hdwallet::eccs::iecc::IEllipticCurveCryptography;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Slip10Nist256p1Ecc;

const NIST_P256_ORDER_BE: [u8; 32] = [
    0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xbc, 0xe6, 0xfa, 0xad, 0xa7, 0x17, 0x9e, 0x84, 0xf3, 0xb9, 0xca, 0xc2, 0xfc, 0x63, 0x25, 0x51,
];

impl Slip10Nist256p1Ecc {
    pub const NAME: &'static str = "SLIP10-Nist256p1";

    pub fn order() -> BigUint {
        BigUint::from_bytes_be(&NIST_P256_ORDER_BE)
    }
}

impl IEllipticCurveCryptography for Slip10Nist256p1Ecc {
    const NAME: &'static str = "SLIP10-Nist256p1";

    fn order() -> BigUint {
        Self::order()
    }
}
