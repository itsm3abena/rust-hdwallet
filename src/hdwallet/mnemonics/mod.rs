//! Mnemonic word lists (mirrors `hdwallet/mnemonics/`).

pub mod bip39;

pub use bip39::{parse_mnemonic, mnemonic_to_seed};
