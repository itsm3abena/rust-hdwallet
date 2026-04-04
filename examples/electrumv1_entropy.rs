//! Print random ElectrumV1-compatible entropy (hex). Usage: `cargo run --example electrumv1_entropy -- [128]`

use hdwallet_rust::hdwallet::entropies::{ElectrumV1Entropy, IEntropy};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bits: usize = std::env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(128);

    let hex = ElectrumV1Entropy.generate(bits)?;
    println!("{} bits Electrum V1 entropy (hex):\n{}", bits, hex);
    Ok(())
}
