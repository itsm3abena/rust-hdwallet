
use super::conftest::TestData;
use super::conftest::{assert_from_public_key_leaf};

#[test]
fn test_bip44_from_public_key_compressed() {
    let data = TestData::load();
    assert_from_public_key_leaf(&data, "BIP44", "compressed");
}

#[test]
fn test_bip44_from_public_key_uncompressed() {
    let data = TestData::load();
    assert_from_public_key_leaf(&data, "BIP44", "uncompressed");
}
