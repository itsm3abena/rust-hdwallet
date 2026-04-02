//! Cryptocurrency / network identifiers (mirrors `hdwallet/cryptocurrencies/icryptocurrency.py`).

/// Network string names used by Python (`"mainnet"`, `"testnet"`, …).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkName {
    Mainnet,
    Testnet,
    Regtest,
}

impl NetworkName {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "mainnet" => Some(Self::Mainnet),
            "testnet" => Some(Self::Testnet),
            "regtest" => Some(Self::Regtest),
            _ => None,
        }
    }
}

/// Built-in chains implemented in Rust (subset of upstream `ICryptocurrency` classes).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CryptocurrencyKind {
    Bitcoin,
    Ethereum,
    Litecoin,
    Dogecoin,
}
