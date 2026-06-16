use std::collections::HashMap;

#[derive(Clone, Copy)]
pub enum ArgSpec {
    Field(&'static str),
    Literal(&'static str),
}

#[derive(Clone, Copy)]
pub struct MethodRule {
    #[allow(dead_code)]
    pub cli_param: &'static str,
    pub method: &'static str,
    pub derivable: bool,
    pub root_changes: &'static [&'static str],
    pub derivation_changes: &'static [&'static str],
}

pub struct HdRules {
    pub available_methods: &'static [(&'static str, MethodRule)],
    pub args: &'static [(&'static str, ArgSpec)],
}

const ROOT_FROM_SEED: &[&str] = &["entropy", "strength", "mnemonic", "passphrase", "language"];
const ROOT_FROM_XPRIVATE: &[&str] = &[
    "entropy", "strength", "mnemonic", "passphrase", "language", "seed",
];
const ROOT_FROM_LEAF: &[&str] = &[
    "entropy", "strength", "mnemonic", "passphrase", "language", "seed",
    "root_xprivate_key", "root_xpublic_key", "root_private_key", "root_wif",
    "root_chain_code", "root_public_key", "strict",
];
const DERIVATION_WIF: &[&str] = &["xprivate_key", "xpublic_key", "chain_code", "parent_fingerprint"];
const DERIVATION_PUBLIC: &[&str] = &[
    "xprivate_key", "xpublic_key", "private_key", "chain_code", "wif", "parent_fingerprint",
];

const BIP44_METHODS: &[(&str, MethodRule)] = &[
    ("entropy", MethodRule { cli_param: "entropy", method: "entropy", derivable: true, root_changes: &[], derivation_changes: &[] }),
    ("mnemonic", MethodRule { cli_param: "mnemonic", method: "mnemonic", derivable: true, root_changes: &[], derivation_changes: &[] }),
    ("seed", MethodRule { cli_param: "seed", method: "seed", derivable: true, root_changes: ROOT_FROM_SEED, derivation_changes: &[] }),
    ("xprivate-key", MethodRule { cli_param: "xprivate-key", method: "root_xprivate_key", derivable: true, root_changes: ROOT_FROM_XPRIVATE, derivation_changes: &[] }),
    ("wif", MethodRule { cli_param: "wif", method: "wif", derivable: false, root_changes: ROOT_FROM_LEAF, derivation_changes: DERIVATION_WIF }),
    ("private-key", MethodRule { cli_param: "private-key", method: "private_key", derivable: false, root_changes: ROOT_FROM_LEAF, derivation_changes: DERIVATION_WIF }),
    ("public-key", MethodRule { cli_param: "public-key", method: "public_key", derivable: false, root_changes: &["entropy", "strength", "mnemonic", "passphrase", "language", "seed", "root_xprivate_key", "root_xpublic_key", "root_private_key", "root_wif", "root_chain_code", "root_public_key", "wif_type", "strict"], derivation_changes: DERIVATION_PUBLIC }),
];

const BIP44_ARGS: &[(&str, ArgSpec)] = &[
    ("public-key-type", ArgSpec::Field("public_key_type")),
    ("language", ArgSpec::Field("language")),
];

const BIP32_METHODS: &[(&str, MethodRule)] = &[
    ("entropy", MethodRule { cli_param: "entropy", method: "entropy", derivable: true, root_changes: &[], derivation_changes: &[] }),
    ("mnemonic", MethodRule { cli_param: "mnemonic", method: "mnemonic", derivable: true, root_changes: &[], derivation_changes: &[] }),
    ("seed", MethodRule { cli_param: "seed", method: "seed", derivable: true, root_changes: ROOT_FROM_SEED, derivation_changes: &[] }),
    ("xprivate-key", MethodRule { cli_param: "xprivate-key", method: "root_xprivate_key", derivable: true, root_changes: ROOT_FROM_XPRIVATE, derivation_changes: &[] }),
    ("xpublic-key", MethodRule { cli_param: "xpublic-key", method: "root_xpublic_key", derivable: true, root_changes: &["entropy", "strength", "mnemonic", "passphrase", "language", "seed", "root_xprivate_key", "root_private_key", "root_wif", "wif_type"], derivation_changes: &["xprivate_key", "private_key", "wif"] }),
    ("wif", MethodRule { cli_param: "wif", method: "wif", derivable: false, root_changes: ROOT_FROM_LEAF, derivation_changes: DERIVATION_WIF }),
    ("private-key", MethodRule { cli_param: "private-key", method: "private_key", derivable: false, root_changes: ROOT_FROM_LEAF, derivation_changes: DERIVATION_WIF }),
    ("public-key", MethodRule { cli_param: "public-key", method: "public_key", derivable: false, root_changes: &["entropy", "strength", "mnemonic", "passphrase", "language", "seed", "root_xprivate_key", "root_xpublic_key", "root_private_key", "root_wif", "root_chain_code", "root_public_key", "wif_type", "strict"], derivation_changes: DERIVATION_PUBLIC }),
];

