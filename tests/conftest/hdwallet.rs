use std::collections::HashSet;

use rust_hdwallet::hdwallet::derivations::Derivations;
use rust_hdwallet::hdwallet::hdwallet::{build_derivation, HDWallet, HDWalletConfig};
use serde_json::{Map, Value};

use super::{assert_json_eq, json_nullable_str, json_str, TestData};

const MONERO_PAYMENT_ID: &str = "ad17dc6e6793d178";

pub fn derivation_hd(hd: &str) -> &str {
    if hd == "Electrum-V2" {
        "Electrum-V1"
    } else {
        hd
    }
}

pub fn entropy_client(hd: &str) -> &str {
    match hd {
        "Electrum-V1" => "Electrum-V1",
        "Electrum-V2" => "Electrum-V2",
        "Monero" => "Monero",
        _ => "BIP39",
    }
}

pub fn wallet_from_variant(variant: &Value, hd: &str) -> HDWallet {
    let payment_id = if hd == "Monero" {
        Some(MONERO_PAYMENT_ID.to_string())
    } else {
        variant
            .get("payment_id")
            .and_then(json_nullable_str)
    };

    let cardano_type = variant
        .get("cardano_type")
        .map(json_str)
        .unwrap_or_else(|| "shelley-icarus".to_string());
    let address_type = variant.get("address_type").and_then(json_nullable_str).or_else(|| {
        if hd == "Cardano" {
            if cardano_type.starts_with("shelley") {
                Some("staking".to_string())
            } else {
                Some("public-key".to_string())
            }
        } else {
            None
        }
    });

    HDWallet::new(HDWalletConfig {
        symbol: json_str(&variant["symbol"]),
        hd: hd.to_string(),
        network: json_str(&variant["network"]),
        language: json_str(&variant["language"]),
        passphrase: json_nullable_str(&variant["passphrase"]),
        public_key_type: json_str(&variant["public_key_type"]),
        cardano_type,
        address_type,
        mode: variant
            .get("mode")
            .map(json_str)
            .unwrap_or_else(|| "standard".to_string()),
        mnemonic_type: variant
            .get("mnemonic_type")
            .map(json_str)
            .unwrap_or_else(|| "standard".to_string()),
        checksum: variant
            .get("checksum")
            .and_then(Value::as_bool)
            .unwrap_or(false),
        payment_id,
        staking_public_key: variant.get("staking_public_key").and_then(json_nullable_str),
        semantic: variant.get("semantic").and_then(json_nullable_str),
        strict: variant.get("strict").and_then(Value::as_bool).unwrap_or(true),
        encoded: true,
    })
    .expect("HDWallet::new")
}

pub fn apply_derivation(wallet: &mut HDWallet, data: &TestData, hd: &str) {
    let spec = &data.hdwallet[derivation_hd(hd)]["derivation"];
    let name = json_str(&spec["name"]);
    let args = &spec["args"];
    let crypto = wallet.cryptocurrency_descriptor();
    let derivation = build_derivation_from_args(&name, crypto, args).expect("derivation");
    wallet
        .from_derivation(&name, derivation)
        .expect("from_derivation");
}

fn build_derivation_from_args(
    name: &str,
    crypto: &rust_hdwallet::hdwallet::cryptocurrencies::icryptocurrency::CryptocurrencyDescriptor,
    args: &Value,
) -> Result<
    Box<dyn rust_hdwallet::hdwallet::derivations::IDerivation>,
    rust_hdwallet::hdwallet::exceptions::Error,
> {
    let get = |k: &str| args.get(k).map(json_str).unwrap_or_else(|| "0".to_string());
    match name {
        "Custom" => build_derivation(
            name,
            crypto,
            "0",
            "0",
            "0",
            "0",
            "0",
            "0",
            "0",
            Some(&get("path")),
            &[],
        ),
        "Electrum" => build_derivation(
            name,
            crypto,
            "0",
            &get("change"),
            &get("address"),
            "0",
            "0",
            "0",
            "0",
            None,
            &[],
        ),
        "Monero" => build_derivation(
            name,
            crypto,
            "0",
            "0",
            "0",
            "0",
            "0",
            &get("minor"),
            &get("major"),
            None,
            &[],
        ),
        "HDW" => build_derivation(
            name,
            crypto,
            &get("account"),
            "0",
            &get("address"),
            "0",
            &get("ecc"),
            "0",
            "0",
            None,
            &[],
        ),
        "CIP1852" => build_derivation(
            name,
            crypto,
            &get("account"),
            "0",
            &get("address"),
            &get("role"),
            "0",
            "0",
            "0",
            None,
            &[],
        ),
        _ => build_derivation(
            name,
            crypto,
            &get("account"),
            &get("change"),
            &get("address"),
            "0",
            "0",
            "0",
            "0",
            None,
            &[],
        ),
    }
}

