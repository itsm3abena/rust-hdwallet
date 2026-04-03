use super::IEntropy;

pub struct AlgorandEntropyStrengths;

impl AlgorandEntropyStrengths {
    pub const TWO_HUNDRED_FIFTY_SIX: usize = 256;
}

#[derive(Clone, Copy, Default)]
pub struct AlgorandEntropy;

impl IEntropy for AlgorandEntropy {
    fn name(&self) -> &'static str {
        "Algorand"
    }

    fn strengths(&self) -> &'static [usize] {
        &[AlgorandEntropyStrengths::TWO_HUNDRED_FIFTY_SIX]
    }
}
