
use super::conftest::TestData;
use super::conftest::{assert_from_public_key_leaf};

#[test]
fn test_bip32_from_public_key_compressed() {
    let data = TestData::load();
    assert_from_public_key_leaf(&data, "BIP32", "compressed");
}

#[test]
fn test_bip32_from_public_key_uncompressed() {
    let data = TestData::load();
    assert_from_public_key_leaf(&data, "BIP32", "uncompressed");
}
