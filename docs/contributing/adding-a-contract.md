# Adding a new contract to the workspace

This guide walks you through creating, configuring, integrating, and documenting a new [Soroban](../glossary.md#soroban) smart contract crate in this repository. Use it alongside [Adding a Component to Shade](adding-a-component.md) when deciding whether functionality belongs in a new crate or inside the existing `shade` hub.

## Prerequisites

- Rust toolchain pinned in [`rust-toolchain.toml`](../../rust-toolchain.toml) with the `wasm32v1-none` target (installed automatically by `rustup` on the first `cargo` run).
- Stellar CLI (`stellar`) for WASM optimization and deployment targets.
- Read [Workspace and Crate Layout](../architecture/workspace-layout.md) and [Contract coding conventions](coding-conventions.md).
- Confirm workspace scope under [ADR 0007](../architecture/decisions/0007-core-gateway-scope.md): the root Cargo workspace under `contracts/` holds only the core payment gateway (`shade` and `account`), while non-gateway contracts (`escrow`, `escrow_factory`, `ticketing`, `ticketing_factory`, `subscription`, `crowdfund`, `crowdfund_factory`, `platform_extras`) live in [`example/`](../../example/README.md). Confirm with maintainers and record an [ADR](../architecture/decisions/README.md) before adding a crate under `contracts/`.

## Steps

### 1. Create the crate and configure `Cargo.toml`

Create a directory for the new contract under `contracts/<contract_name>/` (or `example/<contract_name>/` for non-gateway contracts):

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

Configure `contracts/<contract_name>/Cargo.toml` with `crate-type = ["cdylib"]`, workspace dependencies, and `testutils` restricted to `[dev-dependencies]`:

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

Choose `crate-type` based on how other crates consume the contract:

| Crate role | `crate-type` setting | Repository examples |
|---|---|---|
| Standalone WASM contract not imported as a Rust crate | `["cdylib"]` | Deployable-only Soroban contracts |
| Contract also imported as a Rust dependency by sibling crates or tests | `["lib", "cdylib"]` | [`contracts/shade/Cargo.toml`](../../contracts/shade/Cargo.toml), [`contracts/account/Cargo.toml`](../../contracts/account/Cargo.toml), [`example/escrow_factory/Cargo.toml`](../../example/escrow_factory/Cargo.toml), [`example/ticketing_factory/Cargo.toml`](../../example/ticketing_factory/Cargo.toml) |

Follow these `Cargo.toml` conventions across every contract crate:

- **Version and publishing:** Set `version = "0.0.0"`, `edition = "2021"`, and `publish = false`. Deployed builds are identified by their [WASM hash](../glossary.md#wasm-hash), not crate versions.
- **Workspace dependencies:** Inherit `soroban-sdk` from `[workspace.dependencies]` via `soroban-sdk = { workspace = true }` so every crate uses the exact version pinned in the root [`Cargo.toml`](../../Cargo.toml).
- **Dev-only test utilities:** Enable `features = ["testutils"]` only in `[dev-dependencies]` so test helpers never compile into release WASM binaries.
- **Doctests:** Set `doctest = false` under `[lib]`.

### 2. Register the crate in the root `Cargo.toml`

Verify that the root [`Cargo.toml`](../../Cargo.toml) includes your crate in `[workspace].members`:

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

Because `members = ["contracts/*"]` uses a directory glob, any crate placed directly under `contracts/<contract_name>/` becomes a workspace member automatically. If you place a contract at a non-globbed path or under `example/`, add its directory name explicitly to `members` in the root `Cargo.toml` or [`example/Cargo.toml`](../../example/Cargo.toml).

Confirm Cargo resolves the package:

```bash
cargo metadata --no-deps --format-version 1 | grep '"name"'
```

### 3. Set up the source layout and interface trait

Place `#![no_std]` at the top of `src/lib.rs` and separate types, errors, events, implementation, and tests into dedicated modules:

| File | Responsibility | Rules |
|---|---|---|
| `src/lib.rs` | Crate attributes (`#![no_std]`) and module exports | Declare `errors`, `events`, `types`, contract modules, and `#[cfg(test)] pub mod tests;` |
| `src/errors.rs` | `#[contracterror]` enum (`ContractError`) with `#[repr(u32)]` | Append-only `u32` discriminants; fail with `panic_with_error!` |
| `src/events.rs` | `#[contractevent]` structs and `publish_*_event` helpers | Keep derived snake_case topics `<= 32` chars; include `timestamp: u64` |
| `src/types.rs` | [`DataKey`](../glossary.md#datakey) enum and stored `#[contracttype]` structs | Use persistent storage; stay within the 50-variant enum limit; follow [ADR 0009](../architecture/decisions/0009-bounded-reads-and-pagination.md) |
| `src/interface.rs` | Optional `#[contracttrait]` defining the public interface | Recommended for multi-method or cross-contract APIs (see below) |
| `src/<contract_name>.rs` | `#[contract]` struct and `#[contractimpl]` implementation | Verify [`require_auth`](../glossary.md#require_auth) first; emit [events](../glossary.md#event) after storage writes |
| `src/tests/` | Unit and integration tests (`mod.rs` + `test_*.rs`) | Cover happy paths, unauthorized callers, invalid inputs, and boundary conditions |

For a crate following `contracts/account/`'s layout, `src/lib.rs` declares the modules and test module gate:

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

Add a per-crate `Makefile` matching [`contracts/account/Makefile`](../../contracts/account/Makefile) if contributors run `stellar contract build` directly inside the crate directory.

#### When to split the interface into a trait

Move the public entrypoint signatures into a `#[contracttrait]` in `src/interface.rs` (or `src/<name>_interface.rs` as in [`contracts/shade/src/shade_interface.rs`](../../contracts/shade/src/shade_interface.rs)) when any of the following applies:

- **Other contracts or Rust clients invoke it:** A trait provides one canonical contract interface for cross-contract callers and test suites (as in [`contracts/account/src/interface.rs`](../../contracts/account/src/interface.rs)).
- **The contract exposes a large or evolving surface:** Implementing `#[contractimpl] impl MyContractTrait for MyContract` turns any signature drift between the interface specification and implementation into a compile-time error.
- **Multiple components back one contract:** Separating `shade_interface.rs` from `shade.rs` keeps the exported API readable independently of component dispatch.

For small, single-purpose contracts with only a few entrypoints (such as [`example/escrow_factory/src/lib.rs`](../../example/escrow_factory/src/lib.rs) and [`example/ticketing_factory/src/lib.rs`](../../example/ticketing_factory/src/lib.rs)), defining `#[contract]` and `#[contractimpl]` directly on the struct avoids unnecessary boilerplate.

### 4. Integrate the crate's WASM into the `Makefile`

Update the root [`Makefile`](../../Makefile) so `build`, `optimize`, and deploy targets build and manage your contract's WASM alongside `shade` and `account`:

1. **Declare WASM path variables** near the top of `Makefile`:

   ```makefile
   MY_CONTRACT_WASM = target/wasm32v1-none/release/<contract_name>.wasm
   MY_CONTRACT_WASM_OPT = target/wasm32v1-none/release/<contract_name>.optimized.wasm
   ```

2. **Register `.PHONY` targets** for `deploy-<contract_name>` so Make does not confuse the target with a file name.

3. **Include the binary in `build` and `optimize`:** `cargo build --workspace --target wasm32v1-none --release` compiles every workspace member; extend the `ls -lh` and `stellar contract optimize` lines to include the new artifact:

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

4. **Add a `deploy-<contract_name>` target and extend `deploy-all`:**

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

5. **Update fixtures and deployment scripts if applicable:** If another contract's test suite loads your WASM with `include_bytes!`, add a `cp` step to the `fixtures` target (matching `contracts/shade/src/tests/fixtures/account.wasm`). If the contract is part of scripted network deployments (`make deploy`), add its installation/deployment steps to [`scripts/deploy.py`](../../scripts/deploy.py) and `deploy/*.toml`.

### 5. Follow the factory pattern for per-user or per-merchant contracts

Contracts instantiated once per user, event, or [merchant](../glossary.md#merchant) are deployed on-chain by a factory contract rather than manually from off-chain scripts. Use [`example/escrow_factory/src/lib.rs`](../../example/escrow_factory/src/lib.rs), [`example/ticketing_factory/src/lib.rs`](../../example/ticketing_factory/src/lib.rs), and [`contracts/shade/src/components/account_factory.rs`](../../contracts/shade/src/components/account_factory.rs) as references.

Each factory implements four steps:

1. **Store the child WASM hash:** The [admin](../glossary.md#admin) installs the child WASM on the ledger and registers its `BytesN<32>` hash in persistent storage (`initialize` in `escrow_factory`, `set_ticketing_wasm_hash` in `ticketing_factory`, or `set_account_wasm_hash` in `shade`). Panic with `WasmHashNotSet` (or `NotInitialized`) if deployment runs before the hash is set.
2. **Derive a deployment salt:**
   - **Multiple instances per actor (`escrow_factory`, `ticketing_factory`):** Generate a unique salt from on-chain randomness so repeated deployments land at distinct addresses:

     ```rust
     let random: BytesN<32> = env.prng().gen();
     let salt = env
         .crypto()
         .keccak256(&Bytes::from_slice(&env, &random.to_array()));
     ```

   - **Single deterministic instance per ID (`account_factory`):** Hash a unique identifier such as `merchant_id.to_be_bytes()` with `keccak256` so each merchant maps to one predictable [contract ID](../glossary.md#contract-id).
3. **Deploy and initialize the child contract:** Instantiate the WASM with `env.deployer().with_current_contract(salt).deploy_v2(wasm_hash, ())`, then invoke its `initialize` function via `env.invoke_contract::<()>(...)` when setup arguments are required (`escrow_factory`, `account_factory`).
4. **Index deployed instances without unbounded vectors:** Follow `ticketing_factory`'s monotonic counter and keyed record pattern (`DataKey::EventRefCount` + `DataKey::EventRef(u64)` + `EventContractDeployedEvent`) instead of appending to a single `Vec<Address>` entry (as older `escrow_factory` did with `DataKey::Escrows`). Storing one persistent entry per deployed reference satisfies [ADR 0009](../architecture/decisions/0009-bounded-reads-and-pagination.md).

### 6. Complete the documentation obligations

Every new contract crate must update the repository documentation in the same PR:

1. **Contract reference page:** Create `docs/contracts/<contract_name>.md` from [`docs/contributing/templates/contract-reference-template.md`](templates/contract-reference-template.md) covering the interface trait or public functions, data types, storage keys, and errors. Link the new page from [`docs/contracts/README.md`](../contracts/README.md) and [`docs/SUMMARY.md`](../SUMMARY.md).
2. **Errors and events reference entries:**
   - In [`docs/reference/errors.md`](../reference/errors.md), add a `## ` section whose heading includes `` `contracts/<contract_name>/src/errors.rs` ``, a `### ` subsection for `` `ContractError` `` (or your error enum name), and a table row starting with `` | `VariantName` | <code> | `` for every variant.
   - In [`docs/reference/events.md`](../reference/events.md), add a `## ` section whose heading includes `` `contracts/<contract_name>/src/events.rs` `` and a table row starting with `` | `publish_<name>_event` | `` for every `publish_*_event` helper.
   - Both reference formats are validated by [`scripts/check_reference_docs.py`](../../scripts/check_reference_docs.py). See [Docs Tooling](docs-tooling.md#quality-checks) for the exact parser rules.
3. **Workspace layout:** Add the new crate to the directory tree and crate table in [`docs/architecture/workspace-layout.md`](../architecture/workspace-layout.md).
4. **Architecture Decision Record (ADR):** Adding a new contract or factory requires an ADR under [`docs/architecture/decisions/`](../architecture/decisions/README.md) copied from [`template.md`](../architecture/decisions/template.md), numbered sequentially, and linked from [`docs/architecture/decisions/README.md`](../architecture/decisions/README.md) and [`docs/SUMMARY.md`](../SUMMARY.md).

## Verify it worked

Run the workspace checks, WASM build, and docs checks before opening a PR:

```bash
make fmt
make lint
make test
make build
make docs-check
```

Confirm `git status` shows no staged `test_snapshots/` directories and complete the [Security review checklist for contract PRs](../security/pr-review-checklist.md) before requesting review.

## Troubleshooting

| Problem | Cause | Fix |
|---|---|---|
| `cargo build` skips the new crate | Crate is not under `contracts/*` and not listed in `members` | Place the crate under `contracts/` (if gateway scope) or add its path to `members` in `Cargo.toml` |
| `make docs-check` fails with missing error or event entries | `docs/reference/errors.md` or `docs/reference/events.md` lacks the matching section heading or table rows | Add the `` `contracts/<name>/src/...` `` heading and rows matching [Docs Tooling](docs-tooling.md#quality-checks) |
| Cross-contract test cannot import the contract client | `Cargo.toml` declares only `crate-type = ["cdylib"]` | Change `crate-type` to `["lib", "cdylib"]` so test targets can link the crate as a library |
| `cargo clippy` fails on entrypoints with >7 arguments | Soroban entrypoints often accept more than seven parameters | Add `#![allow(clippy::too_many_arguments)]` in `src/lib.rs` or on the specific entrypoint |

## Related pages

- [Adding a Component to Shade](adding-a-component.md) — adding a feature module inside `contracts/shade/`.
- [Contract coding conventions](coding-conventions.md) — error, event, auth, and storage standards.
- [Security review checklist for contract PRs](../security/pr-review-checklist.md) — security checks required for PRs touching `contracts/`.
- [Cross-Contract Calls](../architecture/cross-contract-calls.md) — deployer mechanics and authorization propagation.
- [ADR 0007: Limit the workspace to the core payment gateway](../architecture/decisions/0007-core-gateway-scope.md) — what belongs in `contracts/` vs `example/`.
