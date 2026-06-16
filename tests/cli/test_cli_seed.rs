use serde_json::Value;

use super::conftest::{cli_tester, TestData};

#[test]
fn test_cli_seed() {
    let data = TestData::load();
    let seeds = data.seeds.as_object().expect("seeds object");

    for (client, words_map) in seeds {
        let words_map = words_map.as_object().expect("words map");
        for (words, client_words) in words_map {
            if client == "Electrum-V2" {
                let mnemonic_types = client_words.as_object().expect("mnemonic types");
                for (mnemonic_type, languages) in mnemonic_types {
                    let languages = languages.as_object().expect("languages");
                    for (language, language_data) in languages {
                        let mut cli_args = vec![
                            "generate",
                            "seed",
                            "--client",
                            client,
                            "--mnemonic-type",
                            mnemonic_type,
                            "--mnemonic",
                            language_data["mnemonic"].as_str().expect("mnemonic"),
                        ];
                        let seed = if language_data["passphrases"].is_null() {
                            language_data["non-passphrase-seed"]
                                .as_str()
                                .expect("non-passphrase-seed")
                        } else {
                            cli_args.push("--passphrase");
                            cli_args.push("hdwallet");
                            language_data["passphrases"]["hdwallet"]
                                .as_str()
                                .expect("passphrase seed")
                        };

                        let output = cli_tester(&cli_args)
                            .unwrap_or_else(|_| panic!("{client}/{words}/{mnemonic_type}/{language}"));
                        let parsed: Value = serde_json::from_str(&output).expect("seed json");
                        assert_eq!(parsed["client"], *client);
                        assert_eq!(parsed["seed"], seed);
                    }
                }
            } else if client == "Cardano" {
                let cardano_types = client_words.as_object().expect("cardano types");
                for (cardano_type, languages) in cardano_types {
                    let languages = languages.as_object().expect("languages");
                    for (language, language_data) in languages {
                        let mut cli_args = vec![
                            "generate",
                            "seed",
                            "--client",
                            client,
                            "--cardano-type",
                            cardano_type,
                            "--mnemonic",
                            language_data["mnemonic"].as_str().expect("mnemonic"),
                        ];
                        let seed = if language_data["passphrases"].is_null() {
                            language_data["non-passphrase-seed"]
                                .as_str()
                                .expect("non-passphrase-seed")
                        } else {
                            cli_args.push("--passphrase");
                            cli_args.push("hdwallet");
                            language_data["passphrases"]["hdwallet"]
                                .as_str()
                                .expect("passphrase seed")
                        };

                        let output = cli_tester(&cli_args)
                            .unwrap_or_else(|_| panic!("{client}/{words}/{cardano_type}/{language}"));
                        let parsed: Value = serde_json::from_str(&output).expect("seed json");
                        assert_eq!(parsed["client"], *client);
                        assert_eq!(parsed["seed"], seed);
                    }
                }
            } else {
                let languages = client_words.as_object().expect("languages");
                for (language, language_data) in languages {
                    let mut cli_args = vec![
                        "generate",
                        "seed",
                        "--client",
                        client,
                        "--mnemonic",
                        language_data["mnemonic"].as_str().expect("mnemonic"),
                    ];
                    let seed = if language_data["passphrases"].is_null() {
                        language_data["non-passphrase-seed"]
                            .as_str()
                            .expect("non-passphrase-seed")
                    } else {
                        cli_args.push("--passphrase");
                        cli_args.push("hdwallet");
                        language_data["passphrases"]["hdwallet"]
                            .as_str()
                            .expect("passphrase seed")
                    };

                    let output = cli_tester(&cli_args)
                        .unwrap_or_else(|_| panic!("{client}/{words}/{language}"));
                    let parsed: Value = serde_json::from_str(&output).expect("seed json");
                    assert_eq!(parsed["client"], *client);
                    assert_eq!(parsed["seed"], seed);
                }
            }
        }
    }
}
