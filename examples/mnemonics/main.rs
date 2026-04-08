//! Generate a mnemonic for any supported scheme.
//!
//! ```text
//! cargo run --example mnemonics -- <scheme> [args...]
//! ```
//!
//! **BIP39 / Monero / Electrum V2** accept:
//! - Positional: `12 korean` or `korean 12` (number and language in any order among tokens)
//! - GNU-style: `--12` or `--korean` (double-dash + number or language name)
//! - Flags: `-w 12` / `--words 12`, `-l korean` / `--language korean`
//!
//! Schemes:
//! - `bip39` — default `12 english`
//! - `monero` — default `13 english`
//! - `electrum-v1` — 12 words; `[language]` or `-l`
//! - `electrum-v2` — random: default `12 english standard`; optional `-t` / `-l`
//!   - **From hex entropy:** `electrum-v2 from-entropy <hex>` or `electrum-v2 --entropy <hex>`
//!     (optional `-l` / `-t`). Electrum may bump the integer and retry until the HMAC prefix matches.
//! - `algorand` — 25 words; `[language]` or `-l`
//!
//! Legacy: `12 english` (no scheme) is treated as **bip39**.
//!
//! Bare `--` tokens are stripped. `--` before english still works: `bip39 12 -- english`.

use hdwallet_rust::hdwallet::mnemonics::{
    AlgorandMnemonic, AlgorandMnemonicLanguages, BIP39Mnemonic, BIP39MnemonicLanguages,
    ElectrumV1Mnemonic, ElectrumV1MnemonicLanguages, ElectrumV2Mnemonic,
    ElectrumV2MnemonicLanguages, ElectrumV2MnemonicTypes, IMnemonic, MoneroMnemonic,
    MoneroMnemonicLanguages,
};

fn normalize_lang_token(s: &str) -> String {
    s.trim()
        .trim_start_matches('-')
        .to_lowercase()
        .replace('_', "-")
}

/// Pulls word counts and language-like strings from mixed positional / `--flag` argv.
fn partition_words_and_langs(tail: &[String]) -> (Vec<usize>, Vec<String>) {
    let mut nums = Vec::new();
    let mut strs = Vec::new();
    let mut i = 0;
    while i < tail.len() {
        match tail[i].as_str() {
            "-w" | "--words" => {
                i += 1;
                if i < tail.len() {
                    if let Ok(n) = tail[i].parse::<usize>() {
                        nums.push(n);
                    }
                }
            }
            "-l" | "--language" | "--lang" => {
                i += 1;
                if i < tail.len() {
                    strs.push(normalize_lang_token(&tail[i]));
                }
            }
            "-t" | "--type" => {
                i += 1;
                if i < tail.len() {
                    strs.push(tail[i].clone());
                }
            }
            other => {
                if other == "--" {
                    // already filtered in outer args; ignore if any slipped through
                } else if let Some(r) = other.strip_prefix("--") {
                    if r.is_empty() {
                        // nothing
                    } else if let Ok(n) = r.parse::<usize>() {
                        nums.push(n);
                    } else {
                        strs.push(normalize_lang_token(r));
                    }
                } else if let Ok(n) = other.parse::<usize>() {
                    nums.push(n);
                } else {
                    strs.push(normalize_lang_token(other));
                }
            }
        }
        i += 1;
    }
    (nums, strs)
}

fn first_word_count(nums: &[usize], default: usize) -> usize {
    nums.first().copied().unwrap_or(default)
}

fn first_language(strs: &[String], default: &str) -> String {
    strs.iter()
        .find(|s| {
            let t = s.to_lowercase();
            !matches!(
                t.as_str(),
                "standard" | "segwit" | "standard-2fa" | "segwit-2fa"
            )
        })
        .cloned()
        .unwrap_or_else(|| default.to_string())
}

/// `from-entropy` / `--entropy` / `-e` → hex string; remaining args are parsed for `-l`/`-t`/positional.
fn strip_electrum_v2_entropy_source(tail: &[String]) -> (Option<String>, Vec<String>) {
    let mut hex: Option<String> = None;
    let mut out = Vec::new();
    let mut i = 0;
    while i < tail.len() {
        match tail[i].as_str() {
            "from-entropy" | "from_entropy" => {
                i += 1;
                if i < tail.len() {
                    hex = Some(
                        tail[i]
                            .trim()
                            .trim_start_matches("0x")
                            .trim_start_matches("0X")
                            .to_string(),
                    );
                    i += 1;
                }
            }
            "-e" | "--entropy" => {
                i += 1;
                if i < tail.len() {
                    hex = Some(
                        tail[i]
                            .trim()
                            .trim_start_matches("0x")
                            .trim_start_matches("0X")
                            .to_string(),
                    );
                    i += 1;
                }
            }
            other => {
                out.push(other.to_string());
                i += 1;
            }
        }
    }
    (hex, out)
}