const CARDANO_METHODS: &[(&str, MethodRule)] = &[
    ("entropy", MethodRule { cli_param: "entropy", method: "entropy", derivable: true, root_changes: &[], derivation_changes: &[] }),
    ("mnemonic", MethodRule { cli_param: "mnemonic", method: "mnemonic", derivable: true, root_changes: &[], derivation_changes: &[] }),
    ("seed", MethodRule { cli_param: "seed", method: "seed", derivable: true, root_changes: ROOT_FROM_SEED, derivation_changes: &[] }),
    ("xprivate-key", MethodRule { cli_param: "xprivate-key", method: "root_xprivate_key", derivable: true, root_changes: ROOT_FROM_XPRIVATE, derivation_changes: &[] }),
];

const CARDANO_ARGS: &[(&str, ArgSpec)] = &[
    ("cardano-type", ArgSpec::Field("cardano_type")),
    ("language", ArgSpec::Field("language")),
    ("seed-client", ArgSpec::Literal("Cardano")),
];

const ELECTRUM_V1_METHODS: &[(&str, MethodRule)] = &[
    ("entropy", MethodRule { cli_param: "entropy", method: "entropy", derivable: true, root_changes: &[], derivation_changes: &[] }),
    ("mnemonic", MethodRule { cli_param: "mnemonic", method: "mnemonic", derivable: true, root_changes: &[], derivation_changes: &[] }),
    ("seed", MethodRule { cli_param: "seed", method: "seed", derivable: true, root_changes: ROOT_FROM_SEED, derivation_changes: &[] }),
    ("wif", MethodRule { cli_param: "wif", method: "master_wif", derivable: true, root_changes: ROOT_FROM_XPRIVATE, derivation_changes: &[] }),
    ("private-key", MethodRule { cli_param: "private-key", method: "master_private_key", derivable: true, root_changes: ROOT_FROM_XPRIVATE, derivation_changes: &[] }),
];

const ELECTRUM_V1_ARGS: &[(&str, ArgSpec)] = &[
    ("public-key-type", ArgSpec::Field("public_key_type")),
    ("entropy-client", ArgSpec::Literal("Electrum-V1")),
    ("mnemonic-client", ArgSpec::Literal("Electrum-V1")),
    ("seed-client", ArgSpec::Literal("Electrum-V1")),
    ("language", ArgSpec::Field("language")),
];

const ELECTRUM_V2_METHODS: &[(&str, MethodRule)] = &[
    ("entropy", MethodRule { cli_param: "entropy", method: "entropy", derivable: true, root_changes: &[], derivation_changes: &[] }),
    ("mnemonic", MethodRule { cli_param: "mnemonic", method: "mnemonic", derivable: true, root_changes: &[], derivation_changes: &[] }),
    ("seed", MethodRule { cli_param: "seed", method: "seed", derivable: true, root_changes: ROOT_FROM_SEED, derivation_changes: &[] }),
];

const ELECTRUM_V2_ARGS: &[(&str, ArgSpec)] = &[
    ("mnemonic-type", ArgSpec::Field("mnemonic_type")),
    ("mode", ArgSpec::Field("mode")),
    ("entropy-client", ArgSpec::Literal("Electrum-V2")),
    ("mnemonic-client", ArgSpec::Literal("Electrum-V2")),
    ("seed-client", ArgSpec::Literal("Electrum-V2")),
    ("language", ArgSpec::Field("language")),
];

const MONERO_METHODS: &[(&str, MethodRule)] = &[
    ("entropy", MethodRule { cli_param: "entropy", method: "entropy", derivable: true, root_changes: &[], derivation_changes: &[] }),
    ("mnemonic", MethodRule { cli_param: "mnemonic", method: "mnemonic", derivable: true, root_changes: &[], derivation_changes: &[] }),
    ("seed", MethodRule { cli_param: "seed", method: "seed", derivable: true, root_changes: ROOT_FROM_SEED, derivation_changes: &[] }),
    ("spend-private-key", MethodRule { cli_param: "spend-private-key", method: "spend_private_key", derivable: true, root_changes: ROOT_FROM_XPRIVATE, derivation_changes: &[] }),
];

const MONERO_ARGS: &[(&str, ArgSpec)] = &[
    ("language", ArgSpec::Field("language")),
    ("network", ArgSpec::Field("network")),
    ("entropy-client", ArgSpec::Literal("Monero")),
    ("mnemonic-client", ArgSpec::Literal("Monero")),
    ("seed-client", ArgSpec::Literal("Monero")),
    ("payment-id", ArgSpec::Literal("ad17dc6e6793d178")),
];

pub fn rules() -> HashMap<&'static str, HdRules> {
    HashMap::from([
        ("BIP32", HdRules { available_methods: BIP32_METHODS, args: BIP44_ARGS }),
        ("BIP44", HdRules { available_methods: BIP44_METHODS, args: BIP44_ARGS }),
        ("BIP49", HdRules { available_methods: BIP44_METHODS, args: BIP44_ARGS }),
        ("BIP84", HdRules { available_methods: BIP44_METHODS, args: BIP44_ARGS }),
        ("BIP86", HdRules { available_methods: BIP44_METHODS, args: BIP44_ARGS }),
        ("Cardano", HdRules { available_methods: CARDANO_METHODS, args: CARDANO_ARGS }),
        ("Electrum-V1", HdRules { available_methods: ELECTRUM_V1_METHODS, args: ELECTRUM_V1_ARGS }),
        ("Electrum-V2", HdRules { available_methods: ELECTRUM_V2_METHODS, args: ELECTRUM_V2_ARGS }),
        ("Monero", HdRules { available_methods: MONERO_METHODS, args: MONERO_ARGS }),
    ])
}
