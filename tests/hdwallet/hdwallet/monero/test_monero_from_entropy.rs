
use super::conftest::TestData;
use super::conftest::{assert_monero_from_entropy};

#[test]
fn test_monero_from_entropy() {
    let data = TestData::load();
    assert_monero_from_entropy(&data);
}
