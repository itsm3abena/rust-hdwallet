
use super::conftest::TestData;
use super::conftest::{assert_from_seed};

#[test]
fn test_byron_icarus_from_seed() {
    let data = TestData::load();
    assert_from_seed(&data, "Cardano", "byron-icarus");
}
