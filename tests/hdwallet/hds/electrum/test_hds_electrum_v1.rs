use super::conftest::{run_electrum_v1_test, TestData};

#[test]
fn test_electrum_v1_hd() {
    run_electrum_v1_test(TestData::load());
}
