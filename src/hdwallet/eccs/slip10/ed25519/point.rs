use std::marker::PhantomData;

use num_bigint::BigUint;

use crate::hdwallet::libs::ed25519::{
    int_encode, point_add, point_bytes_to_coord, point_coord_to_bytes, point_is_generator_bytes,
    point_is_on_curve_bytes, point_scalar_mul, point_scalar_mul_base, PointRef, Ed25519Error,
    COORD_BYTE_LEN,
};

use crate::hdwallet::eccs::EccsError;
use crate::hdwallet::eccs::ipoint::IPoint;

use super::variant::Ed25519Variant;

fn map_ed(e: Ed25519Error) -> EccsError {
    match e {
        Ed25519Error::NotOnCurve | Ed25519Error::InvalidCompressedPoint => EccsError::InvalidEd25519,
        _ => EccsError::InvalidEd25519,
    }
}

pub(crate) fn ed25519_compressed_from_xy(xy: &[u8; COORD_BYTE_LEN * 2]) -> [u8; COORD_BYTE_LEN] {
    use curve25519_dalek::edwards::CompressedEdwardsY;
    let mut yb: [u8; COORD_BYTE_LEN] = xy[COORD_BYTE_LEN..].try_into().unwrap();
    if xy[0] & 1 != 0 {
        yb[COORD_BYTE_LEN - 1] |= 0x80;
    }
    CompressedEdwardsY(yb)
        .decompress()
        .expect("on-curve checked")
        .compress()
        .to_bytes()
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Slip10Ed25519Point<V: Ed25519Variant> {
    pub(crate) enc: [u8; COORD_BYTE_LEN],
    pub(crate) _v: PhantomData<V>,
}

impl<V: Ed25519Variant> Slip10Ed25519Point<V> {
    pub fn encoded(&self) -> &[u8; COORD_BYTE_LEN] {
        &self.enc
    }

    fn is_generator(&self) -> bool {
        point_is_generator_bytes(self.enc.as_slice()).unwrap_or(false)
    }
}

impl<V: Ed25519Variant> IPoint for Slip10Ed25519Point<V> {
    fn ecc_name() -> &'static str {
        V::NAME
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, EccsError> {
        if !point_is_on_curve_bytes(bytes).map_err(map_ed)? {
            return Err(EccsError::InvalidEd25519);
        }
        let enc: [u8; COORD_BYTE_LEN] = if bytes.len() == COORD_BYTE_LEN * 2 {
            let b: [u8; COORD_BYTE_LEN * 2] = bytes.try_into().map_err(|_| EccsError::InvalidEd25519)?;
            ed25519_compressed_from_xy(&b)
        } else {
            bytes.try_into().map_err(|_| EccsError::InvalidEd25519)?
        };
        Ok(Self {
            enc,
            _v: PhantomData,
        })
    }

    fn from_coordinates(x: &BigUint, y: &BigUint) -> Result<Self, EccsError> {
        let xy = point_coord_to_bytes(x, y);
        Self::from_bytes(&xy)
    }

    fn x(&self) -> BigUint {
        point_bytes_to_coord(self.enc.as_slice())
            .map_err(map_ed)
            .expect("validated in from_bytes")
            .0
    }

    fn y(&self) -> BigUint {
        point_bytes_to_coord(self.enc.as_slice())
            .map_err(map_ed)
            .expect("validated in from_bytes")
            .1
    }

    fn raw_encoded(&self) -> Vec<u8> {
        self.enc.to_vec()
    }

    fn raw_decoded(&self) -> Vec<u8> {
        let x = self.x();
        let y = self.y();
        [int_encode(&x).as_slice(), int_encode(&y).as_slice()].concat()
    }

    fn add(&self, other: &Self) -> Result<Self, EccsError> {
        let out = point_add(
            PointRef::Encoded(&self.enc),
            PointRef::Encoded(&other.enc),
        )
        .map_err(map_ed)?;
        Ok(Self {
            enc: out,
            _v: PhantomData,
        })
    }

    fn mul_scalar(&self, scalar: &BigUint) -> Result<Self, EccsError> {
        let sb = crate::hdwallet::libs::ed25519::scalar_reduce_from_int(scalar);
        let enc = if self.is_generator() {
            point_scalar_mul_base(&sb)
        } else {
            point_scalar_mul(&sb, PointRef::Encoded(&self.enc)).map_err(map_ed)?
        };
        Ok(Self {
            enc,
            _v: PhantomData,
        })
    }
}
