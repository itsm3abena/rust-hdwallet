//! SLIP-44 coin types (mirrors `hdwallet/slip44.py` — selected constants).

pub mod coin_types {
    pub const BITCOIN: u32 = 0;
    pub const TESTNET: u32 = 1;
    pub const LITECOIN: u32 = 2;
    pub const DOGECOIN: u32 = 3;
    pub const DASH: u32 = 5;
    pub const ETHEREUM: u32 = 60;
    pub const BITCOIN_CASH: u32 = 145;
    pub const ZCASH: u32 = 133;
    pub const MONERO: u32 = 128;
    pub const COSMOS: u32 = 118;
    pub const AKASH_NETWORK: u32 = 118;
    pub const TRON: u32 = 195;
}
