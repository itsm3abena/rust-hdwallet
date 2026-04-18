use std::borrow::Cow;

use p256::ecdsa::SigningKey;

use crate::hdwallet::eccs::EccsError;
use crate::hdwallet::eccs::iprivate_key::IPrivateKey;

use super::public_key::Slip10Nist256p1PublicKey;

/// NIST P-256 32-byte secret scalar.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Slip10Nist256p1PrivateKey(pub(crate) SigningKey);

impl Slip10Nist256p1PrivateKey {
    pub fn inner(&self) -> &SigningKey {
        &self.0
    }

    pub fn into_inner(self) -> SigningKey {
        self.0
    }

    pub fn public_key(&self) -> Slip10Nist256p1PublicKey {
        Slip10Nist256p1PublicKey(*self.0.verifying_key())
    }
}

impl IPrivateKey for Slip10Nist256p1PrivateKey {
    fn ecc_name() -> &'static str {
        super::Slip10Nist256p1Ecc::NAME
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, EccsError> {
        SigningKey::from_slice(bytes)
            .map(Self)
            .map_err(|_| EccsError::InvalidNist256p1)
    }

    fn raw_bytes(&self) -> Cow<'_, [u8]> {
        Cow::Owned(self.0.to_bytes().to_vec())
    }

    fn byte_len() -> usize {
        32
    }
}
