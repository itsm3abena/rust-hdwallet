
use super::conftest::TestData;
use super::conftest::{assert_from_private_key_leaf};

#[test]
fn test_bip86_from_private_key_compressed() {
    let data = TestData::load();
    assert_from_private_key_leaf(&data, "BIP86", "compressed");
}

#[test]
fn test_bip86_from_private_key_uncompressed() {
    let data = TestData::load();
    assert_from_private_key_leaf(&data, "BIP86", "uncompressed");
}
