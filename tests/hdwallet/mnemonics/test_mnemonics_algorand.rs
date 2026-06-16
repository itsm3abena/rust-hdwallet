use super::conftest::{assert_error_contains, get_bytes, json_str, json_usize, TestData};
use hdwallet_rust::hdwallet::exceptions::EntropyError;
use hdwallet_rust::hdwallet::mnemonics::{
    algorand::{AlgorandMnemonicLanguages, AlgorandMnemonicWords},
    imnemonic::IMnemonic,
    AlgorandMnemonic,
};

enum MnemonicEntropyInput<'a> {
    Hex(&'a str),
    Invalid,
}

fn algorand_from_entropy(
    entropy: MnemonicEntropyInput<'_>,
    language: &str,
) -> Result<String, EntropyError> {
    match entropy {
        MnemonicEntropyInput::Hex(hex) => AlgorandMnemonic::from_entropy(&get_bytes(hex), language)
            .map_err(|error| EntropyError::new(&error.to_string(), None, None, None)),
        MnemonicEntropyInput::Invalid => Err(EntropyError::new(
            "Invalid entropy instance",
            None,
            Some("[str, bytes, AlgorandEntropy]"),
            None,
        )),
    }
}

#[test]
fn test_algorand_mnemonics() {
    let data = TestData::load();
    let entries = data.mnemonics["Algorand"]
        .as_array()
        .expect("Algorand mnemonics array");

    assert_eq!(AlgorandMnemonicWords::TWENTY_FIVE, 25);
    assert_eq!(AlgorandMnemonicLanguages::ENGLISH, "english");

    for entry in entries {
        let words = json_usize(&entry["words"]);
        assert!(AlgorandMnemonic::is_valid_words(words));

        for (language, mnemonic_text) in entry["languages"].as_object().expect("languages") {
            let mnemonic_text = json_str(mnemonic_text);
            let mnemonic_words: Vec<String> = mnemonic_text
                .split_whitespace()
                .map(String::from)
                .collect();

            assert!(AlgorandMnemonic::is_valid_language(language));
            assert!(AlgorandMnemonic::is_valid(&mnemonic_words));

            let generated = AlgorandMnemonic::from_words(words, language).unwrap();
            let generated_words: Vec<String> = generated
                .split_whitespace()
                .map(String::from)
                .collect();
            assert_eq!(generated_words.len(), words);
            let (_, detected_language) = AlgorandMnemonic::find_language(&generated_words).unwrap();
            assert_eq!(detected_language.to_lowercase(), language.to_lowercase());

            let entropy_hex = json_str(&entry["entropy"]);
            assert_eq!(
                algorand_from_entropy(MnemonicEntropyInput::Hex(&entropy_hex), language).unwrap(),
                mnemonic_text
            );
            assert_eq!(
                hex::encode(AlgorandMnemonic::decode(&mnemonic_words).unwrap()),
                entropy_hex
            );

            let mnemonic =
                AlgorandMnemonic::from_entropy_bytes(&get_bytes(&entropy_hex), language).unwrap();
            assert_eq!(AlgorandMnemonic::name(), json_str(&entry["name"]));
            assert_eq!(mnemonic.language().to_lowercase(), language.to_lowercase());
        }
    }

    let invalid_words: Vec<String> = "flower letter world foil coin poverty romance tongue taste hip cradle follow proud pluck ten improve"
        .split_whitespace()
        .map(String::from)
        .collect();
    assert_error_contains(
        AlgorandMnemonic::decode(&invalid_words).map(|_| ()),
        "Invalid mnemonic words",
    );

    assert_error_contains(
        AlgorandMnemonic::from_words(100, AlgorandMnemonicLanguages::ENGLISH).map(|_| ()),
        "Invalid mnemonic words number",
    );

    assert_error_contains(
        algorand_from_entropy(MnemonicEntropyInput::Invalid, AlgorandMnemonicLanguages::ENGLISH)
            .map(|_| ()),
        "Invalid entropy instance",
    );

    assert_error_contains(
        algorand_from_entropy(
            MnemonicEntropyInput::Hex(
                "cdf694ac868efd01673fc51e897c57a0bd428503080ad4c94c7d6f6d13f095fbc8",
            ),
            AlgorandMnemonicLanguages::ENGLISH,
        )
        .map(|_| ()),
        "Wrong entropy strength",
    );
}
