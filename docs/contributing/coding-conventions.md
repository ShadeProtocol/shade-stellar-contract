# Contract coding conventions

This guide defines the coding conventions, patterns, and style rules for writing Soroban smart contracts in this repository (`contracts/shade/` and `contracts/account/`). Reviewers and contributors use this single reference to ensure new contracts, components, and fixes converge on a predictable, secure style.

For documentation voice, structure, and formatting standards, read the [Documentation style guide](documentation-style-guide.md). For how contracts, factories, and components fit together architecturally, read the [Architecture overview](../architecture/overview.md).

## Errors and failure handling

Soroban smart contracts in this workspace report errors through custom contract error enums derived with `#[contracterror]`.

### When to panic vs. return Result

In this repository, contract entrypoints and component functions follow a strict error-reporting model:

- **Public contract entrypoints:** Use `soroban_sdk::panic_with_error!(env, ContractError::Variant)`. This immediately aborts transaction execution, reverts all state changes atomically, and passes the numeric error code to the Soroban host and client SDKs without forcing cross-contract callers or generated client bindings to unpack nested `Result` types.
- **Internal helper functions:** Use `Option<T>` or `bool` for functions that query lookup state where absence is normal rather than an exceptional error (for example, `find_merchant_id` returning `0`, or `has_role` returning `bool`).
- **Do not use plain `panic!()`:** Never use `panic!()`, `assert!()`, or `unwrap()` with a raw string in contract code. A raw panic produces an opaque host error with no numeric code, preventing SDK clients and transaction signers from identifying the root cause.

### Error naming and grouping

- Name error enums `ContractError`.
- Name variants in UpperCamelCase using explicit descriptive predicates or failure states: `NotAuthorized`, `AlreadyInitialized`, `MerchantNotFound`, `InvalidAmount`, `ContractPaused`.
- Group related error variants logically by domain within the enum: lifecycle and authorization, merchant management, invoice settlement, fees, and oracle pricing.
- Keep error codes scoped to their contract: error code `1` in `contracts/account/src/errors.rs` is `AlreadyInitialized`, whereas code `1` in `contracts/shade/src/errors.rs` is `NotAuthorized`. See the [Errors reference](../reference/errors.md) for full contract-by-contract listings.

### The append-only rule for numeric discriminants

Soroban assigns an explicit `u32` discriminant to each variant via `#[repr(u32)]`. External integrators, indexers, and client bindings rely on these numeric codes.

