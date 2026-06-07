use clap::Args;
use serde_json::Value;

use crate::hdwallet::exceptions::Error;

use super::dump::{build_dump_value, flatten_dump_row, DumpArgs};

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
    let dump = build_dump_value(&args.dump)?;
    let include = args
        .include
        .clone()
        .unwrap_or_else(|| default_include_for_hd(&args.dump.hd));
    let fields: Vec<&str> = include.split(',').filter(|item| !item.is_empty()).collect();
    let row = flatten_dump_row(&dump, &include);

    let mut lines = Vec::new();
    if args.include_header {
        lines.push(fields.join(&args.delimiter));
    }

    let values = fields
        .iter()
        .map(|field| value_to_csv_cell(row.get(*field).unwrap_or(&Value::Null)))
        .collect::<Vec<_>>()
        .join(&args.delimiter);
    lines.push(values);
    Ok(lines.join("\n"))
}

fn dumps_json(args: &DumpsArgs) -> Result<String, Error> {
    let dump = build_dump_value(&args.dump)?;
    serde_json::to_string_pretty(&dump).map_err(|error| Error::new(&error.to_string(), None, None, None))
}

fn default_include_for_hd(hd: &str) -> String {
    match hd {
        "BIP32" => "at:path,addresses:p2pkh,public_key,wif".to_string(),
        "BIP44" | "BIP49" | "BIP84" | "BIP86" | "BIP141" => {
            "at:path,address,public_key,wif".to_string()
        }
        _ => "at:path,address,public_key,wif".to_string(),
    }
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
