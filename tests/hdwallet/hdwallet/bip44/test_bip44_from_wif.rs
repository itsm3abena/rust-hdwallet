
use super::conftest::TestData;
use super::conftest::{assert_from_wif_leaf};

#[test]
fn test_bip44_from_wif_compressed() {
    let data = TestData::load();
    assert_from_wif_leaf(&data, "BIP44", "compressed");
}

#[test]
fn test_bip44_from_wif_uncompressed() {
    let data = TestData::load();
    assert_from_wif_leaf(&data, "BIP44", "uncompressed");
}
