//! Litecoin (mirrors `hdwallet/cryptocurrencies/litecoin.py`).

use super::NetworkName;

#[derive(Debug, Clone, Copy)]
pub struct Litecoin {
    pub network: NetworkName,
}
