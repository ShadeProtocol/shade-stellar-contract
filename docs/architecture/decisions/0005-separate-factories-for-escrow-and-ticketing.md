# ADR-0005: Provide dedicated factories for standalone `escrow` and `ticketing` contracts

This record explains why the workspace includes dedicated factory crates (`contracts/escrow_factory` and `contracts/ticketing_factory`) to deploy standalone `escrow` and `ticketing` contract instances alongside `contracts/crowdfund_factory`.

## Status

Accepted

## Context

Integrators using Shade for two-party [payer](../../glossary.md#payer)/[merchant](../../glossary.md#merchant) deals (`contracts/escrow`) or event ticket sales (`contracts/ticketing`) often want each escrow agreement or ticketed event series isolated in its own Soroban contract instance with its own [admin](../../glossary.md#admin), token custody, and lifecycle. If all standalone deals and events were deployed through a single universal factory contract, that factory would have to store multiple unrelated WASM hashes, expose heterogeneous initialization signatures, and couple upgrades across unrelated product verticals.

## Decision

Maintain separate, domain-specific factory contracts — `contracts/escrow_factory/src/lib.rs` for `contracts/escrow` and `contracts/ticketing_factory/src/lib.rs` for `contracts/ticketing` (matching `contracts/crowdfund_factory/src/lib.rs` for `contracts/crowdfund`).

- Each factory manages a single target WASM hash, tracks only the instances deployed for its domain, and validates domain-specific deployment salts and constructor arguments.

## Consequences

### Positive

- **Independent WASM lifecycle and upgrades:** Updating the `ticketing` contract WASM hash in `ticketing_factory` has zero impact on `escrow_factory` deployments or storage layout.
- **Minimal, auditable factory surface:** Each factory crate stays small and self-contained, making its deterministic deployment and registry indexing easy to verify.
- **Fault isolation across verticals:** Pausing or upgrading one factory does not block deployments in other verticals.

### Negative and trade-offs

- **Multiple factory contracts to deploy and index:** Operators who support all standalone verticals deploy and maintain separate factory addresses (`escrow_factory`, `ticketing_factory`, `crowdfund_factory`).

## Alternatives considered

### A single universal factory contract for all standalone crates

Combining escrow, ticketing, and crowdfunding deployments into one multi-type factory reduces the number of factory addresses to track, but couples storage keys, WASM hash management, and initialization ABIs across unrelated contracts.

### Manual client-side WASM deployment without on-chain factories

Having clients deploy `escrow` or `ticketing` WASM directly via host functions avoids factory contracts, but loses an on-chain registry of authentic protocol-deployed instances and requires every client to manage WASM hashes directly.

← [Back to ADR index](README.md)
