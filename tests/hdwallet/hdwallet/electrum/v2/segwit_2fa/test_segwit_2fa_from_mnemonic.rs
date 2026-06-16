
use super::conftest::TestData;
use super::conftest::{assert_from_mnemonic};

#[test]
fn test_segiwt_2fa_from_mnemonic() {
    let data = TestData::load();
    assert_from_mnemonic(&data, "Electrum-V2", "segwit-2fa");
}
