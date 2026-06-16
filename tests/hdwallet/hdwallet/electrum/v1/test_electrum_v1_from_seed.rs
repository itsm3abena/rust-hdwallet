
use super::conftest::TestData;
use super::conftest::{assert_from_seed};

#[test]
fn test_electrum_v1_from_seed_compressed() {
    let data = TestData::load();
    assert_from_seed(&data, "Electrum-V1", "compressed");
}

#[test]
fn test_electrum_v1_from_seed_uncompressed() {
    let data = TestData::load();
    assert_from_seed(&data, "Electrum-V1", "uncompressed");
}
