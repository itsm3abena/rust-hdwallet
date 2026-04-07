use std::collections::HashMap;

use crate::entropies::{ElectrumV2Entropy, IEntropy, ELECTRUM_V2_ENTROPY_STRENGTHS};
use crate::exceptions::{Error, EntropyError, MnemonicError};
use crate::utils::{get_bytes, integer_to_bytes, bytes_to_string, bytes_to_integer};
use crate::crypto::hmac_sha512;
use crate::mnemonics::bip39::BIP39Mnemonic;
use crate::mnemonics::electrum::v1::ElectrumV1Mnemonic;


pub struct ELECTRUM_V2_MNEMONIC_WORDS;

impl ELECTRUM_V2_MNEMONIC_WORDS {
    pub const TWELVE: usize = 12;
    pub const TWENTY_FOUR: usize = 24;
}

pub struct ELECTRUM_V2_MNEMONIC_LANGUAGES;

impl ELECTRUM_V2_MNEMONIC_LANGUAGES {
    pub const CHINESE_SIMPLIFIED: &'static str = "chinese-simplified";
    pub const ENGLISH: &'static str = "english";
    pub const PORTUGUESE: &'static str = "portuguese";
    pub const SPANISH: &'static str = "spanish";
}

pub struct ELECTRUM_V2_MNEMONIC_TYPES;

impl ELECTRUM_V2_MNEMONIC_TYPES {
    pub const STANDARD: &'static str = "standard";
    pub const SEGWIT: &'static str = "segwit";
    pub const STANDARD_2FA: &'static str = "standard-2fa";
    pub const SEGWIT_2FA: &'static str = "segwit-2fa";
}


pub struct ElectrumV2Mnemonic;

impl ElectrumV2Mnemonic {
    pub const WORD_BIT_LENGTH: usize = 11;

    pub fn words_list() -> Vec<usize> {
        vec![
            ELECTRUM_V2_MNEMONIC_WORDS::TWELVE,
            ELECTRUM_V2_MNEMONIC_WORDS::TWENTY_FOUR,
        ]
    }

    pub fn words_to_entropy_strength() -> HashMap<usize, usize> {
        let mut map = HashMap::new();
        map.insert(
            ELECTRUM_V2_MNEMONIC_WORDS::TWELVE,
            ELECTRUM_V2_ENTROPY_STRENGTHS::ONE_HUNDRED_THIRTY_TWO,
        );
        map.insert(
            ELECTRUM_V2_MNEMONIC_WORDS::TWENTY_FOUR,
            ELECTRUM_V2_ENTROPY_STRENGTHS::TWO_HUNDRED_SIXTY_FOUR,
        );
        map
    }

    pub fn mnemonic_types() -> HashMap<&'static str, &'static str> {
        let mut map = HashMap::new();
        map.insert(ELECTRUM_V2_MNEMONIC_TYPES::STANDARD, "01");
        map.insert(ELECTRUM_V2_MNEMONIC_TYPES::SEGWIT, "100");
        map.insert(ELECTRUM_V2_MNEMONIC_TYPES::STANDARD_2FA, "101");
        map.insert(ELECTRUM_V2_MNEMONIC_TYPES::SEGWIT_2FA, "102");
        map
    }

