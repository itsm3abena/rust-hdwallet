use num_bigint::BigUint;
use num_traits::Zero;
use p256::elliptic_curve::bigint::U256;
use p256::elliptic_curve::ops::Reduce;
use p256::elliptic_curve::sec1::{FromEncodedPoint, ToEncodedPoint};
use p256::elliptic_curve::Field;
use p256::{AffinePoint, EncodedPoint as P256EncodedPoint, FieldBytes, ProjectivePoint, Scalar};

use crate::hdwallet::eccs::EccsError;
use crate::hdwallet::eccs::ipoint::IPoint;

fn be32(n: &BigUint) -> [u8; 32] {
    let mut out = [0u8; 32];
    let v = n.to_bytes_be();
    let start = 32usize.saturating_sub(v.len());
    out[start..].copy_from_slice(&v);
    out
}

fn field_bytes(n: &BigUint) -> Result<FieldBytes, EccsError> {
    Ok(FieldBytes::from(be32(n)))
}

pub(crate) fn affine_xy_uncompressed(aff: &AffinePoint) -> (BigUint, BigUint) {
    let enc = aff.to_encoded_point(false);
    let s = enc.as_bytes();
    let x = BigUint::from_bytes_be(&s[1..33]);
    let y = BigUint::from_bytes_be(&s[33..65]);
    (x, y)
}

fn nist_scalar(n: &BigUint) -> Result<Scalar, EccsError> {
    let order = super::Slip10Nist256p1Ecc::order();
    let m = n % &order;
    if m.is_zero() {
        return Err(EccsError::InvalidScalar);
    }
    let uint = U256::from_be_slice(&be32(&m));
    let s = Scalar::reduce(uint);
    if bool::from(s.is_zero()) {
        return Err(EccsError::InvalidScalar);
    }
    Ok(s)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Slip10Nist256p1Point(pub(crate) ProjectivePoint);

impl Slip10Nist256p1Point {
    pub fn generator() -> Self {
        Self(ProjectivePoint::GENERATOR)
    }

    pub fn inner(&self) -> &ProjectivePoint {
        &self.0
    }

    pub fn into_inner(self) -> ProjectivePoint {
        self.0
    }
}

impl IPoint for Slip10Nist256p1Point {
    fn ecc_name() -> &'static str {
        super::Slip10Nist256p1Ecc::NAME
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, EccsError> {
        let pt = match bytes.len() {
            33 | 65 => {
                let enc = P256EncodedPoint::from_bytes(bytes).map_err(|_| EccsError::InvalidNist256p1)?;
                let affine =
                    Option::<AffinePoint>::from(AffinePoint::from_encoded_point(&enc)).ok_or(EccsError::InvalidNist256p1)?;
                ProjectivePoint::from(affine)
            }
            64 => {
                let x = field_bytes(&BigUint::from_bytes_be(&bytes[..32]))?;
                let y = field_bytes(&BigUint::from_bytes_be(&bytes[32..]))?;
                let enc = P256EncodedPoint::from_affine_coordinates(&x, &y, false);
                let affine =
                    Option::<AffinePoint>::from(AffinePoint::from_encoded_point(&enc)).ok_or(EccsError::InvalidNist256p1)?;
                ProjectivePoint::from(affine)
            }
            _ => return Err(EccsError::InvalidNist256p1),
        };
        Ok(Self(pt))
    }

    fn from_coordinates(x: &BigUint, y: &BigUint) -> Result<Self, EccsError> {
        let xb = field_bytes(x)?;
        let yb = field_bytes(y)?;
        let enc = P256EncodedPoint::from_affine_coordinates(&xb, &yb, false);
        let affine =
            Option::<AffinePoint>::from(AffinePoint::from_encoded_point(&enc)).ok_or(EccsError::InvalidNist256p1)?;
        Ok(Self(ProjectivePoint::from(affine)))
    }

    fn x(&self) -> BigUint {
        affine_xy_uncompressed(&self.0.to_affine()).0
    }

    fn y(&self) -> BigUint {
        affine_xy_uncompressed(&self.0.to_affine()).1
    }

    fn raw_encoded(&self) -> Vec<u8> {
        self.0.to_affine().to_encoded_point(true).as_bytes().to_vec()
    }

    fn raw_decoded(&self) -> Vec<u8> {
        self.0.to_affine().to_encoded_point(false).as_bytes()[1..].to_vec()
    }

    fn add(&self, other: &Self) -> Result<Self, EccsError> {
        Ok(Self(self.0 + other.0))
    }

    fn mul_scalar(&self, scalar: &BigUint) -> Result<Self, EccsError> {
        let s = nist_scalar(scalar)?;
        Ok(Self(self.0 * s))
    }
}

#[cfg(test)]
mod tests {
    use p256::elliptic_curve::Group;

    use super::*;

    #[test]
    fn generator_double_not_identity() {
        let g = Slip10Nist256p1Point::generator();
        let g2 = g.add(&g).unwrap();
        assert!(!bool::from(g2.0.is_identity()));
    }
}
