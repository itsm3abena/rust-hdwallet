//! Internal primitives (mirrors `hdwallet/libs/`).
//!
//! Python ships Base58 / RIPEMD160 here; Rust uses `bitcoin` / `bitcoin_hashes`.

pub use bitcoin::hashes::{hash160, Hash};
