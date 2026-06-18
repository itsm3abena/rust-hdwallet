use super::conftest::{
    assert_address_name, assert_compressed_uncompressed, avalanche_kwargs, cosmos_kwargs, ergo_kwargs,
    filecoin_kwargs, p2pkh_kwargs, p2sh_kwargs, p2tr_kwargs, p2wpkh_kwargs, public_key_type_kwargs,
    ripple_kwargs, skip_checksum_kwargs, TestData,
};
use rust_hdwallet::hdwallet::addresses::{
    AvalancheAddress, CosmosAddress, EOSAddress, ErgoAddress, EthereumAddress, FilecoinAddress,
    HarmonyAddress, InjectiveAddress, OKTChainAddress, P2PKHAddress, P2SHAddress, P2TRAddress,
    P2WPKHAddress, P2WPKHInP2SHAddress, RippleAddress, TronAddress, XinFinAddress, ZilliqaAddress,
};

const ECC: &str = "SLIP10-Secp256k1";

#[test]
fn test_p2pkh_address() {
    let data = TestData::load();
    assert_address_name::<P2PKHAddress>(data, ECC, "P2PKH");
    assert_compressed_uncompressed::<P2PKHAddress, _>(data, ECC, "P2PKH", p2pkh_kwargs);
}

#[test]
fn test_p2sh_address() {
    let data = TestData::load();
    assert_address_name::<P2SHAddress>(data, ECC, "P2SH");
    assert_compressed_uncompressed::<P2SHAddress, _>(data, ECC, "P2SH", p2sh_kwargs);
}

#[test]
fn test_p2wpkh_address() {
    let data = TestData::load();
    assert_address_name::<P2WPKHAddress>(data, ECC, "P2WPKH");
    assert_compressed_uncompressed::<P2WPKHAddress, _>(data, ECC, "P2WPKH", p2wpkh_kwargs);
}

#[test]
fn test_p2pkh_in_p2sh_address() {
    let data = TestData::load();
    assert_address_name::<P2WPKHInP2SHAddress>(data, ECC, "P2WPKH-In-P2SH");
    assert_compressed_uncompressed::<P2WPKHInP2SHAddress, _>(data, ECC, "P2WPKH-In-P2SH", p2sh_kwargs);
}

#[test]
fn test_p2tr_address() {
    let data = TestData::load();
    assert_address_name::<P2TRAddress>(data, ECC, "P2TR");
    assert_compressed_uncompressed::<P2TRAddress, _>(data, ECC, "P2TR", p2tr_kwargs);
}

#[test]
fn test_ethereum_address() {
    let data = TestData::load();
    assert_address_name::<EthereumAddress>(data, ECC, "Ethereum");
    assert_compressed_uncompressed::<EthereumAddress, _>(data, ECC, "Ethereum", skip_checksum_kwargs);
}

#[test]
fn test_xinfin_address() {
    let data = TestData::load();
    assert_address_name::<XinFinAddress>(data, ECC, "XinFin");
    assert_compressed_uncompressed::<XinFinAddress, _>(data, ECC, "XinFin", skip_checksum_kwargs);
}

#[test]
fn test_tron_address() {
    let data = TestData::load();
    assert_address_name::<TronAddress>(data, ECC, "Tron");
    assert_compressed_uncompressed::<TronAddress, _>(data, ECC, "Tron", skip_checksum_kwargs);
}

#[test]
fn test_ripple_address() {
    let data = TestData::load();
    assert_address_name::<RippleAddress>(data, ECC, "Ripple");
    assert_compressed_uncompressed::<RippleAddress, _>(data, ECC, "Ripple", ripple_kwargs);
}

#[test]
fn test_filecoin_address() {
    let data = TestData::load();
    assert_address_name::<FilecoinAddress>(data, ECC, "Filecoin");
    assert_compressed_uncompressed::<FilecoinAddress, _>(data, ECC, "Filecoin", filecoin_kwargs);
}

#[test]
fn test_cosmos_address() {
    let data = TestData::load();
    assert_address_name::<CosmosAddress>(data, ECC, "Cosmos");
    assert_compressed_uncompressed::<CosmosAddress, _>(data, ECC, "Cosmos", cosmos_kwargs);
}

#[test]
fn test_avalanche_address() {
    let data = TestData::load();
    assert_address_name::<AvalancheAddress>(data, ECC, "Avalanche");
    assert_compressed_uncompressed::<AvalancheAddress, _>(data, ECC, "Avalanche", avalanche_kwargs);
}

#[test]
fn test_eos_address() {
    let data = TestData::load();
    assert_address_name::<EOSAddress>(data, ECC, "EOS");
    assert_compressed_uncompressed::<EOSAddress, _>(data, ECC, "EOS", public_key_type_kwargs);
}

#[test]
fn test_ergo_address() {
    let data = TestData::load();
    assert_address_name::<ErgoAddress>(data, ECC, "Ergo");
    assert_compressed_uncompressed::<ErgoAddress, _>(data, ECC, "Ergo", ergo_kwargs);
}

#[test]
fn test_okt_chain_address() {
    let data = TestData::load();
    assert_address_name::<OKTChainAddress>(data, ECC, "OKT-Chain");
    assert_compressed_uncompressed::<OKTChainAddress, _>(data, ECC, "OKT-Chain", public_key_type_kwargs);
}

#[test]
fn test_harmony_address() {
    let data = TestData::load();
    assert_address_name::<HarmonyAddress>(data, ECC, "Harmony");
    assert_compressed_uncompressed::<HarmonyAddress, _>(data, ECC, "Harmony", public_key_type_kwargs);
}

#[test]
fn test_zilliqa_address() {
    let data = TestData::load();
    assert_address_name::<ZilliqaAddress>(data, ECC, "Zilliqa");
    assert_compressed_uncompressed::<ZilliqaAddress, _>(data, ECC, "Zilliqa", public_key_type_kwargs);
}

#[test]
fn test_injective_address() {
    let data = TestData::load();
    assert_address_name::<InjectiveAddress>(data, ECC, "Injective");
    assert_compressed_uncompressed::<InjectiveAddress, _>(data, ECC, "Injective", public_key_type_kwargs);
}
