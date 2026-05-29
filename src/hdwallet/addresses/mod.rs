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
pub use classes::{
    ALGORAND, APTOS, AVALANCHE, BITCOIN_CASH, CARDANO, COSMOS, EOS, ERGO, ETHEREUM, FILECOIN,
    HARMONY, ICON, INJECTIVE, MONERO, MULTIVERS_X, NANO, NEAR, NEO, OKT_CHAIN, P2PKH, P2SH,
    P2WPKH, P2WPKH_IN_P2SH, P2WSH, P2WSH_IN_P2SH, RIPPLE, SOLANA, STELLAR, SUI, TEZOS, TRON,
    XIN_FIN, ZILLIQA, AlgorandAddress, AptosAddress, NanoAddress,
    NearAddress, NeoAddress,
    P2WPKHAddress, P2WPKHInP2SHAddress, P2WSHAddress, P2WSHInP2SHAddress, RippleAddress,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_match_classes_len() {
        assert_eq!(Addresses::names().len(), Addresses::classes().len());
    }

    #[test]
    fn resolve_okt_chain() {
        let a = Addresses::address("OKT-Chain").expect("OKT-Chain");
        assert_eq!(a.name(), "OKT-Chain");
    }

    #[test]
    fn invalid_name_errors() {
        assert!(Addresses::address("NoSuch").is_err());
    }
}
