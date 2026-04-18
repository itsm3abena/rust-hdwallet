use std::sync::OnceLock;

use bitcoin::secp256k1::{All, PublicKey, Scalar, Secp256k1};
use num_bigint::BigUint;
use num_traits::Zero;

use crate::hdwallet::libs::ecc;

use crate::hdwallet::eccs::EccsError;
use crate::hdwallet::eccs::ipoint::IPoint;

static SECP256K1: OnceLock<Secp256k1<All>> = OnceLock::new();

pub(crate) fn secp() -> &'static Secp256k1<All> {
    SECP256K1.get_or_init(Secp256k1::new)
}

fn scalar_be32(n: &BigUint) -> [u8; 32] {
    let mut out = [0u8; 32];
    let v = n.to_bytes_be();
    let start = 32usize.saturating_sub(v.len());
    out[start..].copy_from_slice(&v);
    out
}

pub(crate) fn scalar_from_bn(scalar: &BigUint) -> Result<Scalar, EccsError> {
    let m = scalar % ecc::n();
    if m.is_zero() {
        return Err(EccsError::InvalidScalar);
    }
    Scalar::from_be_bytes(scalar_be32(&m)).map_err(|_| EccsError::InvalidScalar)
}

fn pubkey_xy(pk: &PublicKey) -> (BigUint, BigUint) {
    let u = pk.serialize_uncompressed();
    debug_assert_eq!(u[0], 0x04);
    let x = BigUint::from_bytes_be(&u[1..33]);
    let y = BigUint::from_bytes_be(&u[33..65]);
    (x, y)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Slip10Secp256k1Point(pub(crate) PublicKey);

impl Slip10Secp256k1Point {
    pub const G_COMPRESSED: [u8; 33] = [
        0x02, 0x79, 0xbe, 0x66, 0x7e, 0xf9, 0xdc, 0xbb, 0xac, 0x55, 0xa0, 0x62, 0x95, 0xce, 0x87,
        0x0b, 0x07, 0x02, 0x9b, 0xfc, 0xdb, 0x2d, 0xce, 0x28, 0xd9, 0x59, 0xf2, 0x81, 0x5b, 0x16,
        0xf8, 0x17, 0x98,
    ];

    pub fn generator() -> Self {
        Self(
            PublicKey::from_slice(&Self::G_COMPRESSED).expect("generator is valid compressed pubkey"),
        )
    }

    pub fn inner(&self) -> &PublicKey {
        &self.0
    }

    pub fn into_inner(self) -> PublicKey {
        self.0
    }
}

impl IPoint for Slip10Secp256k1Point {
    fn ecc_name() -> &'static str {
        super::Slip10Secp256k1Ecc::NAME
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, EccsError> {
        let pk = match bytes.len() {
            33 | 65 => PublicKey::from_slice(bytes).map_err(|_| EccsError::InvalidSecp256k1Public)?,
            64 => {
                let mut buf = [0u8; 65];
                buf[0] = 0x04;
                buf[1..].copy_from_slice(bytes);
                PublicKey::from_slice(&buf).map_err(|_| EccsError::InvalidSecp256k1Public)?
            }
            _ => return Err(EccsError::InvalidSecp256k1Public),
        };
        Ok(Self(pk))
    }

    fn from_coordinates(x: &BigUint, y: &BigUint) -> Result<Self, EccsError> {
        let xb = scalar_be32(x);
        let yb = scalar_be32(y);
        let mut buf = [0u8; 65];
        buf[0] = 0x04;
        buf[1..33].copy_from_slice(&xb);
        buf[33..65].copy_from_slice(&yb);
        PublicKey::from_slice(&buf)
            .map(Self)
            .map_err(|_| EccsError::InvalidSecp256k1Public)
    }

    fn x(&self) -> BigUint {
        pubkey_xy(&self.0).0
    }

    fn y(&self) -> BigUint {
        pubkey_xy(&self.0).1
    }

    fn raw_encoded(&self) -> Vec<u8> {
        self.0.serialize().to_vec()
    }

    fn raw_decoded(&self) -> Vec<u8> {
        self.0.serialize_uncompressed()[1..].to_vec()
    }

    fn add(&self, other: &Self) -> Result<Self, EccsError> {
        self.0
            .combine(&other.0)
            .map(Self)
            .map_err(|_| EccsError::InvalidSecp256k1Public)
    }

    fn mul_scalar(&self, scalar: &BigUint) -> Result<Self, EccsError> {
        let mut pk = self.0;
        let sc = scalar_from_bn(scalar)?;
        pk = pk
            .mul_tweak(secp(), &sc)
            .map_err(|_| EccsError::InvalidSecp256k1Public)?;
        Ok(Self(pk))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generator_mul_one_is_g() {
        let g = Slip10Secp256k1Point::generator();
        let one = BigUint::from(1u32);
        let gm = g.mul_scalar(&one).unwrap();
        assert_eq!(gm.raw_encoded(), g.raw_encoded());
    }
}
