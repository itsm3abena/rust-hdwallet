use super::conftest::{run_cardano_shelley_test, TestData};

#[test]
fn test_cardano_shelley_ledger_hd() {
    run_cardano_shelley_test(TestData::load(), "shelley-ledger", "shelley-ledger", None);
}