fn insert_nulls(object: &mut Map<String, Value>, keys: &[&str]) {
    for key in keys {
        object.insert(key.to_string(), Value::Null);
    }
}

fn last_derivation(variant: &Value) -> Value {
    variant["derivations"]
        .as_array()
        .expect("derivations array")
        .last()
        .expect("derivations non-empty")
        .clone()
}

fn assert_dumps_dump(wallet: &mut HDWallet, expected: &Value) {
    let actual = wallet
        .dumps(&HashSet::new())
        .expect("dumps")
        .expect("dumps value");
    assert_json_eq(&actual, expected);

    let mut dump_expected = expected.clone();
    if let Some(object) = dump_expected.as_object_mut() {
        let last = last_derivation(expected);
        object.remove("derivations");
        object.insert("derivation".into(), last);
    }
    let actual_dump = wallet.dump(&HashSet::new()).expect("dump");
    assert_json_eq(&actual_dump, &dump_expected);
}

fn assert_dump_only(wallet: &mut HDWallet, expected_dump: &Value) {
    let dumps = wallet.dumps(&HashSet::new()).expect("dumps");
    assert!(dumps.is_none(), "expected dumps() to be None");
    let actual_dump = wallet.dump(&HashSet::new()).expect("dump");
    assert_json_eq(&actual_dump, expected_dump);
}

fn build_dump_from_variant(variant: &Value) -> Value {
    let mut dump = variant.clone();
    if let Some(object) = dump.as_object_mut() {
        object.remove("derivations");
        object.insert("derivation".into(), last_derivation(variant));
    }
    dump
}

pub fn assert_from_seed(data: &TestData, hd: &str, variant: &str) {
    let v = &data.hdwallet[hd][variant];
    let mut w = wallet_from_variant(v, hd);
    w.from_seed_hex(&json_str(&v["seed"])).expect("from_seed");
    apply_derivation(&mut w, data, hd);

    let mut expected = v.clone();
    if let Some(object) = expected.as_object_mut() {
        insert_nulls(
            object,
            &[
                "entropy",
                "strength",
                "mnemonic",
                "passphrase",
                "language",
            ],
        );
    }
    assert_dumps_dump(&mut w, &expected);
}

pub fn assert_from_entropy(data: &TestData, hd: &str, variant: &str) {
    let v = &data.hdwallet[hd][variant];
    let mut w = wallet_from_variant(v, hd);
    w.from_entropy(&json_str(&v["entropy"]), entropy_client(hd))
        .expect("from_entropy");
    apply_derivation(&mut w, data, hd);
    assert_dumps_dump(&mut w, v);
}

pub fn assert_from_mnemonic(data: &TestData, hd: &str, variant: &str) {
    let v = &data.hdwallet[hd][variant];
    let mut w = wallet_from_variant(v, hd);
    w.from_mnemonic(&json_str(&v["mnemonic"]), entropy_client(hd))
        .expect("from_mnemonic");
    apply_derivation(&mut w, data, hd);
    assert_dumps_dump(&mut w, v);
}

pub fn assert_from_xprivate_key(data: &TestData, hd: &str, variant: &str) {
    let v = &data.hdwallet[hd][variant];
    let strict = v.get("strict").and_then(Value::as_bool).unwrap_or(true);
    let mut w = wallet_from_variant(v, hd);
    w.from_xprivate_key(&json_str(&v["root_xprivate_key"]), true, strict)
        .expect("from_xprivate_key");
    apply_derivation(&mut w, data, hd);

    let mut expected = v.clone();
    if let Some(object) = expected.as_object_mut() {
        insert_nulls(
            object,
            &[
                "entropy",
                "strength",
                "mnemonic",
                "passphrase",
                "language",
                "seed",
            ],
        );
    }
    assert_dumps_dump(&mut w, &expected);
}

