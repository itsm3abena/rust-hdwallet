use clap::Args;
use serde_json::{json, Value};

use crate::hdwallet::exceptions::SeedError;
use crate::hdwallet::mnemonics::{
    AlgorandMnemonic, BIP39Mnemonic, ElectrumV1Mnemonic, ElectrumV2Mnemonic, IMnemonic, MoneroMnemonic,
};
use crate::hdwallet::seeds::{
    AlgorandSeed, BIP39Seed, CardanoSeed, CardanoTypes, ElectrumV1Seed, ElectrumV2Seed, ISeed,
    MoneroSeed,
};

#[derive(Args, Clone, Debug)]
pub struct GenerateSeedArgs {
    #[arg(short = 'c', long = "client", default_value = "BIP39", help = "Set Seed client")]
    pub client: String,

    #[arg(short = 'm', long = "mnemonic", help = "Set Seed mnemonic")]
    pub mnemonic: Option<String>,

    #[arg(short = 'p', long = "passphrase", help = "Set Seed passphrase")]
    pub passphrase: Option<String>,

    #[arg(
        long = "cardano-type",
        default_value = CardanoTypes::BYRON_ICARUS,
        help = "Set Cardano Seed type"
    )]
    pub cardano_type: String,

    #[arg(
        long = "mnemonic-type",
        default_value = "standard",
        help = "Set Mnemonic type for Electrum-V2"
    )]
    pub mnemonic_type: String,
}

fn seed_names() -> Vec<&'static str> {
    vec![
        AlgorandSeed::name(),
        BIP39Seed::name(),
        CardanoSeed::name(),
        ElectrumV1Seed::name(),
        ElectrumV2Seed::name(),
        MoneroSeed::name(),
    ]
}

fn is_seed(name: &str) -> bool {
    seed_names().contains(&name)
}

pub fn generate_seed(args: GenerateSeedArgs) -> Result<String, SeedError> {
    let mnemonic = args.mnemonic.as_deref().ok_or_else(|| {
        SeedError::new(
            &format!("Mnemonic is required for {} client", args.client),
            None,
            None,
            None,
        )
    })?;

    if !is_seed(&args.client) {
        return Err(SeedError::new(
            "Wrong seed client",
            None,
            Some(&seed_names().join(", ")),
            Some(&args.client),
        ));
    }

    validate_mnemonic(&args.client, mnemonic, &args.mnemonic_type)?;

    let seed_hex = match args.client.as_str() {
        "BIP39" => BIP39Seed::from_mnemonic_with_passphrase(mnemonic, args.passphrase.as_deref())?,
        "Cardano" => CardanoSeed::from_mnemonic_with_type(
            mnemonic,
            args.passphrase.as_deref(),
            &args.cardano_type,
        )?,
        "Electrum-V2" => ElectrumV2Seed::from_mnemonic_phrase(
            mnemonic,
            args.passphrase.as_deref(),
            &args.mnemonic_type,
        )?,
        "Electrum-V1" => ElectrumV1Seed::from_mnemonic_phrase(mnemonic)?,
        "Monero" => MoneroSeed::from_mnemonic_phrase(mnemonic)?,
        "Algorand" => AlgorandSeed::from_mnemonic_str(mnemonic)?,
        other => {
            return Err(SeedError::new(
                "Wrong seed client",
                None,
                Some(&seed_names().join(", ")),
                Some(other),
            ));
        }
    };

    Ok(serde_json::to_string(&output_json(
        &args.client,
        &seed_hex,
        &args.cardano_type,
        &args.mnemonic_type,
    ))
    .map_err(|error| SeedError::new(&error.to_string(), None, None, None))?)
}

fn output_json(client: &str, seed: &str, cardano_type: &str, mnemonic_type: &str) -> Value {
    let mut output = json!({
        "client": client,
        "seed": seed,
    });
    if client == CardanoSeed::name() {
        output["cardano_type"] = json!(cardano_type);
    } else if client == ElectrumV2Seed::name() {
        output["mnemonic_type"] = json!(mnemonic_type);
    }
    output
}

fn validate_mnemonic(client: &str, mnemonic: &str, mnemonic_type: &str) -> Result<(), SeedError> {
    let words: Vec<String> = mnemonic
        .split_whitespace()
        .map(|word| word.to_string())
        .collect();

    let valid = match client {
        "Electrum-V2" => ElectrumV2Mnemonic::is_valid(&words, mnemonic_type),
        "Cardano" => BIP39Mnemonic::is_valid_mnemonic_words(&words),
        "BIP39" => BIP39Mnemonic::is_valid_mnemonic_words(&words),
        "Electrum-V1" => ElectrumV1Mnemonic::is_valid_phrase(mnemonic, None, None),
        "Monero" => MoneroMnemonic::parse_phrase(mnemonic).is_ok(),
        "Algorand" => AlgorandMnemonic::decode(&words).is_ok(),
        other => {
            return Err(SeedError::new(
                "Wrong seed client",
                None,
                Some(&seed_names().join(", ")),
                Some(other),
            ));
        }
    };

    if valid {
        Ok(())
    } else {
        let mnemonic_name = if client == CardanoSeed::name() {
            BIP39Mnemonic::name()
        } else if client == "Electrum-V2" {
            ElectrumV2Mnemonic::name()
        } else {
            client
        };
        Err(SeedError::new(
            &format!("Invalid {mnemonic_name} mnemonic"),
            None,
            None,
            None,
        ))
    }
}
