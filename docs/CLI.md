# HDWallet CLI Guide

Command-line interface for **rust-hdwallet** (`0.1.0-beta`). Use this guide to install the CLI and run copy-paste examples against known test vectors.

> **Beta:** Behavior and flags may change. If something fails, [open an issue](https://github.com/itsm3abena/rust-hdwallet/issues) with the exact command and output.

## Install

**From crates.io (recommended):**

```bash
cargo install rust-hdwallet --version 0.1.0-beta
hdwallet --version
```

**From source:**

```bash
git clone https://github.com/itsm3abena/rust-hdwallet.git
cd rust-hdwallet
cargo build --release
```

Run without installing:

```bash
cargo run --bin hdwallet -- --help
```

Install globally from a local clone:

```bash
cargo install --path .
hdwallet --version
```

On Windows (PowerShell), use the same commands. For a one-off run:

```powershell
cargo run --bin hdwallet -- list cryptocurrencies
```

## Quick smoke test

Run these in order. All use the public BIP39 test mnemonic — **never use real funds with example phrases.**

```bash
# 1. Version
hdwallet --version

# 2. List coins
hdwallet list cryptocurrencies

# 3. Generate entropy
hdwallet generate entropy --client BIP39 --strength 128

# 4. Generate mnemonic
hdwallet generate mnemonic --client BIP39 --words 12

# 5. Dump Bitcoin address (BIP44)
hdwallet dump --symbol BTC --hd BIP44 --mnemonic "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about" --address 0

# 6. Dump multiple Ethereum addresses (JSON)
hdwallet dumps --symbol ETH --hd BIP44 --format json --path "m/44'/60'/0'/0/0-2" --mnemonic "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about"
```

Expected Bitcoin address at `m/44'/0'/0'/0/0`:

```
1LqBGSKuX5yYUonjxT5qGfpUsXKYYWeabA
```

Expected first Ethereum address at `m/44'/60'/0'/0/0`:

```
0x9858EfFD232B4033E47d90003D41EC34EcaEda94
```

## Command overview

| Command | Alias | Description |
|:--------|:------|:------------|
| `hdwallet list cryptocurrencies` | `l c` | All supported coins |
| `hdwallet list languages` | `l l` | Mnemonic languages |
| `hdwallet list strengths` | `l s` | Entropy strengths |
| `hdwallet generate entropy` | `g e` | Random entropy (JSON) |
| `hdwallet generate mnemonic` | `g m` | Random or from-entropy mnemonic (JSON) |
| `hdwallet generate seed` | `g s` | Seed from mnemonic (JSON) |
| `hdwallet dump` | `d` | Single wallet dump (JSON) |
| `hdwallet dumps` | `ds` | Multiple derivations (CSV or JSON) |

Get help for any command:

```bash
hdwallet dump --help
hdwallet dumps --help
hdwallet generate mnemonic --help
```

---

## `list` — discover options

### Cryptocurrencies

```bash
hdwallet list cryptocurrencies
```

Shows name, symbol, coin type, networks, and ECC for each coin.

### Languages

```bash
hdwallet list languages
```

### Entropy strengths

```bash
hdwallet list strengths
```

---

## `generate` — entropy, mnemonics, seeds

All `generate` subcommands print **JSON** to stdout.

### Entropy

```bash
# BIP39, 128-bit (default strength for BIP39)
hdwallet generate entropy --client BIP39 --strength 128

# Other clients: Algorand, Electrum-V1, Electrum-V2, Monero
hdwallet generate entropy --client Electrum-V2 --strength 132
```

Example output:

```json
{"client":"BIP39","entropy":"c3b6e0e54e77bc0b3d2d0d8aea515e69","strength":128}
```

### Mnemonic

Random 12-word BIP39 mnemonic:

```bash
hdwallet generate mnemonic --client BIP39 --words 12 --language english
```

From fixed entropy (deterministic):

```bash
hdwallet generate mnemonic --client BIP39 --entropy 00000000000000000000000000000000 --language english
```

Expected mnemonic for zero entropy:

```
abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about
```

### Seed

```bash
hdwallet generate seed --client BIP39 --mnemonic "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about"
```

With passphrase:

```bash
hdwallet generate seed --client BIP39 --mnemonic "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about" --passphrase talonlab
```

Cardano seed:

```bash
hdwallet generate seed --client Cardano --cardano-type shelley-icarus --mnemonic "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about"
```

---

## `dump` — single wallet JSON

`dump` returns one JSON object with root keys and a single derived address (by default `m/44'/…/0'/0/0` for BIP44).

### From mnemonic (Bitcoin legacy, BIP44)

```bash
hdwallet dump --symbol BTC --hd BIP44 --mnemonic "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about" --address 0
```

### From mnemonic (Ethereum)

```bash
hdwallet dump --symbol ETH --hd BIP44 --mnemonic "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about" --address 0
```

### From entropy

```bash
hdwallet dump --symbol BTC --hd BIP32 --entropy 00000000000000000000000000000000 --derivation Custom --path "m/0'/0/0"
```

### From seed hex

```bash
hdwallet dump --symbol BTC --hd BIP44 --seed 5eb00bbddcf069084889a8ab9155568165f5c453ccb85e70811aaed6f6da5fc19a5ac40b389cd370d086206dec8aa6c43daea6690f20ad3d8d48b2d2ce9e38e4 --address 0
```

### From xprivate key

```bash
hdwallet dump --symbol BTC --hd BIP44 --xprivate-key "xprv9s21ZrQH143K3GJpoapnV8SFfukcVBSfeCficPSGfubmSFDxo1kuHnLisriDvSnRRuL2Qrg5ggqHKNVpxR86QEC8w35uxmGoggxtQTPvfUu" --address 0
```

### Exclude fields from output

Comma-separated field names:

```bash
hdwallet dump --symbol BTC --hd BIP44 --mnemonic "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about" --exclude "seed,mnemonic,entropy"
```

### Common flags

| Flag | Default | Description |
|:-----|:--------|:------------|
| `--symbol` | `BTC` | Ticker (e.g. `ETH`, `SOL`, `ADA`) |
| `--hd` | `BIP44` | HD standard (`BIP32`, `BIP49`, `BIP84`, `BIP86`, `Cardano`, …) |
| `--network` | `mainnet` | `mainnet` or `testnet` |
| `--derivation` | `BIP44` | Derivation scheme (`Custom`, `Electrum`, `Monero`, …) |
| `--path` | — | Custom path (with `Custom` derivation) |
| `--account` | `0` | BIP44 account index |
| `--change` | `0` | `0` = external, `1` = internal |
| `--address` | `0` | Address index |
| `--language` | `english` | Mnemonic language |
| `--passphrase` | — | BIP39 passphrase |
| `--public-key-type` | `compressed` | `compressed` or `uncompressed` |

You must provide **one** master key source: `--entropy`, `--mnemonic`, `--seed`, `--xprivate-key`, `--xpublic-key`, `--private-key`, `--wif`, or Monero-specific keys.

---

## `dumps` — multiple addresses

`dumps` derives a **range** of paths and prints either **CSV** (default) or **JSON**.

### CSV with header (Bitcoin)

```bash
hdwallet dumps --symbol BTC --hd BIP44 --include-header --path "m/44'/0'/0'/0/0-3" --mnemonic "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about"
```

Example output:

```
at:path address public_key private_key
m/44'/0'/0'/0/0 1LqBGSKuX5yYUonjxT5qGfpUsXKYYWeabA 03aaeb52dd7494c361049de67cc680e83ebcbbbdbeb13637d92cd845f70308af5e e284129cc0922579a535bbf4d1a3b25773090d28c909bc0fed73b5e0222cc372
...
```

### JSON (Ethereum, MetaMask-style path)

```bash
hdwallet dumps --symbol ETH --hd BIP44 --format json --path "m/44'/60'/0'/0/0-5" --mnemonic "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about"
```

### Custom columns (CSV)

```bash
hdwallet dumps --symbol BTC --hd BIP84 --path "m/84'/0'/0'/0/0-2" --include "at:path,address,wif" --include-header --mnemonic "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about"
```

### Path range syntax

| Pattern | Meaning |
|:--------|:--------|
| `m/44'/0'/0'/0/0` | Single address |
| `m/44'/0'/0'/0/0-5` | Addresses 0 through 5 |
| `m/0'/0/0-1` | Custom BIP32 path (use `--derivation Custom`) |

### `dumps`-only flags

| Flag | Default | Description |
|:-----|:--------|:------------|
| `--format` | `csv` | `csv` or `json` |
| `--include` | (per HD) | CSV columns, comma-separated |
| `--include-header` | off | Print CSV header row |
| `--delimiter` | space | CSV column separator |

---

## Recipes

### MetaMask / Hardhat style (Ethereum)

```bash
hdwallet dumps --symbol ETH --hd BIP44 --format json --path "m/44'/60'/0'/0/0-10" --mnemonic "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about"
```

### Bitcoin SegWit native (BIP84)

```bash
hdwallet dump --symbol BTC --hd BIP84 --mnemonic "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about" --address 0
```

### Solana (custom derivation path)

```bash
hdwallet dump --symbol SOL --hd BIP44 --derivation Custom --path "m/44'/501'/0'/0'" --mnemonic "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about"
```

### Nested SegWit (BIP49)

```bash
hdwallet dump --symbol BTC --hd BIP49 --mnemonic "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about" --address 0
```

---

## Running the CLI test suite

The project includes integration tests that exercise every CLI code path against JSON fixtures:

```bash
cargo test --test cli
```

To run all tests:

```bash
cargo test
```

---

## Troubleshooting

| Problem | What to try |
|:--------|:------------|
| `No master key source provided` | Add `--mnemonic`, `--entropy`, `--seed`, or another key flag |
| `Wrong HD name` | Run `hdwallet dump --help` and check valid `--hd` values |
| `Wrong from derivation name` | Use `--derivation BIP44` (or `Custom`, `Electrum`, etc.) |
| `Wrong entropy/mnemonic client` | Match client to HD (usually `BIP39`) |
| Command not found | Use `cargo run --bin hdwallet -- …` or `cargo install --path .` |
| Path quoting on Windows | Wrap paths in double quotes: `--path "m/44'/0'/0'/0/0-3"` |

## Report issues

When reporting CLI bugs during beta, include:

1. `hdwallet --version`
2. Full command (redact real mnemonics)
3. Complete stdout/stderr
4. Expected vs actual output

See [CONTRIBUTING.md](../CONTRIBUTING.md) for the full contribution workflow.
