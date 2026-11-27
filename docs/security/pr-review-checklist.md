# Security review checklist for contract PRs

Every pull request that modifies contract code under `contracts/` requires a structured security review before merge, not only during external audits. Complete the applicable checks below when you open a PR, and verify each item against the diff, the [threat model](threat-model.md), and the protocol's [documented invariants](audit-package.md#architecture-trust-assumptions-and-invariants) when reviewing.

> **Security:** Pull requests touching payment, refund, withdrawal, authorization, or storage layout code under `contracts/*/src/` require at least one maintainer approval and an independent second review — see [Review expectations](../../CONTRIBUTING.md#review-expectations).

## Reference documents

Review these documents alongside any contract diff:

- [Threat model and security assumptions](threat-model.md) — assets at risk, trust boundaries, attacker classes, and attack surfaces.
- [Security audit readiness package](audit-package.md#architecture-trust-assumptions-and-invariants) — consolidated protocol invariants (invoice amount/nonce preservation, cumulative refund bounds, merchant account isolation, and security boundaries).
- [Access control and roles](access-control.md) — `Admin`, `Manager`, `Operator`, and caller permission matrix.
- [Pausable emergency-stop mechanism](pausable.md) — functions blocked vs allowed while paused.
- [Reentrancy protection](reentrancy.md) — `reentrancy::enter`/`exit` guard rules and guarded paths.
- [Amounts, token decimals, and rounding](../concepts/amounts-and-rounding.md) — rounding direction, overflow protection, and negative-amount checks.
- [Refunds and voids](../concepts/refunds-and-voids.md) — cumulative refund invariant (`amount_refunded + amount <= invoice.amount`).
- [Upgradeability and storage compatibility](../architecture/upgradeability.md#storage-compatibility-rules) — `#[contracttype]` and `#[repr(u32)]` backward-compatibility rules.
- [Bounded reads and pagination policy (ADR 0009)](../architecture/decisions/0009-bounded-reads-and-pagination.md) — cursor pagination, constant-size storage entries, and pure read functions.

## Authorization

Every state-changing entrypoint must verify both **who signed** the transaction and **who is permitted** to run the operation before reading or mutating state.

- [ ] **Explicit signer parameter:** Public state-changing functions accept the authorizing actor (`admin: Address`, `merchant: Address`, `payer: Address`, or `caller: Address`) explicitly.
- [ ] [`require_auth`](../glossary.md#require_auth) **called early:** Every state-changing function calls `require_auth()` on the authorizing address before performing storage writes, cross-contract calls, or token transfers (or verifies an Ed25519 signature and unused 32-byte nonce via `signature_util`, as in `create_invoice_signed`).
- [ ] **Right address checked:** Admin-gated functions call `core::assert_admin(env, &admin)` (or `core::get_admin` + `core::assert_admin` in `upgrade`), role-gated functions call `access_control::assert_has_role(env, &caller, role)`, [merchant](../glossary.md#merchant) operations verify the registered merchant ID/address, and `account` vault methods enforce `get_merchant` or `get_manager` (`shade` hub) authorization.
- [ ] **Two-step privileged handovers:** Changes to [admin](../glossary.md#admin) ownership preserve the propose/accept pattern (`propose_admin_transfer` / `accept_admin_transfer`) rather than transferring authority in one step. See [Admin ownership](admin-and-ownership.md).
- [ ] **No cross-contract auth confusion:** When `shade` invokes `account` or a SEP-41 token, the contract passes `env.current_contract_address()` only for funds it holds or manager actions it is authorized to perform.

## Pause and reentrancy

Emergency-stop and reentrancy protections must match the [threat model](threat-model.md) and [reentrancy policy](reentrancy.md).

- [ ] **Pause guard applied:** Every state-changing entrypoint in `contracts/shade/src/shade.rs` calls `pausable_component::assert_not_paused(&env)` at the top of the method, unless it is intentionally exempt for initialization, incident recovery, or admin continuity (`initialize`, `pause`, `unpause`, `upgrade`, `propose_admin_transfer`, `accept_admin_transfer`, `grant_role`, `revoke_role`, `set_account_wasm_hash`, `restrict_merchant_account`, `set_merchant_status`, `verify_merchant`, `set_merchant_key`). See the [pausable function table](pausable.md#blocked-vs-allowed-while-paused).
- [ ] **Read-only queries remain unpaused:** View functions (`get_*`, `is_*`, `has_*`, `search_*`, `calculate_fee`) do not call `assert_not_paused`.
- [ ] **Reentrancy guard where required:** Functions that make external cross-contract calls or token callbacks and require mutex protection wrap execution with `reentrancy::enter(&env)` before external calls and `reentrancy::exit(&env)` before returning. See [Reentrancy protection](reentrancy.md) and the [audit package risk notes](audit-package.md#known-issues-and-accepted-risks).
- [ ] **No stuck guard paths:** `reentrancy::exit(&env)` executes on every non-panicking return path (Soroban automatically rolls back `DataKey::ReentrancyStatus` on panic).

## Arithmetic

All monetary calculations use `i128` base units and must prevent overflows, underflows, negative inputs, and rounding exploits.

- [ ] **Checked or overflow-safe operations:** Arithmetic operations guard against overflow (using checked arithmetic such as `checked_add`, `checked_sub`, or `checked_mul` where returning a typed `ContractError` is required, and relying on `overflow-checks = true` in [`Cargo.toml`](../../Cargo.toml) for release traps).
- [ ] **No negative or zero amounts:** Amount-taking entrypoints validate `amount > 0` (rejecting `amount <= 0` with `ContractError::InvalidAmount`) and enforce upper bounds such as `amount_paid + amount <= invoice.amount` and `amount_refunded + amount <= invoice.amount`.
- [ ] **Correct rounding direction:** Integer division truncates toward zero (floor) on fee splits (`amount * fee_bps / 10_000`) and fiat [invoice](../glossary.md#invoice) quotes so remainders favor the protocol invariant documented in [Amounts, token decimals, and rounding](../concepts/amounts-and-rounding.md#rounding-rules-by-path).
- [ ] **Fee bounds preserved:** Payment and invoice creation paths verify that `platform_fee < amount` (`merchant_amount = amount - platform_fee > 0`) and per-merchant fee overrides enforce `0..=10_000` basis points.
- [ ] **Safe integer casts:** Casts between signed and unsigned types (such as `u128` filter bounds to `i128`) are range-checked or documented when bounds exceed `i128::MAX`.

## Token transfers

Fund movements via SEP-41 `TokenClient` must match on-chain accounting and prevent unauthorized or mismatched transfers.

- [ ] **Correct `from` and `to` addresses:** Transfers move tokens between the intended parties (for payments, [`payer`](../glossary.md#payer) directly to `merchant_account` and `platform_account` via `transfer` or `transfer_from`; for refunds, `merchant_account` to `payer`) and never pull from an arbitrary third party.
- [ ] **Transfer amounts match accounting:** The split transfers sum to the gross amount (`platform_fee + merchant_amount == amount`), and stored `Invoice` fields (`amount_paid`, `amount_refunded`, `status`) and analytics totals match the transferred amounts.
- [ ] **Transfers ordered safely with state updates:** Validation and eligibility checks run before external token transfers, and state updates + reentrancy protections follow checks-effects-interactions where required so a callback cannot exploit stale state (see [Reentrancy protection](reentrancy.md#contributor-rules)).
- [ ] **Token allowlist enforced:** Payments and invoice creation verify the token against the global accepted-token list (`DataKey::AcceptedTokens`) and, where applicable, the merchant's accepted-token allowlist.
- [ ] **Account restriction respected:** Refunds, withdrawals, and sweeps on `account` check `!is_restricted_account(&env)` before transferring tokens out.

## Storage

[Soroban](../glossary.md#soroban) storage layout changes are permanent across upgrades and bounded by ledger resource limits.

- [ ] **No unbounded vectors or maps in single keys:** New storage keys never store an append-only `Vec`, `Map`, or `Set` that grows with usage under a single [`DataKey`](../glossary.md#datakey) (per [ADR 0009](../architecture/decisions/0009-bounded-reads-and-pagination.md#rule-2-no-storage-entry-may-grow-with-usage)). Scalable collections use monotonic `u64` IDs (`DataKey::<Entity>(u64)` + `<Entity>Count`).
- [ ] **Bounded reads and pagination:** List or search functions use cursor-based pagination with `PageInfo` and enforce `MAX_PAGE_SIZE`; read functions perform zero storage writes, zero `extend_ttl` calls, and zero event emissions.
- [ ] **Consistent storage tier:** Production state uses `env.storage().persistent()` consistently, as documented in the [Storage model](../architecture/storage-model.md#storage-tier-usage-in-this-contract).
- [ ] **Enum variant cap respected:** Every `#[contracttype]` enum (including `DataKey` and domain key enums) remains at or below Soroban's 50-variant limit.
- [ ] **Upgrade-compatible layout:** No existing `DataKey` variant is renamed, retyped, or moved across enums; no `#[repr(u32)]` enum variant is renumbered; no field is added to, removed from, or renamed on an already-persisted `#[contracttype]` struct without a migration shipping in the same WASM. See [Upgradeability](../architecture/upgradeability.md#storage-compatibility-rules) and [Storage migrations](../contributing/storage-migrations.md).

## Events and errors

Off-chain indexers and client SDKs depend on stable error codes and complete [event](../glossary.md#event) logs.

- [ ] **Events emitted after state changes:** Every state-changing function publishes a `#[contractevent]` (defined in `events.rs` via a `publish_*` helper) **after** validation and storage writes succeed, and includes relevant IDs, actors, amounts, and `timestamp: u64`.
- [ ] **Event topic length:** Derived snake_case event topic names do not exceed Soroban's 32-character `Symbol` limit.
- [ ] **Typed errors appended only:** All failure paths use `panic_with_error!(env, ContractError::...)` rather than raw `panic!()` or `unwrap()`. New `#[contracterror]` variants in [`contracts/shade/src/errors.rs`](../../contracts/shade/src/errors.rs) or [`contracts/account/src/errors.rs`](../../contracts/account/src/errors.rs) are appended with new unique `u32` discriminants; existing codes are never renumbered or reused.
- [ ] **Reference docs synced:** Any new or modified public function, type, event, or error variant is updated in [`docs/reference/`](../reference/README.md) and passes `python3 scripts/check_reference_docs.py`.

## Tests

Contract changes must prove both intended behavior and failure modes in the workspace test suite.

- [ ] **Negative authorization tests:** Tests verify that unauthorized callers (missing auth, wrong address, wrong role, non-merchant, or non-admin) fail with `NotAuthorized` or an auth trap.
- [ ] **Invalid input and boundary tests:** Tests cover zero/negative amounts (`InvalidAmount`), expired invoices (`InvoiceExpired`), invalid statuses (`InvalidInvoiceStatus`), over-payment and over-refund attempts, unaccepted tokens, and arithmetic boundaries.
- [ ] **Guard and lifecycle tests:** Tests verify `ContractPaused` behavior when paused, reentrancy rejection (`ContractError::Reentrancy`) for newly guarded paths, and pre/post-upgrade storage compatibility when storage types or upgrade logic change.
- [ ] **Clean workspace verification:** `cargo fmt --all -- --check`, `cargo clippy --workspace --all-features -- -D warnings`, and `cargo test --workspace --all-features` pass, `make fixtures` was run if `contracts/account/` changed, and no `test_snapshots/` artifacts are staged.

## Related pages

- [Threat model and security assumptions](threat-model.md)
- [Security audit readiness package](audit-package.md)
- [Contract coding conventions](../contributing/coding-conventions.md)
- [Pull request template](../../.github/pull_request_template.md)
- [Contributing guidelines](../../CONTRIBUTING.md#pre-pr-checklist)
