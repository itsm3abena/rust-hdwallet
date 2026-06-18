# Contributing to rust-hdwallet

Thank you for helping improve **rust-hdwallet**. This project is in **beta**, so feedback, bug reports, and contributions are especially valuable right now.

Please read this guide before opening a pull request. For larger changes, open an [issue](https://github.com/itsm3abena/rust-hdwallet/issues) first so we can agree on the approach.

## Ways to Contribute

You do not need to write code to help. Useful contributions include:

- **Reporting bugs** — Include Rust version, OS, the command or code you ran, expected vs. actual output, and steps to reproduce.
- **Testing** — Try the library or CLI against known test vectors, other wallet software, or chains you use daily.
- **Documentation** — Fix typos, clarify examples, or document edge cases you discovered.
- **Code** — Fix bugs, add cryptocurrency support, improve error messages, or extend test coverage.
- **Reviews** — Comment on open issues and pull requests with reproduction notes or suggestions.

## Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) **1.85** or later
- Git

### Fork and clone

1. Fork [itsm3abena/rust-hdwallet](https://github.com/itsm3abena/rust-hdwallet) on GitHub.
2. Clone your fork:

```bash
git clone https://github.com/YOUR_USERNAME/rust-hdwallet.git
cd rust-hdwallet
```

3. Add the upstream remote:

```bash
git remote add upstream https://github.com/itsm3abena/rust-hdwallet.git
```

### Build and test

```bash
cargo build
cargo test
```

Build the CLI:

```bash
cargo build --bin hdwallet
cargo run --bin hdwallet -- --help
```

Before submitting a PR, make sure all tests pass locally.

## Development Workflow

1. **Sync with upstream** (recommended before starting work):

```bash
git fetch upstream
git checkout main
git merge upstream/main
```

2. **Create a feature branch**:

```bash
git checkout -b fix/bip44-derivation-path
```

Use a short, descriptive branch name (e.g. `feat/add-cosmos-address`, `docs/readme-cli-examples`, `fix/monero-seed-encoding`).

3. **Make your changes** — Keep diffs focused. One logical change per pull request is easier to review.

4. **Run tests** after your changes:

```bash
cargo test
```

5. **Commit** with a clear message describing *why* the change was made:

```bash
git add .
git commit -m "Fix BIP44 change index parsing for hardened paths"
```

6. **Push** to your fork:

```bash
git push origin fix/bip44-derivation-path
```

7. **Open a pull request** on GitHub from your branch into `main`. Describe what changed, how you tested it, and link any related issues (e.g. `Fixes #42`).

If reviewers request changes, push additional commits to the same branch — the PR will update automatically.

## Coding Guidelines

- **Match existing style** — Follow patterns in neighboring modules for naming, error handling, and structure.
- **Keep scope small** — Avoid unrelated refactors in the same PR.
- **Add tests when fixing bugs** — Prefer tests that use existing JSON fixtures under `tests/data/` when applicable.
- **Cryptocurrency additions** — Follow the structure in `src/hdwallet/cryptocurrencies/` and register the coin in `mod.rs`. Add or extend tests where vectors exist.
- **Security** — Never commit real mnemonics, private keys, or seeds. Use test vectors from `tests/data/` or well-known public examples (e.g. BIP39 test mnemonics).
- **CLI changes** — Update help text and mention new flags in the PR description; update README examples if user-facing behavior changes.

## Reporting Issues (Beta Feedback)

When filing a bug during beta testing, please include:

| Field | Example |
|:------|:--------|
| `rust-hdwallet` version / commit | `0.1.0-beta` or `abc1234` |
| Rust version | `rustc 1.85.0` |
| OS | Windows 11, Ubuntu 24.04, macOS 15 |
| Input | CLI flags, mnemonic type, symbol, HD standard, path |
| Expected | What you expected to happen |
| Actual | Error message or wrong output |

Label suggestions: `bug`, `enhancement`, `documentation`, `good first issue` (maintainers may add these).

## Pull Request Checklist

Before marking a PR ready for review:

- [ ] `cargo test` passes
- [ ] Changes are limited to the stated purpose
- [ ] New behavior has tests where practical
- [ ] No secrets or real wallet material in commits
- [ ] README or CONTRIBUTING updated if user-facing docs changed

## Code of Conduct

Be respectful and constructive. Assume good intent. Disagreement about technical choices is fine; personal attacks are not.

## Questions?

Open a [GitHub issue](https://github.com/itsm3abena/rust-hdwallet/issues) with the `question` label, or reach out via the contact in the repository metadata.

## License

By contributing, you agree that your contributions will be licensed under the same [MIT License](LICENSE) that covers the project.
