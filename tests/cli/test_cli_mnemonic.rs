use serde_json::Value;

use super::conftest::{cli_tester, json_usize, TestData};

fn check_mnemonics(
    cli_word: &str,
    cli_entropy: &str,
    client: &str,
    language: &str,
    words: usize,
    mnemonic: &str,
) {
    let output_word: Value = serde_json::from_str(cli_word).expect("word json");
    let output_entropy: Value = serde_json::from_str(cli_entropy).expect("entropy json");

    assert_eq!(output_word["client"], client);
    assert_eq!(output_entropy["client"], client);
    assert_eq!(json_usize(&output_word["words"]), words);
    assert_eq!(json_usize(&output_entropy["words"]), words);
    assert_eq!(
        output_word["language"].as_str().unwrap().to_lowercase(),
        language.to_lowercase()
    );
    assert_eq!(
        output_entropy["language"].as_str().unwrap().to_lowercase(),
        language.to_lowercase()
    );
    assert_eq!(output_entropy["mnemonic"], mnemonic);
}

#[test]
fn test_cli_mnemonic() {
    let data = TestData::load();
    let mnemonics = data.mnemonics.as_object().expect("mnemonics object");

    for (client, entries) in mnemonics {
        let entries = entries.as_array().expect("mnemonic entries");
        for mnemonic_data in entries {
            if client == "Electrum-V2" {
                let mnemonic_types = mnemonic_data["mnemonic-types"]
                    .as_object()
                    .expect("mnemonic-types");
                for (mnemonic_type, languages) in mnemonic_types {
                    let languages = languages.as_object().expect("languages");
                    for (language, language_data) in languages {
                        let cli_word = cli_tester(&[
                            "generate",
                            "mnemonic",
                            "--client",
                            client,
                            "--mnemonic-type",
                            mnemonic_type,
                            "--words",
                            &mnemonic_data["words"].to_string(),
                            "--language",
                            language,
                        ])
                        .expect("word mnemonic");

                        let cli_entropy = cli_tester(&[
                            "generate",
                            "mnemonic",
                            "--client",
                            client,
                            "--mnemonic-type",
                            mnemonic_type,
                            "--entropy",
                            mnemonic_data["entropy-not-suitable"]
                                .as_str()
                                .expect("entropy-not-suitable"),
                            "--language",
                            language,
                        ])
                        .expect("entropy mnemonic");

                        check_mnemonics(
                            &cli_word,
                            &cli_entropy,
                            client,
                            language,
                            json_usize(&mnemonic_data["words"]),
                            language_data["mnemonic"].as_str().expect("mnemonic"),
                        );
                    }
                }
            } else {
                let languages = mnemonic_data["languages"].as_object().expect("languages");
                for (language, mnemonic) in languages {
                    let cli_word = cli_tester(&[
                        "generate",
                        "mnemonic",
                        "--client",
                        client,
                        "--words",
                        &mnemonic_data["words"].to_string(),
                        "--language",
                        language,
                    ])
                    .expect("word mnemonic");

                    let mut entropy_args = vec![
                        "generate",
                        "mnemonic",
                        "--client",
                        client,
                        "--entropy",
                        mnemonic_data["entropy"].as_str().expect("entropy"),
                        "--language",
                        language,
                    ];
                    let checksum_value;
                    if client == "Monero" {
                        checksum_value = mnemonic_data["checksum"].to_string();
                        entropy_args.push("--checksum");
                        entropy_args.push(&checksum_value);
                    }

                    let cli_entropy = cli_tester(&entropy_args).expect("entropy mnemonic");

                    check_mnemonics(
                        &cli_word,
                        &cli_entropy,
                        client,
                        language,
                        json_usize(&mnemonic_data["words"]),
                        mnemonic.as_str().expect("mnemonic"),
                    );
                }
            }
        }
    }
}
