use std::collections::HashMap;

use crate::hdwallet::exceptions::MnemonicError;

pub trait IMnemonic {
    fn name() -> &'static str
    where
        Self: Sized;

    fn from_words(words: usize, language: &str) -> Result<String, MnemonicError>
    where
        Self: Sized;

    fn from_entropy(entropy: &[u8], language: &str) -> Result<String, MnemonicError>
    where
        Self: Sized;

    fn encode(entropy: &[u8], language: &str) -> Result<String, MnemonicError>
    where
        Self: Sized;

    fn decode(mnemonic: &[String]) -> Result<Vec<u8>, MnemonicError>
    where
        Self: Sized;

    fn mnemonic(&self) -> String;
    fn language(&self) -> &str;
    fn words(&self) -> usize;

    fn mnemonic_type(&self) -> Option<&str> {
        None
    }

    fn words_list() -> &'static [usize]
    where
        Self: Sized;

    fn languages() -> &'static [&'static str]
    where
        Self: Sized;

    fn wordlist_path() -> HashMap<&'static str, &'static str>
    where
        Self: Sized;

    fn normalize(mnemonic: &str) -> Vec<String>
    where
        Self: Sized,
    {
        mnemonic.split_whitespace().map(|s| s.to_string()).collect()
    }

    fn is_valid(mnemonic: &[String]) -> bool
    where
        Self: Sized,
    {
        Self::decode(mnemonic).is_ok()
    }

    fn is_valid_language(language: &str) -> bool
    where
        Self: Sized,
    {
        Self::languages().contains(&language)
    }

    fn is_valid_words(words: usize) -> bool
    where
        Self: Sized,
    {
        Self::words_list().contains(&words)
    }

    fn get_words_list_by_language(language: &str) -> Result<Vec<String>, MnemonicError>
    where
        Self: Sized,
    {
        let _ = language;
        Err(MnemonicError::simple(
            "get_words_list_by_language is not implemented for this mnemonic type",
        ))
    }

    fn find_language(mnemonic: &[String]) -> Result<(Vec<String>, String), MnemonicError>
    where
        Self: Sized,
    {
        let _ = mnemonic;
        Err(MnemonicError::simple(
            "find_language is not implemented for this mnemonic type",
        ))
    }
}