pub fn assert_from_xpublic_key(data: &TestData, hd: &str, variant: &str) {
    let v = &data.hdwallet[hd][variant];
    let strict = v.get("strict").and_then(Value::as_bool).unwrap_or(true);
    let mut w = wallet_from_variant(v, hd);
    w.from_xpublic_key(&json_str(&v["root_xpublic_key"]), true, strict)
        .expect("from_xpublic_key");
    apply_derivation(&mut w, data, hd);

    let mut expected = v.clone();
    if let Some(object) = expected.as_object_mut() {
        insert_nulls(
            object,
            &[
                "entropy",
                "strength",
                "mnemonic",
                "passphrase",
                "language",
                "seed",
                "root_xprivate_key",
                "root_private_key",
                "root_wif",
                "wif_type",
            ],
        );
        if let Some(derivations) = object.get_mut("derivations").and_then(Value::as_array_mut) {
            for step in derivations {
                if let Some(step_object) = step.as_object_mut() {
                    insert_nulls(
                        step_object,
                        &["xprivate_key", "private_key", "wif"],
                    );
                }
            }
        }
    }
    assert_dumps_dump(&mut w, &expected);
}

pub fn assert_from_private_key_leaf(data: &TestData, hd: &str, variant: &str) {
    let v = &data.hdwallet[hd][variant];
    let private_key = json_str(&last_derivation(v)["private_key"]);
    let mut w = wallet_from_variant(v, hd);
    w.from_private_key(&private_key).expect("from_private_key");

    let mut dump = build_dump_from_variant(v);
    if let Some(object) = dump.as_object_mut() {
        insert_nulls(
            object,
            &[
                "entropy",
                "strength",
                "mnemonic",
                "passphrase",
                "language",
                "seed",
                "root_xprivate_key",
                "root_xpublic_key",
                "root_private_key",
                "root_wif",
                "root_chain_code",
                "root_public_key",
                "strict",
            ],
        );
        if let Some(derivation) = object.get_mut("derivation").and_then(Value::as_object_mut) {
            insert_nulls(
                derivation,
                &[
                    "xprivate_key",
                    "xpublic_key",
                    "chain_code",
                    "parent_fingerprint",
                ],
            );
            derivation.remove("at");
        }
    }
    assert_dump_only(&mut w, &dump);
}

pub fn assert_from_wif_leaf(data: &TestData, hd: &str, variant: &str) {
    let v = &data.hdwallet[hd][variant];
    let wif = json_str(&last_derivation(v)["wif"]);
    let mut w = wallet_from_variant(v, hd);
    w.from_wif(&wif).expect("from_wif");

    let mut dump = build_dump_from_variant(v);
    if let Some(object) = dump.as_object_mut() {
        insert_nulls(
            object,
            &[
                "entropy",
                "strength",
                "mnemonic",
                "passphrase",
                "language",
                "seed",
                "root_xprivate_key",
                "root_xpublic_key",
                "root_private_key",
                "root_wif",
                "root_chain_code",
                "root_public_key",
                "strict",
            ],
        );
        if let Some(derivation) = object.get_mut("derivation").and_then(Value::as_object_mut) {
            insert_nulls(
                derivation,
                &[
                    "xprivate_key",
                    "xpublic_key",
                    "chain_code",
                    "parent_fingerprint",
                ],
            );
            derivation.remove("at");
        }
    }
    assert_dump_only(&mut w, &dump);
}

