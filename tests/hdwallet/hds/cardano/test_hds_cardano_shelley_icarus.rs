use super::conftest::{run_cardano_shelley_test, TestData};

#[test]
fn test_cardano_shelley_icarus_without_passphrase_hd() {
    run_cardano_shelley_test(TestData::load(), "shelley-icarus", "shelley-icarus", None);
}

#[test]
fn test_cardano_shelley_icarus_with_passphrase_hd() {
    let data = TestData::load();
    let passphrase = data.hds["Cardano"]["shelley-icarus-passphrase"]["passphrase"]
        .as_str()
        .unwrap();
    run_cardano_shelley_test(data, "shelley-icarus-passphrase", "shelley-icarus", Some(passphrase));
}
