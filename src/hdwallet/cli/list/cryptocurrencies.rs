use crate::hdwallet::cryptocurrencies::icryptocurrency::INetwork;
use crate::hdwallet::cryptocurrencies::Cryptocurrencies;
use crate::hdwallet::exceptions::Error;

pub fn list_cryptocurrencies() -> Result<String, Error> {
    let headers = [
        "Cryptocurrency",
        "Symbol",
        "Coin Type",
        "Networks",
        "ECC",
    ];
    let alignments = [
        Alignment::Left,
        Alignment::Center,
        Alignment::Center,
        Alignment::Center,
        Alignment::Center,
    ];

    let rows = Cryptocurrencies::classes()
        .iter()
        .map(|descriptor| {
            let networks = descriptor
                .networks
                .iter()
                .map(|(_, network)| network.name())
                .collect::<Vec<_>>()
                .join(", ");
            vec![
                descriptor.name.to_string(),
                descriptor.symbol.to_string(),
                descriptor.coin_type.to_string(),
                networks,
                descriptor.ecc.to_string(),
            ]
        })
        .collect::<Vec<_>>();

    Ok(tabulate(&headers, &alignments, &rows))
}

#[derive(Clone, Copy)]
enum Alignment {
    Left,
    Center,
}

fn tabulate(headers: &[&str], alignments: &[Alignment], rows: &[Vec<String>]) -> String {
    let mut widths = headers.iter().map(|header| header.len()).collect::<Vec<_>>();
    for row in rows {
        for (index, cell) in row.iter().enumerate() {
            widths[index] = widths[index].max(cell.len());
        }
    }

    let mut output = format_row(headers, &widths, alignments);
    output.push('\n');
    for row in rows {
        let cells = row.iter().map(String::as_str).collect::<Vec<_>>();
        output.push_str(&format_row(&cells, &widths, alignments));
        output.push('\n');
    }
    output.trim_end().to_string()
}

fn format_row(cells: &[&str], widths: &[usize], alignments: &[Alignment]) -> String {
    cells
        .iter()
        .enumerate()
        .map(|(index, cell)| align_cell(cell, widths[index], alignments[index]))
        .collect::<Vec<_>>()
        .join("  ")
}

fn align_cell(text: &str, width: usize, alignment: Alignment) -> String {
    if text.len() >= width {
        return text.to_string();
    }
    let padding = width - text.len();
    match alignment {
        Alignment::Left => format!("{text}{}", " ".repeat(padding)),
        Alignment::Center => {
            let left = padding / 2;
            let right = padding - left;
            format!("{}{text}{}", " ".repeat(left), " ".repeat(right))
        }
    }
}
