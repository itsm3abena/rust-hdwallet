use std::collections::HashMap;

use crate::crypto::sha256;
use crate::utils::{
    get_bytes,
    bytes_to_binary_string,
    bytes_to_string,
    binary_string_to_integer,
    integer_to_binary_string,
    binary_string_to_bytes,
};
use crate::exceptions::{MnemonicError, EntropyError, ChecksumError};
use crate::entropies::BIP39Entropy;

use crate::mnemonics::imnemonic::Mnemonic;

pub struct BIP39MnemonicWords;
impl BIP39MnemonicWords {
    pub const TWELVE: usize = 12;
    pub const FIFTEEN: usize = 15;
    pub const EIGHTEEN: usize = 18;
    pub const TWENTY_ONE: usize = 21;
    pub const TWENTY_FOUR: usize = 24;
}

pub struct BIP39MnemonicLanguages;
impl BIP39MnemonicLanguages {
    pub const ENGLISH: &'static str = "english";
    pub const FRENCH: &'static str = "french";
    pub const ITALIAN: &'static str = "italian";
    pub const SPANISH: &'static str = "spanish";
    pub const JAPANESE: &'static str = "japanese";
    pub const KOREAN: &'static str = "korean";
    pub const CHINESE_SIMPLIFIED: &'static str = "chinese-simplified";
    pub const CHINESE_TRADITIONAL: &'static str = "chinese-traditional";
    pub const CZECH: &'static str = "czech";
    pub const PORTUGUESE: &'static str = "portuguese";
    pub const RUSSIAN: &'static str = "russian";
    pub const TURKISH: &'static str = "turkish";
}

pub struct BIP39Mnemonic {
    mnemonic: Vec<String>,
    language: String,
}

impl Mnemonic for BIP39Mnemonic {

    fn name() -> &'static str {
        "BIP39"
    }

    fn from_words(words: usize, language: &str) -> Result<String, MnemonicError> {
        if !Self::words_list().contains(&words) {
            return Err(MnemonicError(format!("Invalid word count {}", words)));
        }

        let entropy = BIP39Entropy::generate(
            Self::words_to_entropy_strength()
                .get(&words)
                .unwrap()
                .clone()
        );

        Self::from_entropy(&entropy, language)
    }

    fn from_entropy(entropy: &[u8], language: &str) -> Result<String, MnemonicError> {
        Self::encode(entropy, language)
    }

    fn encode(entropy: &[u8], language: &str) -> Result<String, MnemonicError> {
        if !BIP39Entropy::is_valid_bytes_strength(entropy.len()) {
            return Err(EntropyError("Invalid entropy strength".into()).into());
        }

        let entropy_bin = bytes_to_binary_string(entropy, entropy.len() * 8);
        let hash_bin = bytes_to_binary_string(&sha256(entropy), 256);

        let checksum_len = entropy.len() * 8 / 32;
        let mnemonic_bin = format!(
            "{}{}",
            entropy_bin,
            &hash_bin[..checksum_len]
        );

        let words_list = Self::get_words_list_by_language(language)?;

        if words_list.len() != 2048 {
            return Err(MnemonicError("Invalid wordlist length".into()));
        }

        let mut result = Vec::new();

        for i in 0..(mnemonic_bin.len() / 11) {
            let chunk = &mnemonic_bin[i * 11..(i + 1) * 11];
            let index = binary_string_to_integer(chunk);
            result.push(words_list[index as usize].clone());
        }

        Ok(result.join(" "))
    }

    fn decode(mnemonic: &[String]) -> Result<Vec<u8>, MnemonicError> {
        if !Self::words_list().contains(&mnemonic.len()) {
            return Err(MnemonicError("Invalid mnemonic length".into()));
        }

        let (words_list, _) = Self::find_language(mnemonic)?;

        if words_list.len() != 2048 {
            return Err(MnemonicError("Invalid wordlist length".into()));
        }

        let mut map = HashMap::new();
        for (i, w) in words_list.iter().enumerate() {
            map.insert(w, i);
        }

        let mut mnemonic_bin = String::new();

        for word in mnemonic {
            let index = map.get(word)
                .ok_or_else(|| MnemonicError(format!("Invalid word {}", word)))?;

            mnemonic_bin.push_str(&integer_to_binary_string(*index as u32, 11));
        }

        let bit_len = mnemonic_bin.len();
        let checksum_len = bit_len / 33;

        let entropy_bin = &mnemonic_bin[..bit_len - checksum_len];
        let checksum_bin = &mnemonic_bin[bit_len - checksum_len..];

        let entropy = binary_string_to_bytes(entropy_bin, entropy_bin.len());

        let hash_bin = bytes_to_binary_string(&sha256(&entropy), 256);
        let expected_checksum = &hash_bin[..checksum_len];

        if checksum_bin != expected_checksum {
            return Err(ChecksumError("Invalid checksum".into()).into());
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
        &[
            BIP39MnemonicWords::TWELVE,
            BIP39MnemonicWords::FIFTEEN,
            BIP39MnemonicWords::EIGHTEEN,
            BIP39MnemonicWords::TWENTY_ONE,
            BIP39MnemonicWords::TWENTY_FOUR,
        ]
    }

    fn languages() -> &'static [&'static str] {
        &[
            BIP39MnemonicLanguages::ENGLISH,
            BIP39MnemonicLanguages::FRENCH,
            BIP39MnemonicLanguages::ITALIAN,
            BIP39MnemonicLanguages::SPANISH,
            BIP39MnemonicLanguages::JAPANESE,
            BIP39MnemonicLanguages::KOREAN,
            BIP39MnemonicLanguages::CHINESE_SIMPLIFIED,
            BIP39MnemonicLanguages::CHINESE_TRADITIONAL,
            BIP39MnemonicLanguages::CZECH,
            BIP39MnemonicLanguages::PORTUGUESE,
            BIP39MnemonicLanguages::RUSSIAN,
            BIP39MnemonicLanguages::TURKISH,
        ]
    }

    fn wordlist_path() -> HashMap<&'static str, &'static str> {
        HashMap::from([
            (BIP39MnemonicLanguages::ENGLISH, "bip39/wordlist/english.txt"),
            (BIP39MnemonicLanguages::FRENCH, "bip39/wordlist/french.txt"),
            (BIP39MnemonicLanguages::ITALIAN, "bip39/wordlist/italian.txt"),
            (BIP39MnemonicLanguages::SPANISH, "bip39/wordlist/spanish.txt"),
            (BIP39MnemonicLanguages::JAPANESE, "bip39/wordlist/japanese.txt"),
            (BIP39MnemonicLanguages::KOREAN, "bip39/wordlist/korean.txt"),
            (BIP39MnemonicLanguages::CHINESE_SIMPLIFIED, "bip39/wordlist/chinese_simplified.txt"),
            (BIP39MnemonicLanguages::CHINESE_TRADITIONAL, "bip39/wordlist/chinese_traditional.txt"),
            (BIP39MnemonicLanguages::CZECH, "bip39/wordlist/czech.txt"),
            (BIP39MnemonicLanguages::PORTUGUESE, "bip39/wordlist/portuguese.txt"),
            (BIP39MnemonicLanguages::RUSSIAN, "bip39/wordlist/russian.txt"),
            (BIP39MnemonicLanguages::TURKISH, "bip39/wordlist/turkish.txt"),
        ])
    }
}

impl BIP39Mnemonic {

    fn words_to_entropy_strength() -> HashMap<usize, usize> {
        HashMap::from([
            (12, 128),
            (15, 160),
            (18, 192),
            (21, 224),
            (24, 256),
        ])
    }
}