pub fn assert_from_public_key_leaf(data: &TestData, hd: &str, variant: &str) {
    let v = &data.hdwallet[hd][variant];
    let public_key = json_str(&last_derivation(v)["public_key"]);
    let mut w = wallet_from_variant(v, hd);
    w.from_public_key(&public_key).expect("from_public_key");

    let mut dump = build_dump_from_variant(v);
    if let Some(object) = dump.as_object_mut() {
        insert_nulls(
            object,
            &[
                "entropy",
                "strength",
                "mnemonic",
                "passphrase",
                "language",
                "seed",
                "root_xprivate_key",
                "root_xpublic_key",
                "root_private_key",
                "root_wif",
                "root_chain_code",
                "root_public_key",
                "wif_type",
                "strict",
            ],
        );
        if let Some(derivation) = object.get_mut("derivation").and_then(Value::as_object_mut) {
            insert_nulls(
                derivation,
                &[
                    "xprivate_key",
                    "xpublic_key",
                    "private_key",
                    "chain_code",
                    "wif",
                    "parent_fingerprint",
                ],
            );
            derivation.remove("at");
        }
    }
    assert_dump_only(&mut w, &dump);
}

pub fn assert_from_private_key_cardano(data: &TestData, variant: &str) {
    let hd = "Cardano";
    let v = &data.hdwallet[hd][variant];
    let private_key = json_str(&last_derivation(v)["private_key"]);
    let mut w = wallet_from_variant(v, hd);
    w.from_private_key(&private_key).expect("from_private_key");

    let mut dump = build_dump_from_variant(v);
    if let Some(object) = dump.as_object_mut() {
        insert_nulls(
            object,
            &[
                "entropy",
                "strength",
                "mnemonic",
                "passphrase",
                "language",
                "seed",
                "root_xprivate_key",
                "root_xpublic_key",
                "root_private_key",
                "root_chain_code",
                "root_public_key",
                "strict",
            ],
        );
        if let Some(derivation) = object.get_mut("derivation").and_then(Value::as_object_mut) {
            insert_nulls(
                derivation,
                &[
                    "xprivate_key",
                    "xpublic_key",
                    "chain_code",
                    "parent_fingerprint",
                ],
            );
            derivation.remove("at");
        }
    }
    assert_dump_only(&mut w, &dump);
}

pub fn assert_from_public_key_cardano(data: &TestData, variant: &str) {
    let hd = "Cardano";
    let v = &data.hdwallet[hd][variant];
    let public_key = json_str(&last_derivation(v)["public_key"]);
    let mut w = wallet_from_variant(v, hd);
    w.from_public_key(&public_key).expect("from_public_key");

    let mut dump = build_dump_from_variant(v);
    if let Some(object) = dump.as_object_mut() {
        insert_nulls(
            object,
            &[
                "entropy",
                "strength",
                "mnemonic",
                "passphrase",
                "language",
                "seed",
                "root_xprivate_key",
                "root_xpublic_key",
                "root_private_key",
                "root_chain_code",
                "root_public_key",
                "strict",
            ],
        );
        if let Some(derivation) = object.get_mut("derivation").and_then(Value::as_object_mut) {
            insert_nulls(
                derivation,
                &[
                    "xprivate_key",
                    "xpublic_key",
                    "private_key",
                    "chain_code",
                    "parent_fingerprint",
                ],
            );
            derivation.remove("at");
        }
    }
    assert_dump_only(&mut w, &dump);
}

pub fn assert_electrum_v1_from_master_private_key(data: &TestData, variant: &str) {
    let hd = "Electrum-V1";
    let v = &data.hdwallet[hd][variant];
    let mut w = wallet_from_variant(v, hd);
    w.from_private_key(&json_str(&v["master_private_key"]))
        .expect("from_private_key");
    apply_derivation(&mut w, data, hd);

    let mut expected = v.clone();
    if let Some(object) = expected.as_object_mut() {
        insert_nulls(
            object,
            &[
                "entropy",
                "strength",
                "mnemonic",
                "passphrase",
                "language",
                "seed",
            ],
        );
    }
    assert_dumps_dump(&mut w, &expected);
}

pub fn assert_electrum_v1_from_master_wif(data: &TestData, variant: &str) {
    let hd = "Electrum-V1";
    let v = &data.hdwallet[hd][variant];
    let mut w = wallet_from_variant(v, hd);
    w.from_wif(&json_str(&v["master_wif"])).expect("from_wif");
    apply_derivation(&mut w, data, hd);

    let mut expected = v.clone();
    if let Some(object) = expected.as_object_mut() {
        insert_nulls(
            object,
            &[
                "entropy",
                "strength",
                "mnemonic",
                "passphrase",
                "language",
                "seed",
            ],
        );
    }
    assert_dumps_dump(&mut w, &expected);
}

