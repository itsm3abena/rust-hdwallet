//! Per-chain parameters (mirrors `hdwallet/cryptocurrencies/`).

pub mod bitcoin;
pub mod dogecoin;
pub mod ethereum;
pub mod icryptocurrency;
pub mod litecoin;

pub use bitcoin::Bitcoin;
pub use dogecoin::Dogecoin;
pub use ethereum::Ethereum;
pub use icryptocurrency::{CryptocurrencyKind, NetworkName};
pub use litecoin::Litecoin;
