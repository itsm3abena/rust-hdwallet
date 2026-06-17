use super::conftest::{
    assert_cardano_address_name, assert_cardano_byron_icarus, assert_cardano_byron_legacy,
    assert_cardano_shelley_payment, assert_cardano_shelley_staking,
};

#[test]
fn test_cardano_addresses() {
    assert_cardano_address_name();
}

#[test]
fn test_cardano_byron_icarus_address() {
    assert_cardano_byron_icarus(super::conftest::TestData::load());
}

#[test]
fn test_cardano_byron_legacy_address() {
    assert_cardano_byron_legacy(super::conftest::TestData::load());
}

#[test]
fn test_cardano_shelley_address() {
    assert_cardano_shelley_payment(super::conftest::TestData::load());
}

#[test]
fn test_cardano_shelley_staking_address() {
    assert_cardano_shelley_staking(super::conftest::TestData::load());
}
