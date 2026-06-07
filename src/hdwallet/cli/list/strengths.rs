use crate::hdwallet::entropies::Entropies;
use crate::hdwallet::exceptions::Error;

pub fn list_strengths() -> Result<String, Error> {
    let clients = ["Algorand", "BIP39", "Electrum-V1", "Electrum-V2", "Monero"];
    let mut blocks = Vec::new();

    for (index, client) in clients.iter().enumerate() {
        let scheme = Entropies::entropy(client).map_err(|error| {
            Error::new(&error.to_string(), None, None, None)
        })?;
        let strengths = scheme
            .strengths()
            .iter()
            .map(|value| value.to_string())
            .collect::<Vec<_>>();
        blocks.push(tabulate_single_column(
            &format!("{} Strengths", scheme.name()),
            &strengths,
        ));
        if index + 1 != clients.len() {
            blocks.push(String::new());
        }
    }

    Ok(blocks.join("\n\n"))
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