pub fn assert_electrum_v1_from_master_public_key(data: &TestData, variant: &str) {
    let hd = "Electrum-V1";
    let v = &data.hdwallet[hd][variant];
    let mut w = wallet_from_variant(v, hd);
    w.from_public_key(&json_str(&v["master_public_key"]))
        .expect("from_public_key");
    apply_derivation(&mut w, data, hd);

    let mut expected = v.clone();
    if let Some(object) = expected.as_object_mut() {
        insert_nulls(
            object,
            &[
                "entropy",
                "strength",
                "mnemonic",
                "passphrase",
                "language",
                "seed",
                "master_private_key",
                "master_wif",
                "wif_type",
            ],
        );
        if let Some(derivations) = object.get_mut("derivations").and_then(Value::as_array_mut) {
            for step in derivations {
                if let Some(step_object) = step.as_object_mut() {
                    insert_nulls(step_object, &["private_key", "wif"]);
                }
            }
        }
    }
    assert_dumps_dump(&mut w, &expected);
}

pub fn assert_monero_from_seed(data: &TestData) {
    let hd = "Monero";
    let variant = "dumps";
    let v = &data.hdwallet[hd][variant];
    let mut w = wallet_from_variant(v, hd);
    w.from_seed_hex(&json_str(&v["entropy"]))
        .expect("from_seed");
    apply_derivation(&mut w, data, hd);

    let mut expected = v.clone();
    if let Some(object) = expected.as_object_mut() {
        insert_nulls(
            object,
            &[
                "entropy",
                "strength",
                "mnemonic",
                "passphrase",
                "language",
            ],
        );
    }
    assert_dumps_dump(&mut w, &expected);
}

pub fn assert_monero_from_entropy(data: &TestData) {
    assert_from_entropy(data, "Monero", "dumps");
}

pub fn assert_monero_from_mnemonic(data: &TestData) {
    assert_from_mnemonic(data, "Monero", "dumps");
}

pub fn assert_monero_from_private_key(data: &TestData) {
    let hd = "Monero";
    let variant = "from-private-key";
    let v = &data.hdwallet[hd][variant];
    let mut w = wallet_from_variant(v, hd);
    w.from_private_key(&json_str(&v["private_key"]))
        .expect("from_private_key");
    apply_derivation(&mut w, data, hd);
    assert_dumps_dump(&mut w, v);
}

pub fn assert_monero_from_spend_private_key(data: &TestData) {
    let hd = "Monero";
    let variant = "dumps";
    let v = &data.hdwallet[hd][variant];
    let mut w = wallet_from_variant(v, hd);
    w.from_spend_private_key(&json_str(&v["spend_private_key"]))
        .expect("from_spend_private_key");
    apply_derivation(&mut w, data, hd);

    let mut expected = v.clone();
    if let Some(object) = expected.as_object_mut() {
        insert_nulls(
            object,
            &[
                "entropy",
                "strength",
                "mnemonic",
                "passphrase",
                "language",
                "seed",
            ],
        );
    }
    assert_dumps_dump(&mut w, &expected);
}

pub fn assert_monero_from_watch_only(data: &TestData) {
    let hd = "Monero";
    let variant = "dumps";
    let v = &data.hdwallet[hd][variant];
    let mut w = wallet_from_variant(v, hd);
    w.from_watch_only(
        &json_str(&v["view_private_key"]),
        &json_str(&v["spend_public_key"]),
    )
    .expect("from_watch_only");
    apply_derivation(&mut w, data, hd);

    let mut expected = v.clone();
    if let Some(object) = expected.as_object_mut() {
        insert_nulls(
            object,
            &[
                "entropy",
                "strength",
                "mnemonic",
                "passphrase",
                "language",
                "seed",
                "spend_private_key",
            ],
        );
    }
    assert_dumps_dump(&mut w, &expected);
}

pub fn kebab(value: &Value, key: &str) -> String {
    json_str(&value[key])
}

pub fn is_derivation(name: &str) -> bool {
    Derivations::is_derivation(name)
}
