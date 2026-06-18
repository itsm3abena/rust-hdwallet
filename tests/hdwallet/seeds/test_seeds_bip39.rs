use super::conftest::{json_str, TestData};
use rust_hdwallet::hdwallet::seeds::BIP39Seed;

#[test]
fn test_bip39_seeds() {
    let data = TestData::load();
    let bip39 = data.seeds["BIP39"].as_object().expect("BIP39 seeds");

    for (words, languages) in bip39 {
        let languages = languages.as_object().expect("languages");
        for (lang, language_data) in languages {
            let mnemonic = json_str(&language_data["mnemonic"]);
            assert_eq!(
                BIP39Seed::from_mnemonic_with_passphrase(&mnemonic, None).unwrap(),
                json_str(&language_data["non-passphrase-seed"])
            );

            if language_data["passphrases"].is_null() {
                continue;
            }

            for (passphrase, expected_seed) in language_data["passphrases"]
                .as_object()
                .expect("passphrases")
            {
                assert_eq!(
                    BIP39Seed::from_mnemonic_with_passphrase(&mnemonic, Some(passphrase)).unwrap(),
                    json_str(expected_seed)
                );
                assert_eq!(
                    BIP39Seed::from_mnemonic_with_passphrase(&mnemonic, Some(passphrase)).unwrap(),
                    json_str(expected_seed)
                );
            }

            let _ = (words, lang);
        }
    }
}
