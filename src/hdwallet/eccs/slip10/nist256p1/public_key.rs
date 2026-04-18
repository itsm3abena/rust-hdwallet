use p256::ecdsa::VerifyingKey;
use p256::elliptic_curve::sec1::ToEncodedPoint;

use crate::hdwallet::eccs::EccsError;
use crate::hdwallet::eccs::ipublic_key::IPublicKey;

use super::point::Slip10Nist256p1Point;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Slip10Nist256p1PublicKey(pub(crate) VerifyingKey);

impl Slip10Nist256p1PublicKey {
    pub fn inner(&self) -> &VerifyingKey {
        &self.0
    }

    pub fn into_inner(self) -> VerifyingKey {
        self.0
    }
}

impl IPublicKey for Slip10Nist256p1PublicKey {
    type Point = Slip10Nist256p1Point;

    fn ecc_name() -> &'static str {
        super::Slip10Nist256p1Ecc::NAME
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, EccsError> {
        VerifyingKey::from_sec1_bytes(bytes)
            .map(Self)
            .map_err(|_| EccsError::InvalidNist256p1)
    }

    fn from_point(point: &Slip10Nist256p1Point) -> Result<Self, EccsError> {
        let aff = point.0.to_affine();
        Self::from_bytes(aff.to_encoded_point(false).as_bytes())
    }

    fn raw_compressed(&self) -> Vec<u8> {
        self.0.to_encoded_point(true).as_bytes().to_vec()
    }

    fn raw_uncompressed(&self) -> Vec<u8> {
        self.0.to_encoded_point(false).as_bytes().to_vec()
    }

    fn point(&self) -> Slip10Nist256p1Point {
        let aff = *self.0.as_affine();
        Slip10Nist256p1Point(p256::ProjectivePoint::from(aff))
    }

    fn compressed_len() -> usize {
        33
    }

    fn uncompressed_len() -> usize {
        65
    }
}
