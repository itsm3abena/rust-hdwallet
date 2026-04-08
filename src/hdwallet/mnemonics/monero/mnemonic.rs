use std::collections::{HashMap, HashSet};

use crate::hdwallet::entropies::monero::MoneroEntropyStrengths;
use crate::hdwallet::entropies::{IEntropy, MoneroEntropy};
use crate::hdwallet::exceptions::MnemonicError;
use crate::hdwallet::utils::{
    bytes_chunk_to_words_le, crc32_ieee, parse_wordlist_text, words_to_bytes_chunk_le,
};
use crate::hdwallet::mnemonics::imnemonic::IMnemonic;

pub struct MoneroMnemonicWordCounts;

impl MoneroMnemonicWordCounts {
    pub const TWELVE: usize = 12;
    pub const THIRTEEN: usize = 13;
    pub const TWENTY_FOUR: usize = 24;
    pub const TWENTY_FIVE: usize = 25;
}

pub struct MoneroMnemonicLanguages;

impl MoneroMnemonicLanguages {
    pub const CHINESE_SIMPLIFIED: &'static str = "chinese-simplified";
    pub const DUTCH: &'static str = "dutch";
    pub const ENGLISH: &'static str = "english";
    pub const FRENCH: &'static str = "french";
    pub const GERMAN: &'static str = "german";
    pub const ITALIAN: &'static str = "italian";
    pub const JAPANESE: &'static str = "japanese";
    pub const PORTUGUESE: &'static str = "portuguese";
    pub const RUSSIAN: &'static str = "russian";
    pub const SPANISH: &'static str = "spanish";
}

#[derive(Clone, Debug)]
pub struct MoneroMnemonic {
    words: Vec<String>,
    language: String,
}

impl MoneroMnemonic {
    pub const WORDS_LIST_NUMBER: usize = 1626;

    pub fn words_list() -> Vec<usize> {
        vec![
            MoneroMnemonicWordCounts::TWELVE,
            MoneroMnemonicWordCounts::THIRTEEN,
            MoneroMnemonicWordCounts::TWENTY_FOUR,
            MoneroMnemonicWordCounts::TWENTY_FIVE,
        ]
    }

    pub fn words_checksum() -> Vec<usize> {
        vec![
            MoneroMnemonicWordCounts::THIRTEEN,
            MoneroMnemonicWordCounts::TWENTY_FIVE,
        ]
    }

    pub fn words_to_entropy_strength() -> HashMap<usize, usize> {
        HashMap::from([
            (
                MoneroMnemonicWordCounts::TWELVE,
                MoneroEntropyStrengths::ONE_HUNDRED_TWENTY_EIGHT,
            ),
            (
                MoneroMnemonicWordCounts::THIRTEEN,
                MoneroEntropyStrengths::ONE_HUNDRED_TWENTY_EIGHT,
            ),
            (
                MoneroMnemonicWordCounts::TWENTY_FOUR,
                MoneroEntropyStrengths::TWO_HUNDRED_FIFTY_SIX,
            ),
            (
                MoneroMnemonicWordCounts::TWENTY_FIVE,
                MoneroEntropyStrengths::TWO_HUNDRED_FIFTY_SIX,
            ),
        ])
    }

