use super::conftest::{TestData, assert_hdw_derivation};

#[test]
fn derivation_hdw() {
    let data = TestData::load();
    assert_hdw_derivation(data);
}