- **Never renumber an existing error variant:** Changing a variant's numeric value changes the on-chain interface and breaks existing clients.
- **Never reorder variants:** New error variants must be added to the end of the enum with the next unused numeric discriminant.
- **Tombstone removed variants:** If an error variant is retired, leave a numeric gap (or comment) preserving the retired number rather than shifting following variants. For example, [`contracts/shade/src/errors.rs#L20-L26`](../../contracts/shade/src/errors.rs#L20-L26) maintains gaps between variants `18`, `20`, and `27` to preserve historical numbers.

```rust
// Good: Explicit discriminant, descriptive name, panics with typed error
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum ContractError {
    NotAuthorized = 1,
    AlreadyInitialized = 2,
    InvoiceNotFound = 8,
    // Strictly append new variants:
    InvoiceNotExpired = 50,
}

pub fn assert_not_paused(env: &Env) {
    if is_paused(env) {
        panic_with_error!(env, ContractError::ContractPaused);
    }
}
```

```rust
// Bad: Raw panic with string, implicit/shifting discriminants, missing repr(u32)
pub enum BadError {
    NotAuthorized, // Implicitly 0, will shift if a variant is inserted above
    Failed,        // Vague name
}

pub fn assert_not_paused(env: &Env) {
    if is_paused(env) {
        panic!("contract is currently paused"); // Reverts with opaque error, no contract code
    }
}
```

## Events and state notifications

Every state mutation in this workspace must emit a Soroban contract event before returning.

### Topic structure and the 32-character cap

Events in this workspace derive topics automatically through the `#[contractevent]` macro on a dedicated struct.

- **Derived snake_case topic:** The SDK derives the event's topic Symbol from the struct name converted to snake_case (for example, `InvoicePaidEvent` publishes under the topic symbol `invoice_paid_event`).
- **32-character symbol cap:** Soroban limits event topic symbols to 32 characters. Keep your event struct names short enough that their snake_case representation does not exceed 32 characters.
- **Single-topic default:** All core events publish under a single topic (the derived event name). Do not define multi-element topic tuples unless an explicit indexing requirement mandates it.

### Event naming and payload contents

- Name event structs in UpperCamelCase using a past-tense verb or participle to signal a completed state transition: `InvoiceCreatedEvent`, `InvoicePaidEvent`, `TokenAddedEvent`, `ContractPausedEvent`.
- Always include an emission timestamp: Add `pub timestamp: u64` to every event payload and populate it with `env.ledger().timestamp()`.
- Include relevant entity identifiers and actor addresses: Include the actor who performed the action, affected resource IDs, token addresses, and gross/net amounts.
- Provide a companion publisher function: Define a helper `pub fn publish_<event_name>(env: &Env, ...)` alongside the struct in [`contracts/shade/src/events.rs`](../../contracts/shade/src/events.rs) or [`contracts/account/src/events.rs`](../../contracts/account/src/events.rs) so call sites remain clean. See the [Events reference](../reference/events.md) for full payload schemas.

```rust
// Good: Past tense, <= 32 chars, includes actors, amounts, and ledger timestamp
#[contractevent]
pub struct TokenAddedEvent {
    pub token: Address,
    pub timestamp: u64,
}

pub fn publish_token_added_event(env: &Env, token: Address, timestamp: u64) {
    TokenAddedEvent { token, timestamp }.publish(env);
}
```

```rust
// Bad: Present tense, exceeds 32 char symbol limit, omits timestamp
#[contractevent]
pub struct AddAcceptedTokenToAllowlistEvent { // snake_case is 33 chars -> exceeds 32-char limit
    pub token: Address,
    // Missing timestamp: u64
}

// Bad: Mutating persistent state without publishing an event
pub fn add_token(env: &Env, token: &Address) {
    let mut tokens = get_tokens(env);
    tokens.push_back(token.clone());
    env.storage().persistent().set(&DataKey::AcceptedTokens, &tokens);
    // Silent state change: off-chain indexers cannot detect this update!
}
```

## Authorization and caller verification

Soroban execution does not provide an implicit `msg.sender` or `env.caller()` context. Contracts must authenticate callers explicitly.

### Explicit caller parameters

- **Pass actors explicitly:** Public entrypoint functions that require authorization must declare the authorizing actor address explicitly as an argument (for example, `admin: Address`, `merchant: Address`, or `caller: Address`).
- **Verify immediately:** Call `.require_auth()` on the passed address at the start of the function, before reading, checking, or mutating contract storage.

### Composing admin and role checks

- **Admin checks:** Admin-gated functions call `core::assert_admin(env, &admin)`, which invokes `admin.require_auth()` and asserts that `*admin == get_admin(env)`. If the check fails, it reverts with `ContractError::NotAuthorized`.
- **Role checks:** Role-gated functions call `access_control::assert_has_role(env, &caller, role)`, which invokes `caller.require_auth()` and checks whether the address matches the admin or holds the requested role in storage.
- **Fail early:** Never perform expensive storage reads, cross-contract calls, or token transfers prior to completing authorization checks. See the [Access control guide](../security/access-control.md) for permission architecture details.

```rust
// Good: Actor passed explicitly, require_auth called first, admin identity verified
pub fn assert_admin(env: &Env, admin: &Address) {
    admin.require_auth();
    if *admin != get_admin(env) {
        panic_with_error!(env, ContractError::NotAuthorized);
    }
}

pub fn pause(env: &Env, admin: &Address) {
    core::assert_admin(env, admin);
    env.storage().persistent().set(&DataKey::Paused, &true);
    events::publish_contract_paused_event(env, admin.clone(), env.ledger().timestamp());
}
```

```rust
// Bad: Reading and mutating storage before verifying authorization
pub fn pause(env: &Env, admin: &Address) {
    env.storage().persistent().set(&DataKey::Paused, &true); // Mutation before auth!
    if *admin != get_admin(env) {
        panic_with_error!(env, ContractError::NotAuthorized);
    }
    admin.require_auth(); // Too late!
}

// Bad: Checking address equality without calling require_auth()
pub fn set_fee(env: &Env, admin: &Address, fee: i128) {
    if *admin == get_admin(env) {
        // Insecure: Anyone can pass the admin's address without their signature!
        env.storage().persistent().set(&DataKey::Fee, &fee);
    }
}
```

## Storage model and key design

All persistent data across `contracts/shade/` and `contracts/account/` is managed through typed `DataKey` enums.

### Storage key enum conventions

- Define all storage keys in an enum named `DataKey` inside `types.rs`.
- Name variants in UpperCamelCase.
- Group keys into singleton config keys (`Admin`, `Paused`, `AccountWasmHash`), indexed records (`Merchant(u64)`, `Invoice(u64)`), relational lookups (`MerchantId(Address)`, `MerchantKey(Address)`), and composite mappings (`Role(Address, Role)`, `TokenFee(Address)`).
- Remember that Soroban limits a single `#[contracttype]` enum to 50 variants.

### Storage tier selection

Soroban provides three durability tiers: `instance`, `persistent`, and `temporary`.

- **Uniform repository standard:** Both `contracts/shade` and `contracts/account` store all contract state uniformly in **persistent storage** via `env.storage().persistent()`. This covers administrative configuration, merchant records, invoices, token allowances, analytics, and ID counters.
- **Instance storage:** Reserved for small contract instance data tied to contract code deployment. In this repository, `.instance()` storage is not used in production contracts; do not introduce instance keys without an approved Architecture Decision Record.
- **Temporary storage:** Reserved for ephemeral data that can safely expire without user loss.
- For a comprehensive analysis of contract storage partitions and costs, read the [Storage model](../architecture/storage-model.md).

### Monotonic counter and ID pattern

Collections with auto-incrementing identifiers (merchants, invoices, withdrawal requests) adhere to a strict monotonic counter pattern:

- **Monotonic `u64`:** Counter keys (`DataKey::MerchantCount`, `DataKey::InvoiceCount`, `DataKey::WithdrawalCount`) store a `u64`.
- **Default to zero:** Read the counter with `.unwrap_or(0)` so the first issued identifier is always `1`.
- **Sentinel zero:** Identifier `0` is never assigned to a valid record. Treat `0` as an invalid or uninitialized sentinel value.
- **Never decrement or reuse:** Counter values increment by `1` upon each record creation. Identifiers are never decremented, deleted, or recycled.

```rust
// Good: Uniform persistent storage, monotonic counter starting at 1, atomic write
let invoice_count: u64 = env
    .storage()
    .persistent()
    .get(&DataKey::InvoiceCount)
    .unwrap_or(0);
let new_id = invoice_count + 1;

env.storage().persistent().set(&DataKey::Invoice(new_id), &invoice);
env.storage().persistent().set(&DataKey::InvoiceCount, &new_id);
```

```rust
// Bad: Decrementing counter, reusing IDs, or using 0 as a valid ID
let id = env.storage().persistent().get(&DataKey::InvoiceCount).unwrap(); // Panics on first use!
env.storage().persistent().set(&DataKey::Invoice(id), &invoice);
// Decrementing or reusing deleted IDs causes key collisions and breaks historical indexing
```

## Naming, style, and linting

Contracts in this workspace conform to standard Rust conventions and project-wide linting policies enforced by CI.

### Function verb conventions

Follow consistent naming patterns for contract methods and internal component functions:

- `get_*`: Read-only queries that retrieve state or return a record (`get_admin`, `get_invoice`, `get_merchant`, `get_fee`).
- `set_*`: State-mutating functions that write or update configuration (`set_fee`, `set_platform_account`, `set_merchant_status`).
- `is_*` / `has_*`: Read-only queries that return a `bool` (`is_accepted_token`, `is_paused`, `has_role`).
- **Domain action verbs:** Use specific action verbs for domain operations: `initialize`, `create_invoice`, `pay_invoice`, `refund_invoice`, `pause`, `unpause`, `upgrade`.

### Argument order

Structure parameter lists in a uniform order across contract traits and internal components:

1. **Environment:** `env: Env` (or `&Env` for internal functions) is always first.
2. **Actor address:** The authorizing signer (`admin: Address`, `merchant: Address`, `caller: Address`) is always second.
3. **Target / Resource ID:** Identifiers or target addresses (`invoice_id: u64`, `token: Address`).
4. **Data payloads / Amounts:** Amounts, configuration objects, or descriptions (`amount: i128`, `description: String`).

### Crate-level attributes and clippy configuration

Continuous integration enforces zero warnings on all workspace packages.

- `#![no_std]`: Mandatory at the root of every contract crate (`contracts/shade/src/lib.rs` and `contracts/account/src/lib.rs`).
- `#![allow(clippy::too_many_arguments)]`: Permitted crate-wide in `contracts/shade/src/lib.rs` because Soroban contract entrypoint functions (such as `create_invoice_signed`) legitimately require more than seven arguments.
- **CI flags:** CI checks code formatting and lints with the following commands:
  - `cargo fmt --all -- --check`
  - `cargo clippy --workspace --all-features -- -D warnings`
  - `cargo test --workspace --all-features`

```rust
// Good: Standard argument ordering (env, actor, target, amount)
fn set_fee(env: Env, admin: Address, token: Address, fee: i128);
fn refund_invoice(env: Env, merchant: Address, invoice_id: u64);
```

```rust
// Bad: Inconsistent argument ordering, actor placed at the end
fn set_fee(fee: i128, token: Address, env: Env, admin: Address);
```

## Related reading

- [Documentation style guide](documentation-style-guide.md) — Voice, formatting, and review rules for documentation pages.
- [Storage model](../architecture/storage-model.md) — Detailed breakdown of persistent storage partitions and keys.
- [Errors reference](../reference/errors.md) — Complete catalogue of contract error codes and resolutions.
- [Events reference](../reference/events.md) — Complete catalogue of topics and event payload schemas.
- [Access control](../security/access-control.md) — Role-based authorization architecture and admin management.
