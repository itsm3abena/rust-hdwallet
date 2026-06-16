
use super::conftest::TestData;
use super::conftest::{assert_from_xpublic_key};

#[test]
fn test_bip141_from_xpublic_key_compressed() {
    let data = TestData::load();
    assert_from_xpublic_key(&data, "BIP141", "compressed");
}

#[test]
fn test_bip141_from_xpublic_key_uncompressed() {
    let data = TestData::load();
    assert_from_xpublic_key(&data, "BIP141", "uncompressed");
}
