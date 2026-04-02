use rand::RngCore;
use regex::Regex;
use std::fmt;

#[derive(Debug)]
pub struct EntropyError {
    pub message: String,
}

impl EntropyError {
    pub fn new(msg: &str) -> Self {
        Self {
            message: msg.to_string(),
        }
    }
}

impl std::error::Error for EntropyError {}

impl fmt::Display for EntropyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "EntropyError: {}", self.message)
    }
}

pub trait IEntropy {
    fn name() -> &'static str
    where
        Self: Sized;

    fn strengths() -> &'static [usize]
    where
        Self: Sized;

    fn is_valid_strength(strength: usize) -> bool {
        Self::strengths().contains(&strength)
    }

    fn is_valid_bytes_strength(bytes_len: usize) -> bool {
        Self::is_valid_strength(bytes_len * 8)
    }

    fn generate(strength: usize) -> Result<String, EntropyError> {
        if !Self::is_valid_strength(strength) {
            return Err(EntropyError::new("Unsupported entropy strength"));
        }

        let mut bytes = vec![0u8; strength / 8];
        rand::thread_rng().fill_bytes(&mut bytes);
        Ok(hex::encode(bytes))
    }

    fn is_valid(entropy: &str) -> bool {
        let re = Regex::new(r"^[0-9a-fA-F]+$").unwrap();
        re.is_match(entropy) && Self::is_valid_strength(entropy.len() * 4)
    }
}

pub struct EntropyInstance {
    entropy: String,
    strength: usize,
}

impl EntropyInstance {
    pub fn new<T: IEntropy>(entropy: &str) -> Result<Self, EntropyError> {
        let bytes_len = entropy.len() / 2;

        if !T::is_valid_bytes_strength(bytes_len) {
            return Err(EntropyError::new("Unsupported entropy strength"));
        }

        if !IEntropy::is_valid(entropy) {
            return Err(EntropyError::new("Invalid entropy data"));
        }

        Ok(Self {
            entropy: entropy.to_string(),
            strength: bytes_len * 8,
        })
    }

    pub fn entropy(&self) -> &str {
        &self.entropy
    }

    pub fn strength(&self) -> usize {
        self.strength
    }
}