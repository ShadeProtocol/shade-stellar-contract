# Architecture

How the Shade Protocol contracts, factories, and shared components fit together.

- Workspace and crate layout (`shade`, `account`, `escrow`, `escrow_factory`, `subscription`, `ticketing`, `ticketing_factory`, `crowdfund`, `crowdfund_factory`) — *planned*.
- The `shade` contract's component structure (`contracts/shade/src/components/`) — *planned*.
- Storage key partitioning across `DataKey`, `EventKey`, `CampaignKey`, and the other per-feature key enums — *planned*.
- [Upgradeability, WASM Hash Management, and Versioning](upgradeability.md) — the admin and governance upgrade paths, storage compatibility rules for persisted types, migration patterns, and how a deployed build is identified.

← [Back to documentation home](../README.md)
