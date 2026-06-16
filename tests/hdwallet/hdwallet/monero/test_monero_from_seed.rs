
use super::conftest::TestData;
use super::conftest::{assert_monero_from_seed};

#[test]
fn test_monero_from_seed() {
    let data = TestData::load();
    assert_monero_from_seed(&data);
}
