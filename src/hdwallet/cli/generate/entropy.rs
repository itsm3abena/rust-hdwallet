use clap::Args;
use serde_json::json;

use crate::hdwallet::entropies::algorand::AlgorandEntropyStrengths;
use crate::hdwallet::entropies::bip39::BIP39EntropyStrengths;
use crate::hdwallet::entropies::electrum::{ElectrumV1EntropyStrengths, ElectrumV2EntropyStrengths};
use crate::hdwallet::entropies::monero::MoneroEntropyStrengths;
use crate::hdwallet::entropies::{
    AlgorandEntropy, BIP39Entropy, ElectrumV1Entropy, ElectrumV2Entropy, Entropies, EntropyInstance,
    MoneroEntropy,
};
use crate::hdwallet::entropies::ientropy::EntropyError as RawEntropyError;
use crate::hdwallet::exceptions::EntropyError;

#[derive(Args, Clone, Debug)]
pub struct GenerateEntropyArgs {
    #[arg(short = 'c', long = "client", default_value = "BIP39", help = "Set Entropy client")]
    pub client: String,

    #[arg(short = 's', long = "strength", help = "Set Strength for entropy")]
    pub strength: Option<u32>,
}

pub fn generate_entropy(args: GenerateEntropyArgs) -> Result<String, EntropyError> {
    if !Entropies::is_entropy(&args.client) {
        return Err(EntropyError::new(
            "Wrong entropy client",
            None,
            Some(&Entropies::names().join(", ")),
            Some(&args.client),
        ));
    }

    let strength = match args.strength {
        Some(strength) => strength as usize,
        None => default_strength(&args.client)?,
    };

    let scheme = Entropies::entropy(&args.client)?;
    if !scheme.is_valid_strength(strength) {
        let expected = scheme
            .strengths()
            .iter()
            .map(|value| value.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        return Err(EntropyError::new(
            &format!("Wrong {} entropy strength", args.client),
            None,
            Some(&expected),
            Some(&strength.to_string()),
        ));
    }

    let generated = scheme
        .generate(strength)
        .map_err(map_raw_entropy_error)?;
    let instance = wrap_entropy(&args.client, &generated)?;

    Ok(json!({
        "client": instance.client(),
        "entropy": instance.entropy(),
        "strength": instance.strength(),
    })
    .to_string())
}

fn default_strength(client: &str) -> Result<usize, EntropyError> {
    Ok(match client {
        "Algorand" => AlgorandEntropyStrengths::TWO_HUNDRED_FIFTY_SIX,
        "BIP39" => BIP39EntropyStrengths::ONE_HUNDRED_TWENTY_EIGHT,
        "Electrum-V1" => ElectrumV1EntropyStrengths::ONE_HUNDRED_TWENTY_EIGHT,
        "Electrum-V2" => ElectrumV2EntropyStrengths::ONE_HUNDRED_THIRTY_TWO,
        "Monero" => MoneroEntropyStrengths::ONE_HUNDRED_TWENTY_EIGHT,
        other => {
            return Err(EntropyError::new(
                "Wrong entropy client",
                None,
                Some(&Entropies::names().join(", ")),
                Some(other),
            ));
        }
    })
}

struct WrappedEntropy {
    client: String,
    entropy: String,
    strength: usize,
}

impl WrappedEntropy {
    fn client(&self) -> &str {
        &self.client
    }

    fn entropy(&self) -> &str {
        &self.entropy
    }

    fn strength(&self) -> usize {
        self.strength
    }
}

fn wrap_entropy(client: &str, entropy: &str) -> Result<WrappedEntropy, EntropyError> {
    let instance = match client {
        "Algorand" => EntropyInstance::new::<AlgorandEntropy>(entropy),
        "BIP39" => EntropyInstance::new::<BIP39Entropy>(entropy),
        "Electrum-V1" => EntropyInstance::new::<ElectrumV1Entropy>(entropy),
        "Electrum-V2" => EntropyInstance::new::<ElectrumV2Entropy>(entropy),
        "Monero" => EntropyInstance::new::<MoneroEntropy>(entropy),
        other => {
            return Err(EntropyError::new(
                "Wrong entropy client",
                None,
                Some(&Entropies::names().join(", ")),
                Some(other),
            ));
        }
    }
    .map_err(map_raw_entropy_error)?;

    Ok(WrappedEntropy {
        client: client.to_string(),
        entropy: instance.entropy().to_string(),
        strength: instance.strength_bits(),
    })
}

fn map_raw_entropy_error(error: RawEntropyError) -> EntropyError {
    EntropyError::new(&error.message, None, None, None)
}
