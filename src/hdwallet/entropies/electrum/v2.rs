use super::super::IEntropy;
use rand::Rng;
use num_bigint::BigUint;
use num_traits::{One, ToPrimitive};

pub struct ElectrumV2EntropyStrengths;

impl ElectrumV2EntropyStrengths {
    pub const ONE_HUNDRED_THIRTY_TWO: usize = 132;
    pub const TWO_HUNDRED_SIXTY_FOUR: usize = 264;
}

pub struct ElectrumV2Entropy;

impl IEntropy for ElectrumV2Entropy {
    fn name() -> &'static str {
        "Electrum-V2"
    }

    fn strengths() -> &'static [usize] {
        &[
            ElectrumV2EntropyStrengths::ONE_HUNDRED_THIRTY_TWO,
            ElectrumV2EntropyStrengths::TWO_HUNDRED_SIXTY_FOUR,
        ]
    }

    fn generate(strength: usize) -> Result<String, crate::entropies::ientropy::EntropyError> {
        if !Self::is_valid_strength(strength) {
            return Err(crate::entropies::ientropy::EntropyError::new(
                "Unsupported entropy strength",
            ));
        }

        let mut rng = rand::thread_rng();
        let mut entropy = BigUint::one() << (strength - 1);
        let random_bits = rng.gen_biguint(strength);
        entropy |= random_bits;

        Ok(hex::encode(entropy.to_bytes_be()))
    }

    fn is_valid_strength(strength: usize) -> bool {
        for &_strength in Self::strengths() {
            if (_strength.saturating_sub(11)) <= strength && strength <= _strength {
                return true;
            }
        }
        false
    }

    fn are_entropy_bits_enough(entropy: &BigUint) -> bool {
        let entropy_strength = if entropy.is_zero() {
            0
        } else {
            let bits = entropy.bits();
            if bits > 0 { bits - 1 } else { 0 }
        } as usize;

        Self::is_valid_strength(entropy_strength)
    }
}