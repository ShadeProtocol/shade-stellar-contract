# ADR-0001: Use a component-based module structure inside the `shade` hub contract

This record explains why the primary `shade` contract organizes its domain logic into internal component modules (`contracts/shade/src/components/`) behind a thin `ShadeTrait` entrypoint rather than splitting every payment feature into a separate Soroban contract.

## Status

Accepted

## Context

In the Shade protocol, core payment workflows — creating and settling an [invoice](../../glossary.md#invoice), verifying a [merchant](../../glossary.md#merchant), routing platform fees from a [payer](../../glossary.md#payer), checking global pause state, and recording transaction history — all read and mutate shared protocol state. On Soroban, every cross-contract invocation incurs host dispatch overhead, requires explicit authorization sub-invocations, and serializes arguments across WASM boundaries. At the same time, placing 34 feature domains into a single monolithic `shade.rs` file would make reviews, storage-key ownership checks, and unit testing unmanageable.

## Decision

Keep the core payment hub inside a single deployable `shade` contract (`contracts/shade/src/shade.rs`), declare its complete exported interface in `contracts/shade/src/shade_interface.rs` (`ShadeTrait`), and partition all business logic into domain modules under `contracts/shade/src/components/`.

- `shade.rs` performs only the global pause check (`pausable_component::assert_not_paused(&env)` where applicable) and immediately delegates to the target component function.
- Each module under `contracts/shade/src/components/` owns its domain's storage keys in `contracts/shade/src/types.rs` and exposes public Rust functions for cross-component reads and writes.

## Consequences

### Positive

- **Atomic single-contract execution:** `pay_invoice` reads merchant configuration, checks accepted tokens, computes platform fee splits, transfers tokens, and updates invoice state within a single Soroban contract storage namespace without cross-contract round trips for shared metadata.
- **Compile-time surface verification:** Implementing `ShadeTrait` on `Shade` ensures every exported WASM entrypoint matches the documented trait signature.
- **Clear review boundaries:** Reviewers can audit auth, storage access, and event emission per domain module under `contracts/shade/src/components/`.

### Negative and trade-offs

- **Larger single WASM binary:** Packaging 34 components into `shade` increases the compiled WASM size compared to tiny single-purpose contracts, requiring careful attention to Soroban `"#[contracttype]"` enum variant limits (capped at 50 variants per enum in `contracts/shade/src/types.rs`).
- **Discipline required on storage boundaries:** Because components share one contract address, reviewers must verify that components access other domains via public getters rather than reading foreign `DataKey` variants directly.

## Alternatives considered

### Separate micro-contracts for every feature domain

Deploying independent contracts for merchants, fees, invoices, and governance would isolate each module's WASM bytecode, but every `pay_invoice` call would require multiple cross-contract calls just to verify the merchant status, token whitelist, and fee basis points. That increases ledger read/write fees and complicates atomic state updates.

### A single monolithic `shade.rs` implementation

Writing all entrypoint bodies directly inside `contracts/shade/src/shade.rs` without a `components/` layer avoids module delegation boilerplate, but makes a multi-thousand-line file impossible to navigate or audit by domain.

← [Back to ADR index](README.md)
