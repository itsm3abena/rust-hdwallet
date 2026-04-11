//! Mnemonic -> hex seed demos.
//!
//! ```text
//! cargo run --example seeds -- <scheme> [options] <mnemonic words...>
//! ```
//!
//! Schemes:
//! - **`bip39`** — optional `-p` / `--passphrase` (BIP39 PBKDF2 seed, 64 bytes as hex)
//! - **`algorand`** — 25-word Algorand phrase → entropy hex
//! - **`monero`** — 13 / 25 words → entropy hex
//! - **`electrum-v1`** (alias `electrumv1`) — Electrum v1 phrase → seed hex
//! - **`electrum-v2`** (alias `electrumv2`) — optional `-p` / `--passphrase`, `-t` / `--type`
//!   (`standard`, `segwit`, `standard-2fa`, `segwit-2fa`)
//! - **`cardano`** — optional `-t` / `--type` (`byron-icarus`, `byron-ledger`, …), optional `-p` for ledger paths
//!
//! Bare `--` tokens are stripped.

use hdwallet_rust::hdwallet::mnemonics::ElectrumV2MnemonicTypes;
use hdwallet_rust::hdwallet::seeds::{
    AlgorandSeed, BIP39Seed, CardanoSeed, CardanoTypes, ElectrumV1Seed, ElectrumV2Seed, MoneroSeed,
};

fn parse_passphrase_type_tail(args: &[String], default_type: &str) -> (Option<String>, String, Vec<String>) {
    let mut passphrase: Option<String> = None;
    let mut kind = default_type.to_string();
    let mut words: Vec<String> = Vec::new();
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "-p" | "--passphrase" => {
                i += 1;
                if i < args.len() {
                    passphrase = Some(args[i].clone());
                }
            }
            "-t" | "--type" => {
                i += 1;
                if i < args.len() {
                    kind = args[i].clone();
                }
            }
            other => words.push(other.to_string()),
        }
        i += 1;
    }
    (passphrase, kind, words)
}

fn parse_bip39_tail(args: &[String]) -> (Option<String>, Vec<String>) {
    let mut passphrase: Option<String> = None;
    let mut words: Vec<String> = Vec::new();
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "-p" | "--passphrase" => {
                i += 1;
                if i < args.len() {
                    passphrase = Some(args[i].clone());
                }
            }
            other => words.push(other.to_string()),
        }
        i += 1;
    }
    (passphrase, words)
}

fn normalize_type_token(s: &str) -> String {
    s.trim().to_lowercase().replace('_', "-")
}

fn resolve_cardano_type(token: &str) -> Option<&'static str> {
    let n = normalize_type_token(token);
    CardanoTypes::all()
        .into_iter()
        .find(|t| normalize_type_token(t) == n)
}

fn print_usage() {
    eprintln!(
        "Usage: cargo run --example seeds -- <scheme> [options] <mnemonic words...>\n\
         Schemes: bip39 | algorand | monero | electrum-v1 | electrum-v2 | cardano\n\
         bip39:        [-p|--passphrase SECRET]\n\
         electrum-v2: [-p|--passphrase SECRET] [-t|--type standard|segwit|...]\n\
         cardano:      [-t|--type byron-icarus|...] [-p|--passphrase SECRET]"
    );
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).filter(|a| a != "--").collect();

    let scheme = args.first().map(|s| s.as_str()).unwrap_or("");
    if scheme.is_empty() || matches!(scheme, "-h" | "--help" | "help") {
        print_usage();
        std::process::exit(1);
    }

    let tail: &[String] = &args[1..];

    match scheme {
        "bip39" => {
            let (passphrase, word_vec) = parse_bip39_tail(tail);
            if word_vec.is_empty() {
                return Err("bip39 needs mnemonic words (after any -p)".into());
            }
            let joined = word_vec.join(" ");
            let seed_hex = BIP39Seed::from_mnemonic_with_passphrase(&joined, passphrase.as_deref())?;
            println!("bip39 seed (hex):\n{}", seed_hex);
        }
        "algorand" => {
            let joined = tail.join(" ");
            if joined.trim().is_empty() {
                return Err("algorand needs 25 mnemonic words".into());
            }
            let seed_hex = AlgorandSeed::from_mnemonic_str(&joined)?;
            println!("algorand seed / entropy (hex):\n{}", seed_hex);
        }
        "monero" => {
            let joined = tail.join(" ");
            if joined.trim().is_empty() {
                return Err("monero needs mnemonic words".into());
            }
            let seed_hex = MoneroSeed::from_mnemonic_phrase(&joined)?;
            println!("monero seed (hex):\n{}", seed_hex);
        }
        "electrum-v1" | "electrumv1" => {
            let joined = tail.join(" ");
            if joined.trim().is_empty() {
                return Err("electrum-v1 needs mnemonic words".into());
            }
            let seed_hex = ElectrumV1Seed::from_mnemonic_phrase(&joined)?;
            println!("electrum-v1 seed (hex):\n{}", seed_hex);
        }
        "electrum-v2" | "electrumv2" => {
            let (passphrase, mnemonic_type, word_vec) =
                parse_passphrase_type_tail(tail, ElectrumV2MnemonicTypes::STANDARD);
            if word_vec.is_empty() {
                return Err("electrum-v2 needs mnemonic words after any flags".into());
            }
            let joined = word_vec.join(" ");
            let seed_hex = ElectrumV2Seed::from_mnemonic_phrase(
                &joined,
                passphrase.as_deref(),
                &mnemonic_type,
            )?;
            println!(
                "electrum-v2 seed (hex), type {:?}:\n{}",
                mnemonic_type, seed_hex
            );
        }
        "cardano" => {
            let (passphrase, type_token, word_vec) =
                parse_passphrase_type_tail(tail, CardanoTypes::BYRON_ICARUS);
            if word_vec.is_empty() {
                return Err("cardano needs mnemonic words (BIP39-style)".into());
            }
            let final_type = resolve_cardano_type(&type_token).ok_or_else(|| {
                format!(
                    "Unknown cardano type {:?}. Use one of {:?}",
                    type_token,
                    CardanoTypes::all()
                )
            })?;
            let joined = word_vec.join(" ");
            let seed_hex = CardanoSeed::from_mnemonic_with_type(
                &joined,
                passphrase.as_deref(),
                final_type,
            )?;
            println!("cardano seed (hex), type {:?}:\n{}", final_type, seed_hex);
        }
        _ => {
            eprintln!(
                "Unknown scheme {:?}. Use: bip39 | algorand | monero | electrum-v1 | electrum-v2 | cardano",
                scheme
            );
            std::process::exit(1);
        }
    }

    Ok(())
}
