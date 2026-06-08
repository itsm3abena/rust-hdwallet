use super::super::ientropy::{EntropyError, IEntropy};
use num_bigint::{BigUint, RandBigInt};
use num_traits::One;
use rand::thread_rng;

pub struct ElectrumV2EntropyStrengths;

impl ElectrumV2EntropyStrengths {
    pub const ONE_HUNDRED_THIRTY_TWO: usize = 132;
    pub const TWO_HUNDRED_SIXTY_FOUR: usize = 264;
}

#[derive(Clone, Copy, Default)]
pub struct ElectrumV2Entropy;

impl IEntropy for ElectrumV2Entropy {
    fn name(&self) -> &'static str {
        "Electrum-V2"
    }

    fn strengths(&self) -> &'static [usize] {
        &[
            ElectrumV2EntropyStrengths::ONE_HUNDRED_THIRTY_TWO,
            ElectrumV2EntropyStrengths::TWO_HUNDRED_SIXTY_FOUR,
        ]
    }

    fn is_valid_bytes_strength(&self, _bytes_len: usize) -> bool {
        // Electrum-V2 entropy bit length is derived from the integer value, not byte width.
        true
    }

    fn is_valid_entropy_hex(&self, entropy: &str) -> bool {
        let re = regex::Regex::new(r"^[0-9a-fA-F]+$").unwrap();
        if !re.is_match(entropy) {
            return false;
        }
        let bytes = match hex::decode(entropy) {
            Ok(value) => value,
            Err(_) => return false,
        };
        let value = BigUint::from_bytes_be(&bytes);
        Self::are_entropy_bits_enough(&value)
    }

    fn strength_bits_from_hex(&self, entropy: &str) -> Result<usize, EntropyError> {
        let bytes = hex::decode(entropy).map_err(|_| EntropyError::new("Invalid entropy data"))?;
        let value = BigUint::from_bytes_be(&bytes);
        if !Self::are_entropy_bits_enough(&value) {
            return Err(EntropyError::new("Entropy bits are not enough"));
        }
        Ok(value.bits() as usize)
    }

    fn generate(&self, strength: usize) -> Result<String, EntropyError> {
        if !self.is_valid_strength(strength) {
            return Err(EntropyError::new("Unsupported entropy strength"));
        }

        let mut rng = thread_rng();
        let mut entropy = BigUint::one() << (strength - 1);
        let random_bits = rng.gen_biguint(strength as u64);
        entropy |= random_bits;

        Ok(hex::encode(entropy.to_bytes_be()))
    }

    fn is_valid_strength(&self, strength: usize) -> bool {
        for &s in self.strengths() {
            if (s.saturating_sub(11)) <= strength && strength <= s {
                return true;
            }
        }
        false
    }
}

impl ElectrumV2Entropy {
    pub fn are_entropy_bits_enough(entropy: &BigUint) -> bool {
        let scheme = ElectrumV2Entropy;
        let entropy_strength = if *entropy == BigUint::from(0u8) {
            0
        } else {
            let bits = entropy.bits();
            if bits > 0 {
                (bits - 1) as usize
            } else {
                0
            }
        };

        scheme.is_valid_strength(entropy_strength)
    }
}
