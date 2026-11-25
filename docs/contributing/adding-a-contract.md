# Adding a new contract to the workspace

This guide walks contributors through creating, configuring, integrating, and documenting a new Soroban smart contract crate in this repository. Use it alongside [Adding a Component to Shade](adding-a-component.md) when deciding whether functionality belongs in a new crate or inside the existing `shade` hub.

## Before creating a contract: check workspace scope

Under [ADR 0007](../architecture/decisions/0007-core-gateway-scope.md), the root Cargo workspace under `contracts/` contains only the core payment gateway: `shade` and `account`. Non-gateway contracts (`escrow`, `escrow_factory`, `ticketing`, `ticketing_factory`, `subscription`, `crowdfund`, `crowdfund_factory`, `platform_extras`) live in [`example/`](../../example/README.md), a separate workspace excluded from root CI builds.

Before adding a crate under `contracts/`, confirm with maintainers (and record an [ADR](../architecture/decisions/README.md)) that the contract is part of the core gateway rather than an extension in `example/`. The crate setup, module layout, factory pattern, and documentation conventions below apply consistently across contracts in both locations.

## Prerequisites

- Rust toolchain pinned in [`rust-toolchain.toml`](../../rust-toolchain.toml) with the `wasm32v1-none` target (installed automatically by `rustup` on the first `cargo` run).
- Stellar CLI (`stellar`) for WASM optimization and deployment targets.
- Read [Workspace and Crate Layout](../architecture/workspace-layout.md) and [Contract coding conventions](coding-conventions.md).

## Steps

### 1. Create the crate and configure `Cargo.toml`

Create a directory for the new contract under `contracts/<contract_name>/` (or `example/<contract_name>/` for standalone example contracts):

```text
contracts/<contract_name>/
├── Cargo.toml
├── Makefile
└── src/
    ├── lib.rs
    ├── <contract_name>.rs
    ├── interface.rs        # Required when splitting the public API into a trait
    ├── types.rs
    ├── errors.rs
    ├── events.rs
    └── tests/
        ├── mod.rs
        └── test_<contract_name>.rs
```

Configure `contracts/<contract_name>/Cargo.toml` to match the existing contracts (`contracts/shade/Cargo.toml`, `contracts/account/Cargo.toml`, and the factory crates in `example/`):

```toml
[package]
name = "<contract_name>"
version = "0.0.0"
edition = "2021"
publish = false

[lib]
crate-type = ["cdylib"]
doctest = false

[dependencies]
soroban-sdk = { workspace = true }

[dev-dependencies]
soroban-sdk = { workspace = true, features = ["testutils"] }
```

Choose `crate-type` based on how the crate is consumed:

| Crate role | `crate-type` setting | Existing examples |
|---|---|---|
| Standalone or factory WASM contract | `["cdylib"]` | [`example/escrow_factory/Cargo.toml`](../../example/escrow_factory/Cargo.toml), [`example/ticketing_factory/Cargo.toml`](../../example/ticketing_factory/Cargo.toml) |
| Contract also imported as a Rust dependency in tests or sibling crates | `["lib", "cdylib"]` | [`contracts/shade/Cargo.toml`](../../contracts/shade/Cargo.toml), [`contracts/account/Cargo.toml`](../../contracts/account/Cargo.toml) |

Key `Cargo.toml` rules across the workspace:

