use std::collections::HashSet;
use std::str::FromStr;

use bitcoin::bip32::{DerivationPath, Xpub};
use bitcoin::network::Network;
use bitcoin::secp256k1::{PublicKey as SecpPublicKey, Secp256k1};
use clap::Args;
use serde_json::{json, Map, Value};

use crate::hdwallet::addresses::iaddress::{EncodeKwargs, EncodePublicKey, IAddress};
use crate::hdwallet::addresses::{P2PKHAddress, P2TRAddress, P2WPKHAddress, P2WPKHInP2SHAddress};
use crate::hdwallet::consts::{Modes, PublicKeyTypes};
use crate::hdwallet::cryptocurrencies::icryptocurrency::{CryptocurrencyDescriptor, NetworkDescriptor};
use crate::hdwallet::cryptocurrencies::{get_cryptocurrency, resolve_network};
use crate::hdwallet::derivations::{
    Bip44Derivation, Bip49Derivation, Bip84Derivation, Bip86Derivation, ChangeInput, ChangeValue,
    Derivations, IDerivation, IndexInput,
};
use crate::hdwallet::entropies::Entropies;
use crate::hdwallet::exceptions::Error;
use crate::hdwallet::hds::bip32::Bip32Hd;
use crate::hdwallet::hds::Hds;
use crate::hdwallet::libs::ecc::hash160;
use crate::hdwallet::mnemonics::{BIP39Mnemonic, IMnemonic, Mnemonics, ElectrumV2MnemonicTypes};
use crate::hdwallet::seeds::{BIP39Seed, CardanoTypes};
use crate::hdwallet::wif::encode_wif;

#[derive(Args, Clone, Debug)]
pub struct DumpArgs {
    #[arg(long = "symbol", default_value = "BTC", help = "Set Cryptocurrency ticker symbol")]
    pub symbol: String,

    #[arg(long = "hd", default_value = "BIP44", help = "Select HD")]
    pub hd: String,

    #[arg(long = "network", default_value = "mainnet", help = "Select Network type")]
    pub network: String,

    #[arg(long = "entropy-client", default_value = "BIP39", help = "Select Entropy client")]
    pub entropy_client: String,

    #[arg(long = "entropy", help = "Set Master key from Entropy hex string")]
    pub entropy: Option<String>,

    #[arg(long = "mnemonic-client", default_value = "BIP39", help = "Select Mnemonic client")]
    pub mnemonic_client: String,

    #[arg(long = "mnemonic", help = "Set Master key from Mnemonic words")]
    pub mnemonic: Option<String>,

    #[arg(long = "language", default_value = "english", help = "Select Language for mnemonic")]
    pub language: String,

    #[arg(long = "seed-client", default_value = "BIP39", help = "Select Seed client")]
    pub seed_client: String,

    #[arg(long = "seed", help = "Set Master key from Seed hex string")]
    pub seed: Option<String>,

    #[arg(long = "passphrase", help = "Set Passphrase for mnemonic & seed")]
    pub passphrase: Option<String>,

    #[arg(long = "xprivate-key", help = "Set Master key from XPrivate key")]
    pub xprivate_key: Option<String>,

    #[arg(long = "xpublic-key", help = "Set Master key from XPublic key")]
    pub xpublic_key: Option<String>,

    #[arg(long = "encoded", default_value_t = true, help = "Set Encoded for XPrivate & XPublic keys")]
    pub encoded: bool,

    #[arg(long = "strict", default_value_t = false, help = "Set Strict for Root XPrivate & XPublic keys")]
    pub strict: bool,

