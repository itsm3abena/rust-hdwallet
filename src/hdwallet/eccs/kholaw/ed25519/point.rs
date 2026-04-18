use crate::hdwallet::eccs::slip10::ed25519::Slip10Ed25519Point;

use super::id::KholawEd25519Id;

pub type KholawEd25519Point = Slip10Ed25519Point<KholawEd25519Id>;
