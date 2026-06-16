
use super::conftest::TestData;
use super::conftest::{assert_monero_from_mnemonic};

#[test]
fn test_monero_from_mnemonic() {
    let data = TestData::load();
    assert_monero_from_mnemonic(&data);
}
