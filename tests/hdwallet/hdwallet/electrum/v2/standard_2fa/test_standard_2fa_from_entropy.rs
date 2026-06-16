
use super::conftest::TestData;
use super::conftest::{assert_from_entropy};

#[test]
fn test_standard_2fa_from_entropy() {
    let data = TestData::load();
    assert_from_entropy(&data, "Electrum-V2", "standard-2fa");
}
