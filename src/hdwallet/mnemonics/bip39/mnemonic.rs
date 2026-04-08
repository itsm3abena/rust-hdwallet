use std::collections::HashMap;

use sha2::{Digest, Sha256};
use unicode_normalization::UnicodeNormalization;

use crate::hdwallet::entropies::{BIP39Entropy, IEntropy};
use crate::hdwallet::exceptions::MnemonicError;
use crate::hdwallet::mnemonics::imnemonic::IMnemonic;
use crate::hdwallet::utils::parse_wordlist_text;

const WORD_BIT_LENGTH: usize = 11;
const WORDS_LIST_NUMBER: usize = 2048;

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
    pub const CHINESE_SIMPLIFIED: &'static str = "chinese-simplified";
    pub const CHINESE_TRADITIONAL: &'static str = "chinese-traditional";
    pub const CZECH: &'static str = "czech";
    pub const ENGLISH: &'static str = "english";
    pub const FRENCH: &'static str = "french";
    pub const ITALIAN: &'static str = "italian";
    pub const JAPANESE: &'static str = "japanese";
    pub const KOREAN: &'static str = "korean";
    pub const PORTUGUESE: &'static str = "portuguese";
    pub const RUSSIAN: &'static str = "russian";
    pub const SPANISH: &'static str = "spanish";
    pub const TURKISH: &'static str = "turkish";
}

#[derive(Clone, Debug)]
pub struct BIP39Mnemonic {
    phrase: String,
    language: String,
}

fn normalize_word(s: &str) -> String {
    s.to_lowercase().nfkd().collect()
}

fn wordlist_embedded(language: &str) -> Result<&'static str, MnemonicError> {
    match language {
        BIP39MnemonicLanguages::CHINESE_SIMPLIFIED => Ok(include_str!("wordlist/chinese_simplified.txt")),
        BIP39MnemonicLanguages::CHINESE_TRADITIONAL => {
            Ok(include_str!("wordlist/chinese_traditional.txt"))
        }
        BIP39MnemonicLanguages::CZECH => Ok(include_str!("wordlist/czech.txt")),
        BIP39MnemonicLanguages::ENGLISH => Ok(include_str!("wordlist/english.txt")),
        BIP39MnemonicLanguages::FRENCH => Ok(include_str!("wordlist/french.txt")),
        BIP39MnemonicLanguages::ITALIAN => Ok(include_str!("wordlist/italian.txt")),
        BIP39MnemonicLanguages::JAPANESE => Ok(include_str!("wordlist/japanese.txt")),
        BIP39MnemonicLanguages::KOREAN => Ok(include_str!("wordlist/korean.txt")),
        BIP39MnemonicLanguages::PORTUGUESE => Ok(include_str!("wordlist/portuguese.txt")),
        BIP39MnemonicLanguages::RUSSIAN => Ok(include_str!("wordlist/russian.txt")),
        BIP39MnemonicLanguages::SPANISH => Ok(include_str!("wordlist/spanish.txt")),
        BIP39MnemonicLanguages::TURKISH => Ok(include_str!("wordlist/turkish.txt")),
        _ => Err(MnemonicError::simple(&format!(
            "Unknown or unsupported BIP39 language: {}",
            language
        ))),
    }
}

fn load_normalized_wordlist(language: &str) -> Result<Vec<String>, MnemonicError> {
    let text = wordlist_embedded(language)?;
    let raw = parse_wordlist_text(text);
    if raw.len() != WORDS_LIST_NUMBER {
        return Err(MnemonicError::simple("Invalid number of loaded words list"));
    }
    Ok(raw.into_iter().map(|w| normalize_word(&w)).collect())
}

fn push_entropy_bits(entropy: &[u8], bits: &mut Vec<u8>) {
    for &b in entropy {
        for shift in (0..8).rev() {
            bits.push((b >> shift) & 1);
        }
    }
}

fn push_checksum_bits(hash: &[u8], bit_count: usize, bits: &mut Vec<u8>) {
    for i in 0..bit_count {
        let byte = hash[i / 8];
        let bit = (byte >> (7 - (i % 8))) & 1;
        bits.push(bit);
    }
}

