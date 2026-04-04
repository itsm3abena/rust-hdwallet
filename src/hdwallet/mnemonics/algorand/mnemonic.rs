use std::collections::HashMap;

use crate::crypto::sha512_256;
use crate::utils::{convert_bits, get_bytes, bytes_to_string};
use crate::exceptions::{MnemonicError};
use crate::entropies::{AlgorandEntropy};

use crate::mnemonics::imnemonic::Mnemonic;

pub struct AlgorandMnemonicWords;
impl AlgorandMnemonicWords {
    pub const TWENTY_FIVE: usize = 25;
}

pub struct AlgorandMnemonicLanguages;
impl AlgorandMnemonicLanguages {
    pub const ENGLISH: &'static str = "english";
}

pub struct AlgorandMnemonic {
    mnemonic: Vec<String>,
    language: String,
}

impl Mnemonic for AlgorandMnemonic {

    fn name() -> &'static str {
        "Algorand"
    }

    fn from_words(words: usize, language: &str) -> Result<String, MnemonicError> {
        if !Self::words_list().contains(&words) {
            return Err(MnemonicError(format!(
                "Invalid mnemonic words number: {}",
                words
            )));
        }

        let entropy = AlgorandEntropy::generate(256);
        Self::from_entropy(&entropy, language)
    }

    fn from_entropy(entropy: &[u8], language: &str) -> Result<String, MnemonicError> {
        Self::encode(entropy, language)
    }

    fn encode(entropy: &[u8], language: &str) -> Result<String, MnemonicError> {
        if !AlgorandEntropy::is_valid_bytes_strength(entropy.len()) {
            return Err(MnemonicError("Invalid entropy strength".into()));
        }

        let checksum = sha512_256(entropy);
        let checksum = &checksum[..2];

        let checksum_indexes = convert_bits(checksum, 8, 11)
            .ok_or_else(|| MnemonicError("Checksum conversion failed".into()))?;

        let word_indexes = convert_bits(entropy, 8, 11)
            .ok_or_else(|| MnemonicError("Entropy conversion failed".into()))?;

        let words_list = Self::get_words_list_by_language(language)?;

        let mut indexes = word_indexes;
        indexes.push(checksum_indexes[0]);

        let mnemonic: Vec<String> = indexes
            .iter()
            .map(|&i| words_list[i as usize].clone())
            .collect();

        Ok(mnemonic.join(" "))
    }

    fn decode(mnemonic: &[String]) -> Result<Vec<u8>, MnemonicError> {
        if !Self::words_list().contains(&mnemonic.len()) {
            return Err(MnemonicError("Invalid mnemonic word count".into()));
        }

        let (words_list, _) = Self::find_language(mnemonic)?;

        let mut map = HashMap::new();
        for (i, w) in words_list.iter().enumerate() {
            map.insert(w, i as u32);
        }

        let word_indexes: Vec<u32> = mnemonic
            .iter()
            .map(|w| {
                map.get(w)
                    .cloned()
                    .ok_or_else(|| MnemonicError(format!("Invalid word {}", w)))
            })
            .collect::<Result<Vec<_>, _>>()?;

        let entropy_bytes = convert_bits(&word_indexes[..word_indexes.len() - 1], 11, 8)
            .ok_or_else(|| MnemonicError("Entropy conversion failed".into()))?;

        let mut entropy = entropy_bytes;
        entropy.pop();

        let checksum = sha512_256(&entropy);
        let checksum = &checksum[..2];

        let checksum_indexes = convert_bits(checksum, 8, 11)
            .ok_or_else(|| MnemonicError("Checksum conversion failed".into()))?;

        if checksum_indexes[0] != word_indexes[word_indexes.len() - 1] {
            return Err(MnemonicError("Invalid checksum".into()));
        }

        Ok(entropy)
    }

    fn mnemonic(&self) -> String {
        self.mnemonic.join(" ")
    }

    fn language(&self) -> &str {
        &self.language
    }

    fn words(&self) -> usize {
        self.mnemonic.len()
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
}