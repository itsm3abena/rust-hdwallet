use crate::hdwallet::libs::ed25519::COORD_BYTE_LEN;

pub trait Ed25519Variant: Copy + 'static {
    const NAME: &'static str;

    fn seed_hash(seed: &[u8; COORD_BYTE_LEN]) -> Vec<u8>;

    fn public_key_prefix() -> Option<u8> {
        Some(0x00)
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Slip10Ed25519Id;

impl Ed25519Variant for Slip10Ed25519Id {
    const NAME: &'static str = "SLIP10-Ed25519";

    fn seed_hash(seed: &[u8; COORD_BYTE_LEN]) -> Vec<u8> {
        crate::hdwallet::utils::sha512(seed)
    }
}
