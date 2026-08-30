# Protocol Glossary

Alphabetised definitions of the domain and Soroban terms used across Shade Protocol documentation and source. Link to a term here on its first use in any page — see the [style guide](contributing/documentation-style-guide.md#terminology).

## Domain terms

### Accepted Token

A token address the protocol admin has whitelisted for payments. Invoices, subscriptions, and campaigns may only settle in an accepted token.

- Storage: `DataKey::AcceptedTokens` — [`contracts/shade/src/types.rs:40`](../contracts/shade/src/types.rs#L40)
- Interface: `add_accepted_token`, `add_accepted_tokens`, `remove_accepted_token`, `is_accepted_token` — [`contracts/shade/src/interface.rs:27-30`](../contracts/shade/src/interface.rs#L27-L30)

### Admin

The single address with contract-wide administrative authority: whitelisting tokens, setting fees, verifying merchants, pausing the contract, and more. Transferred via a two-step propose/accept handover rather than a direct write.

- Storage: `DataKey::Admin`, `DataKey::PendingAdmin` — [`contracts/shade/src/types.rs:35-36`](../contracts/shade/src/types.rs#L35-L36)
- Interface: `get_admin`, `propose_admin_transfer`, `accept_admin_transfer` — [`contracts/shade/src/interface.rs:163-166`](../contracts/shade/src/interface.rs#L163-L166)
- Concept: `docs/concepts/` admin & governance page (planned)

### Bridge Payload

The data describing a cross-chain transfer request or a confirmed external-chain deposit, used by Shade's bridge-listener integration. Two shapes exist: `CrossChainBridgePayload` (outbound intent) and `BridgeDeposit` (a confirmed, de-duplicated inbound credit).

- Types: `CrossChainBridgePayload` — [`contracts/shade/src/types.rs:711-723`](../contracts/shade/src/types.rs#L711-L723); `BridgeDeposit` — [`contracts/shade/src/types.rs:730-741`](../contracts/shade/src/types.rs#L730-L741)
- Storage: `BridgeKey` — [`contracts/shade/src/types.rs:287-301`](../contracts/shade/src/types.rs#L287-L301)
- Interface: `emit_bridge_placeholder`, `record_bridge_deposit`, `is_bridge_deposit_processed` — [`contracts/shade/src/interface.rs:209,224-232,238`](../contracts/shade/src/interface.rs#L209)
- Concept: `docs/concepts/` cross-chain bridge page (planned)

### DataKey

The core payment-engine storage-key enum (admin, merchants, invoices, subscriptions, fees, analytics, escrow). Soroban caps every `#[contracttype]` enum at 50 cases, so Shade partitions storage keys into one dedicated enum per feature domain (`EventKey`, `CampaignKey`, `BackerKey`, `StretchKey`, `VestingKey`, `FiatGoalKey`, `AnalyticsKey`, `NftKey`, `GovKey`, `BridgeKey`, `MultiSigKey`) rather than one monolithic enum.

- Definition and rationale: [`contracts/shade/src/types.rs:1-84`](../contracts/shade/src/types.rs#L1-L84)
- Concept: `docs/architecture/` storage layout page (planned)

### Draft Invoice

An invoice created in `InvoiceStatus::Draft` state via `create_invoice_draft`, editable by the merchant before it is finalized and made payable by `finalize_invoice`.

- Interface: `create_invoice_draft`, `finalize_invoice` — [`contracts/shade/src/interface.rs:67-75`](../contracts/shade/src/interface.rs#L67-L75)
- Type: `InvoiceStatus::Draft` — [`contracts/shade/src/types.rs:358-366`](../contracts/shade/src/types.rs#L358-L366)

### Dynamic Pricing

Time-based ticket pricing for events: an early-bird discount before a cutoff timestamp, and a late markup after it, both expressed in basis points.

- Interface: `configure_dynamic_pricing`, `get_current_ticket_price` — [`contracts/shade/src/interface.rs:292-300`](../contracts/shade/src/interface.rs#L292-L300)
- Type: `Event` fields `early_bird_end`, `early_bird_discount_bps`, `late_markup_bps` — [`contracts/shade/src/types.rs:604-627`](../contracts/shade/src/types.rs#L604-L627)

### Escrow

A buyer/seller-funded holding of tokens tied to an optional invoice, released to the seller or refunded to the buyer as the deal resolves. Shade's current implementation resolves escrow directly between buyer and seller (`release_escrow` callable by the buyer, `refund_escrow` callable by the seller) — see [Escrow Arbiter](#escrow-arbiter) for why there's no separate third-party arbiter role in the code today.

- Type: `Escrow`, `EscrowStatus` — [`contracts/shade/src/types.rs:567-590`](../contracts/shade/src/types.rs#L567-L590)
- Interface: `create_escrow`, `fund_escrow`, `release_escrow`, `refund_escrow` — [`contracts/shade/src/interface.rs:510-529`](../contracts/shade/src/interface.rs#L510-L529)
- Concept: `docs/concepts/` escrow page (planned)

### Escrow Arbiter

A third-party role empowered to resolve a disputed escrow, distinct from the buyer and seller. **Not currently implemented**: as of this writing, `contracts/shade/src/components/escrow.rs` and the `ShadeTrait` escrow functions resolve every escrow through the buyer (`release_escrow`) or the seller (`refund_escrow`) directly, with no arbiter address or dispute function. Document this term here so future arbiter work has a fixed place to link from; do not describe arbiter behavior on other pages until it exists in code.

- Related code: [`contracts/shade/src/components/escrow.rs`](../contracts/shade/src/components/escrow.rs)

### Fiat Pricing

An invoice or campaign goal denominated in a fiat currency (e.g. USD) rather than raw token units, valued against an oracle price at the moment of the relevant action.

- Type: `FiatPricing`, `FiatPricingData` — [`contracts/shade/src/types.rs:376-394`](../contracts/shade/src/types.rs#L376-L394); `CampaignFiatGoal`, `FiatGoalQuote` — [`contracts/shade/src/types.rs:1205-1262`](../contracts/shade/src/types.rs#L1205-L1262)
- Interface: `create_fiat_invoice` — [`contracts/shade/src/interface.rs:57-66`](../contracts/shade/src/interface.rs#L57-L66)
- Concept: `docs/concepts/` fiat-pegged goals page (planned)

### Invoice

A payable request created by a merchant for a fixed amount (crypto or fiat-pegged) in an accepted token, tracked through its [Invoice Status](#invoice-status) lifecycle.

- Type: `Invoice` — [`contracts/shade/src/types.rs:396-413`](../contracts/shade/src/types.rs#L396-L413)
- Interface: `create_invoice`, `get_invoice`, `pay_invoice` — [`contracts/shade/src/interface.rs:49-56,87,139`](../contracts/shade/src/interface.rs#L49-L56)

### Invoice Status

The lifecycle state of an invoice: `Pending`, `Paid`, `Cancelled`, `Refunded`, `PartiallyRefunded`, `PartiallyPaid`, or `Draft`.

- Type: `InvoiceStatus` — [`contracts/shade/src/types.rs:355-366`](../contracts/shade/src/types.rs#L355-L366)

### Merchant

A registered business or individual that can create invoices, subscription plans, events, and campaigns, and receive payments. Identified both by wallet address and by a numeric merchant ID.

- Type: `Merchant` — [`contracts/shade/src/types.rs:338-353`](../contracts/shade/src/types.rs#L338-L353)
- Interface: `register_merchant`, `get_merchant`, `is_merchant` — [`contracts/shade/src/interface.rs:41-44`](../contracts/shade/src/interface.rs#L41-L44)

### Merchant Account

The per-merchant `account` contract instance (deployed from the `account` crate) that holds and disburses a merchant's funds, distinct from the merchant's own wallet address.

- Storage: `DataKey::MerchantAccount(u64)` — [`contracts/shade/src/types.rs:62`](../contracts/shade/src/types.rs#L62)
- Interface: `set_merchant_account`, `get_merchant_account` — [`contracts/shade/src/interface.rs:132-133`](../contracts/shade/src/interface.rs#L132-L133)
- Related crate: [`contracts/account/`](../contracts/account/)

### Merchant Key

A 32-byte key a merchant registers to verify signed invoices and webhook payloads, distinct from their on-chain address.

- Storage: `DataKey::MerchantKey(Address)` — [`contracts/shade/src/types.rs:57`](../contracts/shade/src/types.rs#L57)
- Interface: `set_merchant_key`, `get_merchant_key` — [`contracts/shade/src/interface.rs:91-92`](../contracts/shade/src/interface.rs#L91-L92)

### Oracle Config

The price-feed contract and decimal configuration Shade uses to convert between a token's base units and a fiat currency.

- Type: `OracleConfig` — [`contracts/shade/src/types.rs:448-454`](../contracts/shade/src/types.rs#L448-L454)
- Interface: `set_token_oracle`, `get_token_oracle` — [`contracts/shade/src/interface.rs:36-37`](../contracts/shade/src/interface.rs#L36-L37)

### Payment Payload

Describes how an incoming payment should be routed and settled: the input token, the settlement token, a direct or swap `PaymentRoute`, and an optional maximum slippage in basis points.

- Type: `PaymentPayload`, `PaymentRoute` — [`contracts/shade/src/types.rs:642-681`](../contracts/shade/src/types.rs#L642-L681)
- Interface: `validate_payment_payload` — [`contracts/shade/src/interface.rs:142`](../contracts/shade/src/interface.rs#L142)

### Payer

The address paying an invoice or subscription charge. Recorded on the `Invoice` as `payer` once payment lands. See also [Merchant](#merchant) (the receiving party).

- Type: `Invoice::payer` — [`contracts/shade/src/types.rs:405`](../contracts/shade/src/types.rs#L405)
- Interface: `pay_invoice`, `pay_invoices_batch`, `pay_invoice_partial` — [`contracts/shade/src/interface.rs:139-141`](../contracts/shade/src/interface.rs#L139-L141)

### Pending Fee (time-locked)

A proposed fee change for a token that must be explicitly executed after being proposed, giving merchants advance notice before a new fee takes effect.

- Type: `PendingFee` — [`contracts/shade/src/types.rs:479-485`](../contracts/shade/src/types.rs#L479-L485)
- Storage: `DataKey::PendingTokenFee(Address)` — [`contracts/shade/src/types.rs:51`](../contracts/shade/src/types.rs#L51)
- Interface: `propose_fee`, `execute_fee`, `get_pending_fee` — [`contracts/shade/src/interface.rs:38-40`](../contracts/shade/src/interface.rs#L38-L40)

### Platform Account

The protocol-level address that receives platform fees split out of merchant payments.

- Storage: `DataKey::PlatformAccount` — [`contracts/shade/src/types.rs:43`](../contracts/shade/src/types.rs#L43)
- Interface: `set_platform_account`, `get_platform_account` — [`contracts/shade/src/interface.rs:34-35`](../contracts/shade/src/interface.rs#L34-L35)

### Protocol Fee

The fee (in basis points or a flat amount) the protocol takes from a payment before crediting the merchant, computed per token and optionally overridden per merchant.

- Storage: `DataKey::FeeInBasisPoints`, `DataKey::FeeAmount`, `DataKey::TokenFee`, `DataKey::MerchantPlatformFee` — [`contracts/shade/src/types.rs:47-53`](../contracts/shade/src/types.rs#L47-L53)
- Type: `PlatformFeeSplit` — [`contracts/shade/src/types.rs:657-663`](../contracts/shade/src/types.rs#L657-L663)
- Interface: `calculate_fee`, `compute_platform_fee_split` — [`contracts/shade/src/interface.rs:108-114`](../contracts/shade/src/interface.rs#L108-L114)

### Reentrancy Guard

A storage-flag guard (`DataKey::ReentrancyStatus`) that a fund-moving function sets on entry and clears on exit, panicking if it's already set — preventing a call from re-entering itself via a token contract callback.

- Storage: `DataKey::ReentrancyStatus` — [`contracts/shade/src/types.rs:41`](../contracts/shade/src/types.rs#L41)
- Implementation: [`contracts/shade/src/components/reentrancy.rs`](../contracts/shade/src/components/reentrancy.rs)
- Concept: `docs/security/` reentrancy page (planned)

### Role

An access-control tier grantable to any address beyond the single `Admin`: `Manager` or `Operator`, each authorized for a narrower set of privileged calls.

- Type: `Role` — [`contracts/shade/src/types.rs:433-439`](../contracts/shade/src/types.rs#L433-L439)
- Storage: `DataKey::Role(Address, Role)` — [`contracts/shade/src/types.rs:44`](../contracts/shade/src/types.rs#L44)
- Interface: `grant_role`, `revoke_role`, `has_role` — [`contracts/shade/src/interface.rs:93-95`](../contracts/shade/src/interface.rs#L93-L95)

### Signed Invoice

An invoice created off-chain by a merchant and submitted on-chain with an Ed25519 signature and a single-use nonce, letting a relayer submit the transaction on the merchant's behalf without the merchant needing to sign the Soroban transaction itself.

- Interface: `create_invoice_signed` — [`contracts/shade/src/interface.rs:76-86`](../contracts/shade/src/interface.rs#L76-L86)
- Storage: `DataKey::UsedNonce(Address, BytesN<32>)` — [`contracts/shade/src/types.rs:45`](../contracts/shade/src/types.rs#L45)

### Subscription

A customer's active enrollment in a `Subscription Plan`, tracking billing status and the last charge timestamp.

- Type: `Subscription`, `SubscriptionStatus` — [`contracts/shade/src/types.rs:509-530`](../contracts/shade/src/types.rs#L509-L530)
- Interface: `subscribe`, `charge_subscription`, `cancel_subscription` — [`contracts/shade/src/interface.rs:188-199`](../contracts/shade/src/interface.rs#L188-L199)

### Subscription Plan

A recurring-billing plan a merchant defines: token, amount per interval, and interval length in seconds.

- Type: `SubscriptionPlan` — [`contracts/shade/src/types.rs:489-507`](../contracts/shade/src/types.rs#L489-L507)
- Interface: `create_subscription_plan`, `get_subscription_plan`, `deactivate_plan` — [`contracts/shade/src/interface.rs:172-203`](../contracts/shade/src/interface.rs#L172-L203)

### Swap Route

A `PaymentRoute::Swap` variant naming the router contract and token path a payment should be swapped through before settlement, as opposed to `PaymentRoute::Direct`.

- Type: `SwapRoute`, `PaymentRoute` — [`contracts/shade/src/types.rs:642-654`](../contracts/shade/src/types.rs#L642-L654)

### Ticket

A minted claim on a seat at an `Event`, owned by an address, optionally resellable on the built-in secondary market via a `TicketListing`.

- Type: `Ticket`, `TicketListing` — [`contracts/shade/src/types.rs:629-638`](../contracts/shade/src/types.rs#L629-L638), [`contracts/shade/src/types.rs:1565-1575`](../contracts/shade/src/types.rs#L1565-L1575)
- Interface: `purchase_ticket`, `resell_ticket`, `get_ticket` — [`contracts/shade/src/interface.rs:291-304`](../contracts/shade/src/interface.rs#L291-L304)

## Stellar / Soroban terms

These are Stellar-platform concepts referenced throughout the docs but defined by Stellar itself, not by this repository. See the linked official docs for authoritative detail.

### Contract ID

The address a Soroban smart contract is deployed at and invoked through. See [Stellar docs: Contract Interactions](https://developers.stellar.org/docs/build/guides/conventions).

### Event

An on-chain record published for indexers and clients to observe (e.g. a merchant registration, an invoice payment). Shade's own domain concept "Event" (a ticketed happening merchants sell tickets to) is a different thing — see the `Event` type under [`contracts/shade/src/types.rs:602-627`](../contracts/shade/src/types.rs#L602-L627) and [`contracts/shade/src/events.rs`](../contracts/shade/src/events.rs) for the emitted-events mechanism.

### Host Function

A function the Soroban runtime exposes to a contract (storage access, cryptography, auth checks) rather than one the contract implements itself. See [Stellar docs: Soroban environment](https://developers.stellar.org/docs/learn/fundamentals/contract-development/environment-concepts).

### Instance / Persistent / Temporary Storage

The three storage durability tiers Soroban offers: instance storage (small, tied to the contract instance's own lifetime), persistent storage (long-lived, rent-paying, used for most Shade records), and temporary storage (cheap, expires quickly, unsuitable for anything that must survive). See [Stellar docs: State archival](https://developers.stellar.org/docs/learn/encyclopedia/storage/state-archival).

### require_auth

The Soroban host call a contract uses to assert that a given `Address` has authorized the current invocation, either directly or via a signed authorization entry. Used throughout Shade wherever a call must be attributable to a specific merchant, admin, or signer (e.g. `merchant.require_auth()`).

### Soroban

The smart contract platform built into the Stellar network that Shade's contracts are written for and deployed to. See [Stellar docs: Soroban overview](https://developers.stellar.org/docs/learn/fundamentals/contract-development/overview).

### TTL / Rent

The time-to-live mechanism by which Soroban charges "rent" to keep persistent (and temporary) storage entries alive, and archives entries whose TTL expires. Several comments in `contracts/shade/src/types.rs` (e.g. around `AnalyticsKey` and `CreatorVesting`) explicitly trade off extra fields against extra storage reads because of this cost — see [`contracts/shade/src/types.rs:234-258`](../contracts/shade/src/types.rs#L234-L258). See [Stellar docs: State archival](https://developers.stellar.org/docs/learn/encyclopedia/storage/state-archival) for the underlying mechanism.

### WASM Hash

The hash identifying a specific compiled contract WASM binary, used both to install/deploy new merchant `account` instances (`DataKey::AccountWasmHash`) and to identify the target of a governance-approved contract upgrade (`UpgradeProposal::wasm_hash`).

- Storage: `DataKey::AccountWasmHash` — [`contracts/shade/src/types.rs:42`](../contracts/shade/src/types.rs#L42)
- Type: `UpgradeProposal` — [`contracts/shade/src/types.rs:1479-1490`](../contracts/shade/src/types.rs#L1479-L1490)
- Interface: `set_account_wasm_hash`, `propose_upgrade` — [`contracts/shade/src/interface.rs:31,264`](../contracts/shade/src/interface.rs#L31)
