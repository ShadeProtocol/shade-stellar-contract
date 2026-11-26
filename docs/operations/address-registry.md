# Address & Hash Registry

Canonical record of deployed Shade Protocol contract addresses and expected WASM SHA-256 hashes per tagged release.

> **Note:** No release has been tagged or deployed to mainnet yet (see [`CHANGELOG.md`](../../CHANGELOG.md)). The tables below will be populated as releases are cut and deployments are recorded following the [Releases Runbook](releases.md) and [Deployment Runbook](deployment.md#artifact-recording).

## Release WASM hashes

Each release publishes `SHA256SUMS` generated from [`Dockerfile.reproducible`](../../Dockerfile.reproducible). To verify any hash independently, follow [Verifying Deployed Contracts](../security/verifying-deployments.md).

| Release tag | Commit SHA | `shade.optimized.wasm` SHA-256 | `account.optimized.wasm` SHA-256 | Toolchain |
|---|---|---|---|---|
| *Pending first release (`v0.1.0`)* | — | — | — | Rust `1.96.0`, Stellar CLI `28.0.0`, `soroban-sdk` `23.5.3` |

## Deployed contract addresses

### Mainnet

| Contract | Release tag | Contract ID | WASM SHA-256 | Deployment ledger |
|---|---|---|---|---|
| `shade` | *Not yet deployed* | — | — | — |
| `account` (template WASM) | *Not yet deployed* | *Uploaded WASM only* | — | — |

### Testnet

| Contract | Release tag | Contract ID | WASM SHA-256 | Notes |
|---|---|---|---|---|
| `shade` | *Unregistered* | — | — | Ephemeral / testing deployments |
| `account` (template WASM) | *Unregistered* | *Uploaded WASM only* | — | Ephemeral / testing deployments |

## Related pages

- [Verifying Deployed Contracts](../security/verifying-deployments.md) — how to reproduce these hashes and compare them against on-chain bytecode.
- [`CHANGELOG.md`](../../CHANGELOG.md) — release notes and breaking-change history.
- [Releases Runbook](releases.md) — how hashes are generated and published per release.
- [Deployment Runbook](deployment.md) — artifact recording template for live deployments.
