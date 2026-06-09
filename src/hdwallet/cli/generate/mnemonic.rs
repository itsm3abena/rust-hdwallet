use clap::Args;
use serde_json::{json, Value};

use crate::hdwallet::exceptions::MnemonicError;
use crate::hdwallet::mnemonics::{
    AlgorandMnemonic, AlgorandMnemonicLanguages, AlgorandMnemonicWords, BIP39Mnemonic,
    BIP39MnemonicLanguages, BIP39MnemonicWords, ElectrumV1Mnemonic, ElectrumV1MnemonicLanguages,
    ElectrumV1MnemonicWordCounts, ElectrumV2Mnemonic, ElectrumV2MnemonicLanguages,
    ElectrumV2MnemonicWordCounts, IMnemonic, Mnemonics, MoneroMnemonic, MoneroMnemonicLanguages,
    MoneroMnemonicWordCounts,
};

#[derive(Args, Clone, Debug)]
pub struct GenerateMnemonicArgs {
    #[arg(short = 'c', long = "client", default_value = "BIP39", help = "Set Mnemonic client")]
    pub client: String,

    #[arg(short = 'l', long = "language", help = "Set Mnemonic language")]
    pub language: Option<String>,

    #[arg(short = 'e', long = "entropy", help = "Set Mnemonic entropy")]
    pub entropy: Option<String>,

    #[arg(short = 'w', long = "words", help = "Set Mnemonic words")]
    pub words: Option<u32>,

    #[arg(
        long = "mnemonic-type",
        default_value = "standard",
        help = "Set Mnemonic type for Electrum-V2"
    )]
    pub mnemonic_type: String,

    #[arg(
        long = "max-attempts",
        default_value = "1000000000000000000000000000000000000000000000000000000000000",
        help = "Set Max attempts for Electrum-V2"
    )]
    pub max_attempts: String,

    #[arg(long = "checksum", default_value_t = false, help = "Set Checksum for Monero")]
    pub checksum: bool,
}

struct MnemonicOutput {
    client: String,
    mnemonic: String,
    language: String,
    words: usize,
    mnemonic_type: Option<String>,
}

pub fn generate_mnemonic(args: GenerateMnemonicArgs) -> Result<String, MnemonicError> {
    if !Mnemonics::is_mnemonic(&args.client) {
        return Err(MnemonicError::new(
            "Wrong mnemonic client",
            None,
            Some(&Mnemonics::names().join(", ")),
            Some(&args.client),
        ));
    }

    let language = args
        .language
        .as_deref()
        .unwrap_or_else(|| default_language(&args.client))
        .to_string();
    let words = args.words.map(|value| value as usize).unwrap_or_else(|| default_words(&args.client));

    validate_language(&args.client, &language)?;
    validate_words(&args.client, words)?;

    let output = if let Some(entropy) = args.entropy.as_deref() {
        generate_from_entropy(&args, &language, entropy)?
    } else {
        generate_from_words(&args, &language, words)?
    };

    Ok(serde_json::to_string(&output.to_json()).map_err(|error| MnemonicError::simple(&error.to_string()))?)
}

impl MnemonicOutput {
    fn to_json(&self) -> Value {
        let mut output = json!({
            "client": self.client,
            "mnemonic": self.mnemonic,
            "language": self.language,
            "words": self.words,
        });
        if let Some(mnemonic_type) = &self.mnemonic_type {
            output["mnemonic_type"] = json!(mnemonic_type);
        }
        output
    }
}

fn default_language(client: &str) -> &'static str {
    match client {
        "Algorand" => AlgorandMnemonicLanguages::ENGLISH,
        "BIP39" => BIP39MnemonicLanguages::ENGLISH,
        "Electrum-V1" => ElectrumV1MnemonicLanguages::ENGLISH,
        "Electrum-V2" => ElectrumV2MnemonicLanguages::ENGLISH,
        "Monero" => MoneroMnemonicLanguages::ENGLISH,
        _ => BIP39MnemonicLanguages::ENGLISH,
    }
}

fn default_words(client: &str) -> usize {
    match client {
        "Algorand" => AlgorandMnemonicWords::TWENTY_FIVE,
        "BIP39" => BIP39MnemonicWords::TWELVE,
        "Electrum-V1" => ElectrumV1MnemonicWordCounts::TWELVE,
        "Electrum-V2" => ElectrumV2MnemonicWordCounts::TWELVE,
        "Monero" => MoneroMnemonicWordCounts::TWELVE,
        _ => BIP39MnemonicWords::TWELVE,
    }
}

