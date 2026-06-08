use std::collections::HashSet;

use clap::Args;
use serde_json::Value;

use crate::hdwallet::cryptocurrencies::get_cryptocurrency;
use crate::hdwallet::derivations::Derivations;
use crate::hdwallet::entropies::Entropies;
use crate::hdwallet::exceptions::Error;
use crate::hdwallet::hdwallet::{build_derivation, HDWallet, HDWalletConfig};
use crate::hdwallet::hds::Hds;
use crate::hdwallet::mnemonics::Mnemonics;
use crate::hdwallet::seeds::Seeds;

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
        default_value = crate::hdwallet::consts::PublicKeyTypes::COMPRESSED,
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
        default_value = crate::hdwallet::seeds::CardanoTypes::SHELLEY_ICARUS,
        help = "Select Cardano type"
    )]
    pub cardano_type: String,

    #[arg(long = "address-type", help = "Select Address type")]
    pub address_type: Option<String>,

    #[arg(
        long = "mode",
        default_value = crate::hdwallet::consts::Modes::STANDARD,
        help = "Select Mode of Electrum-V2"
    )]
    pub mode: String,

    #[arg(
        long = "mnemonic-type",
        default_value = crate::hdwallet::mnemonics::ElectrumV2MnemonicTypes::STANDARD,
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
    let dump = build_dump_value(&args)?;
    serde_json::to_string_pretty(&dump)
        .map_err(|error| Error::new(&error.to_string(), None, None, None))
}

pub(crate) fn build_dump_value(args: &DumpArgs) -> Result<Value, Error> {
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

    let mut wallet = HDWallet::new(HDWalletConfig {
        symbol: args.symbol.clone(),
        hd: args.hd.clone(),
        network: args.network.clone(),
        language: args.language.clone(),
        passphrase: args.passphrase.clone(),
        public_key_type: args.public_key_type.clone(),
        cardano_type: args.cardano_type.clone(),
        address_type: args.address_type.clone(),
        mode: args.mode.clone(),
        mnemonic_type: args.mnemonic_type.clone(),
        checksum: args.checksum,
        payment_id: args.payment_id.clone(),
        staking_public_key: args.staking_public_key.clone(),
        semantic: args.semantic.clone(),
        strict: args.strict,
        encoded: args.encoded,
    })?;

    load_master_key(&mut wallet, args)?;

    if should_apply_derivation(args) {
        let derivation = build_derivation(
            &args.derivation,
            cryptocurrency,
            &args.account,
            &args.change,
            &args.address,
            &args.role,
            &args.ecc,
            &args.minor,
            &args.major,
            args.path.as_deref(),
            &args.indexes,
        )?;
        wallet.from_derivation(&args.derivation, derivation)?;
    }

    let exclude = parse_exclude(&args.exclude);
    wallet.dump(&exclude)
}

pub(crate) fn load_master_key(wallet: &mut HDWallet, args: &DumpArgs) -> Result<(), Error> {
    if let Some(entropy) = args.entropy.as_deref() {
        if !Entropies::is_entropy(&args.entropy_client) {
            return Err(Error::new(
                "Wrong entropy client",
                None,
                Some(&Entropies::names().join(", ")),
                Some(&args.entropy_client),
            ));
        }
        return wallet.from_entropy(entropy, &args.entropy_client);
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
        return wallet.from_mnemonic(mnemonic, &args.mnemonic_client);
    }

    if let Some(seed) = args.seed.as_deref() {
        if !Seeds::is_seed(&args.seed_client) {
            return Err(Error::new(
                "Wrong seed client",
                None,
                Some(&Seeds::names().join(", ")),
                Some(&args.seed_client),
            ));
        }
        return wallet.from_seed_hex(seed);
    }

    if let Some(xprivate_key) = args.xprivate_key.as_deref() {
        return wallet.from_xprivate_key(xprivate_key, args.encoded, args.strict);
    }

    if let Some(xpublic_key) = args.xpublic_key.as_deref() {
        return wallet.from_xpublic_key(xpublic_key, args.encoded, args.strict);
    }

    if let Some(private_key) = args.private_key.as_deref() {
        return wallet.from_private_key(private_key);
    }

    if let Some(wif) = args.wif.as_deref() {
        if args.bip38 {
            use bip38::Decrypt;
            let passphrase = args.passphrase.as_deref().unwrap_or("");
            let decrypted = wif.decrypt_to_wif(passphrase).map_err(|error| {
                Error::new("BIP38 decryption failed", Some(&error.to_string()), None, None)
            })?;
            return wallet.from_wif(&decrypted);
        }
        return wallet.from_wif(wif);
    }

    if let Some(public_key) = args.public_key.as_deref() {
        return wallet.from_public_key(public_key);
    }

    if let Some(spend_private_key) = args.spend_private_key.as_deref() {
        return wallet.from_spend_private_key(spend_private_key);
    }

    if args.view_private_key.is_some() && args.spend_public_key.is_some() {
        return wallet.from_watch_only(
            args.view_private_key.as_deref().unwrap_or_default(),
            args.spend_public_key.as_deref().unwrap_or_default(),
        );
    }

    Err(Error::new(
        "No master key source provided",
        None,
        Some("entropy, mnemonic, seed, xprivate-key, xpublic-key, private-key, wif, spend-private-key, or watch-only keys"),
        None,
    ))
}

fn should_apply_derivation(args: &DumpArgs) -> bool {
    args.entropy.is_some()
        || args.mnemonic.is_some()
        || args.seed.is_some()
        || args.xprivate_key.is_some()
        || args.xpublic_key.is_some()
        || (args.private_key.is_some()
            && matches!(args.hd.as_str(), "Electrum-V1" | "Monero"))
        || (args.wif.is_some() && args.hd == "Electrum-V1")
        || args.spend_private_key.is_some()
        || (args.view_private_key.is_some() && args.spend_public_key.is_some())
}

pub(crate) fn parse_exclude(exclude: &str) -> HashSet<String> {
    exclude
        .split(',')
        .map(str::trim)
        .filter(|item| !item.is_empty())
        .map(str::to_string)
        .collect()
}