    pub fn languages() -> &'static [&'static str] {
        &[
            MoneroMnemonicLanguages::CHINESE_SIMPLIFIED,
            MoneroMnemonicLanguages::DUTCH,
            MoneroMnemonicLanguages::ENGLISH,
            MoneroMnemonicLanguages::FRENCH,
            MoneroMnemonicLanguages::GERMAN,
            MoneroMnemonicLanguages::ITALIAN,
            MoneroMnemonicLanguages::JAPANESE,
            MoneroMnemonicLanguages::PORTUGUESE,
            MoneroMnemonicLanguages::RUSSIAN,
            MoneroMnemonicLanguages::SPANISH,
        ]
    }

    pub fn language_unique_prefix_lengths() -> HashMap<&'static str, usize> {
        HashMap::from([
            (MoneroMnemonicLanguages::CHINESE_SIMPLIFIED, 1),
            (MoneroMnemonicLanguages::DUTCH, 4),
            (MoneroMnemonicLanguages::ENGLISH, 3),
            (MoneroMnemonicLanguages::FRENCH, 4),
            (MoneroMnemonicLanguages::GERMAN, 4),
            (MoneroMnemonicLanguages::ITALIAN, 4),
            (MoneroMnemonicLanguages::JAPANESE, 4),
            (MoneroMnemonicLanguages::PORTUGUESE, 4),
            (MoneroMnemonicLanguages::SPANISH, 4),
            (MoneroMnemonicLanguages::RUSSIAN, 4),
        ])
    }

    fn wordlist_embedded(language: &str) -> Result<&'static str, MnemonicError> {
        match language {
            MoneroMnemonicLanguages::ENGLISH => Ok(include_str!("wordlist/english.txt")),
            MoneroMnemonicLanguages::CHINESE_SIMPLIFIED => {
                Ok(include_str!("wordlist/chinese_simplified.txt"))
            }
            MoneroMnemonicLanguages::DUTCH => Ok(include_str!("wordlist/dutch.txt")),
            MoneroMnemonicLanguages::FRENCH => Ok(include_str!("wordlist/french.txt")),
            MoneroMnemonicLanguages::GERMAN => Ok(include_str!("wordlist/german.txt")),
            MoneroMnemonicLanguages::ITALIAN => Ok(include_str!("wordlist/italian.txt")),
            MoneroMnemonicLanguages::JAPANESE => Ok(include_str!("wordlist/japanese.txt")),
            MoneroMnemonicLanguages::PORTUGUESE => Ok(include_str!("wordlist/portuguese.txt")),
            MoneroMnemonicLanguages::RUSSIAN => Ok(include_str!("wordlist/russian.txt")),
            MoneroMnemonicLanguages::SPANISH => Ok(include_str!("wordlist/spanish.txt")),
            _ => Err(MnemonicError::simple(&format!("Unknown Monero language: {}", language))),
        }
    }

    pub fn get_words_list_by_language(language: &str) -> Result<Vec<String>, MnemonicError> {
        let text = Self::wordlist_embedded(language)?;
        let mut words = parse_wordlist_text(text);
        if words.len() != Self::WORDS_LIST_NUMBER {
            return Err(MnemonicError::simple("Invalid Monero wordlist length"));
        }
        for w in &mut words {
            *w = w.to_lowercase();
        }
        Ok(words)
    }

    pub fn find_language(mnemonic_words: &[String]) -> Result<(Vec<String>, String), MnemonicError> {
        let lowered: Vec<String> = mnemonic_words.iter().map(|w| w.to_lowercase()).collect();
        for &lang in Self::languages() {
            let list = Self::get_words_list_by_language(lang)?;
            let set: HashSet<_> = list.iter().cloned().collect();
            if lowered.iter().all(|w| set.contains(w)) {
                return Ok((list, lang.to_string()));
            }
        }
        Err(MnemonicError::simple("Could not detect Monero mnemonic language"))
    }

    pub fn generate_random(word_count: usize, language: &str) -> Result<Self, MnemonicError> {
        if !Self::words_list().contains(&word_count) {
            return Err(MnemonicError::simple(&format!(
                "Invalid Monero word count {}",
                word_count
            )));
        }
        let bits = *Self::words_to_entropy_strength()
            .get(&word_count)
            .ok_or_else(|| MnemonicError::simple("Word count map missing"))?;
        let scheme = MoneroEntropy;
        let hex = scheme
            .generate(bits)
            .map_err(|e| MnemonicError::simple(&e.to_string()))?;
        let entropy = hex::decode(hex.trim())
            .map_err(|e| MnemonicError::simple(&e.to_string()))?;
        let checksum = Self::words_checksum().contains(&word_count);
        Self::from_entropy_with_checksum(&entropy, language, checksum)
    }

    pub fn from_entropy_with_checksum(
        entropy: &[u8],
        language: &str,
        checksum: bool,
    ) -> Result<Self, MnemonicError> {
        let scheme = MoneroEntropy;
        if !scheme.is_valid_bytes_strength(entropy.len()) {
            return Err(MnemonicError::simple("Wrong entropy strength for Monero"));
        }

        let mut mnemonic: Vec<String> = Vec::new();
        let words_list = Self::get_words_list_by_language(language)?;

        if words_list.len() != Self::WORDS_LIST_NUMBER {
            return Err(MnemonicError::simple("Invalid words list length"));
        }

        for chunk in entropy.chunks(4) {
            if chunk.len() != 4 {
                return Err(MnemonicError::simple("Entropy length must be multiple of 4 bytes"));
            }
            mnemonic.extend(bytes_chunk_to_words_le(chunk, &words_list));
        }

        if checksum {
            let prefix_len = *Self::language_unique_prefix_lengths()
                .get(language)
                .ok_or_else(|| MnemonicError::simple("Unknown language for checksum"))?;

            let prefixes: String = mnemonic
                .iter()
                .map(|w| {
                    let end = prefix_len.min(w.len());
                    &w[..end]
                })
                .collect();

            let crc = crc32_ieee(prefixes.as_bytes()) as usize;
            let index = crc % mnemonic.len();
            let checksum_word = mnemonic[index].clone();
            mnemonic.push(checksum_word);
        }

        Ok(Self {
            words: mnemonic,
            language: language.to_string(),
        })
    }

    pub fn parse_phrase(phrase: &str) -> Result<Vec<u8>, MnemonicError> {
        let words: Vec<String> = phrase
            .split_whitespace()
            .map(|w| w.to_lowercase())
            .collect();

        if !Self::words_list().contains(&words.len()) {
            return Err(MnemonicError::simple("Invalid Monero mnemonic length"));
        }

        let (words_list, _) = Self::find_language(&words)?;

        let data_words: &[String] = match words.len() {
            MoneroMnemonicWordCounts::THIRTEEN | MoneroMnemonicWordCounts::TWENTY_FIVE => {
                &words[..words.len() - 1]
            }
            MoneroMnemonicWordCounts::TWELVE | MoneroMnemonicWordCounts::TWENTY_FOUR => &words,
            _ => {
                return Err(MnemonicError::simple("Invalid Monero mnemonic length"));
            }
        };

        if data_words.len() % 3 != 0 {
            return Err(MnemonicError::simple("Invalid Monero mnemonic word count"));
        }

        let mut entropy: Vec<u8> = Vec::new();

        for chunk in data_words.chunks(3) {
            let bytes = words_to_bytes_chunk_le(&chunk[0], &chunk[1], &chunk[2], &words_list);
            entropy.extend(bytes);
        }

        Ok(entropy)
    }

    pub fn phrase(&self) -> String {
        self.words.join(" ")
    }
}