- **Version and publishing:** Set `version = "0.0.0"`, `edition = "2021"`, and `publish = false`. Deployed builds are identified by their [WASM hash](../glossary.md#wasm-hash), not crate versions.
- **Workspace dependencies:** Pull `soroban-sdk` from `[workspace.dependencies]` via `soroban-sdk = { workspace = true }` so all crates stay pinned to the exact SDK version in the root [`Cargo.toml`](../../Cargo.toml).
- **Test utilities:** Enable `features = ["testutils"]` only in `[dev-dependencies]` so test helpers are never compiled into release WASM binaries.
- **Disable doctests:** Set `doctest = false` under `[lib]`.

### 2. Register the crate in the workspace `Cargo.toml`

Verify that the new crate is registered in the root [`Cargo.toml`](../../Cargo.toml):

```toml
[workspace]
resolver = "2"
members = [
  "contracts/*",
]
exclude = [
  "example",
]
```

Because `members = ["contracts/*"]` uses a wildcard glob, any crate placed directly under `contracts/<contract_name>/` is automatically picked up as a workspace member. If you place a contract at a non-globbed path or inside `example/`, add its directory name explicitly to `members` in the root `Cargo.toml` or [`example/Cargo.toml`](../../example/Cargo.toml).

Verify Cargo recognizes the new package:

```bash
cargo metadata --no-deps --format-version 1 | grep '"name"'
```

### 3. Set up the source file layout and interface trait

Every contract crate uses `#![no_std]` at the top of `src/lib.rs` and separates types, errors, events, implementation, and tests into dedicated modules:

| File | Responsibility | Rules |
|---|---|---|
| `src/lib.rs` | Crate attributes (`#![no_std]`) and module declarations | Declare `errors`, `events`, `types`, contract modules, and `#[cfg(test)] pub mod tests;` |
| `src/errors.rs` | `#[contracterror]` enum (`ContractError`) with `#[repr(u32)]` | Append-only integer discriminants; fail with `panic_with_error!` |
| `src/events.rs` | `#[contractevent]` structs and `publish_*_event` helpers | Struct names keep derived snake_case topics `<= 32` chars; include `timestamp: u64` |
| `src/types.rs` | `DataKey` enum and stored `#[contracttype]` structs | All production keys in persistent storage; max 50 variants per enum; follow [ADR 0009](../architecture/decisions/0009-bounded-reads-and-pagination.md) |
| `src/interface.rs` | Optional `#[contracttrait]` defining the public contract surface | Recommended for core/multi-method contracts (see below) |
| `src/<contract_name>.rs` | `#[contract]` struct and `#[contractimpl]` implementation | Auth checks first; emit events after storage writes |
| `src/tests/` | Unit and integration tests (`mod.rs` + `test_*.rs`) | Test happy paths, negative auth checks, invalid inputs, and boundary conditions |

For a crate following `contracts/account/`'s layout, `src/lib.rs` looks like:

```rust
#![no_std]
pub mod account;
pub mod errors;
pub mod events;
pub mod interface;
pub mod types;

#[cfg(test)]
pub mod tests;
```

Add a per-crate `Makefile` matching [`contracts/account/Makefile`](../../contracts/account/Makefile) if contributors will run `stellar contract build` directly inside the crate directory.

#### When to split the interface into a `#[contracttrait]`

Split the public methods into a separate `#[contracttrait]` in `interface.rs` (or `<name>_interface.rs` as in [`contracts/shade/src/shade_interface.rs`](../../contracts/shade/src/shade_interface.rs)) when:

- **Other contracts or Rust clients call it:** A trait gives cross-contract callers and integration suites a single canonical signature list (as in [`contracts/account/src/interface.rs`](../../contracts/account/src/interface.rs)).
- **The contract has a large or evolving surface:** Implementing `#[contractimpl] impl MyContractTrait for MyContract` turns any signature drift between the specification and implementation into a compile-time error.
- **Multiple components back one contract:** Splitting `shade_interface.rs` from `shade.rs` keeps the exported API readable in one file.

For small, self-contained contracts with only a handful of entrypoints (such as [`example/escrow_factory/src/lib.rs`](../../example/escrow_factory/src/lib.rs) and [`example/ticketing_factory/src/lib.rs`](../../example/ticketing_factory/src/lib.rs)), placing `#[contract]` and `#[contractimpl]` directly on the struct in `src/lib.rs` or `src/<name>.rs` avoids unnecessary boilerplate.

### 4. Integrate the crate's WASM into the `Makefile`

Update the root [`Makefile`](../../Makefile) so `build`, `optimize`, and deploy targets handle your new contract's WASM binary alongside `shade` and `account`:

1. **Declare WASM path variables** near the top of `Makefile`:

   ```makefile
   MY_CONTRACT_WASM = target/wasm32v1-none/release/<contract_name>.wasm
   MY_CONTRACT_WASM_OPT = target/wasm32v1-none/release/<contract_name>.optimized.wasm
   ```

2. **Add `.PHONY` targets** for `deploy-<contract_name>` so Make never confuses the target with a file name.

3. **Include the artifact in `build` and `optimize`:** `cargo build --workspace --target wasm32v1-none --release` already compiles all workspace members; extend the `ls -lh` and `stellar contract optimize` commands to include the new binary:

   ```makefile
   build: ## Build all contracts
   	@echo "Building contracts..."
   	@cargo build --workspace --target wasm32v1-none --release
   	@ls -lh $(SHADE_WASM) $(ACCOUNT_WASM) $(MY_CONTRACT_WASM)

   optimize: build ## Optimize WASM binaries
   	@echo "Optimizing contracts..."
   	@stellar contract optimize --wasm $(SHADE_WASM)
   	@stellar contract optimize --wasm $(ACCOUNT_WASM)
   	@stellar contract optimize --wasm $(MY_CONTRACT_WASM)
   	@ls -lh $(SHADE_WASM_OPT) $(ACCOUNT_WASM_OPT) $(MY_CONTRACT_WASM_OPT)
   ```

4. **Add a `deploy-<contract_name>` target and wire `deploy-all`:**

   ```makefile
   deploy-<contract_name>: optimize check-network $(STELLAR_DIR) ## Deploy <Contract> contract
   	@echo "Deploying <Contract> contract to $(NETWORK)..."
   	@stellar contract deploy \
   		--wasm $(MY_CONTRACT_WASM_OPT) \
   		--network $(NETWORK) \
   		--source-account default \
   		> $(STELLAR_DIR)/<contract_name>_contract_id.txt
   	@echo "<Contract> contract deployed!"
   	@echo "Contract ID: $$(cat $(STELLAR_DIR)/<contract_name>_contract_id.txt)"

   deploy-all: deploy-shade deploy-account deploy-<contract_name> ## Deploy all contracts
   ```

5. **Update `fixtures` or `scripts/deploy.py` if applicable:** If another contract's tests load your compiled WASM via `include_bytes!`, add a `cp` step to the `fixtures` target (matching `contracts/shade/src/tests/fixtures/account.wasm`). If your contract participates in scripted deployments (`make deploy`), add its deployment step to `scripts/deploy.py` and `deploy/*.toml`.

### 5. Use the factory pattern for per-user or per-merchant contracts

Contracts instantiated once per user, event, or [merchant](../glossary.md#merchant) (such as per-merchant `account` vaults, per-deal escrows, or per-event ticketing contracts) are deployed on-chain by a factory rather than manually from off-chain scripts. Use [`example/escrow_factory/src/lib.rs`](../../example/escrow_factory/src/lib.rs), [`example/ticketing_factory/src/lib.rs`](../../example/ticketing_factory/src/lib.rs), and [`contracts/shade/src/components/account_factory.rs`](../../contracts/shade/src/components/account_factory.rs) as references.

Every factory follows four steps:

1. **Store the child WASM hash:** The [admin](../glossary.md#admin) installs the child WASM on the ledger and registers its `BytesN<32>` hash in the factory (`initialize` in `escrow_factory`, `set_ticketing_wasm_hash` in `ticketing_factory`, or `set_account_wasm_hash` in `shade`). Fail with `WasmHashNotSet` (or `NotInitialized`) if deployment is attempted before the hash is set.
2. **Derive a deployment salt:**
   - **Multiple instances per actor (`escrow_factory`, `ticketing_factory`):** Generate a unique salt from on-chain randomness so repeated deployments do not collide:

     ```rust
     let random: BytesN<32> = env.prng().gen();
     let salt = env
         .crypto()
         .keccak256(&Bytes::from_slice(&env, &random.to_array()));
     ```

   - **Single deterministic instance per ID (`account_factory`):** Hash a unique identifier such as `merchant_id.to_be_bytes()` with `keccak256` so each merchant maps to one predictable address.
3. **Deploy and initialize the child contract:** Instantiate the WASM with `env.deployer().with_current_contract(salt).deploy_v2(wasm_hash, ())`, then invoke its `initialize` function via `env.invoke_contract::<()>(...)` if it requires setup arguments (`escrow_factory`, `account_factory`).
4. **Index deployed instances without unbounded vectors:** Prefer the monotonic counter + keyed record pattern in `ticketing_factory` (`DataKey::EventRefCount` + `DataKey::EventRef(u64)` + `EventContractDeployedEvent`) over storing a growing `Vec<Address>` under a single storage key (as older `escrow_factory` did with `DataKey::Escrows`). Keeping each deployed reference in its own persistent entry satisfies [ADR 0009](../architecture/decisions/0009-bounded-reads-and-pagination.md).

### 6. Complete the documentation obligations

Every new contract crate must update the documentation in the same PR:

1. **Contract reference page:** Create `docs/contracts/<contract_name>.md` from [`docs/contributing/templates/contract-reference-template.md`](templates/contract-reference-template.md), documenting the interface trait or public functions, types, storage keys, and errors. Link the page from [`docs/contracts/README.md`](../contracts/README.md) and [`docs/SUMMARY.md`](../SUMMARY.md).
2. **Errors and events reference entries:**
   - In [`docs/reference/errors.md`](../reference/errors.md), add a `## ` section whose heading contains `` `contracts/<contract_name>/src/errors.rs` ``, a `### ` subsection for `` `ContractError` `` (or your error enum name), and a table row starting with `` | `VariantName` | <code> | `` for every variant.
   - In [`docs/reference/events.md`](../reference/events.md), add a `## ` section whose heading contains `` `contracts/<contract_name>/src/events.rs` `` and a table row starting with `` | `publish_<name>_event` | `` for every `publish_*_event` helper.
   - These exact heading and table conventions are enforced by [`scripts/check_reference_docs.py`](../../scripts/check_reference_docs.py) (`make docs-check`). See [Docs Tooling](docs-tooling.md#quality-checks) for details.
3. **Workspace layout:** Add the new crate to the directory tree and crate table in [`docs/architecture/workspace-layout.md`](../architecture/workspace-layout.md).
4. **Architecture Decision Record (ADR):** Adding a new contract or factory requires an ADR under [`docs/architecture/decisions/`](../architecture/decisions/README.md) copied from [`template.md`](../architecture/decisions/template.md), numbered sequentially, and added to [`docs/architecture/decisions/README.md`](../architecture/decisions/README.md) and [`docs/SUMMARY.md`](../SUMMARY.md).

## Verify it worked

Run the full workspace checks, WASM build, and docs checks before opening a PR:

```bash
make fmt
make lint
make test
make build
make docs-check
```

Confirm `git status` shows no staged `test_snapshots/` directories and review the [Security Review Checklist for Contract PRs](../security/pr-review-checklist.md) before requesting review.

## Troubleshooting

| Problem | Cause | Fix |
|---|---|---|
| `cargo build` skips the new crate | Crate is not under `contracts/*` and not listed in `members` | Move the crate under `contracts/` (if gateway scope) or add its path to `members` in `Cargo.toml` |
| `make docs-check` fails with missing error or event entries | `docs/reference/errors.md` or `docs/reference/events.md` lacks the matching section heading or table rows | Add the `` `contracts/<name>/src/...` `` heading and rows matching [Docs Tooling](docs-tooling.md#quality-checks) |
| Cross-contract test cannot import the contract client | `Cargo.toml` only declares `crate-type = ["cdylib"]` | Change `crate-type` to `["lib", "cdylib"]` so Rust test targets can link the crate as a library |
| `cargo clippy` fails on entrypoints with >7 arguments | Soroban contract functions often take many parameters | Add `#![allow(clippy::too_many_arguments)]` to `src/lib.rs` or on the specific entrypoint |

## Related pages

- [Adding a Component to Shade](adding-a-component.md) — adding a feature module inside `contracts/shade/`.
- [Contract coding conventions](coding-conventions.md) — error, event, auth, and storage standards.
- [Security review checklist for contract PRs](../security/pr-review-checklist.md) — security checks required for contract PRs.
- [Cross-Contract Calls and the Factory Pattern](../architecture/cross-contract-calls.md) — deployer mechanics and authorization propagation.
- [ADR 0007: Limit the workspace to the core payment gateway](../architecture/decisions/0007-core-gateway-scope.md) — what belongs in `contracts/` vs `example/`.
