# Shade Protocol Documentation

Shade is a decentralized payment gateway built on Stellar/Soroban that lets merchants accept invoices, subscriptions, event tickets, and crowdfunding pledges in crypto, with fee, escrow, KYC, and governance logic enforced entirely on-chain. This is the documentation home for the `shade-stellar-contract` repository: everything you need to build, integrate with, operate, or extend the protocol lives under one of the sections below.

## Where do I start?

- **Contract contributors** (writing or reviewing Rust/Soroban code): start with [Getting Started → Building](getting-started/building.md), then read [Architecture](architecture/README.md) and the [Glossary](glossary.md) before diving into [Contracts](contracts/README.md).
- **Merchant integrators** (building an app or backend against Shade): start with [Getting Started](getting-started/README.md), then [Guides](guides/README.md) for task-oriented walkthroughs and [Reference](reference/README.md) for the full contract interface.
- **Protocol operators** (running deployments, admin/governance duties): start with [Operations](operations/README.md) and [Security](security/README.md).

## Table of contents

| Section | What lives here |
|---|---|
| [Getting Started](getting-started/README.md) | Installing tooling, building the workspace, running a local deployment. |
| [Architecture](architecture/README.md) | How the contracts, factories, and components fit together. |
| [Contracts](contracts/README.md) | Per-contract reference pages (Shade, Account, Escrow, Subscription, Ticketing, Crowdfund, and their factories). |
| [Concepts](concepts/README.md) | Deep dives into individual mechanisms (fees, escrow, vesting, dynamic pricing, governance, KYC, etc.). |
| [Security](security/README.md) | Threat model, access control, reentrancy protections, and audit-relevant notes. |
| [Reference](reference/README.md) | Glossary, error codes, storage key layout, and other lookup material. |
| [Guides](guides/README.md) | Task-oriented how-tos for common integration and contribution tasks. |
| [Operations](operations/README.md) | Runbooks for deploying, pausing, upgrading, and administering live contracts. |

## Glossary

Domain and Soroban terminology used throughout these docs is defined in [`docs/glossary.md`](glossary.md). Link to it on first use of a term, per the [style guide](contributing/documentation-style-guide.md#terminology).

## Contributing to the docs

See the [Documentation Style Guide](contributing/documentation-style-guide.md) for tone, formatting, and terminology rules, and [`docs/contributing/templates/`](contributing/templates/) for the page templates to start a new page from.
