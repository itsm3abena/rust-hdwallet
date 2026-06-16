use super::conftest::{TestData, assert_cip1852_derivation};

#[test]
fn derivation_cip1852() {
    let data = TestData::load();
    assert_cip1852_derivation(data);
}
