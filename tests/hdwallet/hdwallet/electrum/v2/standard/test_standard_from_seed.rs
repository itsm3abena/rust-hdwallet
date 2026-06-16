
use super::conftest::TestData;
use super::conftest::{assert_from_seed};

#[test]
fn test_standard_from_seed() {
    let data = TestData::load();
    assert_from_seed(&data, "Electrum-V2", "standard");
}
