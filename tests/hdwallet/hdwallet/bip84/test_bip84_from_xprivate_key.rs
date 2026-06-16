
use super::conftest::TestData;
use super::conftest::{assert_from_xprivate_key};

#[test]
fn test_bip84_from_xprivate_key_compressed() {
    let data = TestData::load();
    assert_from_xprivate_key(&data, "BIP84", "compressed");
}

#[test]
fn test_bip84_from_xprivate_key_uncompressed() {
    let data = TestData::load();
    assert_from_xprivate_key(&data, "BIP84", "uncompressed");
}
