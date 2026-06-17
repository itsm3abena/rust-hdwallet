use super::conftest::{run_cardano_byron_bip44_test, TestData};

#[test]
fn test_cardano_byron_ledger_hd() {
    run_cardano_byron_bip44_test(TestData::load(), "byron-ledger", "byron-ledger", None);
}
