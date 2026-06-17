use super::conftest::{assert_monero_address, TestData};

#[test]
fn test_monero_address() {
    assert_monero_address(TestData::load());
}
