
use super::conftest::TestData;
use super::conftest::{assert_electrum_v1_from_master_private_key};

#[test]
fn test_electrum_v1_from_private_key_compressed() {
    let data = TestData::load();
    assert_electrum_v1_from_master_private_key(&data, "compressed");
}

#[test]
fn test_electrum_v1_from_private_key_uncompressed() {
    let data = TestData::load();
    assert_electrum_v1_from_master_private_key(&data, "uncompressed");
}
