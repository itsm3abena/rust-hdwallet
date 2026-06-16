
use super::conftest::TestData;
use super::conftest::{assert_electrum_v1_from_master_public_key};

#[test]
fn test_electrum_v1_from_public_key_compressed() {
    let data = TestData::load();
    assert_electrum_v1_from_master_public_key(&data, "compressed");
}

#[test]
fn test_electrum_v1_from_public_key_uncompressed() {
    let data = TestData::load();
    assert_electrum_v1_from_master_public_key(&data, "uncompressed");
}
