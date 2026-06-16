
use super::conftest::TestData;
use super::conftest::{assert_from_wif_leaf};

#[test]
fn test_bip32_from_wif_compressed() {
    let data = TestData::load();
    assert_from_wif_leaf(&data, "BIP32", "compressed");
}

#[test]
fn test_bip32_from_wif_uncompressed() {
    let data = TestData::load();
    assert_from_wif_leaf(&data, "BIP32", "uncompressed");
}
