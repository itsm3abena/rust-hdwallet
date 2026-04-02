//! Address encoders (mirrors `hdwallet/addresses/`).

pub mod bitcoin_addresses;
pub mod ethereum_address;
pub mod utxo_alt;

pub use bitcoin_addresses::encode_bitcoin_address;
pub use ethereum_address::ethereum_address_from_pubkey;
pub use utxo_alt::{
    encode_utxo_coin_address, UtxoAddressParams, DOGECOIN_MAINNET, DOGECOIN_TESTNET,
    LITECOIN_MAINNET, LITECOIN_TESTNET,
};
