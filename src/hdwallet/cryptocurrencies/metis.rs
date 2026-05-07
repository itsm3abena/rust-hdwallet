use crate::hdwallet::slip44::coin_types;

use super::icryptocurrency::{
    AddressTypes, CryptocurrencyDescriptor, Info, InfoItem, InfoValue, NetworkDescriptor,
    XPrivateKeyVersions, XPublicKeyVersions,
};
use super::NetworkName;

pub struct Mainnet;

impl Mainnet {
    pub const NAME: &'static str = "mainnet";
    pub const WIF_PREFIX: u8 = 0x80;
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
    wif_prefix: Some(Mainnet::WIF_PREFIX),
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

pub static ADDRESS_TYPE_LABELS: AddressTypes = &[("ETHEREUM", "Ethereum")];

pub struct Metis {
    pub network: NetworkName,
}

impl Metis {
    pub const NAME: &'static str = "Metis";
    pub const SYMBOL: &'static str = "METIS";
    pub const COIN_TYPE: u32 = coin_types::METIS;
    pub const ECC: &'static str = "SLIP10Secp256k1ECC";
    pub const SUPPORT_BIP38: bool = false;
    pub const DEFAULT_HD: &'static str = "BIP44";
    pub const DEFAULT_PATH: &'static str = "m/44'/60'/0'/0/0";
    pub const DEFAULT_ADDRESS: &'static str = "ETHEREUM";
    pub const ENTROPIES: &'static [&'static str] = &["BIP39"];
    pub const MNEMONICS: &'static [&'static str] = &["BIP39"];
    pub const SEEDS: &'static [&'static str] = &["BIP39"];
    pub const HDS: &'static [&'static str] = &["BIP32", "BIP44"];
    pub const ADDRESSES: &'static [&'static str] = &["ETHEREUM"];
    pub const SEMANTICS: &'static [&'static str] = &["p2pkh"];
    pub const DEFAULT_SEMANTIC: Option<&'static str> = Some("p2pkh");

    pub const INFO: Info<'static> = Info(&[
        InfoItem {
            key: "SOURCE_CODE",
            value: InfoValue::Str("https://github.com/MetisProtocol/metis"),
        },
        InfoItem {
            key: "WHITEPAPER",
            value: InfoValue::Str(
                "https://drive.google.com/file/d/1LS7CmKFt-FkfVXxSNu06hNgoZXxMzTC-/view",
            ),
        },
        InfoItem {
            key: "WEBSITES",
            value: InfoValue::List(&["https://www.metis.io"]),
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
    name: Metis::NAME,
    symbol: Metis::SYMBOL,
    info: Metis::INFO,
    ecc: Metis::ECC,
    coin_type: Metis::COIN_TYPE,
    support_bip38: Metis::SUPPORT_BIP38,
    networks: NETWORKS,
    default_network: &MAINNET,
    entropies: Metis::ENTROPIES,
    mnemonics: Metis::MNEMONICS,
    seeds: Metis::SEEDS,
    hds: Metis::HDS,
    default_hd: Metis::DEFAULT_HD,
    default_path: Metis::DEFAULT_PATH,
    addresses: Metis::ADDRESSES,
    default_address: Metis::DEFAULT_ADDRESS,
    address_types: Some(ADDRESS_TYPE_LABELS),
    default_address_type: None,
    address_prefixes: None,
    default_address_prefix: None,
    semantics: Metis::SEMANTICS,
    default_semantic: Metis::DEFAULT_SEMANTIC,
    params: None,
};