impl IMnemonic for MoneroMnemonic {
    fn name() -> &'static str {
        "Monero"
    }

    fn from_words(words: usize, language: &str) -> Result<String, MnemonicError> {
        Self::generate_random(words, language).map(|m| m.mnemonic())
    }

    fn from_entropy(entropy: &[u8], language: &str) -> Result<String, MnemonicError> {
        let checksum = matches!(entropy.len(), 16 | 32);
        Self::from_entropy_with_checksum(entropy, language, checksum).map(|m| m.mnemonic())
    }

    fn encode(entropy: &[u8], language: &str) -> Result<String, MnemonicError> {
        Self::from_entropy(entropy, language)
    }

    fn decode(mnemonic: &[String]) -> Result<Vec<u8>, MnemonicError> {
        let s = mnemonic.join(" ");
        Self::parse_phrase(&s)
    }

    fn mnemonic(&self) -> String {
        self.phrase()
    }

    fn language(&self) -> &str {
        &self.language
    }

    fn words(&self) -> usize {
        self.words.len()
    }

    fn words_list() -> &'static [usize] {
        &[
            MoneroMnemonicWordCounts::TWELVE,
            MoneroMnemonicWordCounts::THIRTEEN,
            MoneroMnemonicWordCounts::TWENTY_FOUR,
            MoneroMnemonicWordCounts::TWENTY_FIVE,
        ]
    }

    fn languages() -> &'static [&'static str] {
        MoneroMnemonic::languages()
    }

    fn wordlist_path() -> HashMap<&'static str, &'static str> {
        HashMap::from([(
            MoneroMnemonicLanguages::ENGLISH,
            "monero/wordlist/english.txt",
        )])
    }

    fn get_words_list_by_language(language: &str) -> Result<Vec<String>, MnemonicError> {
        MoneroMnemonic::get_words_list_by_language(language)
    }

    fn find_language(mnemonic: &[String]) -> Result<(Vec<String>, String), MnemonicError> {
        MoneroMnemonic::find_language(mnemonic)
    }
}
