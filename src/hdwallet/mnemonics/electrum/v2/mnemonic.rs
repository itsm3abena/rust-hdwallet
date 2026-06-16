use std::collections::HashMap;

use crate::hdwallet::mnemonics::bip39::BIP39Mnemonic;
use num_bigint::BigUint;
use num_traits::{One, ToPrimitive, Zero};

use crate::hdwallet::entropies::electrum::ElectrumV2EntropyStrengths;
use crate::hdwallet::entropies::{ElectrumV2Entropy, IEntropy};
use crate::hdwallet::exceptions::MnemonicError;
use crate::hdwallet::mnemonics::electrum::v1::ElectrumV1Mnemonic;
use crate::hdwallet::utils::{hmac_sha512, parse_wordlist_text};
use crate::hdwallet::mnemonics::imnemonic::IMnemonic;

struct EncodeContext {
    words_list: Vec<String>,
    wordlist_len: BigUint,
    bip39_words: Vec<String>,
    v1_words: Vec<String>,
    v1_index: HashMap<String, usize>,
}

impl EncodeContext {
    fn new(language: &str) -> Result<Self, MnemonicError> {
        let words_list = ElectrumV2Mnemonic::get_words_list(language)?;
        let wordlist_len = BigUint::from(words_list.len());
        let bip39_words = BIP39Mnemonic::get_words_list_by_language(language)?;

        let (v1_words, v1_index) = match ElectrumV1Mnemonic::get_words_list_by_language(language) {
            Ok(list) => {
                let mut index = HashMap::with_capacity(list.len());
                for (i, word) in list.iter().enumerate() {
                    index.insert(word.clone(), i);
                }
                (list, index)
            }
            Err(_) => (Vec::new(), HashMap::new()),
        };

        Ok(Self {
            words_list,
            wordlist_len,
            bip39_words,
            v1_words,
            v1_index,
        })
    }
}

pub struct ElectrumV2MnemonicWordCounts;

impl ElectrumV2MnemonicWordCounts {
    pub const TWELVE: usize = 12;
    pub const TWENTY_FOUR: usize = 24;
}

pub struct ElectrumV2MnemonicLanguages;

impl ElectrumV2MnemonicLanguages {
    pub const CHINESE_SIMPLIFIED: &'static str = "chinese-simplified";
    pub const ENGLISH: &'static str = "english";
    pub const PORTUGUESE: &'static str = "portuguese";
    pub const SPANISH: &'static str = "spanish";
}

pub struct ElectrumV2MnemonicTypes;

impl ElectrumV2MnemonicTypes {
    pub const STANDARD: &'static str = "standard";
    pub const SEGWIT: &'static str = "segwit";
    pub const STANDARD_2FA: &'static str = "standard-2fa";
    pub const SEGWIT_2FA: &'static str = "segwit-2fa";
}

#[derive(Clone, Debug)]
pub struct ElectrumV2Mnemonic {
    phrase: String,
    mnemonic_type: String,
    language: String,
}

impl ElectrumV2Mnemonic {
    pub fn mnemonic_types_map() -> HashMap<&'static str, &'static str> {
        HashMap::from([
            (ElectrumV2MnemonicTypes::STANDARD, "01"),
            (ElectrumV2MnemonicTypes::SEGWIT, "100"),
            (ElectrumV2MnemonicTypes::STANDARD_2FA, "101"),
            (ElectrumV2MnemonicTypes::SEGWIT_2FA, "102"),
        ])
    }

    pub fn words_list() -> Vec<usize> {
        vec![
            ElectrumV2MnemonicWordCounts::TWELVE,
            ElectrumV2MnemonicWordCounts::TWENTY_FOUR,
        ]
    }

    pub fn words_to_entropy_strength() -> HashMap<usize, usize> {
        HashMap::from([
            (
                ElectrumV2MnemonicWordCounts::TWELVE,
                ElectrumV2EntropyStrengths::ONE_HUNDRED_THIRTY_TWO,
            ),
            (
                ElectrumV2MnemonicWordCounts::TWENTY_FOUR,
                ElectrumV2EntropyStrengths::TWO_HUNDRED_SIXTY_FOUR,
            ),
        ])
    }