fn validate_language(client: &str, language: &str) -> Result<(), MnemonicError> {
    let valid = match client {
        "Algorand" => AlgorandMnemonic::is_valid_language(language),
        "BIP39" => BIP39Mnemonic::is_valid_language(language),
        "Electrum-V1" => ElectrumV1Mnemonic::is_valid_language(language),
        "Electrum-V2" => ElectrumV2Mnemonic::is_valid_language(language),
        "Monero" => MoneroMnemonic::is_valid_language(language),
        _ => false,
    };
    if valid {
        Ok(())
    } else {
        let expected = languages_for(client).join(", ");
        Err(MnemonicError::new(
            &format!("Wrong {client} mnemonic language"),
            None,
            Some(&expected),
            Some(language),
        ))
    }
}

fn validate_words(client: &str, words: usize) -> Result<(), MnemonicError> {
    let valid = match client {
        "Algorand" => AlgorandMnemonic::is_valid_words(words),
        "BIP39" => BIP39Mnemonic::is_valid_words(words),
        "Electrum-V1" => ElectrumV1Mnemonic::is_valid_words(words),
        "Electrum-V2" => ElectrumV2Mnemonic::is_valid_words(words),
        "Monero" => MoneroMnemonic::is_valid_words(words),
        _ => false,
    };
    if valid {
        Ok(())
    } else {
        let expected = words_for(client)
            .iter()
            .map(|value| value.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        Err(MnemonicError::new(
            &format!("Wrong {client} mnemonic words"),
            None,
            Some(&expected),
            Some(&words.to_string()),
        ))
    }
}

fn languages_for(client: &str) -> Vec<&'static str> {
    match client {
        "Algorand" => AlgorandMnemonic::languages().to_vec(),
        "BIP39" => BIP39Mnemonic::languages().to_vec(),
        "Electrum-V1" => ElectrumV1Mnemonic::languages().to_vec(),
        "Electrum-V2" => ElectrumV2Mnemonic::languages().to_vec(),
        "Monero" => MoneroMnemonic::languages().to_vec(),
        _ => Vec::new(),
    }
}

fn words_for(client: &str) -> Vec<usize> {
    match client {
        "Algorand" => AlgorandMnemonic::words_list().to_vec(),
        "BIP39" => BIP39Mnemonic::words_list().to_vec(),
        "Electrum-V1" => ElectrumV1Mnemonic::words_list().to_vec(),
        "Electrum-V2" => ElectrumV2Mnemonic::words_list(),
        "Monero" => MoneroMnemonic::words_list(),
        _ => Vec::new(),
    }
}

fn parse_max_attempts(value: &str) -> Result<u128, MnemonicError> {
    if let Ok(parsed) = value.parse::<u128>() {
        return Ok(parsed);
    }
    if !value.is_empty() && value.chars().all(|ch| ch.is_ascii_digit()) {
        return Ok(u128::MAX);
    }
    Err(MnemonicError::simple("Invalid max-attempts value"))
}

