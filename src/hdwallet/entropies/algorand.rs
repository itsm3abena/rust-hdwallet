use super::IEntropy;

pub struct AlgorandEntropyStrengths;

impl AlgorandEntropyStrengths {
    pub const TWO_HUNDRED_FIFTY_SIX: usize = 256;
}

pub struct AlgorandEntropy;

impl IEntropy for AlgorandEntropy {
    fn name() -> &'static str {
        "Algorand"
    }

    fn strengths() -> &'static [usize] {
        &[AlgorandEntropyStrengths::TWO_HUNDRED_FIFTY_SIX]
    }
}