    #[arg(
        long = "public-key-type",
        default_value = PublicKeyTypes::COMPRESSED,
        help = "Select Public key type"
    )]
    pub public_key_type: String,

    #[arg(long = "derivation", default_value = "BIP44", help = "Select Derivation name")]
    pub derivation: String,

    #[arg(long = "account", default_value = "0", help = "Set Account index for derivation")]
    pub account: String,

    #[arg(long = "change", default_value = "0", help = "Set Change index for derivation")]
    pub change: String,

    #[arg(long = "ecc", default_value = "0", help = "Set ECC index for HDW derivation")]
    pub ecc: String,

    #[arg(long = "role", default_value = "0", help = "Set Role index for CIP1852 derivation")]
    pub role: String,

    #[arg(long = "address", default_value = "0", help = "Set Address index for derivation")]
    pub address: String,

    #[arg(long = "minor", default_value = "1", help = "Set Minor index for Monero derivation")]
    pub minor: String,

    #[arg(long = "major", default_value = "0", help = "Set Major index for Monero derivation")]
    pub major: String,

    #[arg(long = "path", help = "Set Path for derivation")]
    pub path: Option<String>,

    #[arg(long = "indexes", help = "Set Indexes for derivation")]
    pub indexes: Vec<u32>,

    #[arg(long = "private-key", help = "Set Private key")]
    pub private_key: Option<String>,

    #[arg(long = "wif", help = "Set Wallet Import Format (WIF)")]
    pub wif: Option<String>,

    #[arg(long = "bip38", default_value_t = false, help = "Is BIP38 Encrypted Wallet Import Format")]
    pub bip38: bool,

    #[arg(long = "public-key", help = "Set Public key")]
    pub public_key: Option<String>,

    #[arg(long = "spend-private-key", help = "Set Spend Private key for Monero")]
    pub spend_private_key: Option<String>,

    #[arg(long = "view-private-key", help = "Set View Private key for Monero")]
    pub view_private_key: Option<String>,

    #[arg(long = "spend-public-key", help = "Set Spend Public key for Monero")]
    pub spend_public_key: Option<String>,

    #[arg(
        long = "staking-public-key",
        help = "Set Staking Public key for Cardano Shelley"
    )]
    pub staking_public_key: Option<String>,

    #[arg(
        long = "cardano-type",
        default_value = CardanoTypes::SHELLEY_ICARUS,
        help = "Select Cardano type"
    )]
    pub cardano_type: String,

    #[arg(long = "address-type", help = "Select Address type")]
    pub address_type: Option<String>,

    #[arg(
        long = "mode",
        default_value = Modes::STANDARD,
        help = "Select Mode of Electrum-V2"
    )]
    pub mode: String,

    #[arg(
        long = "mnemonic-type",
        default_value = ElectrumV2MnemonicTypes::STANDARD,
        help = "Select Mnemonic type of Electrum-V2"
    )]
    pub mnemonic_type: String,

    #[arg(long = "checksum", default_value_t = false, help = "Set Checksum for Monero")]
    pub checksum: bool,

    #[arg(long = "semantic", help = "Set Semantic for BIP141")]
    pub semantic: Option<String>,

    #[arg(long = "payment-id", help = "Set Payment ID for Monero")]
    pub payment_id: Option<String>,

    #[arg(long = "exclude", default_value = "", help = "Set Exclude keys from dumped")]
    pub exclude: String,
}

pub fn dump(args: DumpArgs) -> Result<String, Error> {
    let dump = build_dump(&args)?;
    serde_json::to_string_pretty(&dump).map_err(|error| Error::new(&error.to_string(), None, None, None))
}

