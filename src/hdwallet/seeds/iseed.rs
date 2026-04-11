use regex::Regex;

use crate::hdwallet::exceptions::SeedError;
use crate::hdwallet::mnemonics::imnemonic::IMnemonic;

pub trait ISeed {
    const LENGTH: usize;

    fn new(seed: String) -> Self
    where
        Self: Sized;

    fn name() -> &'static str
    where
        Self: Sized;

    fn is_valid(seed: &str) -> bool
    where
        Self: Sized,
    {
        let re = Regex::new(r"^[0-9a-fA-F]+$").unwrap();

        re.is_match(seed) && seed.len() == Self::LENGTH
    }

    fn seed(&self) -> &str;

    fn from_mnemonic(
        mnemonic: &dyn IMnemonic,
    ) -> Result<String, SeedError>
    where
        Self: Sized;
}