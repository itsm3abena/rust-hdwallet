use crate::hdwallet::slip44::coin_types;

use super::icryptocurrency::{
    CryptocurrencyDescriptor, Info, InfoItem, InfoValue, NetworkDescriptor, Params,
    XPrivateKeyVersions, XPublicKeyVersions,
};
use super::NetworkName;

pub struct Mainnet;

impl Mainnet {
    pub const NAME: &'static str = "mainnet";
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
    network_type: None,
    payment_address_hrp: None,
    reward_address_hrp: None,
};

pub static NETWORKS: &[(&'static str, &'static NetworkDescriptor)] = &[("MAINNET", &MAINNET)];

pub static ALGORAND_PARAMS: Params = Params {
    checksum_length: Some(4),
    suffix: None,
    address_prefix: None,
    chain_prefixes_by_address_type: None,
    alphabet: None,
    field_size_hex: None,
    tap_tweak_sha256: None,
};

pub struct Algorand {
    pub network: NetworkName,
}

impl Algorand {
    pub const NAME: &'static str = "Algorand";
    pub const SYMBOL: &'static str = "ALGO";
    pub const COIN_TYPE: u32 = coin_types::ALGORAND;
    pub const ECC: &'static str = "KholawEd25519ECC";
    pub const SUPPORT_BIP38: bool = false;
    pub const DEFAULT_HD: &'static str = "Algorand";
    pub const DEFAULT_PATH: &'static str = "m/44'/283'/0'/0/0";
    pub const DEFAULT_ADDRESS: &'static str = "ALGORAND";
    pub const ENTROPIES: &'static [&'static str] = &["Algorand", "BIP39"];
    pub const MNEMONICS: &'static [&'static str] = &["Algorand", "BIP39"];
    pub const SEEDS: &'static [&'static str] = &["Algorand", "BIP39"];
    pub const HDS: &'static [&'static str] = &["Algorand", "BIP32", "BIP44"];
    pub const ADDRESSES: &'static [&'static str] = &["ALGORAND"];
    pub const SEMANTICS: &'static [&'static str] = &["p2pkh"];
    pub const DEFAULT_SEMANTIC: Option<&'static str> = Some("p2pkh");

    pub const INFO: Info<'static> = Info(&[
        InfoItem {
            key: "SOURCE_CODE",
            value: InfoValue::Str("https://github.com/algorand/go-algorand"),
        },
        InfoItem {
            key: "WHITEPAPER",
            value: InfoValue::Str("https://www.algorand.com/resources/white-papers"),
        },
        InfoItem {
            key: "WEBSITES",
            value: InfoValue::List(&[
                "http://algorand.foundation",
                "https://www.algorand.com",
            ]),
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
    name: Algorand::NAME,
    symbol: Algorand::SYMBOL,
    info: Algorand::INFO,
    ecc: Algorand::ECC,
    coin_type: Algorand::COIN_TYPE,
    support_bip38: Algorand::SUPPORT_BIP38,
    networks: NETWORKS,
    default_network: &MAINNET,
    entropies: Algorand::ENTROPIES,
    mnemonics: Algorand::MNEMONICS,
    seeds: Algorand::SEEDS,
    hds: Algorand::HDS,
    default_hd: Algorand::DEFAULT_HD,
    default_path: Algorand::DEFAULT_PATH,
    addresses: Algorand::ADDRESSES,
    default_address: Algorand::DEFAULT_ADDRESS,
    address_types: None,
    default_address_type: None,
    address_prefixes: None,
    default_address_prefix: None,
    semantics: Algorand::SEMANTICS,
    default_semantic: Algorand::DEFAULT_SEMANTIC,
    params: Some(ALGORAND_PARAMS),
};
