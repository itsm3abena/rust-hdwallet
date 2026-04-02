//! BIP-39 seed wrapper (mirrors `hdwallet/seeds/bip39.py` — minimal).

use zeroize::Zeroizing;

/// Raw BIP-39 seed bytes (64 bytes).
#[derive(Clone)]
pub struct Bip39Seed(pub Zeroizing<Vec<u8>>);

impl Bip39Seed {
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}
