# ADR-0006: Maintain a hub subscription component alongside the standalone `subscription` contract

This record explains why the repository maintains both the built-in subscription module inside the `shade` hub (`contracts/shade/src/components/subscription.rs`) and the standalone `contracts/subscription` crate (`contracts/subscription/src/lib.rs`).

## Status

Accepted

## Context

Recurring billing serves two distinct integration profiles in Shade:

1. **Full-suite hub integrators:** A [merchant](../../glossary.md#merchant) already registered in the `shade` hub who accepts one-off [invoice](../../glossary.md#invoice) payments, uses a hub-deployed `account` vault, and wants recurring [payer](../../glossary.md#payer) charges routed through the same merchant ID, accepted-token whitelist, and platform fee rules.
2. **Self-contained recurring billing deployments:** An integrator or merchant who wants a dedicated, isolated recurring-billing contract (`contracts/subscription`) with its own [admin](../../glossary.md#admin), plan catalog, grace-period rules, and plan upgrade/downgrade flows without deploying or depending on the full 34-component `shade` hub.

## Decision

Keep both implementations in the workspace as explicit, parallel options:

- **Hub subscription component (`contracts/shade/src/components/subscription.rs`):** Integrated directly into `shade` storage (`contracts/shade/src/types.rs`), sharing merchant registration, pause guards, and hub fee routing.
- **Standalone `subscription` contract (`contracts/subscription/src/lib.rs`):** A self-contained Soroban contract with its own storage namespace, plan management, grace periods, and billing lifecycle that does not make cross-contract calls into `shade`.

As documented in [`docs/architecture/overview.md`](../overview.md), `shade` never calls into `contracts/subscription`; integrators choose the deployment model that matches their architecture.

## Consequences

### Positive

- **Right-sized deployment options:** Lightweight integrators can deploy `contracts/subscription` alone with a small WASM footprint, while full platform deployments can use `shade`'s integrated subscription component alongside invoices and merchant vaults.
- **Zero cross-contract coupling:** Because neither contract calls the other, changes to standalone subscription features (such as standalone plan proration or grace-period handling) cannot break `shade` hub storage invariants.

### Negative and trade-offs

- **Parallel maintenance:** Bug fixes or conceptual improvements to recurring billing logic must be evaluated for both `contracts/shade/src/components/subscription.rs` and `contracts/subscription/src/lib.rs`, and documentation must clearly distinguish which surface an integrator is calling.

## Alternatives considered

### Hub-only subscriptions (removing `contracts/subscription`)

Removing the standalone crate eliminates duplicate recurring-billing concepts, but forces every integrator who only needs recurring billing to deploy the entire `shade` hub and `account` factory stack.

### Delegating hub subscriptions to `contracts/subscription` via cross-contract calls

Having `shade` forward subscription calls to a deployed `contracts/subscription` instance unifies the billing code, but introduces cross-contract call overhead, split storage namespaces, and complex authorization forwarding whenever a hub subscription charge interacts with hub merchant accounts and platform fees.

← [Back to ADR index](README.md)
