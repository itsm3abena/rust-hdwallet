
use super::conftest::TestData;
use super::conftest::{assert_from_mnemonic};

#[test]
fn test_electrum_v1_from_mnemonic_compressed() {
    let data = TestData::load();
    assert_from_mnemonic(&data, "Electrum-V1", "compressed");
}

#[test]
fn test_electrum_v1_from_mnemonic_uncompressed() {
    let data = TestData::load();
    assert_from_mnemonic(&data, "Electrum-V1", "uncompressed");
}
