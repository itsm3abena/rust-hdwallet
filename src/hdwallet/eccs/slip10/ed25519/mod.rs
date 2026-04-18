pub mod blake2b;
pub mod monero;

mod point;
mod private_key;
mod public_key;
pub mod variant;

use num_bigint::BigUint;

pub use point::Slip10Ed25519Point;
pub use private_key::Slip10Ed25519PrivateKey;
pub use public_key::Slip10Ed25519PublicKey;
pub use variant::{Ed25519Variant, Slip10Ed25519Id};

use crate::hdwallet::eccs::iecc::IEllipticCurveCryptography;

pub type Slip10Ed25519PointStd = Slip10Ed25519Point<Slip10Ed25519Id>;
pub type Slip10Ed25519PublicKeyStd = Slip10Ed25519PublicKey<Slip10Ed25519Id>;
pub type Slip10Ed25519PrivateKeyStd = Slip10Ed25519PrivateKey<Slip10Ed25519Id>;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Slip10Ed25519Ecc;

impl Slip10Ed25519Ecc {
    pub const NAME: &'static str = Slip10Ed25519Id::NAME;

    pub fn order() -> BigUint {
        crate::hdwallet::libs::ed25519::l().clone()
    }
}

impl IEllipticCurveCryptography for Slip10Ed25519Ecc {
    const NAME: &'static str = Slip10Ed25519Id::NAME;

    fn order() -> BigUint {
        Slip10Ed25519Ecc::order()
    }
}

#[cfg(test)]
mod tests {
    use crate::hdwallet::eccs::ipoint::IPoint;

    use super::*;

    #[test]
    fn slip10_ed25519_g_times_one() {
        let g = Slip10Ed25519Point::<Slip10Ed25519Id>::from_bytes(
            crate::hdwallet::libs::ed25519::G_ENC_BYTES.as_slice(),
        )
        .unwrap();
        let g2 = g.mul_scalar(&BigUint::from(1u32)).unwrap();
        assert_eq!(g2.enc, g.enc);
    }
}
