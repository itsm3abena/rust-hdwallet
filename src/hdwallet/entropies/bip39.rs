use super::IEntropy;

pub struct BIP39EntropyStrengths;

impl BIP39EntropyStrengths {
    pub const ONE_HUNDRED_TWENTY_EIGHT: usize = 128;
    pub const ONE_HUNDRED_SIXTY: usize = 160;
    pub const ONE_HUNDRED_NINETY_TWO: usize = 192;
    pub const TWO_HUNDRED_TWENTY_FOUR: usize = 224;
    pub const TWO_HUNDRED_FIFTY_SIX: usize = 256;
}

#[derive(Clone, Copy, Default)]
pub struct BIP39Entropy;

impl IEntropy for BIP39Entropy {
    fn name(&self) -> &'static str {
        "BIP39"
    }

    fn strengths(&self) -> &'static [usize] {
        &[
            BIP39EntropyStrengths::ONE_HUNDRED_TWENTY_EIGHT,
            BIP39EntropyStrengths::ONE_HUNDRED_SIXTY,
            BIP39EntropyStrengths::ONE_HUNDRED_NINETY_TWO,
            BIP39EntropyStrengths::TWO_HUNDRED_TWENTY_FOUR,
            BIP39EntropyStrengths::TWO_HUNDRED_FIFTY_SIX,
        ]
    }
}
