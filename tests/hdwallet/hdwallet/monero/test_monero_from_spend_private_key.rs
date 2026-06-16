
use super::conftest::TestData;
use super::conftest::{assert_monero_from_spend_private_key};

#[test]
fn test_monero_from_spend_private_key() {
    let data = TestData::load();
    assert_monero_from_spend_private_key(&data);
}
