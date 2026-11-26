# Example contracts

Contracts and modules that are not part of the core Shade payment gateway. This is a separate Cargo workspace: the root workspace excludes it, and CI, pre-commit and the docs checks do not run against it.

```bash
cd example
cargo test -p escrow
```

## Standalone contracts

`crowdfund`, `crowdfund_factory`, `escrow`, `escrow_factory`, `subscription`, `ticketing` and `ticketing_factory` were moved here unchanged. `crowdfund` and `crowdfund_factory` did not compile before the move; they now build and their tests pass.

## Code moved out of the Shade hub

Each `src/from_shade/` folder holds modules that used to live inside `contracts/shade`, along with their types, events, errors, tests and the hub entry points that exposed them (`entrypoints.rs`).

| Crate | Modules |
|---|---|
| `crowdfund` | `analytics`, `backer_rewards`, `campaign`, `campaigns`, `comments`, `cross_chain_pledge`, `fiat_goals`, `leaderboard`, `nft`, `pledge`, `stretch_goals`, `vesting`, `voting` |
| `escrow` | `escrow` |
| `subscription` | `subscription`, subscription searches |
| `ticketing` | `event`, event searches |
| `platform_extras` | `governance`, `bridge`, `multisig_withdrawal`, withdrawal proposal searches |

Calls into the gateway use the `shade` crate (`shade::components::merchant`, `shade::types::Invoice`, and so on). This code is reference material. It sits behind the `from-shade` feature, which is off by default, so the default build of each crate is unaffected. With the feature on, every crate builds. Two `crowdfund` modules were removed instead of ported (see "Removed modules" below).

## Removed modules

These `crowdfund` modules sat behind `from-shade` and depended on hub events, types and error variants that no longer exist, so they were removed instead of ported.

| Module | Needed (no longer present) | Recover from git history |
|---|---|---|
| `kyc` | `publish_kyc_*_event` functions, KYC types and error variants | introduced in commit `a887737` |
| `campaign_royalties` | `CampaignRoyaltyConfig`, `CampaignSecondarySale`, royalty error variants, `publish_campaign_*` events | introduced in commit `76a10bd` |

## Restoring a module into the hub

If you need one of these modules to run inside the shade hub contract rather than as a standalone contract, follow the step-by-step instructions in [Restoring an Example Module into the Hub](docs/restoring-a-module.md). It walks through porting components, types, events, and entry points, choosing non-colliding error discriminants, and evaluating the trade-offs in audit surface, WASM size, and storage compatibility.