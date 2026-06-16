use super::conftest::{TestData, assert_custom_derivation};

#[test]
fn derivation_custom() {
    let data = TestData::load();
    assert_custom_derivation(data);
}
