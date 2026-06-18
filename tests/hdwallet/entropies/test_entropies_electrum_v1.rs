use super::conftest::{assert_error_contains, get_bytes, json_str, json_usize, TestData};
use rust_hdwallet::hdwallet::entropies::{
    electrum::ElectrumV1EntropyStrengths, ientropy::EntropyError, ElectrumV1Entropy,
    EntropyInstance, IEntropy,
};

fn new_entropy_instance(entropy: &str) -> Result<EntropyInstance, EntropyError> {
    if rust_hdwallet::hdwallet::utils::hex_to_bytes(entropy).is_err() {
        return Err(EntropyError::new("Invalid entropy data"));
    }
    EntropyInstance::new::<ElectrumV1Entropy>(entropy)
}

#[test]
fn test_electrum_v1_entropy() {
    let data = TestData::load();
    let ev1 = &data.entropies["Electrum-V1"];
    let scheme = ElectrumV1Entropy;

    assert_eq!(ElectrumV1EntropyStrengths::ONE_HUNDRED_TWENTY_EIGHT, 128);
    assert!(scheme.is_valid_strength(ElectrumV1EntropyStrengths::ONE_HUNDRED_TWENTY_EIGHT));
    assert!(scheme.is_valid_bytes_strength(
        get_bytes(&json_str(&ev1["128"]["entropy"])).len()
    ));
    assert_eq!(
        EntropyInstance::new::<ElectrumV1Entropy>(
            &ElectrumV1Entropy
                .generate(ElectrumV1EntropyStrengths::ONE_HUNDRED_TWENTY_EIGHT)
                .unwrap()
        )
        .unwrap()
        .strength_bits(),
        128
    );

    let ev1_128 =
        EntropyInstance::new::<ElectrumV1Entropy>(&json_str(&ev1["128"]["entropy"])).unwrap();

    assert_eq!(scheme.name(), json_str(&ev1["128"]["name"]));
    assert_eq!(ev1_128.strength_bits(), json_usize(&ev1["128"]["strength"]));
    assert_eq!(ev1_128.entropy(), json_str(&ev1["128"]["entropy"]));

    assert_error_contains(
        new_entropy_instance("INVALID_ENTROPY").map(|_| ()),
        "Invalid entropy data",
    );

    assert_error_contains(
        EntropyInstance::new::<ElectrumV1Entropy>(
            "cdf694ac868efd01673fc51e897c57a0bd428503080ad4c94c7d6f6d13f095fbc8",
        )
        .map(|_| ()),
        "Unsupported entropy strength",
    );
}
