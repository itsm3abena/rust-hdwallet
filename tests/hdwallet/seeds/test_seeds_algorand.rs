use super::conftest::{json_str, TestData};
use hdwallet_rust::hdwallet::seeds::AlgorandSeed;

#[test]
fn test_algorand_seeds() {
    let data = TestData::load();
    let language_data = &data.seeds["Algorand"]["25"]["english"];

    assert_eq!(
        AlgorandSeed::from_mnemonic_str(&json_str(&language_data["mnemonic"])).unwrap(),
        json_str(&language_data["non-passphrase-seed"])
    );
}