fn build_dump(args: &DumpArgs) -> Result<Value, Error> {
    let cryptocurrency = get_cryptocurrency(&args.symbol)?;
    if !Hds::is_hd(&args.hd) {
        return Err(Error::new(
            "Wrong HD name",
            None,
            Some(&Hds::names().join(", ")),
            Some(&args.hd),
        ));
    }
    if !Derivations::is_derivation(&args.derivation) {
        return Err(Error::new(
            "Wrong from derivation name",
            None,
            Some(&Derivations::names().join(", ")),
            Some(&args.derivation),
        ));
    }
    let network = resolve_network(cryptocurrency, &args.network)?;
    let semantic = resolve_semantic(&args.hd, args.semantic.as_deref(), cryptocurrency);

    let (entropy, strength, mnemonic, seed_hex) = load_dump_source(args)?;
    let bitcoin_network = map_bitcoin_network(network.name);
    let compressed = args.public_key_type != PublicKeyTypes::UNCOMPRESSED;
    let wif_prefix = network.wif_prefix.ok_or_else(|| {
        Error::new(
            "Dump is not supported for this cryptocurrency/network yet",
            None,
            None,
            Some(cryptocurrency.name),
        )
    })?;

    let seed_bytes = hex::decode(seed_hex.trim())
        .map_err(|error| Error::new("Invalid seed hex", Some(&error.to_string()), None, None))?;
    let hd_wallet = Bip32Hd::from_seed(bitcoin_network, &seed_bytes)?;
    let secp = Secp256k1::new();
    let master = hd_wallet.master_xpriv();
    let derivation = build_dump_derivation(args, cryptocurrency)?;
    let path = DerivationPath::from_str(derivation.path())
        .map_err(|error| Error::new("Invalid derivation path", Some(&error.to_string()), None, None))?;
    let derived = hd_wallet.derive_xpriv(&path)?;
    let derived_pub = Xpub::from_priv(&secp, &derived);

    let master_public_key = SecpPublicKey::from_secret_key(&secp, &master.private_key);
    let derived_public_key = SecpPublicKey::from_secret_key(&secp, &derived.private_key);

    let root_private = hex::encode(master.private_key.secret_bytes());
    let root_wif = select_wif(&root_private, wif_prefix, compressed)?;
    let derived_private = hex::encode(derived.private_key.secret_bytes());
    let derived_wif = select_wif(&derived_private, wif_prefix, compressed)?;
    let root_public = hex::encode(master_public_key.serialize());
    let derived_public = hex::encode(derived_public_key.serialize());
    let derived_uncompressed = hex::encode(derived_public_key.serialize_uncompressed());

    let pubkey_hash = hash160(&derived_public_key.serialize());
    let fingerprint = hex::encode(&pubkey_hash[..4]);
    let parent_fingerprint = hex::encode(derived.parent_fingerprint.to_bytes());

    let address = encode_dump_address(
        args,
        cryptocurrency,
        network,
        semantic.as_deref(),
        &derived_public,
        compressed,
    )?;

    let mut dump = json!({
        "cryptocurrency": cryptocurrency.name,
        "symbol": cryptocurrency.symbol,
        "network": network.name,
        "coin_type": cryptocurrency.coin_type,
        "entropy": entropy,
        "strength": strength,
        "mnemonic": mnemonic,
        "passphrase": args.passphrase,
        "language": args.language,
        "seed": seed_hex,
        "ecc": display_ecc_name(cryptocurrency.ecc),
        "hd": args.hd,
        "semantic": semantic,
        "root_xprivate_key": master.to_string(),
        "root_xpublic_key": Xpub::from_priv(&secp, master).to_string(),
        "root_private_key": root_private,
        "root_wif": root_wif,
        "root_chain_code": hex::encode(master.chain_code.as_ref() as &[u8]),
        "root_public_key": root_public,
        "strict": true,
        "public_key_type": args.public_key_type,
        "wif_type": if compressed { "wif-compressed" } else { "wif" },
        "derivation": {
            "at": derivation_at(&args.derivation, derivation.as_ref()),
            "xprivate_key": derived.to_string(),
            "xpublic_key": derived_pub.to_string(),
            "private_key": derived_private,
            "wif": derived_wif,
            "chain_code": hex::encode(derived.chain_code.as_ref() as &[u8]),
            "public_key": derived_public,
            "uncompressed": derived_uncompressed,
            "compressed": derived_public,
            "hash": hex::encode(pubkey_hash),
            "fingerprint": fingerprint,
            "parent_fingerprint": parent_fingerprint,
            "address": address,
        }
    });

    apply_dump_exclude(&mut dump, &args.exclude);
    Ok(dump)
}

pub(crate) fn build_dump_value(args: &DumpArgs) -> Result<Value, Error> {
    build_dump(args)
}

pub(crate) fn flatten_dump_row(dump: &Value, include: &str) -> Map<String, Value> {
    let mut row = Map::new();
    let derivation = dump.get("derivation");
    for key in include.split(',').filter(|item| !item.is_empty()) {
        let parts: Vec<&str> = key.split(':').collect();
        if parts.len() == 2 {
            if let Some(value) = derivation
                .and_then(|item| item.get(parts[0]))
                .and_then(|item| item.get(parts[1]))
            {
                row.insert(key.to_string(), value.clone());
            }
        } else if let Some(value) = derivation.and_then(|item| item.get(parts[0])) {
            row.insert(parts[0].to_string(), value.clone());
        }
    }
    row
}

