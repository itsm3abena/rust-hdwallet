use super::conftest::{
    assert_bip32_derivation, assert_bip32_root, assert_derivation_p2wpkh_address, bip84_derivation_internal,
    load_bip_hd, TestData,
};
use hdwallet_rust::hdwallet::derivations::IDerivation;
use hdwallet_rust::hdwallet::hds::ihd::Ihd;

#[test]
fn test_bip84_hd() {
    let data = TestData::load();
    let semantic = "P2WPKH";
    let mut hd = load_bip_hd(data, "BIP84", semantic);
    let fixture = &data.hds["BIP84"];
    assert_bip32_root(&hd, fixture, semantic);
    hd.from_derivation(bip84_derivation_internal().path())
        .expect("from_derivation");
    assert_bip32_derivation(&hd, fixture, semantic);
    assert_derivation_p2wpkh_address(&hd, fixture);
}
