//! Print random ElectrumV2-compatible entropy (hex). Usage: `cargo run --example electrumv2_entropy -- [132 | 264]`

use hdwallet_rust::hdwallet::entropies::{ElectrumV2Entropy, IEntropy};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bits: usize = std::env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(264);

    let hex = ElectrumV2Entropy.generate(bits)?;
    println!("{} bits Electrum V2 entropy (hex):\n{}", bits, hex);
    Ok(())
}