fn load_dump_source(
    args: &DumpArgs,
) -> Result<(Option<String>, Option<usize>, Option<String>, String), Error> {
    if args.entropy.is_some() {
        if !Entropies::is_entropy(&args.entropy_client) {
            return Err(Error::new(
                "Wrong entropy client",
                None,
                Some(&Entropies::names().join(", ")),
                Some(&args.entropy_client),
            ));
        }
        return Err(Error::new("Dump from entropy is not implemented yet", None, None, None));
    }

    if let Some(mnemonic) = args.mnemonic.as_deref() {
        if !Mnemonics::is_mnemonic(&args.mnemonic_client) {
            return Err(Error::new(
                "Wrong mnemonic client",
                None,
                Some(&Mnemonics::names().join(", ")),
                Some(&args.mnemonic_client),
            ));
        }
        if args.mnemonic_client != "BIP39" {
            return Err(Error::new(
                "Dump from this mnemonic client is not implemented yet",
                None,
                Some("BIP39"),
                Some(&args.mnemonic_client),
            ));
        }
        let words: Vec<String> = mnemonic.split_whitespace().map(str::to_string).collect();
        let entropy_bytes = BIP39Mnemonic::decode(&words).map_err(|error| Error::new(&error.to_string(), None, None, None))?;
        let strength = entropy_bytes.len() * 8;
        let seed_hex = BIP39Seed::from_mnemonic_with_passphrase(mnemonic, args.passphrase.as_deref())
            .map_err(|error| Error::new(&error.to_string(), None, None, None))?;
        return Ok((
            Some(hex::encode(&entropy_bytes)),
            Some(strength),
            Some(mnemonic.to_string()),
            seed_hex,
        ));
    }

    if args.seed.is_some() {
        return Err(Error::new("Dump from seed is not implemented yet", None, None, None));
    }

    if args.xprivate_key.is_some()
        || args.xpublic_key.is_some()
        || args.private_key.is_some()
        || args.wif.is_some()
        || args.public_key.is_some()
        || args.spend_private_key.is_some()
        || (args.view_private_key.is_some() && args.spend_public_key.is_some())
    {
        return Err(Error::new(
            "Dump from the provided key material is not implemented yet",
            None,
            None,
            None,
        ));
    }

    Err(Error::new(
        "No master key source provided",
        None,
        Some("entropy, mnemonic, seed, xprivate-key, xpublic-key, private-key, wif, spend-private-key, or watch-only keys"),
        None,
    ))
}

fn build_dump_derivation(
    args: &DumpArgs,
    cryptocurrency: &CryptocurrencyDescriptor,
) -> Result<Box<dyn IDerivation>, Error> {
    let account = parse_dump_index(&args.account)?;
    let change = ChangeInput::Str(&args.change);
    let address = parse_dump_index(&args.address)?;

    let derivation: Box<dyn IDerivation> = match args.derivation.as_str() {
        "BIP44" => Box::new(
            Bip44Derivation::new(
                Some(IndexInput::Int(cryptocurrency.coin_type)),
                Some(IndexInput::Int(account)),
                Some(change),
                Some(IndexInput::Int(address)),
            )
            .map_err(|error| Error::new(&error.to_string(), None, None, None))?,
        ),
        "BIP49" => Box::new(
            Bip49Derivation::new(
                Some(IndexInput::Int(cryptocurrency.coin_type)),
                Some(IndexInput::Int(account)),
                Some(change),
                Some(IndexInput::Int(address)),
            )
            .map_err(|error| Error::new(&error.to_string(), None, None, None))?,
        ),
        "BIP84" => Box::new(
            Bip84Derivation::new(
                Some(IndexInput::Int(cryptocurrency.coin_type)),
                Some(IndexInput::Int(account)),
                Some(change),
                Some(IndexInput::Int(address)),
            )
            .map_err(|error| Error::new(&error.to_string(), None, None, None))?,
        ),
        "BIP86" => Box::new(
            Bip86Derivation::new(
                Some(IndexInput::Int(cryptocurrency.coin_type)),
                Some(IndexInput::Int(account)),
                Some(change),
                Some(IndexInput::Int(address)),
            )
            .map_err(|error| Error::new(&error.to_string(), None, None, None))?,
        ),
        other => {
            return Err(Error::new(
                "Dump for this derivation is not implemented yet",
                None,
                Some("BIP44, BIP49, BIP84, BIP86"),
                Some(other),
            ));
        }
    };

    if !matches!(
        args.hd.as_str(),
        "BIP32" | "BIP44" | "BIP49" | "BIP84" | "BIP86" | "BIP141"
    ) {
        return Err(Error::new(
            "Dump for this HD scheme is not implemented yet",
            None,
            Some("BIP32, BIP44, BIP49, BIP84, BIP86, BIP141"),
            Some(&args.hd),
        ));
    }

    Ok(derivation)
}

