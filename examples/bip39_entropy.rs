//! Print random BIP39-compatible entropy (hex). Usage: `cargo run --example bip39_entropy -- [128|160|192|224|256]`

use hdwallet_rust::hdwallet::entropies::{BIP39Entropy, IEntropy};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bits: usize = std::env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(128);

    let hex = BIP39Entropy.generate(bits)?;
    println!("{} bits BIP39 entropy (hex):\n{}", bits, hex);
    Ok(())
}
