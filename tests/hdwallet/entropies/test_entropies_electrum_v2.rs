use super::conftest::{assert_error_contains, get_bytes, json_str, json_usize, TestData};
use hdwallet_rust::hdwallet::entropies::{
    electrum::ElectrumV2EntropyStrengths, ientropy::EntropyError, ElectrumV2Entropy,
    EntropyInstance, IEntropy,
};
use num_bigint::BigUint;

#[test]
fn test_electrum_v2_entropy() {
    let data = TestData::load();
    let ev2 = &data.entropies["Electrum-V2"];
    let scheme = ElectrumV2Entropy;

    assert_eq!(ElectrumV2EntropyStrengths::ONE_HUNDRED_THIRTY_TWO, 132);
    assert_eq!(ElectrumV2EntropyStrengths::TWO_HUNDRED_SIXTY_FOUR, 264);

    assert!(scheme.is_valid_strength(ElectrumV2EntropyStrengths::ONE_HUNDRED_THIRTY_TWO));
    assert!(scheme.is_valid_strength(ElectrumV2EntropyStrengths::TWO_HUNDRED_SIXTY_FOUR));

    assert!(ElectrumV2Entropy::are_entropy_bits_enough(&BigUint::from_bytes_be(
        &get_bytes(&json_str(&ev2["132"]["entropy-not-suitable"]))
    )));
    assert!(ElectrumV2Entropy::are_entropy_bits_enough(&BigUint::from_bytes_be(
        &get_bytes(&json_str(&ev2["132"]["entropy-suitable"]))
    )));
    assert!(ElectrumV2Entropy::are_entropy_bits_enough(&BigUint::from_bytes_be(
        &get_bytes(&json_str(&ev2["264"]["entropy-not-suitable"]))
    )));
    assert!(ElectrumV2Entropy::are_entropy_bits_enough(&BigUint::from_bytes_be(
        &get_bytes(&json_str(&ev2["264"]["entropy-suitable"]))
    )));

    assert_eq!(
        EntropyInstance::new::<ElectrumV2Entropy>(&json_str(&ev2["264"]["entropy-not-suitable"]))
            .unwrap()
            .strength_bits(),
        264
    );
    assert_eq!(
        EntropyInstance::new::<ElectrumV2Entropy>(&json_str(&ev2["132"]["entropy-not-suitable"]))
            .unwrap()
            .strength_bits(),
        132
    );

    assert_eq!(
        EntropyInstance::new::<ElectrumV2Entropy>(
            &ElectrumV2Entropy
                .generate(ElectrumV2EntropyStrengths::ONE_HUNDRED_THIRTY_TWO)
                .unwrap()
        )
        .unwrap()
        .strength_bits(),
        132
    );
    assert_eq!(
        EntropyInstance::new::<ElectrumV2Entropy>(
            &ElectrumV2Entropy
                .generate(ElectrumV2EntropyStrengths::TWO_HUNDRED_SIXTY_FOUR)
                .unwrap()
        )
        .unwrap()
        .strength_bits(),
        264
    );

    let ev2_132 = EntropyInstance::new::<ElectrumV2Entropy>(&json_str(
        &ev2["132"]["entropy-suitable"],
    ))
    .unwrap();
    let ev2_264 = EntropyInstance::new::<ElectrumV2Entropy>(&json_str(
        &ev2["264"]["entropy-suitable"],
    ))
    .unwrap();

    assert_eq!(scheme.name(), json_str(&ev2["132"]["name"]));
    assert_eq!(scheme.name(), json_str(&ev2["264"]["name"]));

    assert_eq!(ev2_132.strength_bits(), json_usize(&ev2["132"]["strength"]));
    assert_eq!(ev2_264.strength_bits(), json_usize(&ev2["264"]["strength"]));

    assert_eq!(
        ev2_132.entropy(),
        json_str(&ev2["132"]["entropy-suitable"])
    );
    assert_eq!(
        ev2_264.entropy(),
        json_str(&ev2["264"]["entropy-suitable"])
    );

    assert_error_contains(
        match hdwallet_rust::hdwallet::utils::hex_to_bytes("INVALID_ENTROPY") {
            Ok(_) => EntropyInstance::new::<ElectrumV2Entropy>("INVALID_ENTROPY"),
            Err(_) => Err(EntropyError::new("Invalid entropy data")),
        }
        .map(|_| ()),
        "Invalid entropy data",
    );

    assert_error_contains(
        ElectrumV2Entropy
            .strength_bits_from_hex("7c2abbf52d1861b978792df3dc88e7e27dbe36c7a0287893")
            .map(|_| ()),
        "Entropy bits are not enough",
    );
}
