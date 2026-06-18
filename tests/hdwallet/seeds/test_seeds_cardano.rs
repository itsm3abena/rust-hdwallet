use super::conftest::{json_str, TestData};
use rust_hdwallet::hdwallet::seeds::CardanoSeed;

#[test]
fn test_cardano_seeds() {
    let data = TestData::load();
    let cardano = data.seeds["Cardano"].as_object().expect("Cardano seeds");

    for (words, cardano_types) in cardano {
        let cardano_types = cardano_types.as_object().expect("cardano types");
        for (cardano_type, languages) in cardano_types {
            let languages = languages.as_object().expect("languages");
            for (lang, language_data) in languages {
                let mnemonic = json_str(&language_data["mnemonic"]);
                assert_eq!(
                    CardanoSeed::from_mnemonic_with_type(&mnemonic, None, cardano_type).unwrap(),
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
                        CardanoSeed::from_mnemonic_with_type(
                            &mnemonic,
                            Some(passphrase),
                            cardano_type,
                        )
                        .unwrap(),
                        json_str(expected_seed)
                    );
                    assert_eq!(
                        CardanoSeed::from_mnemonic_with_type(
                            &mnemonic,
                            Some(passphrase),
                            cardano_type,
                        )
                        .unwrap(),
                        json_str(expected_seed)
                    );
                }

                let _ = (words, lang);
            }
        }
    }
}
