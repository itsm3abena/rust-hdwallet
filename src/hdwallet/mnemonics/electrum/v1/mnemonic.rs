use std::collections::HashMap;

use crate::hdwallet::entropies::electrum::ElectrumV1EntropyStrengths;
use crate::hdwallet::entropies::{ElectrumV1Entropy, IEntropy};
use crate::hdwallet::exceptions::MnemonicError;
use crate::hdwallet::utils::{parse_wordlist_text, u32_to_bytes_be};
use crate::hdwallet::mnemonics::imnemonic::IMnemonic;

pub struct ElectrumV1MnemonicWordCounts;

impl ElectrumV1MnemonicWordCounts {
    pub const TWELVE: usize = 12;
}

pub struct ElectrumV1MnemonicLanguages;

impl ElectrumV1MnemonicLanguages {
    pub const ENGLISH: &'static str = "english";
}

#[derive(Clone, Debug)]
pub struct ElectrumV1Mnemonic {
    phrase: String,
}

impl ElectrumV1Mnemonic {
    pub const WORDS_LIST_NUMBER: usize = 1626;

    pub fn words_list() -> Vec<usize> {
        vec![ElectrumV1MnemonicWordCounts::TWELVE]
    }

    pub fn words_to_entropy_strength() -> HashMap<usize, usize> {
        HashMap::from([(
            ElectrumV1MnemonicWordCounts::TWELVE,
            ElectrumV1EntropyStrengths::ONE_HUNDRED_TWENTY_EIGHT,
        )])
    }

    pub fn languages() -> &'static [&'static str] {
        &[ElectrumV1MnemonicLanguages::ENGLISH]
    }

    pub fn get_words_list_by_language(language: &str) -> Result<Vec<String>, MnemonicError> {
        if language != ElectrumV1MnemonicLanguages::ENGLISH {
            return Err(MnemonicError::simple("Electrum V1 only supports english"));
        }
        let text = include_str!("wordlist/english.txt");
        let mut words = parse_wordlist_text(text);
        for w in &mut words {
            *w = w.to_lowercase();
        }
        if words.len() != Self::WORDS_LIST_NUMBER {
            return Err(MnemonicError::simple("Invalid Electrum V1 wordlist length"));
        }
        Ok(words)
    }

    pub fn find_language(mnemonic_words: &[String]) -> Result<(Vec<String>, String), MnemonicError> {
        let list = Self::get_words_list_by_language(ElectrumV1MnemonicLanguages::ENGLISH)?;
        let set: std::collections::HashSet<_> = list.iter().cloned().collect();
        if mnemonic_words
            .iter()
            .all(|w| set.contains(&w.to_lowercase()))
        {
            return Ok((list, ElectrumV1MnemonicLanguages::ENGLISH.to_string()));
        }
        Err(MnemonicError::simple("Not an Electrum V1 english mnemonic"))
    }

    pub fn generate_random(language: &str) -> Result<Self, MnemonicError> {
        let bits = ElectrumV1EntropyStrengths::ONE_HUNDRED_TWENTY_EIGHT;
        let scheme = ElectrumV1Entropy;
        let hex = scheme
            .generate(bits)
            .map_err(|e| MnemonicError::simple(&e.to_string()))?;
        let entropy = hex::decode(hex.trim())
            .map_err(|e| MnemonicError::simple(&e.to_string()))?;
        Self::from_entropy_bytes(&entropy, language)
    }

    pub fn from_entropy_bytes(entropy: &[u8], language: &str) -> Result<Self, MnemonicError> {
        let scheme = ElectrumV1Entropy;
        if !scheme.is_valid_bytes_strength(entropy.len()) {
            return Err(MnemonicError::simple("Wrong entropy strength for Electrum V1"));
        }

        let mut mnemonic: Vec<String> = Vec::new();
        let words_list = Self::get_words_list_by_language(language)?;
        let n = words_list.len();

        for chunk_bytes in entropy.chunks(4) {
            if chunk_bytes.len() != 4 {
                return Err(MnemonicError::simple("Entropy must be multiple of 4 bytes"));
            }
            let chunk = u32::from_be_bytes(chunk_bytes.try_into().unwrap());

            let word_1_index = (chunk as usize) % n;
            let word_2_index = ((chunk as usize) / n + word_1_index) % n;
            let word_3_index = ((chunk as usize) / n / n + word_2_index) % n;

            mnemonic.push(words_list[word_1_index].clone());
            mnemonic.push(words_list[word_2_index].clone());
            mnemonic.push(words_list[word_3_index].clone());
        }

        Ok(Self {
            phrase: mnemonic.join(" "),
        })
    }

    pub fn decode_phrase(
        mnemonic: &str,
        words_list_opt: Option<Vec<String>>,
        words_index_opt: Option<HashMap<String, usize>>,
    ) -> Result<Vec<u8>, MnemonicError> {
        let words: Vec<String> = mnemonic
            .split_whitespace()
            .map(|w| w.to_lowercase())
            .collect();

        if !Self::words_list().contains(&words.len()) {
            return Err(MnemonicError::simple("Invalid mnemonic words"));
        }

        let (words_list, index_map) = match (words_list_opt, words_index_opt) {
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
            let w1 = *index_map
                .get(&chunk[0])
                .ok_or_else(|| MnemonicError::simple("Unknown word"))?;
            let w2 = *index_map
                .get(&chunk[1])
                .ok_or_else(|| MnemonicError::simple("Unknown word"))?;
            let w3 = *index_map
                .get(&chunk[2])
                .ok_or_else(|| MnemonicError::simple("Unknown word"))?;

            let value = w1 + n * ((w2 + n - w1) % n) + n * n * ((w3 + n - w2) % n);
            let v = u32::try_from(value).map_err(|_| MnemonicError::simple("Chunk overflow"))?;
            entropy.extend_from_slice(&u32_to_bytes_be(v));
        }

        Ok(entropy)
    }

    pub fn is_valid_phrase(
        mnemonic: &str,
        words_list: Option<Vec<String>>,
        words_index: Option<HashMap<String, usize>>,
    ) -> bool {
        Self::decode_phrase(mnemonic, words_list, words_index).is_ok()
    }

    pub fn phrase(&self) -> &str {
        &self.phrase
    }
}

