
use super::conftest::TestData;
use super::conftest::{assert_from_entropy};

#[test]
fn test_byron_icarus_from_entropy() {
    let data = TestData::load();
    assert_from_entropy(&data, "Cardano", "byron-icarus");
}
