use bitcoin::bip32::DerivationPath;
use std::str::FromStr;

use crate::hdwallet::exceptions::{Error, Result};

pub fn bip49_path(coin_type: u32, account: u32, change: u32, index: u32) -> Result<DerivationPath> {
    let s = format!("m/49'/{coin_type}'/{account}'/{change}/{index}");
    DerivationPath::from_str(&s)
        .map_err(|e| Error::new("Invalid derivation path", Some(&e.to_string()), None, None))
}
