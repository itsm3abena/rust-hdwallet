
use super::conftest::TestData;
use super::conftest::{assert_from_xpublic_key};

#[test]
fn test_bip32_from_xpublic_key_compressed() {
    let data = TestData::load();
    assert_from_xpublic_key(&data, "BIP32", "compressed");
}

#[test]
fn test_bip32_from_xpublic_key_uncompressed() {
    let data = TestData::load();
    assert_from_xpublic_key(&data, "BIP32", "uncompressed");
}