    fn wordlist_embedded(language: &str) -> Result<&'static str, MnemonicError> {
        match language {
            ElectrumV2MnemonicLanguages::ENGLISH => Ok(include_str!("wordlist/english.txt")),
            ElectrumV2MnemonicLanguages::CHINESE_SIMPLIFIED => {
                Ok(include_str!("wordlist/chinese_simplified.txt"))
            }
            ElectrumV2MnemonicLanguages::PORTUGUESE => Ok(include_str!("wordlist/portuguese.txt")),
            ElectrumV2MnemonicLanguages::SPANISH => Ok(include_str!("wordlist/spanish.txt")),
            _ => Err(MnemonicError::simple(&format!(
                "Unknown Electrum V2 language: {}",
                language
            ))),
        }
    }

    pub fn get_words_list(language: &str) -> Result<Vec<String>, MnemonicError> {
        let text = Self::wordlist_embedded(language)?;
        let mut words = parse_wordlist_text(text);
        for w in &mut words {
            *w = w.to_lowercase();
        }
        Ok(words)
    }

    pub fn find_language(words: &[String]) -> Result<(Vec<String>, String), MnemonicError> {
        for &language in Self::languages() {
            let words_list = Self::get_words_list(language)?;
            if words
                .iter()
                .all(|word| words_list.iter().any(|candidate| candidate == word))
            {
                return Ok((words_list, language.to_string()));
            }
        }
        Err(MnemonicError::simple("Could not detect Electrum V2 language"))
    }

    fn looks_like_bip39(words: &[String], bip39_words: &[String]) -> bool {
        BIP39Mnemonic::is_valid_mnemonic_words_with_wordlist(words, bip39_words)
    }

    fn looks_like_electrum_v1(
        words: &[String],
        v1_words: &[String],
        v1_index: &HashMap<String, usize>,
    ) -> bool {
        if v1_words.is_empty() {
            return false;
        }
        ElectrumV1Mnemonic::decode_phrase(
            &words.join(" "),
            Some(v1_words.to_vec()),
            Some(v1_index.clone()),
        )
        .is_ok()
    }

    fn is_valid_with_context(
        words: &[String],
        mnemonic_type: &str,
        ctx: &EncodeContext,
    ) -> bool {
        if !Self::is_type(words, mnemonic_type) {
            return false;
        }
        if Self::looks_like_bip39(words, &ctx.bip39_words) {
            return false;
        }
        !Self::looks_like_electrum_v1(words, &ctx.v1_words, &ctx.v1_index)
    }

    fn encode_entropy(
        entropy_bn: &BigUint,
        ctx: &EncodeContext,
        mnemonic_type: &str,
    ) -> Result<Vec<String>, MnemonicError> {
        if !ElectrumV2Entropy::are_entropy_bits_enough(entropy_bn) {
            return Err(MnemonicError::simple("Entropy bit length is not enough for Electrum V2"));
        }

        let n = &ctx.wordlist_len;
        let mut e = entropy_bn.clone();
        let mut mnemonic: Vec<String> = Vec::new();

        while e > BigUint::zero() {
            let rem = &e % n;
            let idx = rem
                .to_usize()
                .ok_or_else(|| MnemonicError::simple("Word index overflow"))?;
            mnemonic.push(
                ctx.words_list
                    .get(idx)
                    .ok_or_else(|| MnemonicError::simple("Word index out of range"))?
                    .clone(),
            );
            e /= n;
        }

        if mnemonic.is_empty() {
            return Err(MnemonicError::simple("Empty Electrum V2 mnemonic"));
        }

        if !Self::is_valid_with_context(&mnemonic, mnemonic_type, ctx) {
            return Err(MnemonicError::simple(
                "Generated phrase failed Electrum V2 HMAC check (try another entropy)",
            ));
        }

        Ok(mnemonic)
    }

    pub fn is_type(words: &[String], mnemonic_type: &str) -> bool {
        let types = Self::mnemonic_types_map();
        let Some(expected_prefix) = types.get(mnemonic_type) else {
            return false;
        };
        let joined = words.join(" ");
        let hash = hmac_sha512(b"Seed version", joined.as_bytes());
        let need_bytes = expected_prefix.len().div_ceil(2);
        let partial = hex::encode(&hash[..need_bytes.min(hash.len())]);
        partial.starts_with(expected_prefix)
    }

    pub fn is_valid(words: &[String], mnemonic_type: &str) -> bool {
        if !Self::is_type(words, mnemonic_type) {
            return false;
        }
        if BIP39Mnemonic::is_valid_mnemonic_words(words) {
            return false;
        }
        !ElectrumV1Mnemonic::decode_phrase(&words.join(" "), None, None).is_ok()
    }

    pub fn encode_bytes(
        entropy: &[u8],
        language: &str,
        mnemonic_type: &str,
    ) -> Result<String, MnemonicError> {
        let ctx = EncodeContext::new(language)?;
        let entropy_bn = BigUint::from_bytes_be(entropy);
        let mnemonic = Self::encode_entropy(&entropy_bn, &ctx, mnemonic_type)?;
        Ok(mnemonic.join(" "))
    }

    pub fn from_entropy_with_retries(
        entropy_hex: &str,
        language: &str,
        mnemonic_type: &str,
        max_attempts: u128,
    ) -> Result<Self, MnemonicError> {
        let raw = hex::decode(entropy_hex.trim())
            .map_err(|e| MnemonicError::simple(&e.to_string()))?;
        let mut e = BigUint::from_bytes_be(&raw);
        let ctx = EncodeContext::new(language)?;

        for _ in 0..max_attempts {
            if let Ok(mnemonic) = Self::encode_entropy(&e, &ctx, mnemonic_type) {
                return Ok(Self {
                    phrase: mnemonic.join(" "),
                    mnemonic_type: mnemonic_type.to_string(),
                    language: language.to_string(),
                });
            }
            e += BigUint::one();
        }
        Err(MnemonicError::simple(
            "Unable to generate a valid mnemonic",
        ))
    }

    pub fn generate_random(
        words: usize,
        language: &str,
        mnemonic_type: &str,
    ) -> Result<Self, MnemonicError> {
        if !Self::words_list().contains(&words) {
            return Err(MnemonicError::simple("Invalid mnemonic words number"));
        }
        let strength = *Self::words_to_entropy_strength()
            .get(&words)
            .ok_or_else(|| MnemonicError::simple("Internal word count map"))?;
        let scheme = ElectrumV2Entropy;
        let hex_str = scheme
            .generate(strength)
            .map_err(|e| MnemonicError::simple(&e.to_string()))?;
        Self::from_entropy_with_retries(&hex_str, language, mnemonic_type, 50_000)
    }

    pub fn decode_phrase(mnemonic: &str, mnemonic_type: &str) -> Result<Vec<u8>, MnemonicError> {
        let words: Vec<String> = mnemonic
            .split_whitespace()
            .map(|w| w.to_lowercase())
            .collect();

        if !Self::words_list().contains(&words.len()) {
            return Err(MnemonicError::simple("Invalid mnemonic words"));
        }

        if !Self::is_valid(&words, mnemonic_type) {
            return Err(MnemonicError::simple("Invalid Electrum V2 mnemonic / type"));
        }

        let (words_list, _) = Self::find_language(&words)?;
        let mut map = HashMap::new();
        for (i, w) in words_list.iter().enumerate() {
            map.insert(w.clone(), i);
        }

        let n = BigUint::from(words_list.len());
        let mut entropy = BigUint::zero();

        for word in words.iter().rev() {
            let idx = BigUint::from(
                *map
                    .get(word)
                    .ok_or_else(|| MnemonicError::simple("Unknown word"))?,
            );
            entropy = entropy * &n + idx;
        }

        Ok(entropy.to_bytes_be())
    }

    pub fn phrase(&self) -> &str {
        &self.phrase
    }

    pub fn mnemonic_type_name(&self) -> &str {
        &self.mnemonic_type
    }
}

