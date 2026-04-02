//! Dogecoin (mirrors `hdwallet/cryptocurrencies/dogecoin.py`).

use super::NetworkName;

#[derive(Debug, Clone, Copy)]
pub struct Dogecoin {
    pub network: NetworkName,
}
