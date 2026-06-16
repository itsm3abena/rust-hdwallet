
use super::conftest::TestData;
use super::conftest::{assert_from_wif_leaf};

#[test]
fn test_bip86_from_wif_compressed() {
    let data = TestData::load();
    assert_from_wif_leaf(&data, "BIP86", "compressed");
}

#[test]
fn test_bip86_from_wif_uncompressed() {
    let data = TestData::load();
    assert_from_wif_leaf(&data, "BIP86", "uncompressed");
}
