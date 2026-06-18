# Hierarchical Deterministic (HD) Wallet — Rust

[![License: MIT](https://img.shields.io/badge/License-MIT-black.svg)](LICENSE)
[![Rust Version](https://img.shields.io/badge/rust-1.85%2B-orange.svg)](https://www.rust-lang.org)

Rust implementation of a Hierarchical Deterministic (HD) Wallet generator for **200+ cryptocurrencies**.

> **Beta notice:** This project is in active beta testing. APIs, CLI flags, and behavior may change between releases. If you find bugs or compatibility issues, please [open an issue](https://github.com/itsm3abena/rust-hdwallet/issues) or help us fix them — see [CONTRIBUTING.md](CONTRIBUTING.md).

This library is a Rust port inspired by the excellent [python-hdwallet](https://github.com/hdwallet-io/python-hdwallet) project. It offers a flexible and scalable solution for developers integrating multi-currency wallet functionality: secure seed generation, robust key management, and streamlined account control across many blockchains.

| Features | Protocols |
|:------------------------------|:----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Cryptocurrencies | 200+ (see `hdwallet list cryptocurrencies`) |
| Entropies | `Algorand`, `BIP39`, `Electrum-V1`, `Electrum-V2`, `Monero` |
| Mnemonics | `Algorand`, `BIP39`, `Electrum-V1`, `Electrum-V2`, `Monero` |
| Seeds | `Algorand`, `BIP39`, `Cardano`, `Electrum-V1`, `Electrum-V2`, `Monero` |
| Elliptic Curve Cryptography | `Kholaw-Ed25519`, `SLIP10-Ed25519`, `SLIP10-Ed25519-Blake2b`, `SLIP10-Ed25519-Monero`, `SLIP10-Nist256p1`, `SLIP10-Secp256k1` |
| Hierarchical Deterministic | `Algorand`, `BIP32`, `BIP44`, `BIP49`, `BIP84`, `BIP86`, `BIP141`, `Cardano`, `Electrum-V1`, `Electrum-V2`, `Monero` |
| Derivations | `BIP44`, `BIP49`, `BIP84`, `BIP86`, `CIP1852`, `Custom`, `Electrum`, `Monero`, `HDW` |
| Addresses | `Algorand`, `Aptos`, `Avalanche`, `Cardano`, `Cosmos`, `EOS`, `Ergo`, `Ethereum`, `Filecoin`, `Harmony`, `Icon`, `Injective`, `Monero`, `MultiversX`, `Nano`, `Near`, `Neo`, `OKT-Chain`, `P2PKH`, `P2SH`, `P2TR`, `P2WPKH`, `P2WPKH-In-P2SH`, `P2WSH`, `P2WSH-In-P2SH`, `Ripple`, `Solana`, `Stellar`, `Sui`, `Tezos`, `Tron`, `XinFin`, `Zilliqa` |
| Others | `BIP38`, `Wallet Import Format`, `Serialization` |

## Installation

Add the library to your `Cargo.toml`:

```toml
[dependencies]
rust-hdwallet = "0.1.0-beta"
```

Or install from GitHub:

```toml
[dependencies]
rust-hdwallet = { git = "https://github.com/itsm3abena/rust-hdwallet" }
```

Or clone and build locally:

```bash
git clone https://github.com/itsm3abena/rust-hdwallet.git
cd rust-hdwallet
cargo build --release
```

The CLI binary is named `hdwallet`:

```bash
cargo install rust-hdwallet --version 0.1.0-beta
# or from a local clone:
cargo install --path .
# or
cargo run --bin hdwallet -- --help
```

**Requirements:** Rust 1.85 or later.

## Quick Usage

### Library Example

A simple Bitcoin HD wallet generator:

```rust
use std::collections::HashSet;

use rust_hdwallet::hdwallet::consts::PublicKeyTypes;
use rust_hdwallet::hdwallet::derivations::CustomDerivation;
use rust_hdwallet::hdwallet::hdwallet::{HDWallet, HDWalletConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut wallet = HDWallet::new(HDWalletConfig {
        symbol: "BTC".into(),
        hd: "BIP32".into(),
        network: "mainnet".into(),
        language: "english".into(),
        passphrase: Some("talonlab".into()),
        public_key_type: PublicKeyTypes::COMPRESSED.into(),
        cardano_type: "shelley-icarus".into(),
        address_type: None,
        mode: "standard".into(),
        mnemonic_type: "standard".into(),
        checksum: false,
        payment_id: None,
        staking_public_key: None,
        semantic: None,
        strict: true,
        encoded: true,
    })?;

    wallet.from_entropy("00000000000000000000000000000000", "BIP39")?;

    wallet.from_derivation(
        "Custom",
        Box::new(CustomDerivation::new(Some("m/0'/0/0-1"), None)?),
    )?;

    let dump = wallet.dumps(&HashSet::new())?;
    println!("{}", serde_json::to_string_pretty(&dump)?);

    Ok(())
}
```

### Command Line Interface (CLI)

The `hdwallet` CLI lets you generate wallets, derive addresses, and export keys from the terminal in JSON or CSV format.

**Full CLI guide with copy-paste test examples:** [docs/CLI.md](docs/CLI.md)

List supported cryptocurrencies:

```bash
hdwallet list cryptocurrencies
```

Generate a BIP39 mnemonic:

```bash
hdwallet generate mnemonic --client BIP39 --words 12
```

Dump wallet keys from a mnemonic (Bitcoin, BIP44):

```bash
hdwallet dump \
  --symbol BTC \
  --hd BIP44 \
  --mnemonic "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about"
```

Export multiple derived addresses as JSON:

```bash
hdwallet dumps \
  --symbol ETH \
  --hd BIP44 \
  --mnemonic "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about" \
  --format json \
  --path "m/44'/60'/0'/0/0-5"
```

Run `hdwallet --help` or `hdwallet dump --help` for the full option list.

## Development

Fork the repository, clone it locally, and build:

```bash
git clone https://github.com/itsm3abena/rust-hdwallet.git
cd rust-hdwallet
cargo build
```

## Testing

Run the full test suite:

```bash
cargo test
```

Run a specific test module:

```bash
cargo test --test hdwallet_hdwallet_bip44
```

## Contributing

We welcome bug reports, feature requests, documentation improvements, and pull requests — especially during this beta phase.

- Open an [issue](https://github.com/itsm3abena/rust-hdwallet/issues) to report bugs or discuss changes before large PRs.
- See [CONTRIBUTING.md](CONTRIBUTING.md) for setup steps, coding guidelines, and the pull request workflow.
- Help with testing against real wallets, adding cryptocurrency coverage, and improving docs is highly appreciated.

## Related Projects

- [python-hdwallet](https://github.com/hdwallet-io/python-hdwallet) — Original Python implementation
- [hdwallet.js](https://github.com/hdwallet-io/hdwallet.js) — TypeScript/JavaScript port

## License

Distributed under the [MIT](LICENSE) license.