    pub fn name() -> &'static str {
        "Electrum-V2"
    }

    pub fn from_words(
        words: usize,
        language: &str,
        mnemonic_type: &str,
        max_attempts: u128,
    ) -> Result<String, Error> {
        if !Self::words_list().contains(&words) {
            return Err(MnemonicError::new(
                "Invalid mnemonic words number",
                Some(format!("{:?}", Self::words_list())),
                Some(words.to_string()),
            )
            .into());
        }

        let entropy_strength = Self::words_to_entropy_strength()[&words];

        let entropy = ElectrumV2Entropy::generate(entropy_strength);

        Self::from_entropy(entropy, language, mnemonic_type, max_attempts)
    }

    pub fn from_entropy<T: Into<Vec<u8>>>(
        entropy: T,
        language: &str,
        mnemonic_type: &str,
        max_attempts: u128,
    ) -> Result<String, Error> {
        let entropy_bytes = get_bytes(entropy.into());

        if !ElectrumV2Entropy::are_entropy_bits_enough(&entropy_bytes) {
            return Err(EntropyError::new(
                "Invalid entropy strength",
                None,
                None,
            )
            .into());
        }

        let mut entropy_int = bytes_to_integer(&entropy_bytes);

        for i in 0..max_attempts {
            let new_entropy = entropy_int + i as u128;

            let result = Self::encode(
                integer_to_bytes(new_entropy),
                language,
                mnemonic_type,
            );

            if let Ok(m) = result {
                return Ok(m);
            }
        }

        Err(Error::new("Unable to generate a valid mnemonic", None, None))
    }

    pub fn encode(
        entropy: Vec<u8>,
        language: &str,
        mnemonic_type: &str,
    ) -> Result<String, Error> {
        let mut entropy_int = bytes_to_integer(&entropy);

        if !ElectrumV2Entropy::are_entropy_bits_enough(&entropy) {
            return Err(EntropyError::new(
                "Entropy bit length is not enough",
                None,
                None,
            )
            .into());
        }

        let words_list = Self::get_words_list(language)?;

        let mut mnemonic: Vec<String> = Vec::new();

        while entropy_int > 0 {
            let idx = (entropy_int % words_list.len() as u128) as usize;
            entropy_int /= words_list.len() as u128;
            mnemonic.push(words_list[idx].clone());
        }

        if !Self::is_valid(&mnemonic, mnemonic_type) {
            return Err(EntropyError::new(
                "Invalid mnemonic generated",
                None,
                None,
            )
            .into());
        }

        Ok(mnemonic.join(" "))
    }

    pub fn decode(
        mnemonic: &str,
        mnemonic_type: &str,
    ) -> Result<String, Error> {
        let words = Self::normalize_str(mnemonic);

        if !Self::words_list().contains(&words.len()) {
            return Err(MnemonicError::new(
                "Invalid mnemonic words count",
                None,
                Some(words.len().to_string()),
            )
            .into());
        }

        if !Self::is_valid(&words, mnemonic_type) {
            return Err(MnemonicError::new(
                "Invalid mnemonic type",
                None,
                None,
            )
            .into());
        }

        let words_list = Self::get_words_list("english")?;
        let mut map = HashMap::new();

        for (i, w) in words_list.iter().enumerate() {
            map.insert(w.clone(), i);
        }

        let mut entropy: u128 = 0;

        for word in words.iter().rev() {
            entropy = entropy * words_list.len() as u128 + map[word] as u128;
        }

        Ok(bytes_to_string(&integer_to_bytes(entropy)))
    }

    pub fn is_valid(
        mnemonic: &Vec<String>,
        mnemonic_type: &str,
    ) -> bool {
        if BIP39Mnemonic::is_valid(mnemonic)
            || ElectrumV1Mnemonic::is_valid(mnemonic)
        {
            return false;
        }

        Self::is_type(mnemonic, mnemonic_type)
    }

    pub fn is_type(
        mnemonic: &Vec<String>,
        mnemonic_type: &str,
    ) -> bool {
        let joined = mnemonic.join(" ");
        let hash = hmac_sha512(b"Seed version", joined.as_bytes());

        let prefix = bytes_to_string(&hash);

        prefix.starts_with(Self::mnemonic_types()[mnemonic_type])
    }

    pub fn normalize_str(mnemonic: &str) -> Vec<String> {
        mnemonic
            .split_whitespace()
            .map(|w| w.to_lowercase())
            .collect()
    }


    fn get_words_list(language: &str) -> Result<Vec<String>, Error> {
        // TODO: load from wordlist files
        Err(Error::new("Not implemented", None, None))
    }
}