fn derivation_at(name: &str, derivation: &dyn IDerivation) -> Value {
    match name {
        "BIP44" | "BIP49" | "BIP84" | "BIP86" => json!({
            "path": derivation.path(),
            "indexes": derivation.indexes(),
            "depth": derivation.depth(),
            "purpose": derivation.purpose(),
            "coin_type": derivation.coin_type(),
            "account": derivation.account(),
            "change": match derivation.change() {
                Some(ChangeValue::Label(label)) => label,
                Some(ChangeValue::Index(0)) => "external-chain",
                Some(ChangeValue::Index(1)) => "internal-chain",
                Some(ChangeValue::Index(_)) => "external-chain",
                None => "external-chain",
            },
            "address": derivation.address(),
        }),
        _ => json!({
            "path": derivation.path(),
            "indexes": derivation.indexes(),
            "depth": derivation.depth(),
        }),
    }
}

fn encode_dump_address(
    args: &DumpArgs,
    cryptocurrency: &CryptocurrencyDescriptor,
    network: &NetworkDescriptor,
    semantic: Option<&str>,
    public_key_hex: &str,
    compressed: bool,
) -> Result<String, Error> {
    let kwargs = EncodeKwargs {
        public_key_type: Some(if compressed {
            PublicKeyTypes::COMPRESSED
        } else {
            PublicKeyTypes::UNCOMPRESSED
        }),
        network: Some(network.name),
        public_key_address_prefix: network.public_key_address_prefix,
        script_address_prefix: network.script_address_prefix,
        witness_version: network.witness_versions.map(|versions| versions.0[0]),
        alphabet: cryptocurrency.params.and_then(|params| params.alphabet),
        ..Default::default()
    };

    let public_key = EncodePublicKey::Str(public_key_hex);
    match args.hd.as_str() {
        "BIP49" => P2WPKHInP2SHAddress.encode(public_key, &kwargs),
        "BIP84" | "BIP141" if semantic == Some("p2wpkh") => P2WPKHAddress.encode(public_key, &kwargs),
        "BIP86" => P2TRAddress.encode(public_key, &kwargs),
        _ => P2PKHAddress.encode(public_key, &kwargs),
    }
    .map_err(|error| Error::new("Address encoding failed", Some(&error.to_string()), None, None))
}

fn select_wif(private_key_hex: &str, prefix: u8, compressed: bool) -> Result<String, Error> {
    let bytes = hex::decode(private_key_hex)
        .map_err(|error| Error::new("Invalid private key hex", Some(&error.to_string()), None, None))?;
    let (wif, wif_compressed) = encode_wif(&bytes, prefix)
        .map_err(|error| Error::new("WIF encoding failed", Some(&error.to_string()), None, None))?;
    Ok(if compressed { wif_compressed } else { wif })
}

fn map_bitcoin_network(name: &str) -> Network {
    match name.to_lowercase().as_str() {
        "testnet" => Network::Testnet,
        "regtest" => Network::Regtest,
        _ => Network::Bitcoin,
    }
}

fn parse_dump_index(value: &str) -> Result<u32, Error> {
    value
        .parse()
        .map_err(|_| Error::new("Invalid index value", None, None, Some(value)))
}

fn resolve_semantic(
    hd: &str,
    semantic: Option<&str>,
    cryptocurrency: &CryptocurrencyDescriptor,
) -> Option<String> {
    if let Some(value) = semantic {
        return Some(value.to_string());
    }
    match hd {
        "BIP32" | "BIP44" | "BIP86" | "Cardano" => cryptocurrency.default_semantic.map(str::to_string),
        "BIP49" => Some("p2wpkh-in-p2sh".to_string()),
        "BIP84" | "BIP141" => Some("p2wpkh".to_string()),
        _ => None,
    }
}

fn apply_dump_exclude(value: &mut Value, exclude: &str) {
    let keys: HashSet<&str> = exclude
        .split(',')
        .map(str::trim)
        .filter(|item| !item.is_empty())
        .collect();
    if keys.is_empty() {
        return;
    }
    if let Some(object) = value.as_object_mut() {
        object.retain(|key, _| !keys.contains(key.as_str()));
    }
}

fn display_ecc_name(name: &str) -> String {
    match name {
        "SLIP10Secp256k1ECC" => "SLIP10-Secp256k1".to_string(),
        "SLIP10Ed25519ECC" => "SLIP10-Ed25519".to_string(),
        "SLIP10Ed25519Blake2bECC" => "SLIP10-Ed25519-Blake2b".to_string(),
        "SLIP10Ed25519MoneroECC" => "SLIP10-Ed25519-Monero".to_string(),
        "SLIP10Nist256p1ECC" => "SLIP10-Nist256p1".to_string(),
        "KholawEd25519ECC" => "Kholaw-Ed25519".to_string(),
        other => other.to_string(),
    }
}