fn bits_to_entropy_and_checksum(bits: &[u8]) -> Result<(Vec<u8>, Vec<u8>), MnemonicError> {
    if bits.len() % 33 != 0 {
        return Err(MnemonicError::simple("Invalid BIP39 mnemonic bit length"));
    }
    let cs_len = bits.len() / 33;
    let ent_len = bits.len() - cs_len;
    if ent_len % 8 != 0 {
        return Err(MnemonicError::simple("Invalid BIP39 entropy bit length"));
    }
    let mut entropy = Vec::with_capacity(ent_len / 8);
    for chunk in bits[..ent_len].chunks(8) {
        let mut b = 0u8;
        for &bit in chunk {
            b = (b << 1) | bit;
        }
        entropy.push(b);
    }
    let checksum = bits[ent_len..].to_vec();
    Ok((entropy, checksum))
}

fn is_valid_bip39_word_count(n: usize) -> bool {
    matches!(
        n,
        BIP39MnemonicWords::TWELVE
            | BIP39MnemonicWords::FIFTEEN
            | BIP39MnemonicWords::EIGHTEEN
            | BIP39MnemonicWords::TWENTY_ONE
            | BIP39MnemonicWords::TWENTY_FOUR
    )
}

fn hash_checksum_bits(entropy: &[u8], cs_len: usize) -> Vec<u8> {
    let hash = Sha256::digest(entropy);
    let mut out = Vec::with_capacity(cs_len);
    for i in 0..cs_len {
        let byte = hash[i / 8];
        let bit = (byte >> (7 - (i % 8))) & 1;
        out.push(bit);
    }
    out
}

impl BIP39Mnemonic {
    pub fn generate_random(word_count: usize, language: &str) -> Result<Self, MnemonicError> {
        if !is_valid_bip39_word_count(word_count) {
            return Err(MnemonicError::simple(&format!("Invalid word count {}", word_count)));
        }
        let strength = match word_count {
            BIP39MnemonicWords::TWELVE => 128,
            BIP39MnemonicWords::FIFTEEN => 160,
            BIP39MnemonicWords::EIGHTEEN => 192,
            BIP39MnemonicWords::TWENTY_ONE => 224,
            BIP39MnemonicWords::TWENTY_FOUR => 256,
            _ => unreachable!(),
        };
        let scheme = BIP39Entropy;
        let hex = scheme
            .generate(strength)
            .map_err(|e| MnemonicError::simple(&e.message))?;
        let bytes = hex::decode(hex.trim()).map_err(|e| MnemonicError::simple(&e.to_string()))?;
        Self::from_entropy_bytes(&bytes, language)
    }

    pub fn from_entropy_bytes(entropy: &[u8], language: &str) -> Result<Self, MnemonicError> {
        let phrase = Self::encode_bytes(entropy, language)?;
        Ok(Self {
            phrase,
            language: language.to_string(),
        })
    }

    pub fn parse_phrase(phrase: &str) -> Result<Self, MnemonicError> {
        let words: Vec<String> = phrase.split_whitespace().map(String::from).collect();
        let normalized: Vec<String> = words.iter().map(|w| normalize_word(w)).collect();
        let (_, lang) = Self::decode_with_detection(&normalized)?;
        Ok(Self {
            phrase: normalized.join(" "),
            language: lang,
        })
    }

    pub fn phrase(&self) -> String {
        self.phrase.clone()
    }

    pub fn to_entropy(&self) -> Result<Vec<u8>, MnemonicError> {
        let words: Vec<String> = self.phrase.split_whitespace().map(String::from).collect();
        Self::decode_with_lang(&words, &self.language)
    }

    pub fn is_valid_mnemonic_words(words: &[String]) -> bool {
        Self::decode(words).is_ok()
    }

