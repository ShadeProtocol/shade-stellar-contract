# ADR-0003: Enforce time-locked fee changes via `propose_fee` and `execute_fee`

This record explains why protocol fee updates in `contracts/shade/src/components/admin.rs` and `contracts/shade/src/components/platform_fee.rs` require a two-phase timelock (`propose_fee` followed by `execute_fee`) instead of taking effect immediately.

## Status

Accepted

## Context

The protocol [admin](../../glossary.md#admin) configures the platform fee basis points charged when a [payer](../../glossary.md#payer) settles an [invoice](../../glossary.md#invoice) for a [merchant](../../glossary.md#merchant). If the admin could raise fees instantaneously in a single transaction, an admin key compromise — or a sudden unannounced policy change — could front-run in-flight merchant invoices and extract unexpected fees before merchants or payers could react.

## Decision

Split fee updates into a time-locked propose-and-execute workflow in `contracts/shade/src/components/admin.rs`:

1. `propose_fee` validates admin authorization and fee bounds, records a pending fee proposal with an earliest execution timestamp (`effective_at`), and emits a proposal event without changing the active fee.
2. `execute_fee` can only succeed after the ledger timestamp reaches or exceeds the proposal's `effective_at` timelock window, at which point it promotes the pending fee to active storage.

> **Note:** Fee changes take effect only after `execute_fee` is called; `propose_fee` alone does not change the active fee.

## Consequences

### Positive

- **Protection against fee front-running:** Merchants and payers have an observable on-chain window between `propose_fee` and `execute_fee` to review upcoming fee adjustments before they apply to invoice settlements.
- **Reduced blast radius for key compromise:** Even if an admin key is compromised, an attacker cannot silently spike the fee in the same block as a large invoice payment.

### Negative and trade-offs

- **Two transactions required for every fee adjustment:** Operators must submit `propose_fee`, wait for the timelock duration to elapse, and submit `execute_fee`.
- **Additional state tracking:** The contract stores pending fee proposal metadata alongside the active fee configuration in `contracts/shade/src/types.rs`.

## Alternatives considered

### Immediate single-step `set_fee`

Allowing `set_fee` to overwrite the active fee immediately is simpler to operate, but gives merchants zero advance notice and allows instantaneous fee manipulation on pending invoices.

### Full token-holder DAO vote for every fee parameter tweak

Requiring a multi-day governance quorum vote (`governance.rs`) for routine fee adjustments within pre-approved bounds adds heavy coordination overhead compared to a deterministic administrative timelock bounded by a hard maximum fee cap.

← [Back to ADR index](README.md)
