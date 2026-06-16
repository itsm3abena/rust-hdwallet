use super::conftest::{TestData, assert_bip86};

#[test]
fn derivation_bip86() {
    let data = TestData::load();
    assert_bip86(data);
}
