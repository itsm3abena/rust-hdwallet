use super::conftest::{json_str, TestData};
use hdwallet_rust::hdwallet::seeds::ElectrumV2Seed;

#[test]
fn test_electrum_v2_seeds() {
    let data = TestData::load();
    let electrum_v2 = data.seeds["Electrum-V2"]
        .as_object()
        .expect("Electrum-V2 seeds");

    for (words, mnemonic_types) in electrum_v2 {
        let mnemonic_types = mnemonic_types.as_object().expect("mnemonic types");
        for (mnemonic_type, languages) in mnemonic_types {
            let languages = languages.as_object().expect("languages");
            for (lang, language_data) in languages {
                let mnemonic = json_str(&language_data["mnemonic"]);
                assert_eq!(
                    ElectrumV2Seed::from_mnemonic_phrase(&mnemonic, None, mnemonic_type).unwrap(),
                    json_str(&language_data["non-passphrase-seed"])
                );

                for (passphrase, expected_seed) in language_data["passphrases"]
                    .as_object()
                    .expect("passphrases")
                {
                    assert_eq!(
                        ElectrumV2Seed::from_mnemonic_phrase(
                            &mnemonic,
                            Some(passphrase),
                            mnemonic_type,
                        )
                        .unwrap(),
                        json_str(expected_seed)
                    );
                    assert_eq!(
                        ElectrumV2Seed::from_mnemonic_phrase(
                            &mnemonic,
                            Some(passphrase),
                            mnemonic_type,
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
