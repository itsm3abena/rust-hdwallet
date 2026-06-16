
use super::conftest::TestData;
use super::conftest::{assert_from_entropy};

#[test]
fn test_bip84_from_entropy_compressed() {
    let data = TestData::load();
    assert_from_entropy(&data, "BIP84", "compressed");
}

#[test]
fn test_bip84_from_entropy_uncompressed() {
    let data = TestData::load();
    assert_from_entropy(&data, "BIP84", "uncompressed");
}