impl IMnemonic for ElectrumV1Mnemonic {
    fn name() -> &'static str {
        "Electrum-V1"
    }

    fn from_words(words: usize, language: &str) -> Result<String, MnemonicError> {
        if words != ElectrumV1MnemonicWordCounts::TWELVE {
            return Err(MnemonicError::simple("Invalid mnemonic words number"));
        }
        Self::generate_random(language).map(|m| m.mnemonic())
    }

    fn from_entropy(entropy: &[u8], language: &str) -> Result<String, MnemonicError> {
        ElectrumV1Mnemonic::from_entropy_bytes(entropy, language).map(|m| m.mnemonic())
    }

    fn encode(entropy: &[u8], language: &str) -> Result<String, MnemonicError> {
        ElectrumV1Mnemonic::from_entropy_bytes(entropy, language).map(|m| m.mnemonic())
    }

    fn decode(mnemonic: &[String]) -> Result<Vec<u8>, MnemonicError> {
        let s = mnemonic.join(" ");
        Self::decode_phrase(&s, None, None)
    }

    fn mnemonic(&self) -> String {
        self.phrase.clone()
    }

    fn language(&self) -> &str {
        ElectrumV1MnemonicLanguages::ENGLISH
    }

    fn words(&self) -> usize {
        self.phrase.split_whitespace().count()
    }

    fn words_list() -> &'static [usize] {
        &[ElectrumV1MnemonicWordCounts::TWELVE]
    }

    fn languages() -> &'static [&'static str] {
        &[ElectrumV1MnemonicLanguages::ENGLISH]
    }

    fn wordlist_path() -> HashMap<&'static str, &'static str> {
        HashMap::from([(
            ElectrumV1MnemonicLanguages::ENGLISH,
            "electrum/v1/wordlist/english.txt",
        )])
    }

    fn get_words_list_by_language(language: &str) -> Result<Vec<String>, MnemonicError> {
        ElectrumV1Mnemonic::get_words_list_by_language(language)
    }

    fn find_language(mnemonic: &[String]) -> Result<(Vec<String>, String), MnemonicError> {
        ElectrumV1Mnemonic::find_language(mnemonic)
    }
}
