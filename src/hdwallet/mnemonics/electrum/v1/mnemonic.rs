use std::collections::HashMap;

use crate::entropies::{IEntropy, ElectrumV1Entropy, ELECTRUM_V1_ENTROPY_STRENGTHS};
use crate::exceptions::{EntropyError, MnemonicError};
use crate::utils::{
    get_bytes, integer_to_bytes, bytes_to_integer, bytes_to_string,
};
use crate::mnemonics::imnemonic::IMnemonic;


pub struct ELECTRUM_V1_MNEMONIC_WORDS;

impl ELECTRUM_V1_MNEMONIC_WORDS {
    pub const TWELVE: usize = 12;
}

pub struct ELECTRUM_V1_MNEMONIC_LANGUAGES;

impl ELECTRUM_V1_MNEMONIC_LANGUAGES {
    pub const ENGLISH: &'static str = "english";
}


pub struct ElectrumV1Mnemonic;

impl ElectrumV1Mnemonic {
    pub const WORDS_LIST_NUMBER: usize = 1626;

    pub fn words_list() -> Vec<usize> {
        vec![ELECTRUM_V1_MNEMONIC_WORDS::TWELVE]
    }

    pub fn words_to_entropy_strength() -> HashMap<usize, usize> {
        HashMap::from([
            (12, ELECTRUM_V1_ENTROPY_STRENGTHS::ONE_HUNDRED_TWENTY_EIGHT),
        ])
    }

    pub fn languages() -> Vec<&'static str> {
        vec![ELECTRUM_V1_MNEMONIC_LANGUAGES::ENGLISH]
    }

    pub fn name() -> &'static str {
        "Electrum-V1"
    }


    pub fn from_words(words: usize, language: &str) -> Result<String, MnemonicError> {
        if !Self::words_list().contains(&words) {
            return Err(MnemonicError::new("Invalid mnemonic words number"));
        }

        let entropy = ElectrumV1Entropy::generate(
            *Self::words_to_entropy_strength().get(&words).unwrap()
        );

        Self::from_entropy(entropy, language)
    }


    pub fn from_entropy<T: Into<Vec<u8>>>(
        entropy: T,
        language: &str,
    ) -> Result<String, EntropyError> {
        Self::encode(entropy.into(), language)
    }


    pub fn encode(
        entropy: Vec<u8>,
        language: &str,
    ) -> Result<String, EntropyError> {

        if !ElectrumV1Entropy::is_valid_bytes_strength(entropy.len()) {
            return Err(EntropyError::new("Wrong entropy strength"));
        }

        let mut mnemonic: Vec<String> = Vec::new();

        let words_list = Self::get_words_list_by_language(language)?;
        let words_list = Self::normalize_list(words_list);

        for chunk_bytes in entropy.chunks(4) {
            let chunk = bytes_to_integer(chunk_bytes, "big");

            let n = words_list.len();

            let word_1_index = chunk % n;
            let word_2_index = ((chunk / n) + word_1_index) % n;
            let word_3_index = ((chunk / n / n) + word_2_index) % n;

            mnemonic.push(words_list[word_1_index].clone());
            mnemonic.push(words_list[word_2_index].clone());
            mnemonic.push(words_list[word_3_index].clone());
        }

        Ok(mnemonic.join(" "))
    }


    pub fn decode(
        mnemonic: &str,
        words_list_opt: Option<Vec<String>>,
        words_index_opt: Option<HashMap<String, usize>>,
    ) -> Result<String, MnemonicError> {

        let words = Self::normalize(mnemonic);

        if !Self::words_list().contains(&words.len()) {
            return Err(MnemonicError::new("Invalid mnemonic words count"));
        }

        let (words_list, mut index_map) = match (words_list_opt, words_index_opt) {
            (Some(list), Some(map)) => (list, map),
            _ => {
                let (list, _) = Self::find_language(&words)?;
                let mut map = HashMap::new();
                for (i, w) in list.iter().enumerate() {
                    map.insert(w.clone(), i);
                }
                (list, map)
            }
        };

        let n = words_list.len();
        let mut entropy: Vec<u8> = Vec::new();

        for chunk in words.chunks(3) {
            let w1 = index_map[&chunk[0]];
            let w2 = index_map[&chunk[1]] % n;
            let w3 = index_map[&chunk[2]] % n;

            let value =
                w1 +
                n * ((w2 + n - w1) % n) +
                n * n * ((w3 + n - w2) % n);

            entropy.extend(integer_to_bytes(value, 4, "big"));
        }

        Ok(bytes_to_string(&entropy))
    }


    pub fn is_valid(
        mnemonic: &str,
        words_list: Option<Vec<String>>,
        words_index: Option<HashMap<String, usize>>,
    ) -> bool {
        Self::decode(mnemonic, words_list, words_index).is_ok()
    }


    pub fn normalize(mnemonic: &str) -> Vec<String> {
        mnemonic
            .split_whitespace()
            .map(|w| w.to_lowercase())
            .collect()
    }

    fn normalize_list(words: Vec<String>) -> Vec<String> {
        words.into_iter().map(|w| w.to_lowercase()).collect()
    }


    fn get_words_list_by_language(_language: &str) -> Result<Vec<String>, EntropyError> {
        unimplemented!()
    }

    fn find_language(_words: &[String]) -> Result<(Vec<String>, String), MnemonicError> {
        unimplemented!()
    }
}


impl IMnemonic for ElectrumV1Mnemonic {}