use super::conftest::{TestData, assert_bip84};

#[test]
fn derivation_bip84() {
    let data = TestData::load();
    assert_bip84(data);
}
