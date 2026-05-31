mod algorand;
mod aptos;
mod avalanche;
mod bitcoin_cash;
mod cardano;
mod cosmos;
mod eos;
mod ergo;
mod filecoin;
mod harmony;
mod icon;
mod injective;
mod monero;
mod multivers_x;
mod nano;
mod near;
mod neo;
mod okt_chain;
mod p2pkh;
mod p2sh;
mod p2tr;
mod p2wpkh;
mod p2wpkh_in_p2sh;
mod p2wsh;
mod p2wsh_in_p2sh;
mod ripple;
mod classes;
pub mod iaddress;

pub mod bitcoin;
pub mod ethereum;
pub mod utxo_alt;

pub use bitcoin::encode_bitcoin_address;
pub use avalanche::AvalancheAddress;
pub use bitcoin_cash::BitcoinCashAddress;
pub use cardano::CardanoAddress;
pub use cosmos::CosmosAddress;
pub use eos::EOSAddress;
pub use ergo::ErgoAddress;
pub use ethereum::EthereumAddress;
pub use filecoin::FilecoinAddress;
pub use harmony::HarmonyAddress;
pub use icon::IconAddress;
pub use injective::InjectiveAddress;
pub use monero::MoneroAddress;
pub use multivers_x::MultiversXAddress;
pub use okt_chain::OKTChainAddress;
pub use p2pkh::P2PKHAddress;
pub use p2sh::P2SHAddress;
pub use p2tr::P2TRAddress;
pub use p2wpkh::P2WPKHAddress;
pub use p2wpkh_in_p2sh::P2WPKHInP2SHAddress;
pub use p2wsh::P2WSHAddress;
pub use p2wsh_in_p2sh::P2WSHInP2SHAddress;
pub use ripple::RippleAddress;
pub use classes::{
    ALGORAND, APTOS, AVALANCHE, BITCOIN_CASH, CARDANO, COSMOS, EOS, ERGO, ETHEREUM, FILECOIN,
    HARMONY, ICON, INJECTIVE, MONERO, MULTIVERS_X, NANO, NEAR, NEO, OKT_CHAIN, P2PKH, P2SH,
    RIPPLE, SOLANA, STELLAR, SUI, TEZOS, TRON,
    XIN_FIN, ZILLIQA, AlgorandAddress, AptosAddress, NanoAddress,
    NearAddress, NeoAddress,
    SolanaAddress, StellarAddress, SuiAddress, TezosAddress, TronAddress, XinFinAddress,
    ZilliqaAddress,
};
pub use ethereum::ethereum_address_from_pubkey;
pub use iaddress::{
    decode_address, encode_address, encode_with_ipublic_key, AddressDecodeResult,
    AddressPublicKeyInput, AddressSchemeError, DecodeKwargs, EncodeKwargs, EncodePublicKey,
    IAddress,
};
pub use utxo_alt::{
    encode_utxo_coin_address, UtxoAddressParams, DOGECOIN_MAINNET, DOGECOIN_TESTNET,
    LITECOIN_MAINNET, LITECOIN_TESTNET,
};

use crate::hdwallet::exceptions::AddressError;

#[derive(Debug, Clone, Copy, Default)]
pub struct Addresses;

pub type ADDRESSES = Addresses;

impl Addresses {
    pub fn names() -> Vec<&'static str> {
        classes::CLASS_ORDER.iter().map(|c| c.name()).collect()
    }

    pub fn classes() -> Vec<&'static dyn IAddress> {
        classes::CLASS_ORDER.to_vec()
    }

    pub fn address(name: &str) -> Result<&'static dyn IAddress, AddressError> {
        for class in classes::CLASS_ORDER {
            if class.name() == name {
                return Ok(*class);
            }
        }
        let expected = Self::names().join(", ");
        Err(AddressError::new(
            "Invalid address name",
            None,
            Some(&expected),
            Some(name),
        ))
    }

    pub fn is_address(name: &str) -> bool {
        classes::CLASS_ORDER
            .iter()
            .any(|class| class.name() == name)
    }
}
