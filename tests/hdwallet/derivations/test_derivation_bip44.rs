use super::conftest::{TestData, assert_bip44};

#[test]
fn derivation_bip44() {
    let data = TestData::load();
    assert_bip44(data);
}
