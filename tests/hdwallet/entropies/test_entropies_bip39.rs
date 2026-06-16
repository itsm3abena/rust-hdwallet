use super::conftest::{assert_error_contains, get_bytes, json_str, json_usize, TestData};
use hdwallet_rust::hdwallet::entropies::{
    bip39::BIP39EntropyStrengths, ientropy::EntropyError, BIP39Entropy, EntropyInstance, IEntropy,
};

fn new_entropy_instance(entropy: &str) -> Result<EntropyInstance, EntropyError> {
    if hdwallet_rust::hdwallet::utils::hex_to_bytes(entropy).is_err() {
        return Err(EntropyError::new("Invalid entropy data"));
    }
    EntropyInstance::new::<BIP39Entropy>(entropy)
}

#[test]
fn test_bip39_entropy() {
    let data = TestData::load();
    let bip39 = &data.entropies["BIP39"];
    let scheme = BIP39Entropy;

    assert_eq!(
        BIP39EntropyStrengths::ONE_HUNDRED_TWENTY_EIGHT,
        128
    );
    assert_eq!(BIP39EntropyStrengths::ONE_HUNDRED_SIXTY, 160);
    assert_eq!(BIP39EntropyStrengths::ONE_HUNDRED_NINETY_TWO, 192);
    assert_eq!(BIP39EntropyStrengths::TWO_HUNDRED_TWENTY_FOUR, 224);
    assert_eq!(BIP39EntropyStrengths::TWO_HUNDRED_FIFTY_SIX, 256);

    assert!(scheme.is_valid_strength(BIP39EntropyStrengths::ONE_HUNDRED_TWENTY_EIGHT));
    assert!(scheme.is_valid_strength(BIP39EntropyStrengths::ONE_HUNDRED_SIXTY));
    assert!(scheme.is_valid_strength(BIP39EntropyStrengths::ONE_HUNDRED_NINETY_TWO));
    assert!(scheme.is_valid_strength(BIP39EntropyStrengths::TWO_HUNDRED_TWENTY_FOUR));
    assert!(scheme.is_valid_strength(BIP39EntropyStrengths::TWO_HUNDRED_FIFTY_SIX));

    for strength in ["128", "160", "192", "224", "256"] {
        let entropy_hex = json_str(&bip39[strength]["entropy"]);
        assert!(scheme.is_valid_bytes_strength(get_bytes(&entropy_hex).len()));
    }

    assert_eq!(
        EntropyInstance::new::<BIP39Entropy>(
            &BIP39Entropy
                .generate(BIP39EntropyStrengths::ONE_HUNDRED_TWENTY_EIGHT)
                .unwrap()
        )
        .unwrap()
        .strength_bits(),
        128
    );
    assert_eq!(
        EntropyInstance::new::<BIP39Entropy>(
            &BIP39Entropy
                .generate(BIP39EntropyStrengths::ONE_HUNDRED_SIXTY)
                .unwrap()
        )
        .unwrap()
        .strength_bits(),
        160
    );
    assert_eq!(
        EntropyInstance::new::<BIP39Entropy>(
            &BIP39Entropy
                .generate(BIP39EntropyStrengths::ONE_HUNDRED_NINETY_TWO)
                .unwrap()
        )
        .unwrap()
        .strength_bits(),
        192
    );
    assert_eq!(
        EntropyInstance::new::<BIP39Entropy>(
            &BIP39Entropy
                .generate(BIP39EntropyStrengths::TWO_HUNDRED_TWENTY_FOUR)
                .unwrap()
        )
        .unwrap()
        .strength_bits(),
        224
    );
    assert_eq!(
        EntropyInstance::new::<BIP39Entropy>(
            &BIP39Entropy
                .generate(BIP39EntropyStrengths::TWO_HUNDRED_FIFTY_SIX)
                .unwrap()
        )
        .unwrap()
        .strength_bits(),
        256
    );

    let bip39_128 =
        EntropyInstance::new::<BIP39Entropy>(&json_str(&bip39["128"]["entropy"])).unwrap();
    let bip39_160 =
        EntropyInstance::new::<BIP39Entropy>(&json_str(&bip39["160"]["entropy"])).unwrap();
    let bip39_192 =
        EntropyInstance::new::<BIP39Entropy>(&json_str(&bip39["192"]["entropy"])).unwrap();
    let bip39_224 =
        EntropyInstance::new::<BIP39Entropy>(&json_str(&bip39["224"]["entropy"])).unwrap();
    let bip39_256 =
        EntropyInstance::new::<BIP39Entropy>(&json_str(&bip39["256"]["entropy"])).unwrap();

    assert_eq!(scheme.name(), json_str(&bip39["128"]["name"]));
    assert_eq!(scheme.name(), json_str(&bip39["160"]["name"]));
    assert_eq!(scheme.name(), json_str(&bip39["192"]["name"]));
    assert_eq!(scheme.name(), json_str(&bip39["224"]["name"]));
    assert_eq!(scheme.name(), json_str(&bip39["256"]["name"]));

    assert_eq!(bip39_128.strength_bits(), json_usize(&bip39["128"]["strength"]));
    assert_eq!(bip39_160.strength_bits(), json_usize(&bip39["160"]["strength"]));
    assert_eq!(bip39_192.strength_bits(), json_usize(&bip39["192"]["strength"]));
    assert_eq!(bip39_224.strength_bits(), json_usize(&bip39["224"]["strength"]));
    assert_eq!(bip39_256.strength_bits(), json_usize(&bip39["256"]["strength"]));

    assert_eq!(bip39_128.entropy(), json_str(&bip39["128"]["entropy"]));
    assert_eq!(bip39_160.entropy(), json_str(&bip39["160"]["entropy"]));
    assert_eq!(bip39_192.entropy(), json_str(&bip39["192"]["entropy"]));
    assert_eq!(bip39_224.entropy(), json_str(&bip39["224"]["entropy"]));
    assert_eq!(bip39_256.entropy(), json_str(&bip39["256"]["entropy"]));

    assert_error_contains(
        new_entropy_instance("INVALID_ENTROPY").map(|_| ()),
        "Invalid entropy data",
    );

    assert_error_contains(
        EntropyInstance::new::<BIP39Entropy>(
            "cdf694ac868efd01673fc51e897c57a0bd428503080ad4c94c7d6f6d13f095fbc8",
        )
        .map(|_| ()),
        "Unsupported entropy strength",
    );
}
