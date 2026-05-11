use crate::hdwallet::slip44::coin_types;

use super::icryptocurrency::{
    AddressTypes, CryptocurrencyDescriptor, Info, InfoItem, InfoValue, NetworkDescriptor, Params,
    XPrivateKeyVersions, XPublicKeyVersions,
};
use super::NetworkName;

pub struct Mainnet;

impl Mainnet {
    pub const NAME: &'static str = "mainnet";
    pub const XPRIVATE_KEY_VERSIONS: XPrivateKeyVersions = &[("P2PKH", 0x0488ade4)];
    pub const XPUBLIC_KEY_VERSIONS: XPublicKeyVersions = &[("P2PKH", 0x0488b21e)];
}

pub static MAINNET: NetworkDescriptor = NetworkDescriptor {
    name: Mainnet::NAME,
    public_key_address_prefix: None,
    script_address_prefix: None,
    hrp: None,
    witness_versions: None,
    xprivate_key_versions: Some(Mainnet::XPRIVATE_KEY_VERSIONS),
    xpublic_key_versions: Some(Mainnet::XPUBLIC_KEY_VERSIONS),
    message_prefix: None,
    wif_prefix: None,
    legacy_public_key_address_prefix: None,
    std_public_key_address_prefix: None,
    legacy_script_address_prefix: None,
    std_script_address_prefix: None,
    standard: None,
    integrated: None,
    sub_address: None,
    network_type: None,
    payment_address_hrp: None,
    reward_address_hrp: None,
};

pub static NETWORKS: &[(&'static str, &'static NetworkDescriptor)] = &[("MAINNET", &MAINNET)];

pub static ADDRESS_TYPE_LABELS: AddressTypes = &[("TEZOS", "Tezos")];

pub static TEZOS_PARAMS: Params = Params {
    checksum_length: None,
    payment_id_length: None,
    suffix: None,
    address_prefix: None,
    chain_prefixes_by_address_type: Some(&[
        ("TZ1", "tz1"),
        ("TZ2", "tz2"),
        ("TZ3", "tz3"),
    ]),
    alphabet: None,
    field_size_hex: None,
    tap_tweak_sha256: None,
    cardano_wallet_types: None,
    cardano_payment_prefix: None,
    cardano_reward_prefix: None,
    cardano_public_key_address: None,
    cardano_redemption_address: None,
    address_type_codes: None,
    address_prefix_byte: None,
    address_version_byte: None,
    key_type: None,
    address_type_prefix_bytes: Some(&[
        ("TZ1", &[0x06, 0xa1, 0x9f]),
        ("TZ2", &[0x06, 0xa1, 0xa1]),
        ("TZ3", &[0x06, 0xa1, 0xa4]),
    ]),
};

pub struct Tezos {
    pub network: NetworkName,
}

impl Tezos {
    pub const NAME: &'static str = "Tezos";
    pub const SYMBOL: &'static str = "XTZ";
    pub const COIN_TYPE: u32 = coin_types::TEZOS;
    pub const ECC: &'static str = "SLIP10Ed25519ECC";
    pub const SUPPORT_BIP38: bool = false;
    pub const DEFAULT_HD: &'static str = "BIP44";
    pub const DEFAULT_PATH: &'static str = "m/44'/1729'/0'/0/0";
    pub const DEFAULT_ADDRESS: &'static str = "TEZOS";
    pub const ENTROPIES: &'static [&'static str] = &["BIP39"];
    pub const MNEMONICS: &'static [&'static str] = &["BIP39"];
    pub const SEEDS: &'static [&'static str] = &["BIP39"];
    pub const HDS: &'static [&'static str] = &["BIP32", "BIP44"];
    pub const ADDRESSES: &'static [&'static str] = &["TEZOS"];
    pub const SEMANTICS: &'static [&'static str] = &["p2pkh"];
    pub const DEFAULT_SEMANTIC: Option<&'static str> = Some("p2pkh");

    pub const INFO: Info<'static> = Info(&[
        InfoItem {
            key: "SOURCE_CODE",
            value: InfoValue::Str("https://github.com/tezos/tezos"),
        },
        InfoItem {
            key: "WHITEPAPER",
            value: InfoValue::Str("https://tezos.com/whitepaper.pdf"),
        },
        InfoItem {
            key: "WEBSITES",
            value: InfoValue::List(&["https://www.tezos.com"]),
        },
    ]);

    #[must_use]
    pub fn network_descriptor(&self) -> &'static NetworkDescriptor {
        match self.network {
            NetworkName::Mainnet => &MAINNET,
            NetworkName::Testnet | NetworkName::Regtest => &MAINNET,
        }
    }
}

pub static DESCRIPTOR: CryptocurrencyDescriptor = CryptocurrencyDescriptor {
    name: Tezos::NAME,
    symbol: Tezos::SYMBOL,
    info: Tezos::INFO,
    ecc: Tezos::ECC,
    coin_type: Tezos::COIN_TYPE,
    support_bip38: Tezos::SUPPORT_BIP38,
    networks: NETWORKS,
    default_network: &MAINNET,
    entropies: Tezos::ENTROPIES,
    mnemonics: Tezos::MNEMONICS,
    seeds: Tezos::SEEDS,
    hds: Tezos::HDS,
    default_hd: Tezos::DEFAULT_HD,
    default_path: Tezos::DEFAULT_PATH,
    addresses: Tezos::ADDRESSES,
    default_address: Tezos::DEFAULT_ADDRESS,
    address_types: Some(ADDRESS_TYPE_LABELS),
    default_address_type: Some("TEZOS"),
    address_prefixes: Some(&["TZ1", "TZ2", "TZ3"]),
    default_address_prefix: Some("TZ1"),
    semantics: Tezos::SEMANTICS,
    default_semantic: Tezos::DEFAULT_SEMANTIC,
    params: Some(TEZOS_PARAMS),
};
