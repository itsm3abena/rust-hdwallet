use regex::Regex;

use crate::hdwallet::exceptions::{MnemonicError, SeedError};
use crate::hdwallet::mnemonics::imnemonic::IMnemonic;
use crate::hdwallet::mnemonics::monero::{MoneroMnemonic, MoneroMnemonicWordCounts, mnemonic::normalize_word};
use crate::hdwallet::seeds::iseed::ISeed;
use crate::hdwallet::utils::crc32_ieee;

pub struct MoneroSeed {
    seed: String,
}

fn verify_checksum_words(words: &[String]) -> Result<(), MnemonicError> {
    if !matches!(
        words.len(),
        MoneroMnemonicWordCounts::THIRTEEN | MoneroMnemonicWordCounts::TWENTY_FIVE
    ) {
        return Ok(());
    }

    let (_words_list, language) = MoneroMnemonic::find_language(words)?;
    let prefix_len = *MoneroMnemonic::language_unique_prefix_lengths()
        .get(language.as_str())
        .ok_or_else(|| MnemonicError::simple("Unknown language for checksum"))?;

    let data_words = &words[..words.len() - 1];
    let prefixes: String = data_words
        .iter()
        .map(|w| w.chars().take(prefix_len).collect::<String>())
        .collect();

    let crc = crc32_ieee(prefixes.as_bytes()) as usize;
    let index = crc % data_words.len();
    let expected = &data_words[index];
    if words.last().map(|w| w.as_str()) != Some(expected.as_str()) {
        return Err(MnemonicError::simple("Invalid Monero mnemonic checksum"));
    }

    Ok(())
}

impl ISeed for MoneroSeed {
    const LENGTH: usize = 32;

    fn new(seed: String) -> Self {
        Self { seed }
    }

    fn name() -> &'static str {
        "Monero"
    }

    fn is_valid(seed: &str) -> bool {
        let re = Regex::new(r"^[0-9a-fA-F]+$").unwrap();
        if !re.is_match(seed) {
            return false;
        }
        matches!(seed.len(), 32 | 64)
    }

    fn seed(&self) -> &str {
        &self.seed
    }

    fn from_mnemonic(mnemonic: &dyn IMnemonic) -> Result<String, SeedError> {
        Self::from_mnemonic_phrase(&mnemonic.mnemonic())
    }
}

impl MoneroSeed {
    pub fn from_mnemonic_phrase(phrase: &str) -> Result<String, SeedError> {
        let words: Vec<String> = phrase
            .split_whitespace()
            .map(normalize_word)
            .collect();

        if !MoneroMnemonic::words_list().contains(&words.len()) {
            return Err(SeedError::new(
                &format!("Invalid {} mnemonic words", Self::name()),
                None,
                None,
                None,
            ));
        }

        if !MoneroMnemonic::is_valid(&words) {
            return Err(SeedError::new(
                &format!("Invalid {} mnemonic words", Self::name()),
                None,
                None,
                None,
            ));
        }

        verify_checksum_words(&words).map_err(|e| SeedError::new(&e.to_string(), None, None, None))?;

        let entropy = MoneroMnemonic::parse_phrase(phrase)
            .map_err(|e| SeedError::new(&e.to_string(), None, None, None))?;

        Ok(hex::encode(entropy))
    }
}
