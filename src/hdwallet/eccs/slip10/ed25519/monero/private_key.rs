use std::borrow::Cow;
use std::marker::PhantomData;

use crate::hdwallet::eccs::slip10::ed25519::variant::Ed25519Variant;
use crate::hdwallet::eccs::EccsError;
use crate::hdwallet::eccs::iprivate_key::IPrivateKey;
use crate::hdwallet::libs::ed25519::{point_scalar_mul_base, scalar_is_valid, COORD_BYTE_LEN};

use super::id::Slip10Ed25519MoneroId;
use super::public_key::Slip10Ed25519MoneroPublicKey;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Slip10Ed25519MoneroPrivateKey {
    secret: [u8; COORD_BYTE_LEN],
}

impl Slip10Ed25519MoneroPrivateKey {
    pub fn public_key(&self) -> Slip10Ed25519MoneroPublicKey {
        let enc = point_scalar_mul_base(&self.secret);
        Slip10Ed25519MoneroPublicKey {
            enc,
            _v: PhantomData::<Slip10Ed25519MoneroId>,
        }
    }
}

impl IPrivateKey for Slip10Ed25519MoneroPrivateKey {
    fn ecc_name() -> &'static str {
        Slip10Ed25519MoneroId::NAME
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, EccsError> {
        if bytes.len() != COORD_BYTE_LEN {
            return Err(EccsError::InvalidEd25519);
        }
        let secret: [u8; COORD_BYTE_LEN] = bytes.try_into().map_err(|_| EccsError::InvalidEd25519)?;
        let scalar = crate::hdwallet::libs::ed25519::int_decode(&secret);
        if !scalar_is_valid(&scalar) {
            return Err(EccsError::InvalidEd25519);
        }
        Ok(Self { secret })
    }

    fn raw_bytes(&self) -> Cow<'_, [u8]> {
        Cow::Borrowed(&self.secret)
    }

    fn byte_len() -> usize {
        COORD_BYTE_LEN
    }
}