    pub fn encode_bytes(entropy: &[u8], language: &str) -> Result<String, MnemonicError> {
        let scheme = BIP39Entropy;
        if !scheme.is_valid_bytes_strength(entropy.len()) {
            return Err(MnemonicError::simple("Wrong entropy strength for BIP39"));
        }
        let wl = load_normalized_wordlist(language)?;
        let mut bits = Vec::new();
        push_entropy_bits(entropy, &mut bits);
        let hash = Sha256::digest(entropy);
        let cs_bits = entropy.len() / 4;
        push_checksum_bits(hash.as_ref(), cs_bits, &mut bits);
        debug_assert_eq!(bits.len() % WORD_BIT_LENGTH, 0);

        let mut out = Vec::new();
        for chunk in bits.chunks(WORD_BIT_LENGTH) {
            let mut idx = 0u16;
            for &b in chunk {
                idx = (idx << 1) | b as u16;
            }
            let w = wl
                .get(idx as usize)
                .ok_or_else(|| MnemonicError::simple("BIP39 word index out of range"))?;
            out.push(normalize_word(w));
        }
        Ok(out.join(" "))
    }

    fn decode_with_lang(words: &[String], language: &str) -> Result<Vec<u8>, MnemonicError> {
        if !is_valid_bip39_word_count(words.len()) {
            return Err(MnemonicError::simple("Invalid mnemonic words count"));
        }
        let wl = load_normalized_wordlist(language)?;
        let map: HashMap<&str, u16> = wl
            .iter()
            .enumerate()
            .map(|(i, w)| (w.as_str(), i as u16))
            .collect();

        let mut bits: Vec<u8> = Vec::new();
        for w in words {
            let idx = *map
                .get(w.as_str())
                .ok_or_else(|| MnemonicError::simple("Unknown BIP39 word for language"))?;
            for shift in (0..WORD_BIT_LENGTH).rev() {
                bits.push(((idx >> shift) & 1) as u8);
            }
        }

        let (entropy, checksum_got) = bits_to_entropy_and_checksum(&bits)?;
        let expected = hash_checksum_bits(&entropy, checksum_got.len());
        if checksum_got != expected {
            return Err(MnemonicError::simple("Invalid BIP39 checksum"));
        }
        Ok(entropy)
    }

    fn decode_with_detection(normalized: &[String]) -> Result<(Vec<u8>, String), MnemonicError> {
        let mut last = MnemonicError::simple("Could not detect BIP39 language");
        for &lang in Self::languages() {
            match Self::decode_with_lang(normalized, lang) {
                Ok(entropy) => return Ok((entropy, lang.to_string())),
                Err(e) => last = e,
            }
        }
        Err(last)
    }
}

