use std::collections::HashMap;

use crate::hdwallet::entropies::{AlgorandEntropy, IEntropy};
use crate::hdwallet::exceptions::MnemonicError;
use crate::hdwallet::utils::{
    convert_bits_11_to_8, convert_bits_8_to_11, parse_wordlist_text, sha512_256,
};
use crate::hdwallet::mnemonics::imnemonic::IMnemonic;

pub struct AlgorandMnemonicWords;

impl AlgorandMnemonicWords {
    pub const TWENTY_FIVE: usize = 25;
}

pub struct AlgorandMnemonicLanguages;

impl AlgorandMnemonicLanguages {
    pub const ENGLISH: &'static str = "english";
}

#[derive(Clone, Debug)]
pub struct AlgorandMnemonic {
    words: Vec<String>,
    language: String,
}

impl AlgorandMnemonic {
    pub const WORDS_LIST_NUMBER: usize = 2048;

    fn wordlist_embedded(language: &str) -> Result<&'static str, MnemonicError> {
        match language {
            AlgorandMnemonicLanguages::ENGLISH => Ok(include_str!("wordlist/english.txt")),
            _ => Err(MnemonicError::simple(&format!(
                "Unknown Algorand language: {}",
                language
            ))),
        }
    }

    pub fn get_words_list_by_language(language: &str) -> Result<Vec<String>, MnemonicError> {
        let text = Self::wordlist_embedded(language)?;
        let words = parse_wordlist_text(text);
        if words.len() != Self::WORDS_LIST_NUMBER {
            return Err(MnemonicError::simple("Invalid Algorand wordlist length"));
        }
        Ok(words)
    }

    pub fn find_language(mnemonic_words: &[String]) -> Result<(Vec<String>, String), MnemonicError> {
        let list = Self::get_words_list_by_language(AlgorandMnemonicLanguages::ENGLISH)?;
        let set: std::collections::HashSet<_> = list.iter().cloned().collect();
        if mnemonic_words.iter().all(|w| set.contains(w)) {
            return Ok((list, AlgorandMnemonicLanguages::ENGLISH.to_string()));
        }
        Err(MnemonicError::simple("Not a valid Algorand english mnemonic"))
    }

    pub fn generate_random(language: &str) -> Result<Self, MnemonicError> {
        let scheme = AlgorandEntropy;
        let hex = scheme
            .generate(256)
            .map_err(|e| MnemonicError::simple(&e.to_string()))?;
        let entropy = hex::decode(hex.trim())
            .map_err(|e| MnemonicError::simple(&e.to_string()))?;
        Self::from_entropy_bytes(&entropy, language)
    }

    pub fn from_entropy_bytes(entropy: &[u8], language: &str) -> Result<Self, MnemonicError> {
        let scheme = AlgorandEntropy;
        if !scheme.is_valid_bytes_strength(entropy.len()) {
            return Err(MnemonicError::simple("Invalid entropy strength for Algorand"));
        }

        let checksum_full = sha512_256(entropy);
        let checksum = &checksum_full[..2];

        let checksum_indexes = convert_bits_8_to_11(checksum)
            .ok_or_else(|| MnemonicError::simple("Checksum convert_bits failed"))?;
        let word_indexes = convert_bits_8_to_11(entropy)
            .ok_or_else(|| MnemonicError::simple("Entropy convert_bits failed"))?;

        let words_list = Self::get_words_list_by_language(language)?;

        let mut indexes = word_indexes;
        if checksum_indexes.is_empty() {
            return Err(MnemonicError::simple("Empty checksum indexes"));
        }
        indexes.push(checksum_indexes[0]);

        let mnemonic: Vec<String> = indexes
            .iter()
            .map(|&i| {
                words_list
                    .get(i as usize)
                    .ok_or_else(|| MnemonicError::simple("Word index out of range"))
                    .map(Clone::clone)
            })
            .collect::<Result<_, _>>()?;

        Ok(Self {
            words: mnemonic,
            language: language.to_string(),
        })
    }

    pub fn decode_to_entropy(mnemonic: &[String]) -> Result<Vec<u8>, MnemonicError> {
        if mnemonic.len() != AlgorandMnemonicWords::TWENTY_FIVE {
            return Err(MnemonicError::simple("Algorand mnemonic must be 25 words"));
        }

        let (words_list, _) = Self::find_language(mnemonic)?;

        let mut map = HashMap::new();
        for (i, w) in words_list.iter().enumerate() {
            map.insert(w.as_str(), i as u32);
        }

        let word_indexes: Vec<u32> = mnemonic
            .iter()
            .map(|w| {
                map.get(w.as_str())
                    .copied()
                    .ok_or_else(|| MnemonicError::simple(&format!("Invalid word {}", w)))
            })
            .collect::<Result<_, _>>()?;

        let entropy_with_extra = convert_bits_11_to_8(&word_indexes[..word_indexes.len() - 1])
            .ok_or_else(|| MnemonicError::simple("Entropy conversion failed"))?;

        let mut entropy = entropy_with_extra;
        if entropy.is_empty() {
            return Err(MnemonicError::simple("Empty entropy"));
        }
        entropy.pop();

        let checksum_full = sha512_256(&entropy);
        let checksum = &checksum_full[..2];
        let checksum_indexes = convert_bits_8_to_11(checksum)
            .ok_or_else(|| MnemonicError::simple("Checksum conversion failed"))?;

        if checksum_indexes.is_empty()
            || checksum_indexes[0] != word_indexes[word_indexes.len() - 1]
        {
            return Err(MnemonicError::simple("Invalid checksum"));
        }

        Ok(entropy)
    }

    pub fn phrase(&self) -> String {
        self.words.join(" ")
    }
}

impl IMnemonic for AlgorandMnemonic {
    fn name() -> &'static str {
        "Algorand"
    }

    fn from_words(words: usize, language: &str) -> Result<String, MnemonicError> {
        if words != AlgorandMnemonicWords::TWENTY_FIVE {
            return Err(MnemonicError::simple("Algorand only supports 25 words"));
        }
        Self::generate_random(language).map(|m| m.mnemonic())
    }

    fn from_entropy(entropy: &[u8], language: &str) -> Result<String, MnemonicError> {
        AlgorandMnemonic::from_entropy_bytes(entropy, language).map(|m| m.mnemonic())
    }

    fn encode(entropy: &[u8], language: &str) -> Result<String, MnemonicError> {
        AlgorandMnemonic::from_entropy_bytes(entropy, language).map(|m| m.mnemonic())
    }

    fn decode(mnemonic: &[String]) -> Result<Vec<u8>, MnemonicError> {
        Self::decode_to_entropy(mnemonic)
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
        &[AlgorandMnemonicWords::TWENTY_FIVE]
    }

    fn languages() -> &'static [&'static str] {
        &[AlgorandMnemonicLanguages::ENGLISH]
    }

    fn wordlist_path() -> HashMap<&'static str, &'static str> {
        HashMap::from([(
            AlgorandMnemonicLanguages::ENGLISH,
            "algorand/wordlist/english.txt",
        )])
    }

    fn get_words_list_by_language(language: &str) -> Result<Vec<String>, MnemonicError> {
        AlgorandMnemonic::get_words_list_by_language(language)
    }

    fn find_language(mnemonic: &[String]) -> Result<(Vec<String>, String), MnemonicError> {
        AlgorandMnemonic::find_language(mnemonic)
    }
}
