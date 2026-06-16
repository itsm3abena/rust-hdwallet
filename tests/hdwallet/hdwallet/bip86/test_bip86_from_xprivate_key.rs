
use super::conftest::TestData;
use super::conftest::{assert_from_xprivate_key};

#[test]
fn test_bip86_from_xprivate_key_compressed() {
    let data = TestData::load();
    assert_from_xprivate_key(&data, "BIP86", "compressed");
}

#[test]
fn test_bip86_from_xprivate_key_uncompressed() {
    let data = TestData::load();
    assert_from_xprivate_key(&data, "BIP86", "uncompressed");
}