impl IMnemonic for BIP39Mnemonic {
    fn name() -> &'static str {
        "BIP39"
    }

    fn from_words(words: usize, language: &str) -> Result<String, MnemonicError> {
        Self::generate_random(words, language).map(|m| m.mnemonic())
    }

    fn from_entropy(entropy: &[u8], language: &str) -> Result<String, MnemonicError> {
        Self::encode(entropy, language)
    }

    fn encode(entropy: &[u8], language: &str) -> Result<String, MnemonicError> {
        Self::encode_bytes(entropy, language)
    }

    fn decode(mnemonic: &[String]) -> Result<Vec<u8>, MnemonicError> {
        let normalized: Vec<String> = mnemonic.iter().map(|w| normalize_word(w)).collect();
        let (entropy, _) = Self::decode_with_detection(&normalized)?;
        Ok(entropy)
    }

    fn mnemonic(&self) -> String {
        self.phrase.clone()
    }

    fn language(&self) -> &str {
        &self.language
    }

    fn words(&self) -> usize {
        self.phrase.split_whitespace().count()
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
            BIP39MnemonicLanguages::CHINESE_SIMPLIFIED,
            BIP39MnemonicLanguages::CHINESE_TRADITIONAL,
            BIP39MnemonicLanguages::CZECH,
            BIP39MnemonicLanguages::ENGLISH,
            BIP39MnemonicLanguages::FRENCH,
            BIP39MnemonicLanguages::ITALIAN,
            BIP39MnemonicLanguages::JAPANESE,
            BIP39MnemonicLanguages::KOREAN,
            BIP39MnemonicLanguages::PORTUGUESE,
            BIP39MnemonicLanguages::RUSSIAN,
            BIP39MnemonicLanguages::SPANISH,
            BIP39MnemonicLanguages::TURKISH,
        ]
    }

    fn wordlist_path() -> HashMap<&'static str, &'static str> {
        HashMap::from([
            (
                BIP39MnemonicLanguages::CHINESE_SIMPLIFIED,
                "mnemonics/bip39/wordlist/chinese_simplified.txt",
            ),
            (
                BIP39MnemonicLanguages::CHINESE_TRADITIONAL,
                "mnemonics/bip39/wordlist/chinese_traditional.txt",
            ),
            (BIP39MnemonicLanguages::CZECH, "mnemonics/bip39/wordlist/czech.txt"),
            (BIP39MnemonicLanguages::ENGLISH, "mnemonics/bip39/wordlist/english.txt"),
            (BIP39MnemonicLanguages::FRENCH, "mnemonics/bip39/wordlist/french.txt"),
            (BIP39MnemonicLanguages::ITALIAN, "mnemonics/bip39/wordlist/italian.txt"),
            (BIP39MnemonicLanguages::JAPANESE, "mnemonics/bip39/wordlist/japanese.txt"),
            (BIP39MnemonicLanguages::KOREAN, "mnemonics/bip39/wordlist/korean.txt"),
            (
                BIP39MnemonicLanguages::PORTUGUESE,
                "mnemonics/bip39/wordlist/portuguese.txt",
            ),
            (BIP39MnemonicLanguages::RUSSIAN, "mnemonics/bip39/wordlist/russian.txt"),
            (BIP39MnemonicLanguages::SPANISH, "mnemonics/bip39/wordlist/spanish.txt"),
            (BIP39MnemonicLanguages::TURKISH, "mnemonics/bip39/wordlist/turkish.txt"),
        ])
    }

    fn get_words_list_by_language(language: &str) -> Result<Vec<String>, MnemonicError> {
        load_normalized_wordlist(language)
    }

    fn find_language(mnemonic: &[String]) -> Result<(Vec<String>, String), MnemonicError> {
        let normalized: Vec<String> = mnemonic.iter().map(|w| normalize_word(w)).collect();
        let (_, lang_key) = Self::decode_with_detection(&normalized)?;
        let words = load_normalized_wordlist(&lang_key)?;
        Ok((words, lang_key))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hdwallet::mnemonics::imnemonic::IMnemonic;

    #[test]
    fn generate_and_round_trip() {
        let phrase = BIP39Mnemonic::from_words(12, BIP39MnemonicLanguages::ENGLISH).unwrap();
        let words: Vec<String> = phrase.split_whitespace().map(String::from).collect();
        let entropy = BIP39Mnemonic::decode(&words).unwrap();
        assert_eq!(entropy.len(), 16);
        let again = BIP39Mnemonic::encode(&entropy, BIP39MnemonicLanguages::ENGLISH).unwrap();
        assert_eq!(again, phrase);
    }

    #[test]
    fn russian_round_trip() {
        let phrase = BIP39Mnemonic::from_words(12, BIP39MnemonicLanguages::RUSSIAN).unwrap();
        let words: Vec<String> = phrase.split_whitespace().map(String::from).collect();
        let entropy = BIP39Mnemonic::decode(&words).unwrap();
        let again = BIP39Mnemonic::encode(&entropy, BIP39MnemonicLanguages::RUSSIAN).unwrap();
        assert_eq!(again, phrase);
    }

    #[test]
    fn turkish_round_trip() {
        let phrase = BIP39Mnemonic::from_words(15, BIP39MnemonicLanguages::TURKISH).unwrap();
        let words: Vec<String> = phrase.split_whitespace().map(String::from).collect();
        let entropy = BIP39Mnemonic::decode(&words).unwrap();
        let again = BIP39Mnemonic::encode(&entropy, BIP39MnemonicLanguages::TURKISH).unwrap();
        assert_eq!(again, phrase);
    }
}
