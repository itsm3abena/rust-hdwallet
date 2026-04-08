//! Random entropy (hex) demos. Run: `cargo run --example entropies -- <scheme> [bits]`
//!
//! Schemes: `bip39`, `algorand`, `monero`, `electrum-v1`, `electrum-v2`

use hdwallet_rust::hdwallet::entropies::electrum::{ElectrumV1Entropy, ElectrumV2Entropy};
use hdwallet_rust::hdwallet::entropies::{
    AlgorandEntropy, BIP39Entropy, MoneroEntropy, IEntropy,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1);
    let scheme = args.next().unwrap_or_else(|| "bip39".to_string());
    let bits: Option<usize> = args.next().and_then(|s| s.parse().ok());

    match scheme.as_str() {
        "bip39" => {
            let b = bits.unwrap_or(128);
            let hex = BIP39Entropy.generate(b)?;
            println!("{} bits BIP39 entropy (hex):\n{}", b, hex);
        }
        "algorand" => {
            let b = bits.unwrap_or(256);
            let hex = AlgorandEntropy.generate(b)?;
            println!("{} bits Algorand entropy (hex):\n{}", b, hex);
        }
        "monero" => {
            let b = bits.unwrap_or(256);
            let hex = MoneroEntropy.generate(b)?;
            println!("{} bits Monero entropy (hex):\n{}", b, hex);
        }
        "electrum-v1" | "electrumv1" => {
            let b = bits.unwrap_or(128);
            let hex = ElectrumV1Entropy.generate(b)?;
            println!("{} bits Electrum V1 entropy (hex):\n{}", b, hex);
        }
        "electrum-v2" | "electrumv2" => {
            let b = bits.unwrap_or(264);
            let hex = ElectrumV2Entropy.generate(b)?;
            println!("{} bits Electrum V2 entropy (hex):\n{}", b, hex);
        }
        _ => {
            eprintln!(
                "Unknown scheme {:?}. Use: bip39 | algorand | monero | electrum-v1 | electrum-v2",
                scheme
            );
            std::process::exit(1);
        }
    }

    Ok(())
}
