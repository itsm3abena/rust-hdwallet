use super::conftest::{
    assert_bip32_derivation, assert_bip32_root, assert_derivation_p2wpkh_address, load_bip_hd,
    json_kebab_str, TestData,
};
use rust_hdwallet::hdwallet::hds::ihd::Ihd;

#[test]
fn test_bip141_hd() {
    let data = TestData::load();
    let semantic = "P2WPKH";
    let mut hd = load_bip_hd(data, "BIP141", semantic);
    let fixture = &data.hds["BIP141"];
    assert_bip32_root(&hd, fixture, semantic);
    let path = json_kebab_str(&fixture["derivation"], "path");
    hd.from_derivation(&path).expect("from_derivation");
    assert_bip32_derivation(&hd, fixture, semantic);
    assert_derivation_p2wpkh_address(&hd, fixture);
}
