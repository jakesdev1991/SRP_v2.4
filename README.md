# SRP v2.4

Supply Chain Risk Protocol v2.4 - A Rust-based framework for supply chain security, policy enforcement, and cryptographic proof verification.

## Overview

SRP (Supply Chain Risk Protocol) provides:

- **Policy Engine**: Declarative policy language for supply chain constraints
- **Proof System**: Cryptographic proofs for artifact provenance
- **Specification**: Formal specification of supply chain requirements
- **Supply Chain Tools**: CLI tools for SBOM generation, verification, and auditing

## Structure

```
SRP_v2.4/
├── src/              # Rust source code
├── spec/             # Formal specifications
├── policy/           # Policy definitions
├── proof/            # Proof system implementation
├── supply_chain/     # Supply chain tooling
├── tests/            # Integration tests
├── tools/            # CLI utilities
├── docs/             # Documentation
├── .github/          # CI/CD workflows
└── Cargo.toml        # Rust project manifest
```

## Quick Start

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build
cargo build --all-targets --all-features

# Test
cargo test --all-targets --all-features

# Run CLI
cargo run -- --help
```

## Development

```bash
# Format
cargo fmt --all

# Lint
cargo clippy --all-targets --all-features -- -D warnings

# Check
cargo check --all-targets --all-features
```

## Configuration

See `config/` for policy and specification files.

## License

See [LICENSE](LICENSE) for details.