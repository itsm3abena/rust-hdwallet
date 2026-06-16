
use super::conftest::TestData;
use super::conftest::{assert_from_mnemonic};

#[test]
fn test_byron_legacy_from_mnemonic() {
    let data = TestData::load();
    assert_from_mnemonic(&data, "Cardano", "byron-legacy");
}
