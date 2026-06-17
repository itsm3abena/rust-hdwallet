use super::conftest::{run_cardano_byron_bip44_test, TestData};

#[test]
fn test_cardano_byron_icarus_without_passphrase_hd() {
    run_cardano_byron_bip44_test(TestData::load(), "byron-icarus", "byron-icarus", None);
}

#[test]
fn test_cardano_byron_icarus_with_passphrase_hd() {
    let data = TestData::load();
    let passphrase = data.hds["Cardano"]["byron-icarus-passphrase"]["passphrase"]
        .as_str()
        .unwrap();
    run_cardano_byron_bip44_test(data, "byron-icarus-passphrase", "byron-icarus", Some(passphrase));
}
