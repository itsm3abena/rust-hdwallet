use super::IEntropy;

pub struct MoneroEntropyStrengths;

impl MoneroEntropyStrengths {
    pub const ONE_HUNDRED_TWENTY_EIGHT: usize = 128;
    pub const TWO_HUNDRED_FIFTY_SIX: usize = 256;
}

pub struct MoneroEntropy;

impl IEntropy for MoneroEntropy {
    fn name() -> &'static str {
        "Monero"
    }

    fn strengths() -> &'static [usize] {
        &[
            MoneroEntropyStrengths::ONE_HUNDRED_TWENTY_EIGHT,
            MoneroEntropyStrengths::TWO_HUNDRED_FIFTY_SIX,
        ]
    }
}