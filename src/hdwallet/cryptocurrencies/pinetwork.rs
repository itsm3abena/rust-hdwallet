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

pub static ADDRESS_TYPE_LABELS: AddressTypes = &[
    ("STELLAR", "Stellar"),
    ("PRIVATE_KEY", "private_key"),
    ("PUBLIC_KEY", "public_key"),
];

pub static PI_NETWORK_PARAMS: Params = Params {
    checksum_length: Some(2),
    payment_id_length: None,
    suffix: None,
    address_prefix: None,
    chain_prefixes_by_address_type: None,
    alphabet: None,
    field_size_hex: None,
    tap_tweak_sha256: None,
    cardano_wallet_types: None,
    cardano_payment_prefix: None,
    cardano_reward_prefix: None,
    cardano_public_key_address: None,
    cardano_redemption_address: None,
    address_type_codes: Some(&[
        ("PRIVATE_KEY", (18 << 3) as u8),
        ("PUBLIC_KEY", (6 << 3) as u8),
    ]),
    address_prefix_byte: None,
    address_version_byte: None,
};

pub struct PiNetwork {
    pub network: NetworkName,
}

impl PiNetwork {
    pub const NAME: &'static str = "Pi-Network";
    pub const SYMBOL: &'static str = "PI";
    pub const COIN_TYPE: u32 = coin_types::PI_NETWORK;
    pub const ECC: &'static str = "SLIP10Ed25519ECC";
    pub const SUPPORT_BIP38: bool = false;
    pub const DEFAULT_HD: &'static str = "BIP44";
    pub const DEFAULT_PATH: &'static str = "m/44'/314159'/0'/0/0";
    pub const DEFAULT_ADDRESS: &'static str = "STELLAR";
    pub const ENTROPIES: &'static [&'static str] = &["BIP39"];
    pub const MNEMONICS: &'static [&'static str] = &["BIP39"];
    pub const SEEDS: &'static [&'static str] = &["BIP39"];
    pub const HDS: &'static [&'static str] = &["BIP32", "BIP44"];
    pub const ADDRESSES: &'static [&'static str] = &["STELLAR"];
    pub const SEMANTICS: &'static [&'static str] = &["p2pkh"];
    pub const DEFAULT_SEMANTIC: Option<&'static str> = Some("p2pkh");

    pub const INFO: Info<'static> = Info(&[
        InfoItem {
            key: "SOURCE_CODE",
            value: InfoValue::Str("https://github.com/pi-apps"),
        },
        InfoItem {
            key: "WHITEPAPER",
            value: InfoValue::Str("https://minepi.com/white-paper"),
        },
        InfoItem {
            key: "WEBSITES",
            value: InfoValue::List(&["https://minepi.com"]),
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
    name: PiNetwork::NAME,
    symbol: PiNetwork::SYMBOL,
    info: PiNetwork::INFO,
    ecc: PiNetwork::ECC,
    coin_type: PiNetwork::COIN_TYPE,
    support_bip38: PiNetwork::SUPPORT_BIP38,
    networks: NETWORKS,
    default_network: &MAINNET,
    entropies: PiNetwork::ENTROPIES,
    mnemonics: PiNetwork::MNEMONICS,
    seeds: PiNetwork::SEEDS,
    hds: PiNetwork::HDS,
    default_hd: PiNetwork::DEFAULT_HD,
    default_path: PiNetwork::DEFAULT_PATH,
    addresses: PiNetwork::ADDRESSES,
    default_address: PiNetwork::DEFAULT_ADDRESS,
    address_types: Some(ADDRESS_TYPE_LABELS),
    default_address_type: Some("PUBLIC_KEY"),
    address_prefixes: None,
    default_address_prefix: None,
    semantics: PiNetwork::SEMANTICS,
    default_semantic: PiNetwork::DEFAULT_SEMANTIC,
    params: Some(PI_NETWORK_PARAMS),
};
