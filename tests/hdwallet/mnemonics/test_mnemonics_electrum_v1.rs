use super::conftest::{assert_error_contains, get_bytes, json_str, json_usize, TestData};
use rust_hdwallet::hdwallet::exceptions::EntropyError;
use rust_hdwallet::hdwallet::mnemonics::{
    electrum::{ElectrumV1MnemonicLanguages, ElectrumV1MnemonicWordCounts},
    imnemonic::IMnemonic,
    ElectrumV1Mnemonic,
};

enum MnemonicEntropyInput<'a> {
    Hex(&'a str),
    Invalid,
}

fn electrum_v1_from_entropy(
    entropy: MnemonicEntropyInput<'_>,
    language: &str,
) -> Result<String, EntropyError> {
    match entropy {
        MnemonicEntropyInput::Hex(hex) => {
            ElectrumV1Mnemonic::from_entropy(&get_bytes(hex), language)
                .map_err(|error| EntropyError::new(&error.to_string(), None, None, None))
        }
        MnemonicEntropyInput::Invalid => Err(EntropyError::new(
            "Invalid entropy instance",
            None,
            Some("[str, bytes, ElectrumV1Entropy]"),
            None,
        )),
    }
}

#[test]
fn test_electrum_v1_mnemonics() {
    let data = TestData::load();
    let entries = data.mnemonics["Electrum-V1"]
        .as_array()
        .expect("Electrum-V1 mnemonics array");

    assert_eq!(ElectrumV1MnemonicWordCounts::TWELVE, 12);
    assert_eq!(ElectrumV1MnemonicLanguages::ENGLISH, "english");

    for entry in entries {
        let words = json_usize(&entry["words"]);
        assert!(ElectrumV1Mnemonic::is_valid_words(words));

        for (language, mnemonic_text) in entry["languages"].as_object().expect("languages") {
            let mnemonic_text = json_str(mnemonic_text);
            let mnemonic_words: Vec<String> = mnemonic_text
                .split_whitespace()
                .map(String::from)
                .collect();

            assert!(ElectrumV1Mnemonic::is_valid_language(language));
            assert!(ElectrumV1Mnemonic::is_valid(&mnemonic_words));

            let generated = ElectrumV1Mnemonic::from_words(words, language).unwrap();
            let generated_words: Vec<String> = generated
                .split_whitespace()
                .map(String::from)
                .collect();
            assert_eq!(generated_words.len(), words);
            let (_, detected_language) =
                ElectrumV1Mnemonic::find_language(&generated_words).unwrap();
            assert_eq!(detected_language.to_lowercase(), language.to_lowercase());

            let entropy_hex = json_str(&entry["entropy"]);
            assert_eq!(
                electrum_v1_from_entropy(MnemonicEntropyInput::Hex(&entropy_hex), language)
                    .unwrap(),
                mnemonic_text
            );
            assert_eq!(
                hex::encode(ElectrumV1Mnemonic::decode(&mnemonic_words).unwrap()),
                entropy_hex
            );

            let mnemonic =
                ElectrumV1Mnemonic::from_entropy_bytes(&get_bytes(&entropy_hex), language).unwrap();
            assert_eq!(ElectrumV1Mnemonic::name(), json_str(&entry["name"]));
            assert_eq!(mnemonic.language().to_lowercase(), language.to_lowercase());
        }
    }

    assert_error_contains(
        ElectrumV1Mnemonic::decode_phrase(
            "flower letter world foil coin poverty romance tongue taste hip cradle follow proud pluck ten improve",
            None,
            None,
        )
        .map(|_| ()),
        "Invalid mnemonic words",
    );

    assert_error_contains(
        ElectrumV1Mnemonic::from_words(100, ElectrumV1MnemonicLanguages::ENGLISH).map(|_| ()),
        "Invalid mnemonic words number",
    );

    assert_error_contains(
        electrum_v1_from_entropy(
            MnemonicEntropyInput::Invalid,
            ElectrumV1MnemonicLanguages::ENGLISH,
        )
        .map(|_| ()),
        "Invalid entropy instance",
    );

    assert_error_contains(
        electrum_v1_from_entropy(
            MnemonicEntropyInput::Hex(
                "cdf694ac868efd01673fc51e897c57a0bd428503080ad4c94c7d6f6d13f095fbc8",
            ),
            ElectrumV1MnemonicLanguages::ENGLISH,
        )
        .map(|_| ()),
        "Wrong entropy strength",
    );
}
