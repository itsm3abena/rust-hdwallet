use num_bigint::BigUint;

use super::EccsError;

pub trait IPoint: Sized + Clone {
    fn ecc_name() -> &'static str;
    fn from_bytes(bytes: &[u8]) -> Result<Self, EccsError>;
    fn from_coordinates(x: &BigUint, y: &BigUint) -> Result<Self, EccsError>;
    fn x(&self) -> BigUint;
    fn y(&self) -> BigUint;
    fn raw_encoded(&self) -> Vec<u8>;
    fn raw_decoded(&self) -> Vec<u8>;
    fn add(&self, other: &Self) -> Result<Self, EccsError>;
    fn mul_scalar(&self, scalar: &BigUint) -> Result<Self, EccsError>;
}
