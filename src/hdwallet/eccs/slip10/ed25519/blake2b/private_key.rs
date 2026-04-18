use crate::hdwallet::eccs::slip10::ed25519::Slip10Ed25519PrivateKey;

use super::id::Slip10Ed25519Blake2bId;

pub type Slip10Ed25519Blake2bPrivateKey = Slip10Ed25519PrivateKey<Slip10Ed25519Blake2bId>;
