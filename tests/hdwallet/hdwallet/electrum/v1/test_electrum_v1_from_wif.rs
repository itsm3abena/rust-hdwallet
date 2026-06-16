
use super::conftest::TestData;
use super::conftest::{assert_electrum_v1_from_master_wif};

#[test]
fn test_electrum_v1_from_wif_compressed() {
    let data = TestData::load();
    assert_electrum_v1_from_master_wif(&data, "compressed");
}

#[test]
fn test_electrum_v1_from_wif_uncompressed() {
    let data = TestData::load();
    assert_electrum_v1_from_master_wif(&data, "uncompressed");
}
