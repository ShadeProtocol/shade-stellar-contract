# ADR-0002: Deploy a dedicated `account` contract per merchant through the hub

This record documents why each [merchant](../../glossary.md#merchant) receives an isolated `account` contract instance deployed via `contracts/shade/src/components/account_factory.rs` instead of holding all merchant balances in a single pooled vault inside `shade`.

## Status

Accepted

## Context

When a [payer](../../glossary.md#payer) settles an [invoice](../../glossary.md#invoice), the net proceeds belong to the merchant and may remain on-chain until the merchant withdraws them, triggers an automatic withdrawal threshold, or issues a refund (`refund_invoice`, `refund_invoice_partial`, or `claim_refund`). Storing all merchant token balances inside the `shade` hub's own token account would commingle funds across every merchant on the protocol.

## Decision

Deploy a dedicated `account` contract (`contracts/account/src/lib.rs`) for each registered merchant through the hub's `account_factory` component (`contracts/shade/src/components/account_factory.rs`) using a registered account WASM hash and deterministic salt.

- Token transfers from `platform_fee::route_from_payer` route the merchant's share directly to the merchant's deployed `account` contract address.
- `shade` interacts with the merchant's vault strictly through narrow cross-contract client traits (`MerchantAccountRefund`, `MerchantAccountContract`, and `withdraw_to`) when executing authorized refunds, account restrictions, or withdrawals.

## Consequences

### Positive

- **Per-merchant balance isolation:** Every merchant's funds live at a distinct Soroban contract address, preventing accounting bugs or restrictions on one merchant from affecting any other merchant's held tokens.
- **Deterministic on-chain auditing:** Integrators and auditors can inspect any merchant's vault balance directly via standard SEP-41 token `balance` queries against that merchant's `account` address.
- **Scoped restriction controls:** The [admin](../../glossary.md#admin) can restrict or unrestrict a specific merchant vault via `restrict_merchant_account` without pausing the entire protocol.

### Negative and trade-offs

- **Deployment and TTL cost per merchant:** Registering a merchant and deploying an `account` instance consumes additional ledger footprint and requires an uploaded account WASM hash (`set_account_wasm_hash`).
- **Cross-contract call on refunds and withdrawals:** While invoice payment transfers tokens directly from payer to the merchant account address, refunds and hub-initiated withdrawals require a cross-contract call from `shade` into `account`.

## Alternatives considered

### Single pooled vault inside `shade` with internal balance mapping

Tracking merchant balances in a `DataKey::MerchantBalance(u64, Address)` map while holding all tokens at the `shade` contract address avoids deploying per-merchant contracts. However, it concentrates all merchant capital in a single token balance — a critical blast-radius risk if any withdrawal path miscalculates an internal balance.

### Externally managed EOA / wallet addresses only

Requiring merchants to receive payments only into an external Stellar account removes the `account` contract entirely, but prevents the protocol from enforcing on-chain refund escrows, multi-sig withdrawal proposals (`multisig_withdrawal.rs`), and automated threshold sweeps (`auto_withdrawal.rs`).

← [Back to ADR index](README.md)
