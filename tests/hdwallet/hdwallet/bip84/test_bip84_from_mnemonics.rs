
use super::conftest::TestData;
use super::conftest::{assert_from_mnemonic};

#[test]
fn test_bip84_from_mnemonic_compressed() {
    let data = TestData::load();
    assert_from_mnemonic(&data, "BIP84", "compressed");
}

#[test]
fn test_bip84_from_mnemonic_uncompressed() {
    let data = TestData::load();
    assert_from_mnemonic(&data, "BIP84", "uncompressed");
}
