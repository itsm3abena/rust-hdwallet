use super::conftest::{
    assert_bip32_derivation, assert_bip32_root, assert_derivation_p2tr_address,
    bip86_derivation_external_address_1, load_bip_hd, TestData,
};
use rust_hdwallet::hdwallet::derivations::IDerivation;
use rust_hdwallet::hdwallet::hds::ihd::Ihd;

#[test]
fn test_bip86_hd() {
    let data = TestData::load();
    let semantic = "P2PKH";
    let mut hd = load_bip_hd(data, "BIP86", semantic);
    let fixture = &data.hds["BIP86"];
    assert_bip32_root(&hd, fixture, semantic);
    hd.from_derivation(bip86_derivation_external_address_1().path())
        .expect("from_derivation");
    assert_bip32_derivation(&hd, fixture, semantic);
    assert_derivation_p2tr_address(&hd, fixture);
}
