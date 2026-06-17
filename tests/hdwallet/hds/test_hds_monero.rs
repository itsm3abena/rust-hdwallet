use super::conftest::{run_monero_hd_test, TestData};

#[test]
fn test_monero_hd() {
    run_monero_hd_test(TestData::load());
}
