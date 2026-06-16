
use super::conftest::TestData;
use super::conftest::{assert_from_seed};

#[test]
fn test_byron_legacy_from_seed() {
    let data = TestData::load();
    assert_from_seed(&data, "Cardano", "byron-legacy");
}
