
use super::conftest::TestData;
use super::conftest::{assert_from_public_key_cardano};

#[test]
fn test_shelley_icarus_from_public_key() {
    let data = TestData::load();
    assert_from_public_key_cardano(&data, "shelley-icarus");
}
