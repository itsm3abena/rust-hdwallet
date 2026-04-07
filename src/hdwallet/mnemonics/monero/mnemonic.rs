use std::collections::HashMap;

use crate::entropies::{IEntropy, MoneroEntropy, MONERO_ENTROPY_STRENGTHS};
use crate::crypto::crc32;
use crate::exceptions::{Error, EntropyError, MnemonicError, ChecksumError};
use crate::utils::{
    get_bytes, bytes_to_string, bytes_to_integer,
    bytes_chunk_to_words, words_to_bytes_chunk,
};
use crate::mnemonics::imnemonic::IMnemonic;


pub struct MONERO_MNEMONIC_WORDS;

impl MONERO_MNEMONIC_WORDS {
    pub const TWELVE: usize = 12;
    pub const THIRTEEN: usize = 13;
    pub const TWENTY_FOUR: usize = 24;
    pub const TWENTY_FIVE: usize = 25;
}

pub struct MONERO_MNEMONIC_LANGUAGES;

impl MONERO_MNEMONIC_LANGUAGES {
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


pub struct MoneroMnemonic;

impl MoneroMnemonic {
    pub const WORD_BIT_LENGTH: usize = 11;
    pub const WORDS_LIST_NUMBER: usize = 1626;

    pub fn words_list() -> Vec<usize> {
        vec![
            MONERO_MNEMONIC_WORDS::TWELVE,
            MONERO_MNEMONIC_WORDS::THIRTEEN,
            MONERO_MNEMONIC_WORDS::TWENTY_FOUR,
            MONERO_MNEMONIC_WORDS::TWENTY_FIVE,
        ]
    }

    pub fn words_checksum() -> Vec<usize> {
        vec![
            MONERO_MNEMONIC_WORDS::THIRTEEN,
            MONERO_MNEMONIC_WORDS::TWENTY_FIVE,
        ]
    }

    pub fn words_to_entropy_strength() -> HashMap<usize, usize> {
        HashMap::from([
            (12, MONERO_ENTROPY_STRENGTHS::ONE_HUNDRED_TWENTY_EIGHT),
            (13, MONERO_ENTROPY_STRENGTHS::ONE_HUNDRED_TWENTY_EIGHT),
            (24, MONERO_ENTROPY_STRENGTHS::TWO_HUNDRED_FIFTY_SIX),
            (25, MONERO_ENTROPY_STRENGTHS::TWO_HUNDRED_FIFTY_SIX),
        ])
    }

    pub fn languages() -> Vec<&'static str> {
        vec![
            MONERO_MNEMONIC_LANGUAGES::CHINESE_SIMPLIFIED,
            MONERO_MNEMONIC_LANGUAGES::DUTCH,
            MONERO_MNEMONIC_LANGUAGES::ENGLISH,
            MONERO_MNEMONIC_LANGUAGES::FRENCH,
            MONERO_MNEMONIC_LANGUAGES::GERMAN,
            MONERO_MNEMONIC_LANGUAGES::ITALIAN,
            MONERO_MNEMONIC_LANGUAGES::JAPANESE,
            MONERO_MNEMONIC_LANGUAGES::PORTUGUESE,
            MONERO_MNEMONIC_LANGUAGES::RUSSIAN,
            MONERO_MNEMONIC_LANGUAGES::SPANISH,
        ]
    }

    pub fn language_unique_prefix_lengths() -> HashMap<&'static str, usize> {
        HashMap::from([
            ("chinese-simplified", 1),
            ("dutch", 4),
            ("english", 3),
            ("french", 4),
            ("german", 4),
            ("italian", 4),
            ("japanese", 4),
            ("portuguese", 4),
            ("spanish", 4),
            ("russian", 4),
        ])
    }


    pub fn name() -> &'static str {
        "Monero"
    }

    pub fn from_words(words: usize, language: &str) -> Result<String, MnemonicError> {
        if !Self::words_list().contains(&words) {
            return Err(MnemonicError::new("Invalid mnemonic words number"));
        }

        let entropy = MoneroEntropy::generate(
            *Self::words_to_entropy_strength().get(&words).unwrap()
        );

        Self::from_entropy(entropy, language, Self::words_checksum().contains(&words))
    }

    pub fn from_entropy<T: Into<Vec<u8>>>(
        entropy: T,
        language: &str,
        checksum: bool,
    ) -> Result<String, EntropyError> {
        Self::encode(entropy.into(), language, checksum)
    }

    pub fn encode(
        entropy: Vec<u8>,
        language: &str,
        checksum: bool,
    ) -> Result<String, Error> {
        if !MoneroEntropy::is_valid_bytes_strength(entropy.len()) {
            return Err(Error::new("Wrong entropy strength"));
        }

        let mut mnemonic: Vec<String> = Vec::new();
        let words_list = Self::get_words_list_by_language(language)?;

        if words_list.len() != Self::WORDS_LIST_NUMBER {
            return Err(Error::new("Invalid words list length"));
        }

        for chunk in entropy.chunks(4) {
            let words = bytes_chunk_to_words(chunk, &words_list, "little");
            mnemonic.extend(words);
        }

        if checksum {
            let prefix_len = Self::language_unique_prefix_lengths()[language];

            let prefixes: String = mnemonic
                .iter()
                .map(|w| &w[..prefix_len])
                .collect();

            let index = bytes_to_integer(crc32(prefixes.as_bytes()))
                % mnemonic.len();

            let checksum_word = mnemonic[index].clone();
            mnemonic.push(checksum_word);
        }

        Ok(mnemonic.join(" "))
    }

    pub fn decode(mnemonic: &str) -> Result<String, Error> {
        let words = Self::normalize(mnemonic);

        if !Self::words_list().contains(&words.len()) {
            return Err(Error::new("Invalid mnemonic length"));
        }

        let (words_list, language) = Self::find_language(&words)?;

        let mut entropy: Vec<u8> = Vec::new();

        for chunk in words.chunks(3) {
            let bytes = words_to_bytes_chunk(
                &chunk[0], &chunk[1], &chunk[2],
                &words_list, "little"
            );
            entropy.extend(bytes);
        }

        Ok(bytes_to_string(&entropy))
    }

    pub fn normalize(input: &str) -> Vec<String> {
        input
            .split_whitespace()
            .map(|w| w.to_lowercase())
            .collect()
    }


    fn get_words_list_by_language(_language: &str) -> Result<Vec<String>, Error> {
        unimplemented!()
    }

    fn find_language(_words: &[String]) -> Result<(Vec<String>, String), Error> {
        unimplemented!()
    }
}


impl IMnemonic for MoneroMnemonic {}