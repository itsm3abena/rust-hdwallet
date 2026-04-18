use crate::hdwallet::eccs::slip10::ed25519::Slip10Ed25519PublicKey;

use super::id::KholawEd25519Id;

pub type KholawEd25519PublicKey = Slip10Ed25519PublicKey<KholawEd25519Id>;
