# Resource optimization

This guide helps contract contributors keep Soroban execution costs and deployed WASM size under control. Measure changes to hot paths rather than relying on intuition; the [resource-cost reference](../../example/docs/reference/resource-costs.md) explains current measurements and network limits.

## Storage

Every storage access consumes resources, and writes also consume ledger write capacity, bytes, and rent. Keep each invocation's reads and writes to the minimum needed to preserve the contract's behavior.

Prefer one entry per independently accessed item over an ever-growing `Vec` stored under one key. An unbounded collection must be loaded and rewritten as it grows, costs more per update, and can eventually exceed transaction limits. Use a count or index plus keyed entries, and paginate reads rather than scanning the entire collection (see [ADR 0009](../architecture/decisions/0009-bounded-reads-and-pagination.md)):

```rust
// Prefer independently addressable entries:
env.storage().persistent().set(&DataKey::Invoice(invoice_id), &invoice);
let invoice = env.storage().persistent().get(&DataKey::Invoice(invoice_id));
```

Avoid redundant reads of the same key during one invocation: load the value once, update it in memory, and write it once. Combine related updates when that preserves clear invariants. Do not remove checks or combine state in ways that weaken authorization, accounting, or failure behavior just to save a storage operation.

### Choose storage durability deliberately

Soroban provides instance, persistent, and temporary storage. Choose based on how long the data must remain available and how it is accessed:

- **Instance storage** is appropriate for small, contract-wide configuration that shares the contract instance's lifetime. Keep it small; adding data increases the instance entry's footprint.
- **Persistent storage** is for keyed data that must survive beyond the current invocation and remain available until its TTL expires. Renew the TTL for data that must stay live. Larger or numerous entries increase rent and can require archival restoration after expiry.
- **Temporary storage** is for short-lived data that may safely disappear after its TTL. It does not need TTL extension, but expiry is permanent; never use it for balances, authorization, or other state that must be recoverable.

TTL renewal itself has a cost. Set lifetimes to the actual product requirement, and avoid renewing every entry on every call when only a subset needs to remain live. Review the current network's rent and archival rules before changing durability or TTL behavior.

## Cross-contract calls

Each cross-contract call adds execution work and can add reads, writes, and failure points. Call another contract only when its result or state change is required. Reuse an already loaded address or result, and do not call the same contract repeatedly for identical data in one invocation when one call can serve the operation.

In particular, avoid token `balance` lookups when the amount is already known from the operation being performed. A balance call is another contract invocation and reads token state; it can also reflect unrelated prior transfers rather than the amount this operation intends to process. Query balances only when a current balance is genuinely part of the required behavior, and avoid repeating the lookup without an intervening state change.

Do not replace authoritative external checks with cached assumptions unless the cache's consistency and invalidation are part of the contract's design.

## Events

Events are included in transaction resource usage and are limited in size. Keep topics compact and data limited to fields that consumers need. Prefer identifiers and concise values to serializing a full contract record or large collection.

```rust
// Compact topics identify the event and invoice; data contains only useful fields.
env.events().publish((symbol_short!("paid"), invoice_id), amount);
```

Do not emit duplicate events for the same state transition. When changing an event payload, account for existing indexers and clients as well as the event-size budget.

## WASM size

Keep the dependency graph focused: avoid adding a large general-purpose crate for a small helper, and disable unused default features when the dependency supports doing so safely. Check the transitive dependency and resulting WASM size before adding a dependency.

The workspace already defines a size-oriented release profile in the root [`Cargo.toml`](../../Cargo.toml), including `opt-level = "z"`, LTO, one codegen unit, and stripped symbols. Preserve those settings unless a measured trade-off justifies a change. See [Building the contracts](../getting-started/building.md#release-profile-settings) for the full explanation.

Build and optimize the deployable WASM with the existing commands:

```bash
cargo build --target wasm32v1-none --release
stellar contract optimize --wasm target/wasm32v1-none/release/shade.wasm
```

The Stellar CLI writes `shade.optimized.wasm` alongside the input. Compare optimized output sizes when changing dependencies or release settings; optimize each contract WASM that you intend to deploy.

## Measure resource changes locally

Use the Soroban test environment to compare the same invocation before and after a change. Set up test state before measuring, reset the budget immediately before the operation, invoke it once, then read the estimate:

```rust
env.cost_estimate().budget().reset_unlimited();
client.operation(&args);
let resources = env.cost_estimate().resources();
std::println!(
    "instructions: {}, memory: {}, reads: {}, writes: {}, write bytes: {}, event bytes: {}",
    resources.instructions,
    resources.mem_bytes,
    resources.disk_read_entries + resources.memory_read_entries,
    resources.write_entries,
    resources.write_bytes,
    resources.contract_events_size_bytes,
);
```

This follows the pattern in [`example/subscription/src/test_resource_costs.rs`](../../example/subscription/src/test_resource_costs.rs). Resetting to an unlimited budget lets a test measure operations that would exceed a limit; it does not make an over-limit operation valid on network. Native test execution can understate WASM CPU and memory costs, so confirm important changes with a testnet simulation.

For changes to hot paths, include before-and-after resource numbers in the pull request description. Record the operation, test setup, and relevant resource counts (instructions, memory, read/write entries, bytes written, and event bytes) so reviewers can compare like with like. Consult the [resource-cost reference](../../example/docs/reference/resource-costs.md) for measurements and the documented measurement test.
