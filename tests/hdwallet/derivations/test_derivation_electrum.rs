use super::conftest::{TestData, assert_electrum_derivation};

#[test]
fn derivation_electrum() {
    let data = TestData::load();
    assert_electrum_derivation(data);
}
