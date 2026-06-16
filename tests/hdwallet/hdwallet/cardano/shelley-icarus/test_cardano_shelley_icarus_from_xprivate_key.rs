
use super::conftest::TestData;
use super::conftest::{assert_from_xprivate_key};

#[test]
fn test_shelley_icarus_from_xprivate_key() {
    let data = TestData::load();
    assert_from_xprivate_key(&data, "Cardano", "shelley-icarus");
}
