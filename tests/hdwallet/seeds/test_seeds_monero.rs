use super::conftest::{json_str, TestData};
use rust_hdwallet::hdwallet::seeds::MoneroSeed;

#[test]
fn test_monero_seeds() {
    let data = TestData::load();
    let monero = data.seeds["Monero"].as_object().expect("Monero seeds");

    for (words, languages) in monero {
        let languages = languages.as_object().expect("languages");
        for (lang, language_data) in languages {
            assert_eq!(
                MoneroSeed::from_mnemonic_phrase(&json_str(&language_data["mnemonic"])).unwrap(),
                json_str(&language_data["non-passphrase-seed"])
            );

            let _ = (words, lang);
        }
    }
}
