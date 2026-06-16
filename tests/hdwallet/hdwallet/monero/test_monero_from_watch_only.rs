
use super::conftest::TestData;
use super::conftest::{assert_monero_from_watch_only};

#[test]
fn test_monero_from_watch_only() {
    let data = TestData::load();
    assert_monero_from_watch_only(&data);
}
