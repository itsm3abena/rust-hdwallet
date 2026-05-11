use crate::hdwallet::slip44::coin_types;

use super::icryptocurrency::{
    AddressTypes, CryptocurrencyDescriptor, Info, InfoItem, InfoValue, NetworkDescriptor, Params,
    XPrivateKeyVersions, XPublicKeyVersions,
};
use super::NetworkName;

pub struct Mainnet;

impl Mainnet {
    pub const NAME: &'static str = "mainnet";
    pub const TYPE: u32 = 1;
    pub const PAYMENT_ADDRESS_HRP: &'static str = "addr";
    pub const REWARD_ADDRESS_HRP: &'static str = "stake";
    pub const XPRIVATE_KEY_VERSIONS: XPrivateKeyVersions = &[("P2PKH", 0x0f4331d4)];
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
    network_type: Some(Mainnet::TYPE),
    payment_address_hrp: Some(Mainnet::PAYMENT_ADDRESS_HRP),
    reward_address_hrp: Some(Mainnet::REWARD_ADDRESS_HRP),
};

pub struct Testnet;

impl Testnet {
    pub const NAME: &'static str = "testnet";
    pub const TYPE: u32 = 0;
    pub const PAYMENT_ADDRESS_HRP: &'static str = "addr_test";
    pub const REWARD_ADDRESS_HRP: &'static str = "stake_test";
    pub const XPRIVATE_KEY_VERSIONS: XPrivateKeyVersions = &[("P2PKH", 0x04358394)];
    pub const XPUBLIC_KEY_VERSIONS: XPublicKeyVersions = &[("P2PKH", 0x043587cf)];
}

pub static TESTNET: NetworkDescriptor = NetworkDescriptor {
    name: Testnet::NAME,
    public_key_address_prefix: None,
    script_address_prefix: None,
    hrp: None,
    witness_versions: None,
    xprivate_key_versions: Some(Testnet::XPRIVATE_KEY_VERSIONS),
    xpublic_key_versions: Some(Testnet::XPUBLIC_KEY_VERSIONS),
    message_prefix: None,
    wif_prefix: None,
    legacy_public_key_address_prefix: None,
    std_public_key_address_prefix: None,
    legacy_script_address_prefix: None,
    std_script_address_prefix: None,
    standard: None,
    integrated: None,
    sub_address: None,
    network_type: Some(Testnet::TYPE),
    payment_address_hrp: Some(Testnet::PAYMENT_ADDRESS_HRP),
    reward_address_hrp: Some(Testnet::REWARD_ADDRESS_HRP),
};

pub static NETWORKS: &[(&'static str, &'static NetworkDescriptor)] =
    &[("MAINNET", &MAINNET), ("TESTNET", &TESTNET)];

pub static CARDANO_WALLET_TYPES: AddressTypes = &[
    ("BYRON_ICARUS", "byron-icarus"),
    ("BYRON_LEDGER", "byron-ledger"),
    ("BYRON_LEGACY", "byron-legacy"),
    ("SHELLEY_ICARUS", "shelley-icarus"),
    ("SHELLEY_LEDGER", "shelley-ledger"),
];

pub static CARDANO_ADDRESS_TYPES: AddressTypes = &[
    ("PUBLIC_KEY", "public-key"),
    ("REDEMPTION", "redemption"),
    ("PAYMENT", "payment"),
    ("STAKING", "staking"),
    ("REWARD", "reward"),
];

pub static CARDANO_PARAMS: Params = Params {
    checksum_length: None,
    payment_id_length: None,
    suffix: None,
    address_prefix: None,
    chain_prefixes_by_address_type: None,
    alphabet: None,
    field_size_hex: None,
    tap_tweak_sha256: None,
    cardano_wallet_types: Some(CARDANO_WALLET_TYPES),
    cardano_payment_prefix: Some(0x00),
    cardano_reward_prefix: Some(0x0e),
    cardano_public_key_address: Some(0),
    cardano_redemption_address: Some(2),
    address_type_codes: None,
    address_prefix_byte: None,
    address_version_byte: None,
    key_type: None,
    address_type_prefix_bytes: None,
};

pub struct Cardano {
    pub network: NetworkName,
}

impl Cardano {
    pub const NAME: &'static str = "Cardano";
    pub const SYMBOL: &'static str = "ADA";
    pub const COIN_TYPE: u32 = coin_types::CARDANO;
    pub const ECC: &'static str = "KholawEd25519ECC";
    pub const SUPPORT_BIP38: bool = false;
    pub const DEFAULT_HD: &'static str = "CARDANO";
    pub const DEFAULT_PATH: &'static str = "m/44'/1815'/0'/0/0";
    pub const DEFAULT_ADDRESS: &'static str = "CARDANO";
    pub const ENTROPIES: &'static [&'static str] = &["BIP39"];
    pub const MNEMONICS: &'static [&'static str] = &["BIP39"];
    pub const SEEDS: &'static [&'static str] = &["CARDANO"];
    pub const HDS: &'static [&'static str] = &["CARDANO"];
    pub const ADDRESSES: &'static [&'static str] = &["CARDANO"];
    pub const SEMANTICS: &'static [&'static str] = &["p2pkh"];
    pub const DEFAULT_SEMANTIC: Option<&'static str> = Some("p2pkh");

    pub const INFO: Info<'static> = Info(&[
        InfoItem {
            key: "SOURCE_CODE",
            value: InfoValue::Str("https://cardanoupdates.com"),
        },
        InfoItem {
            key: "WHITEPAPER",
            value: InfoValue::Str("https://docs.cardano.org/en/latest"),
        },
        InfoItem {
            key: "WEBSITES",
            value: InfoValue::List(&["https://www.cardano.org"]),
        },
    ]);

    #[must_use]
    pub fn network_descriptor(&self) -> &'static NetworkDescriptor {
        match self.network {
            NetworkName::Mainnet => &MAINNET,
            NetworkName::Testnet => &TESTNET,
            NetworkName::Regtest => &TESTNET,
        }
    }
}

pub static DESCRIPTOR: CryptocurrencyDescriptor = CryptocurrencyDescriptor {
    name: Cardano::NAME,
    symbol: Cardano::SYMBOL,
    info: Cardano::INFO,
    ecc: Cardano::ECC,
    coin_type: Cardano::COIN_TYPE,
    support_bip38: Cardano::SUPPORT_BIP38,
    networks: NETWORKS,
    default_network: &MAINNET,
    entropies: Cardano::ENTROPIES,
    mnemonics: Cardano::MNEMONICS,
    seeds: Cardano::SEEDS,
    hds: Cardano::HDS,
    default_hd: Cardano::DEFAULT_HD,
    default_path: Cardano::DEFAULT_PATH,
    addresses: Cardano::ADDRESSES,
    default_address: Cardano::DEFAULT_ADDRESS,
    address_types: Some(CARDANO_ADDRESS_TYPES),
    default_address_type: None,
    address_prefixes: None,
    default_address_prefix: None,
    semantics: Cardano::SEMANTICS,
    default_semantic: Cardano::DEFAULT_SEMANTIC,
    params: Some(CARDANO_PARAMS),
};