impl IMnemonic for ElectrumV2Mnemonic {
    fn name() -> &'static str {
        "Electrum-V2"
    }

    fn from_words(words: usize, language: &str) -> Result<String, MnemonicError> {
        Self::generate_random(words, language, ElectrumV2MnemonicTypes::STANDARD)
            .map(|m| m.mnemonic())
    }

    fn from_entropy(entropy: &[u8], language: &str) -> Result<String, MnemonicError> {
        Self::encode_bytes(entropy, language, ElectrumV2MnemonicTypes::STANDARD)
    }

    fn encode(entropy: &[u8], language: &str) -> Result<String, MnemonicError> {
        Self::encode_bytes(entropy, language, ElectrumV2MnemonicTypes::STANDARD)
    }

    fn decode(mnemonic: &[String]) -> Result<Vec<u8>, MnemonicError> {
        Self::decode_phrase(&mnemonic.join(" "), ElectrumV2MnemonicTypes::STANDARD)
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

    fn mnemonic_type(&self) -> Option<&str> {
        Some(&self.mnemonic_type)
    }

    fn words_list() -> &'static [usize] {
        &[
            ElectrumV2MnemonicWordCounts::TWELVE,
            ElectrumV2MnemonicWordCounts::TWENTY_FOUR,
        ]
    }

    fn languages() -> &'static [&'static str] {
        &[
            ElectrumV2MnemonicLanguages::ENGLISH,
            ElectrumV2MnemonicLanguages::CHINESE_SIMPLIFIED,
            ElectrumV2MnemonicLanguages::PORTUGUESE,
            ElectrumV2MnemonicLanguages::SPANISH,
        ]
    }

    fn wordlist_path() -> HashMap<&'static str, &'static str> {
        HashMap::from([(
            ElectrumV2MnemonicLanguages::ENGLISH,
            "electrum/v2/wordlist/english.txt",
        )])
    }
}
