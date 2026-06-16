
use super::conftest::TestData;
use super::conftest::{assert_from_private_key_leaf};

#[test]
fn test_bip49_from_private_key_compressed() {
    let data = TestData::load();
    assert_from_private_key_leaf(&data, "BIP49", "compressed");
}

#[test]
fn test_bip49_from_private_key_uncompressed() {
    let data = TestData::load();
    assert_from_private_key_leaf(&data, "BIP49", "uncompressed");
}