fn electrum_v2_mnemonic_type(strs: &[String]) -> String {
    for s in strs {
        let t = s.to_lowercase();
        if matches!(
            t.as_str(),
            "standard" | "segwit" | "standard-2fa" | "segwit-2fa"
        ) {
            return match t.as_str() {
                "standard" => ElectrumV2MnemonicTypes::STANDARD.to_string(),
                "segwit" => ElectrumV2MnemonicTypes::SEGWIT.to_string(),
                "standard-2fa" => ElectrumV2MnemonicTypes::STANDARD_2FA.to_string(),
                "segwit-2fa" => ElectrumV2MnemonicTypes::SEGWIT_2FA.to_string(),
                _ => s.clone(),
            };
        }
    }
    ElectrumV2MnemonicTypes::STANDARD.to_string()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args()
        .skip(1)
        .filter(|a| a != "--")
        .collect();

    let (scheme, tail) = match args.first().map(|s| s.as_str()) {
        Some(
            s @ ("bip39" | "monero" | "electrum-v1" | "electrum-v2" | "electrumv1" | "electrumv2"
            | "algorand"),
        ) => {
            let norm = match s {
                "electrumv1" => "electrum-v1",
                "electrumv2" => "electrum-v2",
                x => x,
            };
            (norm, &args[1..])
        }
        _ => ("bip39", args.as_slice()),
    };

    match scheme {
        "bip39" => {
            let (nums, strs) = partition_words_and_langs(tail);
            let wc = first_word_count(&nums, 12);
            let lang = first_language(
                &strs,
                BIP39MnemonicLanguages::ENGLISH,
            );
            let phrase = BIP39Mnemonic::from_words(wc, &lang)?;
            println!("bip39, {} words, {:?}:\n{}", wc, lang, phrase);
        }
        "monero" => {
            let (nums, strs) = partition_words_and_langs(tail);
            let wc = first_word_count(&nums, 13);
            let lang = first_language(&strs, MoneroMnemonicLanguages::ENGLISH);
            let phrase = MoneroMnemonic::from_words(wc, &lang)?;
            println!("monero, {} words, {:?}:\n{}", wc, lang, phrase);
        }
        "electrum-v1" => {
            let (_nums, strs) = partition_words_and_langs(tail);
            let lang = first_language(&strs, ElectrumV1MnemonicLanguages::ENGLISH);
            let phrase = ElectrumV1Mnemonic::from_words(12, &lang)?;
            println!("electrum-v1, 12 words, {:?}:\n{}", lang, phrase);
        }
        "electrum-v2" => {
            let (entropy_hex, rest) = strip_electrum_v2_entropy_source(tail);
            let (nums, strs) = partition_words_and_langs(&rest);
            let mtype = electrum_v2_mnemonic_type(&strs);
            let lang = first_language(&strs, ElectrumV2MnemonicLanguages::ENGLISH);

            if let Some(hex) = entropy_hex {
                if !hex.chars().all(|c| c.is_ascii_hexdigit()) {
                    return Err("entropy must be a hex string (even length, 0-9 a-f)".into());
                }
                if hex.len() % 2 == 1 {
                    return Err("entropy hex must have an even number of digits".into());
                }
                let m = ElectrumV2Mnemonic::from_entropy_with_retries(
                    &hex,
                    &lang,
                    &mtype,
                    100_000,
                )?;
                println!(
                    "electrum-v2 from entropy, {:?}, type {:?}:\n{}",
                    lang, mtype, m.phrase()
                );
            } else {
                let wc = first_word_count(&nums, 12);
                let m = ElectrumV2Mnemonic::generate_random(wc, &lang, &mtype)?;
                println!(
                    "electrum-v2, {} words, {:?}, type {:?}:\n{}",
                    wc,
                    lang,
                    mtype,
                    m.phrase()
                );
            }
        }
        "algorand" => {
            let (_nums, strs) = partition_words_and_langs(tail);
            let lang = first_language(&strs, AlgorandMnemonicLanguages::ENGLISH);
            let phrase = AlgorandMnemonic::from_words(25, &lang)?;
            println!("algorand, 25 words, {:?}:\n{}", lang, phrase);
        }
        _ => {
            eprintln!("Unknown scheme {:?}", scheme);
            std::process::exit(1);
        }
    }

    Ok(())
}
