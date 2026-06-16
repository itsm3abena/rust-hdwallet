
use super::conftest::TestData;
use super::conftest::{assert_from_seed};

#[test]
fn test_bip141_from_seed_compressed() {
    let data = TestData::load();
    assert_from_seed(&data, "BIP141", "compressed");
}

#[test]
fn test_bip141_from_seed_uncompressed() {
    let data = TestData::load();
    assert_from_seed(&data, "BIP141", "uncompressed");
}
