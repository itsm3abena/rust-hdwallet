use super::conftest::{assert_address_name, assert_simple_encode_decode, TestData};
use rust_hdwallet::hdwallet::addresses::{
    AlgorandAddress, AptosAddress, MultiversXAddress, NearAddress, SolanaAddress, StellarAddress,
    SuiAddress, TezosAddress,
};

const ECC: &str = "SLIP10-Ed25519";

#[test]
fn test_algorand_address() {
    let data = TestData::load();
    assert_address_name::<AlgorandAddress>(data, ECC, "Algorand");
    assert_simple_encode_decode::<AlgorandAddress>(data, ECC, "Algorand", "public-key");
}

#[test]
fn test_multiversx_address() {
    let data = TestData::load();
    assert_address_name::<MultiversXAddress>(data, ECC, "MultiversX");
    assert_simple_encode_decode::<MultiversXAddress>(data, ECC, "MultiversX", "public-key");
}

#[test]
fn test_solana_address() {
    let data = TestData::load();
    assert_address_name::<SolanaAddress>(data, ECC, "Solana");
    assert_simple_encode_decode::<SolanaAddress>(data, ECC, "Solana", "public-key");
}

#[test]
fn test_stellar_address() {
    let data = TestData::load();
    assert_address_name::<StellarAddress>(data, ECC, "Stellar");
    assert_simple_encode_decode::<StellarAddress>(data, ECC, "Stellar", "public-key");
}

#[test]
fn test_tezos_address() {
    let data = TestData::load();
    assert_address_name::<TezosAddress>(data, ECC, "Tezos");
    assert_simple_encode_decode::<TezosAddress>(data, ECC, "Tezos", "public-key");
}

#[test]
fn test_sui_address() {
    let data = TestData::load();
    assert_address_name::<SuiAddress>(data, ECC, "Sui");
    assert_simple_encode_decode::<SuiAddress>(data, ECC, "Sui", "public-key");
}

#[test]
fn test_aptos_address() {
    let data = TestData::load();
    assert_address_name::<AptosAddress>(data, ECC, "Aptos");
    assert_simple_encode_decode::<AptosAddress>(data, ECC, "Aptos", "public-key");
}

#[test]
fn test_near_address() {
    let data = TestData::load();
    assert_address_name::<NearAddress>(data, ECC, "Near");
    assert_simple_encode_decode::<NearAddress>(data, ECC, "Near", "public-key");
}
