use super::conftest::{
    assert_bip32_derivation, assert_bip32_root, assert_derivation_p2wpkh_in_p2sh_address,
    bip49_derivation_internal, load_bip_hd, TestData,
};
use rust_hdwallet::hdwallet::derivations::IDerivation;
use rust_hdwallet::hdwallet::hds::ihd::Ihd;

#[test]
fn test_bip49_hd() {
    let data = TestData::load();
    let semantic = "P2WPKH_IN_P2SH";
    let mut hd = load_bip_hd(data, "BIP49", semantic);
    let fixture = &data.hds["BIP49"];
    assert_bip32_root(&hd, fixture, semantic);
    hd.from_derivation(bip49_derivation_internal().path())
        .expect("from_derivation");
    assert_bip32_derivation(&hd, fixture, semantic);
    assert_derivation_p2wpkh_in_p2sh_address(&hd, fixture);
}
