use super::conftest::{json_str, TestData};
use rust_hdwallet::hdwallet::seeds::ElectrumV1Seed;

#[test]
fn test_electrum_v1_seeds() {
    let data = TestData::load();
    let language_data = &data.seeds["Electrum-V1"]["12"]["english"];

    assert_eq!(
        ElectrumV1Seed::from_mnemonic_phrase(&json_str(&language_data["mnemonic"])).unwrap(),
        json_str(&language_data["non-passphrase-seed"])
    );
}
