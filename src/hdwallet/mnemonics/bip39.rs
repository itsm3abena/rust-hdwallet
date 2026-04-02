//! BIP-39 mnemonic (mirrors `hdwallet/mnemonics/bip39.py`).

use bip39::{Language, Mnemonic};
use zeroize::Zeroizing;

use crate::hdwallet::exceptions::{Error, Result};

/// Parse a BIP-39 phrase in the given language.
pub fn parse_mnemonic(phrase: &str, language: Language) -> Result<Mnemonic> {
    Mnemonic::parse_in_normalized(language, phrase).map_err(Error::from)
}

/// PBKDF2 seed (512-bit) from mnemonic + passphrase.
pub fn mnemonic_to_seed(mnemonic: &Mnemonic, passphrase: &str) -> Zeroizing<Vec<u8>> {
    Zeroizing::new(mnemonic.to_seed(passphrase).to_vec())
}
