use std::marker::PhantomData;

use crate::hdwallet::libs::ed25519::{point_is_on_curve_bytes, Ed25519Error, COORD_BYTE_LEN};

use crate::hdwallet::eccs::EccsError;
use crate::hdwallet::eccs::ipublic_key::IPublicKey;

use super::point::Slip10Ed25519Point;
use super::variant::Ed25519Variant;

fn map_ed(e: Ed25519Error) -> EccsError {
    match e {
        Ed25519Error::NotOnCurve | Ed25519Error::InvalidCompressedPoint => EccsError::InvalidEd25519,
        _ => EccsError::InvalidEd25519,
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Slip10Ed25519PublicKey<V: Ed25519Variant> {
    pub(crate) enc: [u8; COORD_BYTE_LEN],
    pub(crate) _v: PhantomData<V>,
}

impl<V: Ed25519Variant> Slip10Ed25519PublicKey<V> {
    pub const PREFIX: u8 = 0x00;

    pub fn encoded_point(&self) -> &[u8; COORD_BYTE_LEN] {
        &self.enc
    }
}

impl<V: Ed25519Variant> IPublicKey for Slip10Ed25519PublicKey<V> {
    type Point = Slip10Ed25519Point<V>;

    fn ecc_name() -> &'static str {
        V::NAME
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, EccsError> {
        let body = if bytes.len() == COORD_BYTE_LEN + 1 && bytes[0] == Self::PREFIX {
            &bytes[1..]
        } else {
            bytes
        };
        if body.len() != COORD_BYTE_LEN || !point_is_on_curve_bytes(body).map_err(map_ed)? {
            return Err(EccsError::InvalidEd25519);
        }
        Ok(Self {
            enc: body.try_into().map_err(|_| EccsError::InvalidEd25519)?,
            _v: PhantomData,
        })
    }

    fn from_point(point: &Slip10Ed25519Point<V>) -> Result<Self, EccsError> {
        Ok(Self {
            enc: point.enc,
            _v: PhantomData,
        })
    }

    fn raw_compressed(&self) -> Vec<u8> {
        let mut v = Vec::with_capacity(1 + COORD_BYTE_LEN);
        v.push(Self::PREFIX);
        v.extend_from_slice(&self.enc);
        v
    }

    fn raw_uncompressed(&self) -> Vec<u8> {
        self.raw_compressed()
    }

    fn point(&self) -> Slip10Ed25519Point<V> {
        Slip10Ed25519Point {
            enc: self.enc,
            _v: PhantomData,
        }
    }

    fn compressed_len() -> usize {
        1 + COORD_BYTE_LEN
    }

    fn uncompressed_len() -> usize {
        1 + COORD_BYTE_LEN
    }
}
