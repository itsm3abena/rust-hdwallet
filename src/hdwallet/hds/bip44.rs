use bitcoin::bip32::DerivationPath;
use std::str::FromStr;

use crate::hdwallet::exceptions::{Error, Result};
use crate::hdwallet::slip44::coin_types;

pub fn bip44_path(coin_type: u32, account: u32, change: u32, index: u32) -> Result<DerivationPath> {
    let s = format!("m/44'/{coin_type}'/{account}'/{change}/{index}");
    DerivationPath::from_str(&s)
        .map_err(|e| Error::new("Invalid derivation path", Some(&e.to_string()), None, None))
}

pub fn bitcoin_default_path() -> Result<DerivationPath> {
    bip44_path(coin_types::BITCOIN, 0, 0, 0)
}

pub fn ethereum_default_path() -> Result<DerivationPath> {
    bip44_path(coin_types::ETHEREUM, 0, 0, 0)
}
