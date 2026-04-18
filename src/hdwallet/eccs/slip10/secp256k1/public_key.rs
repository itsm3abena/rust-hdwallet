use bitcoin::secp256k1::PublicKey;

use crate::hdwallet::eccs::EccsError;
use crate::hdwallet::eccs::ipublic_key::IPublicKey;

use super::point::Slip10Secp256k1Point;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Slip10Secp256k1PublicKey(pub(crate) PublicKey);

impl Slip10Secp256k1PublicKey {
    pub fn inner(&self) -> &PublicKey {
        &self.0
    }

    pub fn into_inner(self) -> PublicKey {
        self.0
    }
}

impl IPublicKey for Slip10Secp256k1PublicKey {
    type Point = Slip10Secp256k1Point;

    fn ecc_name() -> &'static str {
        super::Slip10Secp256k1Ecc::NAME
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, EccsError> {
        PublicKey::from_slice(bytes)
            .map(Self)
            .map_err(|_| EccsError::InvalidSecp256k1Public)
    }

    fn from_point(point: &Slip10Secp256k1Point) -> Result<Self, EccsError> {
        Ok(Self(point.0))
    }

    fn raw_compressed(&self) -> Vec<u8> {
        self.0.serialize().to_vec()
    }

    fn raw_uncompressed(&self) -> Vec<u8> {
        self.0.serialize_uncompressed().to_vec()
    }

    fn point(&self) -> Slip10Secp256k1Point {
        Slip10Secp256k1Point(self.0)
    }

    fn compressed_len() -> usize {
        33
    }

    fn uncompressed_len() -> usize {
        65
    }
}
