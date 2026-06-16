use std::collections::HashSet;

use clap::Args;
use serde_json::Value;

use crate::hdwallet::cryptocurrencies::icryptocurrency::CryptocurrencyDescriptor;
use crate::hdwallet::cryptocurrencies::get_cryptocurrency;
use crate::hdwallet::derivations::DerivationStep;
use crate::hdwallet::exceptions::Error;
use crate::hdwallet::hdwallet::{
    build_derivation, default_include_for_hd, flatten_dump_row, HDWallet, HDWalletConfig,
};
use crate::hdwallet::wallet_dumps::{dumps_json_output, rebuild_derivation};

use super::dump::{load_master_key, parse_exclude, should_apply_derivation, DumpArgs};

#[derive(Args, Clone, Debug)]
pub struct DumpsArgs {
    #[command(flatten)]
    pub dump: DumpArgs,

    #[arg(long = "format", default_value = "csv", help = "Show dumps format type")]
    pub format: String,

    #[arg(long = "include", help = "Set Include keys from dumped")]
    pub include: Option<String>,

    #[arg(long = "include-header", help = "Set Include header from dumped")]
    pub include_header: bool,

    #[arg(long = "delimiter", default_value = " ", help = "Set Delimiter for CSV")]
    pub delimiter: String,
}

pub fn dumps(args: DumpsArgs) -> Result<String, Error> {
    match args.format.as_str() {
        "csv" => dumps_csv(&args),
        "json" => dumps_json(&args),
        other => Err(Error::new(
            "Wrong format",
            None,
            Some("json | csv"),
            Some(other),
        )),
    }
}

fn dumps_csv(args: &DumpsArgs) -> Result<String, Error> {
    let cryptocurrency = get_cryptocurrency(&args.dump.symbol)?;
    let mut wallet = build_wallet(&args.dump)?;
    let include = args
        .include
        .clone()
        .unwrap_or_else(|| default_include_for_hd(&args.dump.hd, cryptocurrency));
    let fields: Vec<&str> = include.split(',').filter(|item| !item.is_empty()).collect();

    let derivation = build_derivation(
        &args.dump.derivation,
        cryptocurrency,
        &args.dump.account,
        &args.dump.change,
        &args.dump.address,
        &args.dump.role,
        &args.dump.ecc,
        &args.dump.minor,
        &args.dump.major,
        args.dump.path.as_deref(),
        &args.dump.indexes,
    )?;

    let mut lines = Vec::new();
    if args.include_header {
        lines.push(fields.join(&args.delimiter));
    }

    let template_path = derivation.path().to_string();
    let template_indexes = derivation.indexes().to_vec();
    let steps = derivation.derivations().to_vec();

    drive_derivations(
        &mut wallet,
        &args.dump.derivation,
        cryptocurrency,
        &template_path,
        &template_indexes,
        &steps,
        Vec::new(),
        &include,
        &args.delimiter,
        &mut lines,
    )?;

    Ok(lines.join("\n"))
}

fn dumps_json(args: &DumpsArgs) -> Result<String, Error> {
    if !should_apply_derivation(&args.dump) {
        return Ok(String::new());
    }

    let cryptocurrency = get_cryptocurrency(&args.dump.symbol)?;
    let mut wallet = build_wallet(&args.dump)?;
    let exclude = parse_exclude(&args.dump.exclude);

    let derivation = build_derivation(
        &args.dump.derivation,
        cryptocurrency,
        &args.dump.account,
        &args.dump.change,
        &args.dump.address,
        &args.dump.role,
        &args.dump.ecc,
        &args.dump.minor,
        &args.dump.major,
        args.dump.path.as_deref(),
        &args.dump.indexes,
    )?;
    wallet.from_derivation(&args.dump.derivation, derivation)?;

    let dump = wallet
        .dumps(&exclude)?
        .ok_or_else(|| Error::new("Derivation is not set", None, None, None))?;
    dumps_json_output(&dump)
}

fn build_wallet(args: &DumpArgs) -> Result<HDWallet, Error> {
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
    Ok(wallet)
}

fn drive_derivations(
    wallet: &mut HDWallet,
    derivation_name: &str,
    cryptocurrency: &CryptocurrencyDescriptor,
    template_path: &str,
    template_indexes: &[u32],
    steps: &[DerivationStep],
    current: Vec<(u32, bool)>,
    include: &str,
    delimiter: &str,
    lines: &mut Vec<String>,
) -> Result<(), Error> {
    if steps.is_empty() {
        let derivation = rebuild_derivation(
            derivation_name,
            cryptocurrency,
            template_path,
            template_indexes,
            &current,
        )?;
        wallet.update_derivation(derivation_name, derivation)?;

        let mut exclude = HashSet::new();
        exclude.insert("root".to_string());
        let dump = wallet.dump(&exclude)?;
        let row = flatten_dump_row(&dump, include);
        let fields: Vec<&str> = include.split(',').filter(|item| !item.is_empty()).collect();
        let values = fields
            .iter()
            .map(|field| value_to_csv_cell(row.get(*field).unwrap_or(&Value::Null)))
            .collect::<Vec<_>>()
            .join(delimiter);
        lines.push(values);
        return Ok(());
    }

    match steps[0] {
        DerivationStep::Range {
            from,
            to,
            hardened,
        } => {
            for value in from..=to {
                let mut next = current.clone();
                next.push((value, hardened));
                drive_derivations(
                    wallet,
                    derivation_name,
                    cryptocurrency,
                    template_path,
                    template_indexes,
                    &steps[1..],
                    next,
                    include,
                    delimiter,
                    lines,
                )?;
            }
        }
        DerivationStep::Index { index, hardened } => {
            let mut next = current.clone();
            next.push((index, hardened));
            drive_derivations(
                wallet,
                derivation_name,
                cryptocurrency,
                template_path,
                template_indexes,
                &steps[1..],
                next,
                include,
                delimiter,
                lines,
            )?;
        }
    }
    Ok(())
}

fn value_to_csv_cell(value: &Value) -> String {
    match value {
        Value::Null => String::new(),
        Value::String(text) => text.clone(),
        Value::Number(number) => number.to_string(),
        Value::Bool(flag) => flag.to_string(),
        other => other.to_string(),
    }
}
