//! Print random Algorand-compatible entropy (hex). Usage: `cargo run --example algorand_entropy -- [256]`

use hdwallet_rust::hdwallet::entropies::{AlgorandEntropy, IEntropy};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bits: usize = std::env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(256);

    let hex = AlgorandEntropy.generate(bits)?;
    println!("{} bits Algorand entropy (hex):\n{}", bits, hex);
    Ok(())
}
