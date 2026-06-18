use super::conftest::{
    assert_bip32_derivation, assert_bip32_derivation_addresses, assert_bip32_root, load_bip_hd,
    TestData,
};
use rust_hdwallet::hdwallet::hds::ihd::Ihd;

#[test]
fn test_bip32_hd() {
    let data = TestData::load();
    let semantic = "P2PKH";
    let mut hd = load_bip_hd(data, "BIP32", semantic);
    let fixture = &data.hds["BIP32"];
    assert_bip32_root(&hd, fixture, semantic);
    let path = fixture["derivation"]["path"].as_str().unwrap();
    hd.from_derivation(path).expect("from_derivation");
    assert_bip32_derivation(&hd, fixture, semantic);
    assert_bip32_derivation_addresses(&hd, fixture);
}
