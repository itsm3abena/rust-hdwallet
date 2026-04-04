use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug)]
pub struct MnemonicError(pub String);

impl std::fmt::Display for MnemonicError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MnemonicError: {}", self.0)
    }
}

impl std::error::Error for MnemonicError {}

pub trait Mnemonic {
    fn name() -> &'static str;

    fn from_words(words: usize, language: &str) -> Result<String, MnemonicError>;

    fn from_entropy(
        entropy: &[u8],
        language: &str,
    ) -> Result<String, MnemonicError>;

    fn encode(entropy: &[u8], language: &str) -> Result<String, MnemonicError>;

    fn decode(mnemonic: &[String]) -> Result<Vec<u8>, MnemonicError>;

    fn mnemonic(&self) -> String;
    fn language(&self) -> &str;
    fn words(&self) -> usize;

    fn mnemonic_type(&self) -> Option<&str> {
        None
    }

    fn words_list() -> &'static [usize];
    fn languages() -> &'static [&'static str];
    fn wordlist_path() -> HashMap<&'static str, &'static str>;


    fn normalize(mnemonic: &str) -> Vec<String> {
        mnemonic.split_whitespace().map(|s| s.to_string()).collect()
    }

    fn is_valid(mnemonic: &[String]) -> bool {
        Self::decode(mnemonic).is_ok()
    }

    fn is_valid_language(language: &str) -> bool {
        Self::languages().contains(&language)
    }

    fn is_valid_words(words: usize) -> bool {
        Self::words_list().contains(&words)
    }

    fn get_words_list_by_language(
        language: &str,
    ) -> Result<Vec<String>, MnemonicError> {
        let paths = Self::wordlist_path();

        let path = paths.get(language).ok_or_else(|| {
            MnemonicError(format!("Language '{}' not found", language))
        })?;

        let full_path = Path::new(path);

        let content = fs::read_to_string(full_path)
            .map_err(|e| MnemonicError(format!("File error: {}", e)))?;

        let words = content
            .lines()
            .filter(|line| !line.trim().is_empty() && !line.starts_with('#'))
            .map(|line| line.trim().to_string())
            .collect();

        Ok(words)
    }

    fn find_language(
        mnemonic: &[String],
    ) -> Result<(Vec<String>, String), MnemonicError> {
        for &language in Self::languages() {
            if let Ok(words_list) = Self::get_words_list_by_language(language) {
                let set: std::collections::HashSet<_> =
                    words_list.iter().cloned().collect();

                if mnemonic.iter().all(|w| set.contains(w)) {
                    return Ok((words_list, language.to_string()));
                }
            }
        }

        Err(MnemonicError(format!(
            "Invalid language for mnemonic {:?}",
            mnemonic
        )))
    }
}