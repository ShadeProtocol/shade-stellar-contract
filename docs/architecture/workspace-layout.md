# Workspace and Crate Layout

This document provides a comprehensive guide to the Cargo workspace architecture, directory structure, crate boundaries, and standard file conventions within the Shade Protocol smart contract repository.

---

## Workspace Configuration (`Cargo.toml`)

The repository is configured as a multi-package Cargo workspace with:
- **Member Glob**: `members = ["contracts/*"]` to automatically discover and link all contract crates.
- **Resolver**: `resolver = "2"` ensuring modern, unified feature resolution across compilation targets.
- **Dependency Pinning**: `[workspace.dependencies]` pins `soroban-sdk = "23.4.0"` uniformly across all workspace members.
- **Release Optimization Profile**: Sets `opt-level = "z"`, `codegen-units = 1`, `lto = true`, and `strip = "symbols"` for minimal WASM bytecode footprint on-chain.

---

## Annotated Directory Tree

```text
shade-stellar-contract/
├── .github/
│   └── workflows/              # CI/CD pipelines (test, build, lint, audit)
├── .pre-commit-config.yaml     # Git pre-commit hook definitions
├── Cargo.lock                  # Pinned workspace dependency lockfile
├── Cargo.toml                  # Workspace root configuration
├── Makefile                    # Developer shortcut commands (build, test, fmt, clean)
├── README.md                   # Repository overview and quickstart links
├── contracts/
│   ├── account/                # Per-merchant dedicated account contract
│   │   ├── Cargo.toml
│   │   └── src/
│   ├── crowdfund/              # Crowdfunding campaign contract
│   │   ├── Cargo.toml
│   │   └── src/
│   ├── crowdfund_factory/      # Deterministic crowdfund deployment factory
│   │   ├── Cargo.toml
│   │   └── src/
│   ├── escrow/                 # Conditional release & milestone escrow contract
│   │   ├── Cargo.toml
│   │   └── src/
│   ├── escrow_factory/         # Escrow contract deployment factory
│   │   ├── Cargo.toml
│   │   └── src/
│   ├── shade/                  # Core hub gateway contract (invoices, merchants, fees)
│   │   ├── Cargo.toml
│   │   └── src/
│   ├── subscription/           # Recurring billing & streaming subscription contract
│   │   ├── Cargo.toml
│   │   └── src/
│   ├── ticketing/              # Event ticketing and pass issuance contract
│   │   ├── Cargo.toml
│   │   └── src/
│   └── ticketing_factory/      # Ticketing contract deployment factory
│       ├── Cargo.toml
│       └── src/
├── docs/                       # Architecture, getting-started, and contract reference docs
└── scripts/                    # Deployment, testing, and maintenance automation scripts
```

---

## Crate Registry

| Crate Name | Purpose & Domain | Deployable Contract? | Key Modules | Documentation Reference |
| :--- | :--- | :---: | :--- | :--- |
| **`shade`** | Core payment hub, invoice lifecycle, fee routing, role-based access | **Yes** | `lib`, `interface`, `types`, `errors`, `events`, `components/` | [Hub Reference](../contracts/shade.md) |
| **`account`** | Per-merchant isolated vault for balance tracking & restricted withdrawals | **Yes** | `lib`, `interface`, `types`, `errors`, `events` | [Account Reference](../contracts/account.md) |
| **`crowdfund`** | Decentralized crowdfunding campaigns with goal thresholds & backer refunds | **Yes** | `lib`, `interface`, `types`, `errors` | [Crowdfund Reference](../contracts/crowdfund.md) |
| **`crowdfund_factory`** | Factory contract for deterministic crowdfund instance deployment | **Yes** | `lib`, `factory` | [Cross-Contract Architecture](cross-contract-calls.md) |
| **`escrow`** | Two-party and multi-party milestone-based funds holding and release | **Yes** | `lib`, `interface`, `types`, `errors` | [Escrow Reference](../contracts/escrow.md) |
| **`escrow_factory`** | Factory contract for deterministic escrow instance deployment | **Yes** | `lib`, `factory` | [Cross-Contract Architecture](cross-contract-calls.md) |
| **`subscription`** | Automated recurring merchant subscriptions and billing schedules | **Yes** | `lib`, `interface`, `types`, `errors` | [Subscription Reference](../contracts/subscription.md) |
| **`ticketing`** | NFT-backed event ticket issuance, validation, and check-in | **Yes** | `lib`, `interface`, `types`, `errors` | [Ticketing Reference](../contracts/ticketing.md) |
| **`ticketing_factory`** | Factory contract for deploying event ticket instances | **Yes** | `lib`, `factory` | [Cross-Contract Architecture](cross-contract-calls.md) |

---

## Standard Contract Crate Structure

Every contract crate under `contracts/` adheres to a strict modular structure:

1. **`lib.rs`**: Entry point exporting the `#[contract]` struct, method implementations, and module declarations.
2. **`interface.rs`**: Defines the public Soroban `#[contracttrait]` specifying signatures and argument orders.
3. **`types.rs`**: Custom Soroban `#[contracttype]` data structures (e.g. `Invoice`, `Merchant`, `TokenBalance`).
4. **`errors.rs`**: Strongly-typed `#[contracterror]` enum defining all domain-specific error codes.
5. **`events.rs`**: Event emission helpers publishing structured telemetry to the host ledger.
6. **`components/`** *(Optional)*: Reusable internal sub-modules (e.g. `account_factory`, `payment_engine`).
7. **`tests/`** *(or `test.rs`)*: Comprehensive unit and integration tests using `soroban-sdk::testutils`.

---

## Repository Support Files

- **`Makefile`**: Standard build workflows (`make build`, `make test`, `make fmt`, `make clean`).
- **`.pre-commit-config.yaml`**: Pre-commit hooks for formatting (`cargo fmt`), linting (`cargo clippy`), and trailing whitespace validation.
- **`.github/workflows/`**: Continuous Integration pipelines executing on every PR and main merge.
- **`scripts/`**: Shell and TypeScript scripts automating testnet deployment and cross-contract initialization.
