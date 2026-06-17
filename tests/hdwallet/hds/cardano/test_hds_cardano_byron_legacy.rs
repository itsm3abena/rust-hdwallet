use super::conftest::{run_cardano_byron_legacy_test, TestData};

#[test]
fn test_cardano_byron_legacy_hd() {
    run_cardano_byron_legacy_test(TestData::load());
}
