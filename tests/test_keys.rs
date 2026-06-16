use hdwallet_rust::hdwallet::keys::{is_root_key, is_valid_key};

#[test]
fn test_keys() {
    assert!(is_valid_key(
        "xprv9s21ZrQH143K3hU8mSCiGTDNX8vESS6bVuXKpMgJK8aLfWTsCgakwRfCts6bXoRG51sHYenkyseTeWB12RfS8KgQF7w8e8ner4U7HDBDuMw",
        true
    ));
    assert!(is_root_key(
        "xprv9s21ZrQH143K3hU8mSCiGTDNX8vESS6bVuXKpMgJK8aLfWTsCgakwRfCts6bXoRG51sHYenkyseTeWB12RfS8KgQF7w8e8ner4U7HDBDuMw",
        true
    )
    .unwrap());
    assert!(!is_root_key(
        "xprvA31rahaYRvc6tpxRoBUiymeY5k6c2uCQwWtQxRXsU4cX7FmNyL3fg3T9mXyYc2k7huWwaM3Vi19n6tQ21VgXH65Ws7Snz681y3AmqXrNCL1",
        true
    )
    .unwrap());

    assert!(is_valid_key(
        "xpub661MyMwAqRbcGBYbsTjidbA75AkiqtpSs8Svck5usU7KYJo1kDu1VDygk8tWBWC2hvmdFBFEwuz1W2aPHsqKjRcye6MeeyDFkRPNbUq73N9",
        true
    ));
    assert!(is_root_key(
        "xpub661MyMwAqRbcGBYbsTjidbA75AkiqtpSs8Svck5usU7KYJo1kDu1VDygk8tWBWC2hvmdFBFEwuz1W2aPHsqKjRcye6MeeyDFkRPNbUq73N9",
        true
    )
    .unwrap());
    assert!(!is_root_key(
        "xpub6GAzPQuoyqr6sT251c2dqZkcYU9FZZpvtiJsnADjBxU6yaF577vE6pC9Lyb3ReSLWZsMQMtcVoPKV1qorppQWbTsxwA7r1AgZ9PbfT1iULz",
        true
    )
    .unwrap());

    assert!(is_valid_key(
        "xprv3QESAWYc9vDdZiM9o6Z4Dsny5f4zBVdWgdU2YKw25fGyhT8ZgEkkta4kmkFhn875Mq3xUjRaGyhCnFoK1qb7fDELTyCJMTdiatAHuvG8mjjJgfJesQ9AFBikt59YLVgTEn2HhXKYxjmKSvxHZxdQXzF",
        true
    ));
    assert!(is_root_key(
        "xprv3QESAWYc9vDdZiM9o6Z4Dsny5f4zBVdWgdU2YKw25fGyhT8ZgEkkta4kmkFhn875Mq3xUjRaGyhCnFoK1qb7fDELTyCJMTdiatAHuvG8mjjJgfJesQ9AFBikt59YLVgTEn2HhXKYxjmKSvxHZxdQXzF",
        true
    )
    .unwrap());
    assert!(!is_root_key(
        "xprv3TFbDUeC4U4KfWsziG2VfvZ4C2b9foDhQcaFc7vgLCduXkWARCHRqPQZDLYvnrxVaYmXdzPyf7aMuA8BaavsBXaMCCs1c7QhoGe7gGHMMTB7uenMe4YBh5XDWFNwfL6754zdagnqMq4AgysHuV5jnEv",
        true
    )
    .unwrap());

    let invalid =
        "xprv9s21ZrQH143K3hU8mSCiGTDNX8vESS6bVuX.............enkyseTeWB12RfS8KgQF7w8e8ne4U7HDBDuMw";
    assert!(!is_valid_key(invalid, true));
    let error = is_root_key(invalid, true).expect_err("invalid key");
    assert!(
        error.to_string().contains("Invalid extended(x) key"),
        "unexpected error: {error}"
    );
}