fn generate_from_entropy(
    args: &GenerateMnemonicArgs,
    language: &str,
    entropy_hex: &str,
) -> Result<MnemonicOutput, MnemonicError> {
    match args.client.as_str() {
        "Electrum-V2" => {
            let mnemonic = ElectrumV2Mnemonic::from_entropy_with_retries(
                entropy_hex,
                language,
                &args.mnemonic_type,
                parse_max_attempts(&args.max_attempts)?,
            )?;
            Ok(MnemonicOutput {
                client: ElectrumV2Mnemonic::name().to_string(),
                mnemonic: mnemonic.mnemonic(),
                language: mnemonic.language().to_string(),
                words: mnemonic.words(),
                mnemonic_type: Some(args.mnemonic_type.clone()),
            })
        }
        "Monero" => {
            let entropy = hex::decode(entropy_hex.trim())
                .map_err(|error| MnemonicError::simple(&error.to_string()))?;
            let mnemonic =
                MoneroMnemonic::from_entropy_with_checksum(&entropy, language, args.checksum)?;
            Ok(MnemonicOutput {
                client: MoneroMnemonic::name().to_string(),
                mnemonic: mnemonic.mnemonic(),
                language: mnemonic.language().to_string(),
                words: mnemonic.words(),
                mnemonic_type: None,
            })
        }
        "Algorand" => {
            let entropy = hex::decode(entropy_hex.trim())
                .map_err(|error| MnemonicError::simple(&error.to_string()))?;
            let mnemonic = AlgorandMnemonic::from_entropy_bytes(&entropy, language)?;
            Ok(MnemonicOutput {
                client: AlgorandMnemonic::name().to_string(),
                mnemonic: mnemonic.mnemonic(),
                language: mnemonic.language().to_string(),
                words: mnemonic.words(),
                mnemonic_type: None,
            })
        }
        "BIP39" => {
            let entropy = hex::decode(entropy_hex.trim())
                .map_err(|error| MnemonicError::simple(&error.to_string()))?;
            let mnemonic = BIP39Mnemonic::from_entropy_bytes(&entropy, language)?;
            Ok(MnemonicOutput {
                client: BIP39Mnemonic::name().to_string(),
                mnemonic: mnemonic.mnemonic(),
                language: mnemonic.language().to_string(),
                words: mnemonic.words(),
                mnemonic_type: None,
            })
        }
        "Electrum-V1" => {
            let entropy = hex::decode(entropy_hex.trim())
                .map_err(|error| MnemonicError::simple(&error.to_string()))?;
            let mnemonic = ElectrumV1Mnemonic::from_entropy_bytes(&entropy, language)?;
            Ok(MnemonicOutput {
                client: ElectrumV1Mnemonic::name().to_string(),
                mnemonic: mnemonic.mnemonic(),
                language: mnemonic.language().to_string(),
                words: mnemonic.words(),
                mnemonic_type: None,
            })
        }
        other => Err(MnemonicError::new(
            "Wrong mnemonic client",
            None,
            Some(&Mnemonics::names().join(", ")),
            Some(other),
        )),
    }
}

fn generate_from_words(
    args: &GenerateMnemonicArgs,
    language: &str,
    words: usize,
) -> Result<MnemonicOutput, MnemonicError> {
    match args.client.as_str() {
        "Electrum-V2" => {
            let mnemonic = ElectrumV2Mnemonic::generate_random(words, language, &args.mnemonic_type)?;
            Ok(MnemonicOutput {
                client: ElectrumV2Mnemonic::name().to_string(),
                mnemonic: mnemonic.mnemonic(),
                language: mnemonic.language().to_string(),
                words: mnemonic.words(),
                mnemonic_type: Some(args.mnemonic_type.clone()),
            })
        }
        "Monero" => {
            let mnemonic = MoneroMnemonic::generate_random(words, language)?;
            Ok(MnemonicOutput {
                client: MoneroMnemonic::name().to_string(),
                mnemonic: mnemonic.mnemonic(),
                language: mnemonic.language().to_string(),
                words: mnemonic.words(),
                mnemonic_type: None,
            })
        }
        "Algorand" => {
            let mnemonic = AlgorandMnemonic::generate_random(language)?;
            Ok(MnemonicOutput {
                client: AlgorandMnemonic::name().to_string(),
                mnemonic: mnemonic.mnemonic(),
                language: mnemonic.language().to_string(),
                words: mnemonic.words(),
                mnemonic_type: None,
            })
        }
        "BIP39" => {
            let mnemonic = BIP39Mnemonic::generate_random(words, language)?;
            Ok(MnemonicOutput {
                client: BIP39Mnemonic::name().to_string(),
                mnemonic: mnemonic.mnemonic(),
                language: mnemonic.language().to_string(),
                words: mnemonic.words(),
                mnemonic_type: None,
            })
        }
        "Electrum-V1" => {
            let mnemonic = ElectrumV1Mnemonic::generate_random(language)?;
            Ok(MnemonicOutput {
                client: ElectrumV1Mnemonic::name().to_string(),
                mnemonic: mnemonic.mnemonic(),
                language: mnemonic.language().to_string(),
                words: mnemonic.words(),
                mnemonic_type: None,
            })
        }
        other => Err(MnemonicError::new(
            "Wrong mnemonic client",
            None,
            Some(&Mnemonics::names().join(", ")),
            Some(other),
        )),
    }
}
