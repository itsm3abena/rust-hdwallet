//! BIP-84 native SegWit paths (mirrors `hdwallet/hds/bip84.py`).

use bitcoin::bip32::DerivationPath;
use std::str::FromStr;

use crate::hdwallet::exceptions::{Error, Result};

pub fn bip84_path(coin_type: u32, account: u32, change: u32, index: u32) -> Result<DerivationPath> {
    let s = format!("m/84'/{coin_type}'/{account}'/{change}/{index}");
    DerivationPath::from_str(&s).map_err(|e| Error::Derivation(e.to_string()))
}
