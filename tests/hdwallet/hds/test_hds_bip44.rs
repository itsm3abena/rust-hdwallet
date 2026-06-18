use super::conftest::{
    assert_bip32_derivation, assert_bip32_root, assert_derivation_p2pkh_address, bip44_derivation_internal,
    load_bip_hd, TestData,
};
use rust_hdwallet::hdwallet::derivations::IDerivation;
use rust_hdwallet::hdwallet::hds::ihd::Ihd;

#[test]
fn test_bip44_hd() {
    let data = TestData::load();
    let semantic = "P2PKH";
    let mut hd = load_bip_hd(data, "BIP44", semantic);
    let fixture = &data.hds["BIP44"];
    assert_bip32_root(&hd, fixture, semantic);
    hd.from_derivation(bip44_derivation_internal().path())
        .expect("from_derivation");
    assert_bip32_derivation(&hd, fixture, semantic);
    assert_derivation_p2pkh_address(&hd, fixture);
}
