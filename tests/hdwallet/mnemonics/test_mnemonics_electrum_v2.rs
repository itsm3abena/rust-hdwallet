use super::conftest::{assert_error_contains, json_str, json_usize, TestData};
use hdwallet_rust::hdwallet::exceptions::EntropyError;
use hdwallet_rust::hdwallet::mnemonics::{
    electrum::{
        ElectrumV2MnemonicLanguages, ElectrumV2MnemonicTypes, ElectrumV2MnemonicWordCounts,
    },
    imnemonic::IMnemonic,
    ElectrumV2Mnemonic,
};

enum MnemonicEntropyInput<'a> {
    Hex(&'a str),
    Invalid,
}

fn electrum_v2_from_entropy(
    entropy: MnemonicEntropyInput<'_>,
    language: &str,
    mnemonic_type: &str,
) -> Result<String, EntropyError> {
    match entropy {
        MnemonicEntropyInput::Hex(hex) => ElectrumV2Mnemonic::from_entropy_with_retries(
            hex,
            language,
            mnemonic_type,
            50_000,
        )
        .map(|mnemonic| mnemonic.mnemonic())
        .map_err(|error| EntropyError::new(&error.to_string(), None, None, None)),
        MnemonicEntropyInput::Invalid => Err(EntropyError::new(
            "Invalid entropy instance",
            None,
            Some("[str, bytes, ElectrumV2Entropy]"),
            None,
        )),
    }
}

#[test]
fn test_electrum_v2_mnemonics() {
    let data = TestData::load();
    let entries = data.mnemonics["Electrum-V2"]
        .as_array()
        .expect("Electrum-V2 mnemonics array");

    assert_eq!(ElectrumV2MnemonicWordCounts::TWELVE, 12);
    assert_eq!(ElectrumV2MnemonicWordCounts::TWENTY_FOUR, 24);

    assert_eq!(
        ElectrumV2MnemonicLanguages::CHINESE_SIMPLIFIED,
        "chinese-simplified"
    );
    assert_eq!(ElectrumV2MnemonicLanguages::ENGLISH, "english");
    assert_eq!(ElectrumV2MnemonicLanguages::PORTUGUESE, "portuguese");
    assert_eq!(ElectrumV2MnemonicLanguages::SPANISH, "spanish");

    for entry in entries {
        let words = json_usize(&entry["words"]);
        assert!(ElectrumV2Mnemonic::is_valid_words(words));

        for (mnemonic_type, languages) in entry["mnemonic-types"]
            .as_object()
            .expect("mnemonic-types")
        {
            for (language, language_data) in languages.as_object().expect("languages") {
                let language_data = language_data;
                let mnemonic_text = json_str(&language_data["mnemonic"]);
                let mnemonic_words: Vec<String> = mnemonic_text
                    .split_whitespace()
                    .map(String::from)
                    .collect();

                assert!(ElectrumV2Mnemonic::is_valid_language(language));
                assert!(ElectrumV2Mnemonic::is_valid(&mnemonic_words, mnemonic_type));

                let generated = ElectrumV2Mnemonic::generate_random(words, language, mnemonic_type)
                    .unwrap();
                assert_eq!(generated.phrase().split_whitespace().count(), words);
                assert_eq!(
                    generated.language().to_lowercase(),
                    language.to_lowercase()
                );

                let entropy_not_suitable = json_str(&entry["entropy-not-suitable"]);
                assert_eq!(
                    electrum_v2_from_entropy(
                        MnemonicEntropyInput::Hex(&entropy_not_suitable),
                        language,
                        mnemonic_type,
                    )
                    .unwrap(),
                    mnemonic_text
                );
                assert_eq!(
                    hex::encode(
                        ElectrumV2Mnemonic::decode_phrase(&mnemonic_text, mnemonic_type).unwrap()
                    ),
                    json_str(&language_data["entropy-suitable"])
                );

                let mnemonic = ElectrumV2Mnemonic::from_entropy_with_retries(
                    &entropy_not_suitable,
                    language,
                    mnemonic_type,
                    50_000,
                )
                .unwrap();
                assert_eq!(ElectrumV2Mnemonic::name(), json_str(&entry["name"]));
                assert_eq!(mnemonic.language().to_lowercase(), language.to_lowercase());
                assert_eq!(mnemonic.mnemonic_type().unwrap(), mnemonic_type);
            }
        }
    }

    assert_error_contains(
        ElectrumV2Mnemonic::decode_phrase(
            "flower letter world foil coin poverty romance tongue taste hip cradle follow proud pluck ten improve",
            ElectrumV2MnemonicTypes::STANDARD,
        )
        .map(|_| ()),
        "Invalid mnemonic words",
    );

    assert_error_contains(
        ElectrumV2Mnemonic::from_words(100, ElectrumV2MnemonicLanguages::ENGLISH).map(|_| ()),
        "Invalid mnemonic words number",
    );

    assert_error_contains(
        electrum_v2_from_entropy(
            MnemonicEntropyInput::Invalid,
            ElectrumV2MnemonicLanguages::ENGLISH,
            ElectrumV2MnemonicTypes::STANDARD,
        )
        .map(|_| ()),
        "Invalid entropy instance",
    );

    assert_error_contains(
        electrum_v2_from_entropy(
            MnemonicEntropyInput::Hex(
                "7c2abbf52d1861b978792df3dc88e7e27dbe36c7a0287893",
            ),
            ElectrumV2MnemonicLanguages::ENGLISH,
            ElectrumV2MnemonicTypes::STANDARD,
        )
        .map(|_| ()),
        "Unable to generate a valid mnemonic",
    );
}
