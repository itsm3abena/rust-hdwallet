//! Print random Monero-compatible entropy (hex). Usage: `cargo run --example monero_entropy -- [ 128 | 256]`

use hdwallet_rust::hdwallet::entropies::{MoneroEntropy, IEntropy};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bits: usize = std::env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(256);

    let hex = MoneroEntropy.generate(bits)?;
    println!("{} bits Monero entropy (hex):\n{}", bits, hex);
    Ok(())
}
