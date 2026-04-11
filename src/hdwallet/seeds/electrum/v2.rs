use hmac::{Hmac, KeyInit, Mac};
use sha2::Sha512;
use unicode_normalization::UnicodeNormalization;

use crate::hdwallet::exceptions::SeedError;
use crate::hdwallet::mnemonics::electrum::{ElectrumV2Mnemonic, ElectrumV2MnemonicTypes};
use crate::hdwallet::mnemonics::imnemonic::IMnemonic;
use crate::hdwallet::seeds::iseed::ISeed;

type HmacSha512 = Hmac<Sha512>;

fn pbkdf2_hmac_sha512(password: &[u8], salt: &[u8], iteration_num: u32, out: &mut [u8]) {
    let mut offset = 0usize;
    let mut block_num = 1u32;
    while offset < out.len() {
        let mut mac = HmacSha512::new_from_slice(password).expect("HMAC key length");
        mac.update(salt);
        mac.update(&block_num.to_be_bytes());
        let mut u = mac.finalize().into_bytes();
        let mut t = u;
        for _ in 1..iteration_num {
            let mut mac2 = HmacSha512::new_from_slice(password).expect("HMAC key length");
            mac2.update(&u);
            u = mac2.finalize().into_bytes();
            for j in 0..64 {
                t[j] ^= u[j];
            }
        }
        let len = (out.len() - offset).min(64);
        out[offset..offset + len].copy_from_slice(&t[..len]);
        offset += len;
        block_num = block_num.wrapping_add(1);
    }
}

fn bytes_to_string(dk: &[u8]) -> String {
    hex::encode(dk)
}

pub struct ElectrumV2Seed {
    seed: String,
}

impl ElectrumV2Seed {
    pub const SEED_SALT_MODIFIER: &'static str = "electrum";
    pub const SEED_PBKDF2_ROUNDS: u32 = 2048;
    pub const LENGTH: usize = 128;

    pub fn from_mnemonic_phrase(
        mnemonic_phrase: &str,
        passphrase: Option<&str>,
        mnemonic_type: &str,
    ) -> Result<String, SeedError> {
        let words: Vec<String> = mnemonic_phrase
            .split_whitespace()
            .map(|w| w.to_lowercase())
            .collect();

        if !ElectrumV2Mnemonic::is_valid(&words, mnemonic_type) {
            return Err(SeedError::new(
                &format!("Invalid {} mnemonic words", Self::name()),
                None,
                None,
                None,
            ));
        }

        let mnemonic_normalized = words.join(" ");

        let salt_raw = match passphrase {
            Some(p) => format!("{}{}", Self::SEED_SALT_MODIFIER, p),
            None => Self::SEED_SALT_MODIFIER.to_string(),
        };
        let salt: String = salt_raw.nfkd().collect();

        let mut dk = vec![0u8; 64];
        pbkdf2_hmac_sha512(
            mnemonic_normalized.as_bytes(),
            salt.as_bytes(),
            Self::SEED_PBKDF2_ROUNDS,
            &mut dk,
        );

        Ok(bytes_to_string(&dk))
    }
}

impl ISeed for ElectrumV2Seed {
    const LENGTH: usize = 128;

    fn new(seed: String) -> Self {
        Self { seed }
    }

    fn name() -> &'static str {
        "Electrum-V2"
    }

    fn seed(&self) -> &str {
        &self.seed
    }

    fn from_mnemonic(mnemonic: &dyn IMnemonic) -> Result<String, SeedError> {
        let mtype = mnemonic
            .mnemonic_type()
            .unwrap_or(ElectrumV2MnemonicTypes::STANDARD);
        Self::from_mnemonic_phrase(&mnemonic.mnemonic(), None, mtype)
    }
}
