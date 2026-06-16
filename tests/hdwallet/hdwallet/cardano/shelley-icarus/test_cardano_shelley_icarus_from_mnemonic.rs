
use super::conftest::TestData;
use super::conftest::{assert_from_mnemonic};

#[test]
fn test_shelley_icarus_from_mnemonic() {
    let data = TestData::load();
    assert_from_mnemonic(&data, "Cardano", "shelley-icarus");
}
