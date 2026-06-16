use super::conftest::{assert_error_contains, get_bytes, json_str, json_usize, TestData};
use hdwallet_rust::hdwallet::entropies::{
    ientropy::EntropyError, monero::MoneroEntropyStrengths, EntropyInstance, IEntropy, MoneroEntropy,
};

fn new_entropy_instance(entropy: &str) -> Result<EntropyInstance, EntropyError> {
    if hdwallet_rust::hdwallet::utils::hex_to_bytes(entropy).is_err() {
        return Err(EntropyError::new("Invalid entropy data"));
    }
    EntropyInstance::new::<MoneroEntropy>(entropy)
}

#[test]
fn test_monero_entropy() {
    let data = TestData::load();
    let monero = &data.entropies["Monero"];
    let scheme = MoneroEntropy;

    assert_eq!(MoneroEntropyStrengths::ONE_HUNDRED_TWENTY_EIGHT, 128);
    assert_eq!(MoneroEntropyStrengths::TWO_HUNDRED_FIFTY_SIX, 256);

    assert!(scheme.is_valid_strength(MoneroEntropyStrengths::ONE_HUNDRED_TWENTY_EIGHT));
    assert!(scheme.is_valid_strength(MoneroEntropyStrengths::TWO_HUNDRED_FIFTY_SIX));

    assert!(scheme.is_valid_bytes_strength(
        get_bytes(&json_str(&monero["128"]["entropy"])).len()
    ));
    assert!(scheme.is_valid_bytes_strength(
        get_bytes(&json_str(&monero["256"]["entropy"])).len()
    ));

    assert_eq!(
        EntropyInstance::new::<MoneroEntropy>(
            &MoneroEntropy
                .generate(MoneroEntropyStrengths::ONE_HUNDRED_TWENTY_EIGHT)
                .unwrap()
        )
        .unwrap()
        .strength_bits(),
        128
    );
    assert_eq!(
        EntropyInstance::new::<MoneroEntropy>(
            &MoneroEntropy
                .generate(MoneroEntropyStrengths::TWO_HUNDRED_FIFTY_SIX)
                .unwrap()
        )
        .unwrap()
        .strength_bits(),
        256
    );

    let monero_128 =
        EntropyInstance::new::<MoneroEntropy>(&json_str(&monero["128"]["entropy"])).unwrap();
    let monero_256 =
        EntropyInstance::new::<MoneroEntropy>(&json_str(&monero["256"]["entropy"])).unwrap();

    assert_eq!(scheme.name(), json_str(&monero["128"]["name"]));
    assert_eq!(scheme.name(), json_str(&monero["256"]["name"]));

    assert_eq!(
        monero_128.strength_bits(),
        json_usize(&monero["128"]["strength"])
    );
    assert_eq!(
        monero_256.strength_bits(),
        json_usize(&monero["256"]["strength"])
    );

    assert_eq!(monero_128.entropy(), json_str(&monero["128"]["entropy"]));
    assert_eq!(monero_256.entropy(), json_str(&monero["256"]["entropy"]));

    assert_error_contains(
        new_entropy_instance("INVALID_ENTROPY").map(|_| ()),
        "Invalid entropy data",
    );

    assert_error_contains(
        EntropyInstance::new::<MoneroEntropy>(
            "cdf694ac868efd01673fc51e897c57a0bd428503080ad4c94c7d6f6d13f095fbc8",
        )
        .map(|_| ()),
        "Unsupported entropy strength",
    );
}
