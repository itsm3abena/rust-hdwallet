use super::super::IEntropy;

pub struct ElectrumV1EntropyStrengths;

impl ElectrumV1EntropyStrengths {
    pub const ONE_HUNDRED_TWENTY_EIGHT: usize = 128;
}

pub struct ElectrumV1Entropy;

impl IEntropy for ElectrumV1Entropy {
    fn name() -> &'static str {
        "Electrum-V1"
    }

    fn strengths() -> &'static [usize] {
        &[ElectrumV1EntropyStrengths::ONE_HUNDRED_TWENTY_EIGHT]
    }
}