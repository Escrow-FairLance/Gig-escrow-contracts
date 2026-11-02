# Contributing to Escrow-FairLance Contracts

Thank you for your interest in contributing to **Escrow-FairLance Contracts**! We welcome bug fixes, documentation improvements, gas optimizations, and protocol enhancements from developers worldwide.

---

## Code of Conduct

All contributors and maintainers are expected to adhere to our [Code of Conduct](CODE_OF_CONDUCT.md). Please report unacceptable behavior to security@fairlance.io.

---

## Development Setup

### Prerequisites
- **Rust**: `1.80.0` or higher
- **WASM Target**: `wasm32-unknown-unknown` (or `wasm32v1-none`)
- **Stellar CLI**: `22.0.0` or higher (`cargo install --locked stellar-cli`)
- **Node.js**: `20.x` or higher
- **Bun** (optional): `1.0+`

Install the Rust WASM target:
```bash
rustup target add wasm32-unknown-unknown
```

### Verification & Testing
Before submitting a pull request, ensure all local checks pass:

```bash
# 1. Format check
cargo fmt -- --check

# 2. Run Clippy lints
cargo clippy --all-targets -- -D warnings

# 3. Run unit and integration tests
cargo test

# 4. Build contract WASM in release mode
cargo build --target wasm32-unknown-unknown --release
```

---

## Conventional Commits

We strictly follow the [Conventional Commits v1.0.0](https://www.conventionalcommits.org/) specification:

- `feat:` A new feature or contract capability
- `fix:` A bug fix or math correction
- `chore:` Changes to build scripts, dependencies, or tooling
- `docs:` Documentation improvements
- `test:` Adding or updating tests
- `refactor:` Code improvements that do not alter contract semantics
- `ci:` Changes to GitHub Actions workflows

---

## Security Disclosures

If you discover a security vulnerability in smart contracts or financial accounting logic, please refer to our [Security Policy](SECURITY.md) for responsible disclosure procedures.
