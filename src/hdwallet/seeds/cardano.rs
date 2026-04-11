use blake2::digest::{consts::U32, Digest};
use blake2::Blake2b;
use regex::Regex;

use crate::hdwallet::exceptions::{Error, MnemonicError, SeedError};
use crate::hdwallet::mnemonics::bip39::BIP39Mnemonic;
use crate::hdwallet::mnemonics::imnemonic::IMnemonic;
use crate::hdwallet::seeds::{bip39::BIP39Seed, iseed::ISeed};

fn blake2b_256(data: &[u8]) -> Vec<u8> {
    Blake2b::<U32>::digest(data).to_vec()
}

pub struct CardanoTypes;

impl CardanoTypes {
    pub const BYRON_ICARUS: &'static str = "byron-icarus";
    pub const BYRON_LEDGER: &'static str = "byron-ledger";
    pub const BYRON_LEGACY: &'static str = "byron-legacy";
    pub const SHELLEY_ICARUS: &'static str = "shelley-icarus";
    pub const SHELLEY_LEDGER: &'static str = "shelley-ledger";

    pub fn all() -> Vec<&'static str> {
        vec![
            Self::BYRON_ICARUS,
            Self::BYRON_LEDGER,
            Self::BYRON_LEGACY,
            Self::SHELLEY_ICARUS,
            Self::SHELLEY_LEDGER,
        ]
    }
}

pub struct CardanoSeed {
    seed: String,
    cardano_type: String,
}

impl ISeed for CardanoSeed {
    const LENGTH: usize = 128;

    fn new(seed: String) -> Self {
        Self {
            seed,
            cardano_type: CardanoTypes::BYRON_ICARUS.to_string(),
        }
    }

    fn name() -> &'static str {
        "Cardano"
    }

    fn seed(&self) -> &str {
        &self.seed
    }

    fn from_mnemonic(mnemonic: &dyn IMnemonic) -> Result<String, SeedError> {
        Self::from_mnemonic_with_type(&mnemonic.mnemonic(), None, CardanoTypes::BYRON_ICARUS)
            .map_err(Into::into)
    }
}

impl CardanoSeed {
    const LENGTHS: [usize; 3] = [32, 128, 64];

    pub fn new_with_type(seed: String, cardano_type: &str) -> Result<Self, SeedError> {
        if !CardanoTypes::all().contains(&cardano_type) {
            return Err(SeedError::new(
                &format!(
                    "Invalid Cardano type (expected one of {:?})",
                    CardanoTypes::all()
                ),
                None,
                None,
                Some(cardano_type),
            ));
        }

        Ok(Self {
            seed,
            cardano_type: cardano_type.to_string(),
        })
    }

    pub fn cardano_type(&self) -> &str {
        &self.cardano_type
    }

    pub fn is_valid(seed: &str, cardano_type: &str) -> Result<bool, SeedError> {
        let re = Regex::new(r"^[0-9a-fA-F]+$").unwrap();

        if !re.is_match(seed) {
            return Ok(false);
        }

        match cardano_type {
            CardanoTypes::BYRON_ICARUS | CardanoTypes::SHELLEY_ICARUS => {
                Ok(seed.len() == Self::LENGTHS[0])
            }
            CardanoTypes::BYRON_LEDGER | CardanoTypes::SHELLEY_LEDGER => {
                Ok(seed.len() == Self::LENGTHS[1])
            }
            CardanoTypes::BYRON_LEGACY => Ok(seed.len() == Self::LENGTHS[2]),
            _ => Err(SeedError::new(
                &format!(
                    "Invalid Cardano type (expected one of {:?})",
                    CardanoTypes::all()
                ),
                None,
                None,
                Some(cardano_type),
            )),
        }
    }

    pub fn from_mnemonic_with_type(
        mnemonic: &str,
        passphrase: Option<&str>,
        cardano_type: &str,
    ) -> Result<String, Error> {
        match cardano_type {
            CardanoTypes::BYRON_ICARUS => Self::generate_byron_icarus(mnemonic),
            CardanoTypes::BYRON_LEDGER => Self::generate_byron_ledger(mnemonic, passphrase),
            CardanoTypes::BYRON_LEGACY => Self::generate_byron_legacy(mnemonic),
            CardanoTypes::SHELLEY_ICARUS => Self::generate_shelley_icarus(mnemonic),
            CardanoTypes::SHELLEY_LEDGER => Self::generate_shelley_ledger(mnemonic, passphrase),
            _ => Err(Error::new(
                &format!(
                    "Invalid Cardano type (expected one of {:?})",
                    CardanoTypes::all()
                ),
                None,
                None,
                Some(cardano_type),
            )),
        }
    }

    pub fn generate_byron_icarus(mnemonic: &str) -> Result<String, Error> {
        let words: Vec<String> = mnemonic.split_whitespace().map(String::from).collect();
        if !BIP39Mnemonic::is_valid_mnemonic_words(&words) {
            return Err(MnemonicError::new(
                &format!("Invalid {} mnemonic words", BIP39Mnemonic::name()),
                None,
                None,
                None,
            )
            .into());
        }

        let entropy = BIP39Mnemonic::decode(&words)?;
        Ok(hex::encode(entropy))
    }

    pub fn generate_byron_ledger(
        mnemonic: &str,
        passphrase: Option<&str>,
    ) -> Result<String, Error> {
        BIP39Seed::from_mnemonic_with_passphrase(mnemonic, passphrase).map_err(Into::into)
    }

    pub fn generate_byron_legacy(mnemonic: &str) -> Result<String, Error> {
        let words: Vec<String> = mnemonic.split_whitespace().map(String::from).collect();
        if !BIP39Mnemonic::is_valid_mnemonic_words(&words) {
            return Err(MnemonicError::new(
                &format!("Invalid {} mnemonic words", BIP39Mnemonic::name()),
                None,
                None,
                None,
            )
            .into());
        }

        let entropy = BIP39Mnemonic::decode(&words)?;
        let cbor_bytes = serde_cbor::to_vec(&entropy)
            .map_err(|_| Error::new("CBOR encoding failed", None, None, None))?;

        Ok(hex::encode(blake2b_256(&cbor_bytes)))
    }

    pub fn generate_shelley_icarus(mnemonic: &str) -> Result<String, Error> {
        Self::generate_byron_icarus(mnemonic)
    }

    pub fn generate_shelley_ledger(
        mnemonic: &str,
        passphrase: Option<&str>,
    ) -> Result<String, Error> {
        Self::generate_byron_ledger(mnemonic, passphrase)
    }
}
