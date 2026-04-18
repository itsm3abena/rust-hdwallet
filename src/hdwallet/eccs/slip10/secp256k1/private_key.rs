use std::borrow::Cow;

use bitcoin::secp256k1::{PublicKey, SecretKey};

use crate::hdwallet::eccs::EccsError;
use crate::hdwallet::eccs::iprivate_key::IPrivateKey;

use super::point::secp;
use super::public_key::Slip10Secp256k1PublicKey;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Slip10Secp256k1PrivateKey(pub(crate) SecretKey);

impl Slip10Secp256k1PrivateKey {
    pub fn inner(&self) -> &SecretKey {
        &self.0
    }

    pub fn into_inner(self) -> SecretKey {
        self.0
    }

    pub fn public_key(&self) -> Slip10Secp256k1PublicKey {
        Slip10Secp256k1PublicKey(PublicKey::from_secret_key(secp(), &self.0))
    }
}

impl IPrivateKey for Slip10Secp256k1PrivateKey {
    fn ecc_name() -> &'static str {
        super::Slip10Secp256k1Ecc::NAME
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, EccsError> {
        if bytes.len() != 32 {
            return Err(EccsError::InvalidSecp256k1Secret);
        }
        let sk = SecretKey::from_slice(bytes).map_err(|_| EccsError::InvalidSecp256k1Secret)?;
        Ok(Self(sk))
    }

    fn raw_bytes(&self) -> Cow<'_, [u8]> {
        Cow::Owned(self.0.secret_bytes().to_vec())
    }

    fn byte_len() -> usize {
        32
    }
}

#[cfg(test)]
mod tests {
    use crate::hdwallet::eccs::ipublic_key::IPublicKey;

    use super::*;

    #[test]
    fn priv_to_pubkey_roundtrip() {
        let mut kb = [0u8; 32];
        kb[31] = 1;
        let sk = Slip10Secp256k1PrivateKey::from_bytes(&kb).unwrap();
        let pk = sk.public_key();
        assert_eq!(pk.raw_compressed().len(), 33);
    }
}
