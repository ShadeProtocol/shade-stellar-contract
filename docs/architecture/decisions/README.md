# Architecture decision records (ADRs)

This directory records the architectural decisions that shape the `shade-stellar-contract` workspace. Contributors, integrators, and auditors use this log to understand why the protocol is structured the way it is and which alternatives were evaluated.

First use of domain terms follows the [Glossary](../../glossary.md) ([merchant](../../glossary.md#merchant), [payer](../../glossary.md#payer), [invoice](../../glossary.md#invoice), [admin](../../glossary.md#admin)).

## When to write an ADR

Write a new ADR whenever you propose a change that affects:

1. **Contract topology or boundaries** — adding a new contract crate under `contracts/`, introducing a factory, or splitting/merging a module in `contracts/shade/src/components/`.
2. **State and storage layout** — changing how persistent keys are partitioned in `contracts/shade/src/types.rs` or how balances are isolated across contract addresses.
3. **Security and governance controls** — modifying role transfer flows, timelocks, fee bounds, pause guards, or upgrade authorization.
4. **Hub vs. standalone trade-offs** — deciding whether a feature lives inside the `shade` hub contract, as a standalone contract, or both.

To add a record, copy [`template.md`](template.md) to `NNNN-short-title.md` (using the next four-digit number), fill in every section, and add a row to the index table below in the same pull request.

## ADR index

| ADR | Title | Status |
|---|---|---|
| [ADR-0001](0001-component-based-structure-in-shade.md) | Use a component-based module structure inside the `shade` hub contract | Accepted |
| [ADR-0002](0002-factory-deployed-account-per-merchant.md) | Deploy a dedicated `account` contract per merchant through the hub | Accepted |
| [ADR-0003](0003-time-locked-fee-changes.md) | Enforce time-locked fee changes via `propose_fee` and `execute_fee` | Accepted |
| [ADR-0004](0004-two-step-admin-transfer.md) | Use a two-step propose-and-accept flow for admin role transfers | Accepted |
| [ADR-0005](0005-separate-factories-for-escrow-and-ticketing.md) | Provide dedicated factories for standalone `escrow` and `ticketing` contracts | Accepted |
| [ADR-0006](0006-hub-subscription-alongside-standalone-contract.md) | Maintain a hub subscription component alongside the standalone `subscription` contract | Accepted |

← [Back to Architecture](../README.md)
