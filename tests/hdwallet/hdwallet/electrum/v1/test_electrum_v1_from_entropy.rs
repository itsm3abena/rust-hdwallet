
use super::conftest::TestData;
use super::conftest::{assert_from_entropy};

#[test]
fn test_electrum_v1_from_entropy_compressed() {
    let data = TestData::load();
    assert_from_entropy(&data, "Electrum-V1", "compressed");
}

#[test]
fn test_electrum_v1_from_entropy_uncompressed() {
    let data = TestData::load();
    assert_from_entropy(&data, "Electrum-V1", "uncompressed");
}
