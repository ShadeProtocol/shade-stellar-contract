# ADR-0004: Use a two-step propose-and-accept flow for admin role transfers

This record documents why transferring the protocol [admin](../../glossary.md#admin) role in `contracts/shade/src/components/admin.rs` requires a two-step handover (`propose_admin` / `transfer_admin` followed by acceptance by the pending admin) rather than a single-step overwrite.

## Status

Accepted

## Context

The `DataKey::Admin` address controls privileged operations across the `shade` hub and standalone contracts, including pausing the protocol, proposing fee changes, registering accepted tokens, and managing WASM hashes for factory deployments. In a single-step `set_admin(new_admin)` model, passing a mistyped address, a contract address without admin support, or an address whose private key is inaccessible permanently bricks protocol administration.

## Decision

Require a two-step admin transfer flow in `contracts/shade/src/components/admin.rs`:

1. The current admin authorizes a proposal storing the candidate address in `DataKey::PendingAdmin` while retaining active `DataKey::Admin` privileges.
2. The candidate address must explicitly call `accept_admin` (which calls `pending_admin.require_auth()`) to promote `DataKey::PendingAdmin` to `DataKey::Admin` and clear the pending slot.

## Consequences

### Positive

- **Proof of key control:** The new admin address must sign a live transaction (`accept_admin`) before the old admin loses privileges, eliminating the risk of transferring admin control to a dead or mistyped address.
- **Safe cancellation / overwrite before acceptance:** If the wrong candidate address is proposed, the current admin remains active and can propose a corrected address before acceptance occurs.

### Negative and trade-offs

- **Requires coordination between two signers:** Rotating the admin key requires two separate transactions signed by two different keys (`admin` then `pending_admin`).

## Alternatives considered

### Single-step `set_admin(new_admin)`

Overwriting `DataKey::Admin` in one call requires only one transaction, but a single typo in `new_admin` irrecoverably locks out all administrative and emergency pause functions.

### Time-delayed automatic transfer without recipient signature

Automatically promoting `PendingAdmin` after a timelock without requiring `pending_admin.require_auth()` allows recovery during the window, but still fails to verify that the destination key can actually sign Soroban transactions.

← [Back to ADR index](README.md)
