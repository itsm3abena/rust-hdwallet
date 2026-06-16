use super::conftest::{TestData, assert_monero_derivation};

#[test]
fn derivation_monero() {
    let data = TestData::load();
    assert_monero_derivation(data);
}
