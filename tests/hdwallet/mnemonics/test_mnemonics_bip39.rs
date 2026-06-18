use super::conftest::{assert_error_contains, get_bytes, json_str, json_usize, TestData};
use rust_hdwallet::hdwallet::exceptions::EntropyError;
use rust_hdwallet::hdwallet::mnemonics::{
    bip39::{BIP39MnemonicLanguages, BIP39MnemonicWords},
    imnemonic::IMnemonic,
    BIP39Mnemonic,
};

enum MnemonicEntropyInput<'a> {
    Hex(&'a str),
    Invalid,
}

fn bip39_from_entropy(
    entropy: MnemonicEntropyInput<'_>,
    language: &str,
) -> Result<String, EntropyError> {
    match entropy {
        MnemonicEntropyInput::Hex(hex) => BIP39Mnemonic::from_entropy(&get_bytes(hex), language)
            .map_err(|error| EntropyError::new(&error.to_string(), None, None, None)),
        MnemonicEntropyInput::Invalid => Err(EntropyError::new(
            "Invalid entropy instance",
            None,
            Some("[str, bytes, BIP39Entropy]"),
            None,
        )),
    }
}

#[test]
fn test_bip39_mnemonics() {
    let data = TestData::load();
    let entries = data.mnemonics["BIP39"]
        .as_array()
        .expect("BIP39 mnemonics array");

    assert_eq!(BIP39MnemonicWords::TWELVE, 12);
    assert_eq!(BIP39MnemonicWords::FIFTEEN, 15);
    assert_eq!(BIP39MnemonicWords::EIGHTEEN, 18);
    assert_eq!(BIP39MnemonicWords::TWENTY_ONE, 21);
    assert_eq!(BIP39MnemonicWords::TWENTY_FOUR, 24);

    assert_eq!(
        BIP39MnemonicLanguages::CHINESE_SIMPLIFIED,
        "chinese-simplified"
    );
    assert_eq!(
        BIP39MnemonicLanguages::CHINESE_TRADITIONAL,
        "chinese-traditional"
    );
    assert_eq!(BIP39MnemonicLanguages::CZECH, "czech");
    assert_eq!(BIP39MnemonicLanguages::ENGLISH, "english");
    assert_eq!(BIP39MnemonicLanguages::FRENCH, "french");
    assert_eq!(BIP39MnemonicLanguages::ITALIAN, "italian");
    assert_eq!(BIP39MnemonicLanguages::JAPANESE, "japanese");
    assert_eq!(BIP39MnemonicLanguages::KOREAN, "korean");
    assert_eq!(BIP39MnemonicLanguages::PORTUGUESE, "portuguese");
    assert_eq!(BIP39MnemonicLanguages::RUSSIAN, "russian");
    assert_eq!(BIP39MnemonicLanguages::SPANISH, "spanish");
    assert_eq!(BIP39MnemonicLanguages::TURKISH, "turkish");

    for entry in entries {
        let words = json_usize(&entry["words"]);
        assert!(BIP39Mnemonic::is_valid_words(words));

        for (language, mnemonic_text) in entry["languages"].as_object().expect("languages") {
            let mnemonic_text = json_str(mnemonic_text);
            let mnemonic_words: Vec<String> = mnemonic_text
                .split_whitespace()
                .map(String::from)
                .collect();

            assert!(BIP39Mnemonic::is_valid_language(language));
            assert!(BIP39Mnemonic::is_valid(&mnemonic_words));

            let generated = BIP39Mnemonic::from_words(words, language).unwrap();
            assert_eq!(generated.split_whitespace().count(), words);
            assert_eq!(
                BIP39Mnemonic::parse_phrase(&generated)
                    .unwrap()
                    .language()
                    .to_lowercase(),
                language.to_lowercase()
            );

            let entropy_hex = json_str(&entry["entropy"]);
            assert_eq!(
                bip39_from_entropy(MnemonicEntropyInput::Hex(&entropy_hex), language).unwrap(),
                mnemonic_text
            );
            assert_eq!(
                hex::encode(BIP39Mnemonic::decode(&mnemonic_words).unwrap()),
                entropy_hex
            );

            let mnemonic = BIP39Mnemonic::parse_phrase(&mnemonic_text).unwrap();
            assert_eq!(BIP39Mnemonic::name(), json_str(&entry["name"]));
            assert_eq!(mnemonic.language().to_lowercase(), language.to_lowercase());
        }
    }

    assert_error_contains(
        BIP39Mnemonic::parse_phrase(
            "judge pigeon dove nerve blood fossil wet suggest this level bottom journey tornado \
             hurt ritual hobby label alpha fruit ensure unit animal mouse absorb ramp",
        )
        .map(|_| ()),
        "Invalid mnemonic words",
    );

    assert_error_contains(
        BIP39Mnemonic::from_words(100, BIP39MnemonicLanguages::ENGLISH).map(|_| ()),
        "Invalid mnemonic words number",
    );

    assert_error_contains(
        bip39_from_entropy(MnemonicEntropyInput::Invalid, BIP39MnemonicLanguages::ENGLISH)
            .map(|_| ()),
        "Invalid entropy instance",
    );

    assert_error_contains(
        bip39_from_entropy(
            MnemonicEntropyInput::Hex(
                "cdf694ac868efd01673fc51e897c57a0bd428503080ad4c94c7d6f6d13f095fbc8",
            ),
            BIP39MnemonicLanguages::ENGLISH,
        )
        .map(|_| ()),
        "Wrong entropy strength",
    );
}
