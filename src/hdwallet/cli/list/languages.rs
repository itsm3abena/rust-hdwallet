use crate::hdwallet::exceptions::Error;
use crate::hdwallet::mnemonics::{
    AlgorandMnemonic, BIP39Mnemonic, ElectrumV1Mnemonic, ElectrumV2Mnemonic, IMnemonic,
    MoneroMnemonic,
};

pub fn list_languages() -> Result<String, Error> {
    let clients: [&str; 5] = [
        AlgorandMnemonic::name(),
        BIP39Mnemonic::name(),
        ElectrumV1Mnemonic::name(),
        ElectrumV2Mnemonic::name(),
        MoneroMnemonic::name(),
    ];

    let mut blocks = Vec::new();
    for (index, client) in clients.iter().enumerate() {
        let languages = languages_for(client)
            .into_iter()
            .map(format_language_label)
            .collect::<Vec<_>>();
        blocks.push(tabulate_single_column(
            &format!("{client} Languages"),
            &languages,
        ));
        if index + 1 != clients.len() {
            blocks.push(String::new());
        }
    }

    Ok(blocks.join("\n\n"))
}

fn languages_for(client: &str) -> Vec<&'static str> {
    match client {
        "Algorand" => AlgorandMnemonic::languages().to_vec(),
        "BIP39" => BIP39Mnemonic::languages().to_vec(),
        "Electrum-V1" => ElectrumV1Mnemonic::languages().to_vec(),
        "Electrum-V2" => ElectrumV2Mnemonic::languages().to_vec(),
        "Monero" => MoneroMnemonic::languages().to_vec(),
        _ => Vec::new(),
    }
}

fn format_language_label(language: &str) -> String {
    language
        .split('-')
        .enumerate()
        .map(|(index, part)| {
            let mut chars = part.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => {
                    let mut value = first.to_uppercase().collect::<String>();
                    value.push_str(&chars.as_str().to_lowercase());
                    if index == 0 {
                        value
                    } else {
                        format!("-{value}")
                    }
                }
            }
        })
        .collect::<String>()
}

fn tabulate_single_column(title: &str, values: &[String]) -> String {
    let width = values
        .iter()
        .map(|value| value.len())
        .max()
        .unwrap_or(0)
        .max(title.len());
    let mut output = format!("{title}\n{}\n", "-".repeat(width));
    for value in values {
        output.push_str(value);
        output.push('\n');
    }
    output.trim_end().to_string()
}
