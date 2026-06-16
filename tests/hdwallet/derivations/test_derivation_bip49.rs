use super::conftest::{TestData, assert_bip49};

#[test]
fn derivation_bip49() {
    let data = TestData::load();
    assert_bip49(data);
}
