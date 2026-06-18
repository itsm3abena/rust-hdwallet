use super::conftest::{assert_error_contains, get_bytes, json_str, json_usize, TestData};
use rust_hdwallet::hdwallet::entropies::{
    algorand::AlgorandEntropyStrengths, ientropy::EntropyError, AlgorandEntropy, EntropyInstance,
    IEntropy,
};

fn new_entropy_instance(entropy: &str) -> Result<EntropyInstance, EntropyError> {
    if rust_hdwallet::hdwallet::utils::hex_to_bytes(entropy).is_err() {
        return Err(EntropyError::new("Invalid entropy data"));
    }
    EntropyInstance::new::<AlgorandEntropy>(entropy)
}

#[test]
fn test_algorand_entropy() {
    let data = TestData::load();
    let algorand = &data.entropies["Algorand"];
    let scheme = AlgorandEntropy;

    assert_eq!(AlgorandEntropyStrengths::TWO_HUNDRED_FIFTY_SIX, 256);
    assert!(scheme.is_valid_strength(AlgorandEntropyStrengths::TWO_HUNDRED_FIFTY_SIX));
    assert!(scheme.is_valid_bytes_strength(
        get_bytes(&json_str(&algorand["256"]["entropy"])).len()
    ));
    assert_eq!(
        EntropyInstance::new::<AlgorandEntropy>(
            &AlgorandEntropy
                .generate(AlgorandEntropyStrengths::TWO_HUNDRED_FIFTY_SIX)
                .unwrap()
        )
        .unwrap()
        .strength_bits(),
        256
    );

    let algorand_256 =
        EntropyInstance::new::<AlgorandEntropy>(&json_str(&algorand["256"]["entropy"])).unwrap();

    assert_eq!(scheme.name(), json_str(&algorand["256"]["name"]));
    assert_eq!(
        algorand_256.strength_bits(),
        json_usize(&algorand["256"]["strength"])
    );
    assert_eq!(algorand_256.entropy(), json_str(&algorand["256"]["entropy"]));

    assert_error_contains(
        new_entropy_instance("INVALID_ENTROPY").map(|_| ()),
        "Invalid entropy data",
    );

    assert_error_contains(
        EntropyInstance::new::<AlgorandEntropy>(
            "cdf694ac868efd01673fc51e897c57a0bd428503080ad4c94c7d6f6d13f095fbc8",
        )
        .map(|_| ()),
        "Unsupported entropy strength",
    );
}
