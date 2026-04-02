//! BIP-44 account structure (mirrors `hdwallet/hds/bip44.py` — path templates).

use bitcoin::bip32::DerivationPath;
use std::str::FromStr;

use crate::hdwallet::exceptions::{Error, Result};
use crate::hdwallet::slip44::coin_types;

/// Default BIP-44 path `m/44'/{coin}'/{account}'/{change}/{index}`.
pub fn bip44_path(coin_type: u32, account: u32, change: u32, index: u32) -> Result<DerivationPath> {
    let s = format!("m/44'/{coin_type}'/{account}'/{change}/{index}");
    DerivationPath::from_str(&s).map_err(|e| Error::Derivation(e.to_string()))
}

/// Bitcoin mainnet first receive address path.
pub fn bitcoin_default_path() -> Result<DerivationPath> {
    bip44_path(coin_types::BITCOIN, 0, 0, 0)
}

/// Ethereum default path (Ledger-style account 0, external).
pub fn ethereum_default_path() -> Result<DerivationPath> {
    bip44_path(coin_types::ETHEREUM, 0, 0, 0)
}
