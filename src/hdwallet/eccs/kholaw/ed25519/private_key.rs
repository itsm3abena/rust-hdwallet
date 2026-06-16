use std::borrow::Cow;

use crate::hdwallet::libs::ed25519::{point_scalar_mul_base, COORD_BYTE_LEN};

use crate::hdwallet::eccs::EccsError;
use crate::hdwallet::eccs::iprivate_key::IPrivateKey;
use crate::hdwallet::eccs::slip10::ed25519::{
    Ed25519Variant, Slip10Ed25519Id, Slip10Ed25519PrivateKey, Slip10Ed25519PrivateKeyStd,
};

use super::id::KholawEd25519Id;
use super::public_key::KholawEd25519PublicKey;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KholawEd25519PrivateKey {
    signing: [u8; COORD_BYTE_LEN],
    chain: [u8; COORD_BYTE_LEN],
}

impl IPrivateKey for KholawEd25519PrivateKey {
    fn ecc_name() -> &'static str {
        KholawEd25519Id::NAME
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, EccsError> {
        if bytes.len() != COORD_BYTE_LEN * 2 {
            return Err(EccsError::InvalidEd25519);
        }
        let signing: [u8; COORD_BYTE_LEN] = bytes[..COORD_BYTE_LEN].try_into().unwrap();
        let chain: [u8; COORD_BYTE_LEN] = bytes[COORD_BYTE_LEN..].try_into().unwrap();
        Ok(Self { signing, chain })
    }

    fn raw_bytes(&self) -> Cow<'_, [u8]> {
        let mut v = Vec::with_capacity(64);
        v.extend_from_slice(&self.signing);
        v.extend_from_slice(&self.chain);
        Cow::Owned(v)
    }

    fn byte_len() -> usize {
        COORD_BYTE_LEN * 2
    }
}

impl KholawEd25519PrivateKey {
    pub fn from_signing_and_extended(signing: [u8; COORD_BYTE_LEN], chain: [u8; COORD_BYTE_LEN]) -> Self {
        Self { signing, chain }
    }

    pub fn slip10_signing_key(&self) -> Slip10Ed25519PrivateKeyStd {
        Slip10Ed25519PrivateKey {
            secret: self.signing,
            _v: std::marker::PhantomData::<Slip10Ed25519Id>,
        }
    }

    pub fn public_key(&self) -> KholawEd25519PublicKey {
        let enc = point_scalar_mul_base(&self.signing);
        KholawEd25519PublicKey {
            enc,
            _v: std::marker::PhantomData::<KholawEd25519Id>,
        }
    }

    pub fn chain_code(&self) -> &[u8; COORD_BYTE_LEN] {
        &self.chain
    }
}
