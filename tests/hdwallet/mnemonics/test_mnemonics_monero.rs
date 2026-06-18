use super::conftest::{assert_error_contains, get_bytes, json_str, json_usize, TestData};
use rust_hdwallet::hdwallet::exceptions::EntropyError;
use rust_hdwallet::hdwallet::mnemonics::{
    imnemonic::IMnemonic,
    monero::{MoneroMnemonicLanguages, MoneroMnemonicWordCounts},
    MoneroMnemonic,
};

enum MnemonicEntropyInput<'a> {
    Hex(&'a str),
    Invalid,
}

fn monero_from_entropy(
    entropy: MnemonicEntropyInput<'_>,
    checksum: bool,
    language: &str,
) -> Result<String, EntropyError> {
    match entropy {
        MnemonicEntropyInput::Hex(hex) => MoneroMnemonic::from_entropy_with_checksum(
            &get_bytes(hex),
            language,
            checksum,
        )
        .map(|mnemonic| mnemonic.mnemonic())
        .map_err(|error| EntropyError::new(&error.to_string(), None, None, None)),
        MnemonicEntropyInput::Invalid => Err(EntropyError::new(
            "Invalid entropy instance",
            None,
            Some("[str, bytes, MoneroEntropy]"),
            None,
        )),
    }
}

#[test]
fn test_monero_mnemonics() {
    let data = TestData::load();
    let entries = data.mnemonics["Monero"]
        .as_array()
        .expect("Monero mnemonics array");

    assert_eq!(MoneroMnemonicWordCounts::TWELVE, 12);
    assert_eq!(MoneroMnemonicWordCounts::THIRTEEN, 13);
    assert_eq!(MoneroMnemonicWordCounts::TWENTY_FOUR, 24);
    assert_eq!(MoneroMnemonicWordCounts::TWENTY_FIVE, 25);

    assert_eq!(
        MoneroMnemonicLanguages::CHINESE_SIMPLIFIED,
        "chinese-simplified"
    );
    assert_eq!(MoneroMnemonicLanguages::DUTCH, "dutch");
    assert_eq!(MoneroMnemonicLanguages::ENGLISH, "english");
    assert_eq!(MoneroMnemonicLanguages::FRENCH, "french");
    assert_eq!(MoneroMnemonicLanguages::GERMAN, "german");
    assert_eq!(MoneroMnemonicLanguages::ITALIAN, "italian");
    assert_eq!(MoneroMnemonicLanguages::JAPANESE, "japanese");
    assert_eq!(MoneroMnemonicLanguages::PORTUGUESE, "portuguese");
    assert_eq!(MoneroMnemonicLanguages::RUSSIAN, "russian");
    assert_eq!(MoneroMnemonicLanguages::SPANISH, "spanish");

    for entry in entries {
        let words = json_usize(&entry["words"]);
        assert!(MoneroMnemonic::is_valid_words(words));

        for (language, mnemonic_text) in entry["languages"].as_object().expect("languages") {
            let mnemonic_text = json_str(mnemonic_text);
            let mnemonic_words: Vec<String> = mnemonic_text
                .split_whitespace()
                .map(String::from)
                .collect();

            assert!(MoneroMnemonic::is_valid_language(language));
            assert!(MoneroMnemonic::is_valid(&mnemonic_words));

            let generated = MoneroMnemonic::from_words(words, language).unwrap();
            let generated_words: Vec<String> = generated
                .split_whitespace()
                .map(String::from)
                .collect();
            assert_eq!(generated_words.len(), words);
            let (_, detected_language) = MoneroMnemonic::find_language(&generated_words).unwrap();
            assert_eq!(detected_language.to_lowercase(), language.to_lowercase());

            let entropy_hex = json_str(&entry["entropy"]);
            let checksum = entry["checksum"].as_bool().unwrap();
            assert_eq!(
                monero_from_entropy(MnemonicEntropyInput::Hex(&entropy_hex), checksum, language)
                    .unwrap(),
                mnemonic_text
            );
            assert_eq!(
                hex::encode(MoneroMnemonic::decode(&mnemonic_words).unwrap()),
                entropy_hex
            );

            let mnemonic = MoneroMnemonic::from_entropy_with_checksum(
                &get_bytes(&entropy_hex),
                language,
                checksum,
            )
            .unwrap();
            assert_eq!(MoneroMnemonic::name(), json_str(&entry["name"]));
            assert_eq!(mnemonic.language().to_lowercase(), language.to_lowercase());
        }
    }

    assert_error_contains(
        MoneroMnemonic::parse_phrase(
            "flower letter world foil coin poverty romance tongue taste hip cradle follow proud pluck ten improve",
        )
        .map(|_| ()),
        "Invalid mnemonic words",
    );

    assert_error_contains(
        MoneroMnemonic::from_words(100, MoneroMnemonicLanguages::ENGLISH).map(|_| ()),
        "Invalid mnemonic words number",
    );

    assert_error_contains(
        monero_from_entropy(
            MnemonicEntropyInput::Invalid,
            false,
            MoneroMnemonicLanguages::ENGLISH,
        )
        .map(|_| ()),
        "Invalid entropy instance",
    );

    assert_error_contains(
        monero_from_entropy(
            MnemonicEntropyInput::Hex(
                "cdf694ac868efd01673fc51e897c57a0bd428503080ad4c94c7d6f6d13f095fbc8",
            ),
            false,
            MoneroMnemonicLanguages::ENGLISH,
        )
        .map(|_| ()),
        "Wrong entropy strength",
    );
}
