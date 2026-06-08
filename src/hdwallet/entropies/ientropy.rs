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

/// Object-safe entropy scheme (mnemonic word list family).
pub trait IEntropy {
    fn name(&self) -> &'static str;

    fn strengths(&self) -> &'static [usize];

    fn is_valid_strength(&self, strength: usize) -> bool {
        self.strengths().contains(&strength)
    }

    fn is_valid_bytes_strength(&self, bytes_len: usize) -> bool {
        self.is_valid_strength(bytes_len * 8)
    }

    fn generate(&self, strength: usize) -> Result<String, EntropyError> {
        if !self.is_valid_strength(strength) {
            return Err(EntropyError::new("Unsupported entropy strength"));
        }

        let mut bytes = vec![0u8; strength / 8];
        rand::thread_rng().fill_bytes(&mut bytes);
        Ok(hex::encode(bytes))
    }

    fn is_valid_entropy_hex(&self, entropy: &str) -> bool {
        let re = Regex::new(r"^[0-9a-fA-F]+$").unwrap();
        re.is_match(entropy) && self.is_valid_strength(entropy.len() * 4)
    }

    fn strength_bits_from_hex(&self, entropy: &str) -> Result<usize, EntropyError> {
        let bytes_len = entropy.len() / 2;
        if !self.is_valid_bytes_strength(bytes_len) {
            return Err(EntropyError::new("Unsupported entropy strength"));
        }
        if !self.is_valid_entropy_hex(entropy) {
            return Err(EntropyError::new("Invalid entropy data"));
        }
        Ok(bytes_len * 8)
    }
}

pub struct EntropyInstance {
    entropy: String,
    strength: usize,
}

impl EntropyInstance {
    pub fn new<T: IEntropy + Default>(entropy: &str) -> Result<Self, EntropyError> {
        let scheme = T::default();
        let bytes_len = entropy.len() / 2;

        if !scheme.is_valid_bytes_strength(bytes_len) {
            return Err(EntropyError::new("Unsupported entropy strength"));
        }

        if !scheme.is_valid_entropy_hex(entropy) {
            return Err(EntropyError::new("Invalid entropy data"));
        }

        Ok(Self {
            entropy: entropy.to_string(),
            strength: scheme.strength_bits_from_hex(entropy)?,
        })
    }

    pub fn entropy(&self) -> &str {
        &self.entropy
    }

    pub fn strength_bits(&self) -> usize {
        self.strength
    }
}
