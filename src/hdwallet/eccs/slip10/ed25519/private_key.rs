use std::borrow::Cow;
use std::marker::PhantomData;

use crate::hdwallet::libs::ed25519::{scalar_is_valid, COORD_BYTE_LEN};

use crate::hdwallet::eccs::EccsError;
use crate::hdwallet::eccs::iprivate_key::IPrivateKey;

use super::public_key::Slip10Ed25519PublicKey;
use super::variant::Ed25519Variant;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Slip10Ed25519PrivateKey<V: Ed25519Variant> {
    pub(crate) secret: [u8; COORD_BYTE_LEN],
    pub(crate) _v: PhantomData<V>,
}

impl<V: Ed25519Variant> Slip10Ed25519PrivateKey<V> {
    pub fn public_key(&self) -> Slip10Ed25519PublicKey<V> {
        let enc = crate::hdwallet::libs::ed25519::public_key_from_hash_digest(&V::seed_hash(&self.secret));
        Slip10Ed25519PublicKey {
            enc,
            _v: PhantomData,
        }
    }
}

impl<V: Ed25519Variant> IPrivateKey for Slip10Ed25519PrivateKey<V> {
    fn ecc_name() -> &'static str {
        V::NAME
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, EccsError> {
        if bytes.len() != COORD_BYTE_LEN {
            return Err(EccsError::InvalidEd25519);
        }
        let arr: [u8; COORD_BYTE_LEN] = bytes.try_into().map_err(|_| EccsError::InvalidEd25519)?;
        let v = crate::hdwallet::libs::ed25519::int_decode(&arr);
        if !scalar_is_valid(&v) {
            return Err(EccsError::InvalidEd25519);
        }
        Ok(Self {
            secret: arr,
            _v: PhantomData,
        })
    }

    fn raw_bytes(&self) -> Cow<'_, [u8]> {
        Cow::Borrowed(&self.secret)
    }

    fn byte_len() -> usize {
        COORD_BYTE_LEN
    }
}
