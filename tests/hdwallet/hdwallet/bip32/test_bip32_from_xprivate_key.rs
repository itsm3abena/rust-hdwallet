
use super::conftest::TestData;
use super::conftest::{assert_from_xprivate_key};

#[test]
fn test_bip32_from_xprivate_key_compressed() {
    let data = TestData::load();
    assert_from_xprivate_key(&data, "BIP32", "compressed");
}

#[test]
fn test_bip32_from_xprivate_key_uncompressed() {
    let data = TestData::load();
    assert_from_xprivate_key(&data, "BIP32", "uncompressed");
}
