# Data types reference

Every public type in `contracts/shade/src/types.rs` — structs, enums, and their fields — so integrators can decode contract return values and build correct client-side models.

## Conventions

- **`i128` amounts:** All monetary values use `i128` in token base units (the smallest indivisible unit). For tokens with 7 decimals, `1_0000000` equals 1 whole token.
- **`u64` IDs:** Auto-incremented identifiers for merchants, invoices, campaigns, events, tickets, and other records.
- **Timestamps:** All `u64` timestamp fields are Unix seconds since epoch.
- **`String`/`Bytes`:** Soroban strings are UTF-8 `String`; raw bytes use `Bytes` or `BytesN<N>`. Length limits are Soroban-imposed (1 MB for `Bytes`, 32 bytes for `BytesN<32>`).
- **Persisted types:** Types stored on-chain are subject to [upgrade-compatibility rules](../guides/upgradeability.md). Changing field order or types of a persisted struct requires a storage migration.

## Enums

### `InvoiceStatus`

Lifecycle of an invoice (`contracts/shade/src/types.rs#L356-L366`).

| Variant | `u32` | Description |
|---------|-------|-------------|
| `Pending` | 0 | Created, awaiting payment. |
| `Paid` | 1 | Fully paid. |
| `Cancelled` | 2 | Voided by the merchant. |
| `Refunded` | 3 | Fully refunded after payment. |
| `PartiallyRefunded` | 4 | Some amount refunded. |
| `PartiallyPaid` | 5 | Partial payment received. |
| `Draft` | 6 | Created as a draft; not yet finalized for payment. |

### `InvoicePricingMode`

How the invoice amount is determined (`types.rs#L369-L374`).

| Variant | `u32` | Description |
|---------|-------|-------------|
| `FixedCrypto` | 0 | Amount is denominated in the payment token. |
| `FixedFiat` | 1 | Amount is denominated in fiat; converted to crypto at payment time via oracle. |

### `FiatPricingData`

Optional wrapper for fiat pricing (Soroban does not support `Option<T>` for user-defined structs in `#[contracttype]`) (`types.rs#L391-L394`).

| Variant | Description |
|---------|-------------|
| `None` | No fiat pricing. |
| `Some(FiatPricing)` | Fiat pricing details present. |

### `Role`

Access-control roles (`types.rs#L435-L439`).

| Variant | Description |
|---------|-------------|
| `Admin` | Full administrative control. |
| `Manager` | Can manage merchants and invoices. |
| `Operator` | Can process payments and refunds. |

### `EscrowStatus`

Lifecycle of an escrow (`types.rs#L568-L575`).

| Variant | `u32` | Description |
|---------|-------|-------------|
| `Created` | 0 | Escrow record created; not yet funded. |
| `Funded` | 1 | Buyer has deposited funds. |
| `Released` | 2 | Funds released to seller. |
| `Refunded` | 3 | Funds refunded to buyer. |

### `EventStatus`

Lifecycle of a ticketing event (`types.rs#L597-L600`).

| Variant | `u32` | Description |
|---------|-------|-------------|
| `Active` | 0 | Event is live and accepting tickets. |
| `Cancelled` | 1 | Event has been cancelled. |

### `PaymentRoute`

How a payment is routed (`types.rs#L644-L647`).

| Variant | Description |
|---------|-------------|
| `Direct` | Payer sends the settlement token directly. |
| `Swap(SwapRoute)` | Payment is routed through a DEX router with a swap path. |

### `TransactionType`

Type of on-chain financial transaction (`types.rs#L548-L551`).

| Variant | `u32` | Description |
|---------|-------|-------------|
| `InvoicePayment` | 0 | Payment of an invoice. |
| `SubscriptionCharge` | 1 | Recurring subscription charge. |

### `SubscriptionStatus`

Lifecycle of a subscription (`types.rs#L527-L530`).

| Variant | `u32` | Description |
|---------|-------|-------------|
| `Active` | 0 | Subscription is active and billing. |
| `Cancelled` | 1 | Subscription has been cancelled. |

### `CrossChainPledgeStatus`

Status of a cross-chain pledge (`types.rs#L687-L692`).

| Variant | Description |
|---------|-------------|
| `Pending` | Pledge created but not yet completed. |
| `Completed` | Pledge fulfilled. |
| `Failed` | Pledge failed. |
| `Refunded` | Pledge refunded. |

### `PlatformFeeRouteKind`

Category of platform fee routing (`types.rs#L667-L672`).

| Variant | `u32` | Description |
|---------|-------|-------------|
| `Invoice` | 0 | Fee from an invoice payment. |
| `Subscription` | 1 | Fee from a subscription charge. |
| `TicketPurchase` | 2 | Fee from a ticket purchase. |

### `CampaignStatus`

Lifecycle of a fundraising campaign (`types.rs#L796-L800`).

| Variant | `u32` | Description |
|---------|-------|-------------|
| `Active` | 0 | Campaign is accepting contributions. |
| `Ended` | 1 | Campaign deadline passed. |
| `Cancelled` | 2 | Campaign cancelled by the merchant. |

### `VotingStatus`

Lifecycle of a hard-cap vote (`types.rs#L1075-L1079`).

| Variant | `u32` | Description |
|---------|-------|-------------|
| `Active` | 0 | Voting is open. |
| `Passed` | 1 | Vote passed and was executed. |
| `Failed` | 2 | Vote failed. |

### `VoteDirection`

Direction of a hard-cap vote (`types.rs#L1083-L1087`).

| Variant | `u32` | Description |
|---------|-------|-------------|
| `Increase` | 0 | Vote to increase the hard cap. |
| `Decrease` | 1 | Vote to decrease the hard cap. |

### `StretchGoalStatus`

Lifecycle of a stretch goal (`types.rs#L1128-L1135`).

| Variant | `u32` | Description |
|---------|-------|-------------|
| `Pending` | 0 | Goal not yet unlocked. |
| `Unlocked` | 1 | Campaign reached the target. |
| `Cancelled` | 2 | Retired by the merchant. |

### `FiatGoalStatus`

Lifecycle of a fiat-pegged goal (`types.rs#L1185-L1194`).

| Variant | `u32` | Description |
|---------|-------|-------------|
| `Active` | 0 | Fiat target not yet met. |
| `Reached` | 1 | Fiat target met. |
| `Closed` | 2 | Wound down by merchant or admin. |

### `CreatorVestingStatus`

Lifecycle of creator vesting (`types.rs#L1025-L1033`).

| Variant | `u32` | Description |
|---------|-------|-------------|
| `Active` | 0 | Funds vesting; creator may release vested amounts. |
| `Completed` | 1 | All vested tokens released. |
| `Revoked` | 2 | Admin froze the schedule; only already-vested amounts remain claimable. |

### `PledgeCampaignStatus`

Lifecycle of a pledge-based campaign (`types.rs#L864-L868`).

| Variant | `u32` | Description |
|---------|-------|-------------|
| `Active` | 0 | Accepting pledges. |
| `Executed` | 1 | Goal met; funds released to merchant. |
| `Cancelled` | 2 | Campaign cancelled. |

### `PledgeStatus`

Lifecycle of an individual pledge (`types.rs#L892-L896`).

| Variant | `u32` | Description |
|---------|-------|-------------|
| `Active` | 0 | Pledge active. |
| `Refunded` | 1 | Pledge refunded. |

### `ExportFormat`

Analytics export format (`types.rs#L1274-L1281`).

| Variant | Description |
|---------|-------------|
| `Csv` | Comma-separated values. |
| `Json` | Single JSON object. |
| `Ndjson` | Newline-delimited JSON. |

### `NftStatus`

Lifecycle of an NFT (`types.rs#L1373-L1376`).

| Variant | `u32` | Description |
|---------|-------|-------------|
| `Active` | 0 | NFT exists and is owned. |
| `Burned` | 1 | NFT permanently destroyed. |

### `CommentStatus`

Moderation status of a backer comment (`types.rs#L957-L961`).

| Variant | `u32` | Description |
|---------|-------|-------------|
| `Active` | 0 | Comment is visible. |
| `Flagged` | 1 | Comment has been flagged for review. |
| `Removed` | 2 | Comment removed by admin. |

### `WithdrawalProposalStatus`

Lifecycle of a multi-sig withdrawal proposal (`types.rs#L1413-L1420`).

| Variant | `u32` | Description |
|---------|-------|-------------|
| `Pending` | 0 | Awaiting signer approvals. |
| `Executed` | 1 | Quorum reached; funds transferred. |
| `Cancelled` | 2 | Cancelled before execution. |

### `ProposalStatus`

Lifecycle of a DAO upgrade proposal (`types.rs#L1468-L1475`).

| Variant | `u32` | Description |
|---------|-------|-------------|
| `Active` | 0 | Open for voting. |
| `Executed` | 1 | Passed and upgrade applied. |
| `Defeated` | 2 | Failed quorum or majority. |

## Structs

### `ContractInfo`

Singleton contract metadata (`types.rs#L324-L328`).

| Field | Type | Description |
|-------|------|-------------|
| `admin` | `Address` | Current contract administrator. |
| `timestamp` | `u64` | Ledger timestamp of initialization (Unix seconds). |

### `Merchant`

Registered merchant record (`types.rs#L340-L353`).

| Field | Type | Description |
|-------|------|-------------|
| `id` | `u64` | Auto-incremented merchant ID. |
| `address` | `Address` | Merchant's Stellar address. |
| `active` | `bool` | Whether the merchant is active. |
| `verified` | `bool` | Whether the merchant has been verified by admin. |
| `date_registered` | `u64` | Unix timestamp of registration. |
| `account` | `Address` | Deployed merchant account contract for receiving funds. |
| `webhook` | `String` | Webhook URL for payment notifications. |
| `auto_withdrawal_recipient` | `Option<Address>` | Optional override for auto-withdrawal destination. Defaults to merchant address. |
| `auto_withdrawal_thresholds` | `Vec<AutoWithdrawalThreshold>` | Per-token thresholds that trigger automatic withdrawal. |

### `AutoWithdrawalThreshold`

Per-token auto-withdrawal trigger (`types.rs#L332-L336`).

| Field | Type | Description |
|-------|------|-------------|
| `token` | `Address` | The token this threshold applies to. |
| `threshold` | `i128` | Balance at or above which withdrawal is triggered. |

### `Invoice`

A payable request created by a merchant (`types.rs#L398-L413`).

| Field | Type | Description |
|-------|------|-------------|
| `id` | `u64` | Auto-incremented invoice ID. |
| `description` | `String` | Human-readable description. |
| `amount` | `i128` | Invoice amount in token base units. |
| `token` | `Address` | Accepting payment token address. |
| `status` | `InvoiceStatus` | Current lifecycle status. |
| `merchant_id` | `u64` | Owning merchant's numeric ID. |
| `payer` | `Option<Address>` | Address of the payer once paid. |
| `date_created` | `u64` | Unix timestamp of creation. |
| `date_paid` | `Option<u64>` | Unix timestamp of payment. |
| `amount_paid` | `i128` | Cumulative amount paid (supports partial payments). |
| `amount_refunded` | `i128` | Cumulative amount refunded. |
| `expires_at` | `Option<u64>` | Optional expiration timestamp. |
| `pricing_mode` | `InvoicePricingMode` | Fixed crypto or fixed fiat pricing. |
| `fiat_pricing` | `FiatPricingData` | Fiat pricing details (if applicable). |

### `FiatPricing`

Fiat-denominated pricing details (`types.rs#L378-L382`).

| Field | Type | Description |
|-------|------|-------------|
| `currency` | `String` | ISO 4217 currency code (e.g. `"USD"`). |
| `amount` | `i128` | Fiat amount in minor units scaled by `decimals`. |
| `decimals` | `u32` | Number of fractional digits. |

### `MerchantFilter`

Filter for merchant queries (`types.rs#L416-L420`).

| Field | Type | Description |
|-------|------|-------------|
| `is_active` | `Option<bool>` | Filter by active status. |
| `is_verified` | `Option<bool>` | Filter by verification status. |

### `InvoiceFilter`

Filter for invoice queries (`types.rs#L423-L431`).

| Field | Type | Description |
|-------|------|-------------|
| `status` | `Option<u32>` | Filter by `InvoiceStatus` as `u32`. |
| `merchant` | `Option<Address>` | Filter by merchant address. |
| `min_amount` | `Option<u128>` | Minimum invoice amount. |
| `max_amount` | `Option<u128>` | Maximum invoice amount. |
| `start_date` | `Option<u64>` | Start of date range (Unix seconds). |
| `end_date` | `Option<u64>` | End of date range (Unix seconds). |

### `VolumeDiscount`

Fee discount tier based on merchant volume (`types.rs#L443-L446`).

| Field | Type | Description |
|-------|------|-------------|
| `min_volume` | `i128` | Minimum volume (in token base units) to qualify. |
| `discount_bps` | `i128` | Discount in basis points (10_000 = 100%). |

### `OracleConfig`

Price oracle configuration (`types.rs#L449-L454`).

| Field | Type | Description |
|-------|------|-------------|
| `contract` | `Address` | Oracle contract address. |
| `price_decimals` | `u32` | Decimal precision of the oracle's price output. |
| `token_decimals` | `u32` | Decimal precision of the token. |

### `MerchantAnalytics`

Per-merchant per-token analytics (`types.rs#L457-L465`).

| Field | Type | Description |
|-------|------|-------------|
| `merchant` | `Address` | Merchant address. |
| `token` | `Address` | Token address. |
| `total_volume` | `i128` | Cumulative volume in token base units. |
| `total_fees` | `i128` | Cumulative fees collected. |
| `transaction_count` | `u64` | Number of transactions. |
| `last_updated` | `u64` | Unix timestamp of last update. |

### `MerchantAnalyticsSummary`

Aggregate merchant analytics across all tokens (`types.rs#L468-L475`).

| Field | Type | Description |
|-------|------|-------------|
| `merchant` | `Address` | Merchant address. |
| `total_volume` | `i128` | Total volume across all tokens. |
| `total_fees` | `i128` | Total fees across all tokens. |
| `transaction_count` | `u64` | Total transaction count. |
| `last_updated` | `u64` | Unix timestamp of last update. |

### `PendingFee`

Time-locked pending fee update (`types.rs#L480-L485`).

| Field | Type | Description |
|-------|------|-------------|
| `token` | `Address` | Token the fee change applies to. |
| `fee` | `i128` | Proposed new fee in basis points. |
| `proposed_at` | `u64` | Unix timestamp when the fee was proposed. |

### `SubscriptionPlan`

Recurring billing plan (`types.rs#L490-L507`).

| Field | Type | Description |
|-------|------|-------------|
| `id` | `u64` | Auto-incremented plan ID. |
| `merchant_id` | `u64` | Owning merchant's numeric ID. |
| `merchant` | `Address` | Merchant's wallet address. |
| `description` | `String` | Human-readable plan description. |
| `token` | `Address` | Billing token address. |
| `amount` | `i128` | Charge amount per interval (token base units). |
| `interval` | `u64` | Billing interval in seconds (e.g. `2_592_000` = 30 days). |
| `active` | `bool` | Whether the plan accepts new subscribers. |

### `Subscription`

Active subscription record (`types.rs#L510-L522`).

| Field | Type | Description |
|-------|------|-------------|
| `id` | `u64` | Auto-incremented subscription ID. |
| `plan_id` | `u64` | Linked plan ID. |
| `customer` | `Address` | Subscriber's address. |
| `merchant_id` | `u64` | Owning merchant's numeric ID. |
| `status` | `SubscriptionStatus` | Active or Cancelled. |
| `date_created` | `u64` | Unix timestamp of enrollment. |
| `last_charged` | `u64` | Unix timestamp of last successful charge (starts at 0). |

### `TokenAnalytics`

Aggregate analytics for a token (`types.rs#L535-L543`).

| Field | Type | Description |
|-------|------|-------------|
| `token` | `Address` | Token address. |
| `total_volume` | `i128` | Cumulative volume. |
| `total_fees` | `i128` | Cumulative fees. |
| `transaction_count` | `u64` | Number of transactions. |
| `unique_merchants` | `u64` | Number of distinct merchants. |
| `last_updated` | `u64` | Unix timestamp of last update. |

### `Transaction`

Recorded financial transaction (`types.rs#L555-L563`).

| Field | Type | Description |
|-------|------|-------------|
| `transaction_type` | `TransactionType` | Invoice payment or subscription charge. |
| `ref_id` | `u64` | Invoice or subscription ID. |
| `amount` | `i128` | Transaction amount in token base units. |
| `token` | `Address` | Token address. |
| `description` | `String` | Human-readable description. |
| `date` | `u64` | Unix timestamp. |
| `merchant_id` | `u64` | Owning merchant's numeric ID. |

### `Escrow`

Physical-goods escrow record (`types.rs#L579-L590`).

| Field | Type | Description |
|-------|------|-------------|
| `id` | `u64` | Auto-incremented escrow ID. |
| `buyer` | `Address` | Buyer's address. |
| `seller` | `Address` | Seller's address. |
| `token` | `Address` | Escrowed token address. |
| `amount` | `i128` | Escrowed amount. |
| `status` | `EscrowStatus` | Current lifecycle status. |
| `invoice_id` | `Option<u64>` | Optional linked invoice. |
| `date_created` | `u64` | Unix timestamp of creation. |
| `date_funded` | `Option<u64>` | Unix timestamp when funded. |
| `date_released` | `Option<u64>` | Unix timestamp when released. |

### `Event`

Ticketing event (`types.rs#L604-L627`).

| Field | Type | Description |
|-------|------|-------------|
| `id` | `u64` | Auto-incremented event ID. |
| `merchant_id` | `u64` | Owning merchant's numeric ID. |
| `name` | `String` | Event name. |
| `ticket_price` | `i128` | Base ticket price in token base units. |
| `token` | `Address` | Ticketing token address. |
| `capacity` | `u32` | Maximum tickets. |
| `sold` | `u32` | Tickets sold so far. |
| `date` | `u64` | Unix timestamp of creation. |
| `event_date` | `u64` | Scheduled event date (must be >= creation timestamp). |
| `royalty_bps` | `u32` | Organizer royalty on resale, in basis points. |
| `early_bird_end` | `u64` | Early-bird cutoff timestamp (0 = disabled). |
| `early_bird_discount_bps` | `u32` | Discount during early-bird period, in basis points. |
| `late_markup_bps` | `u32` | Markup after early-bird period, in basis points. |
| `cancelled` | `bool` | Whether the event is cancelled. |
| `refunds_processed` | `bool` | Whether all ticket refunds have been processed. |

### `Ticket`

Individual ticket record (`types.rs#L631-L638`).

| Field | Type | Description |
|-------|------|-------------|
| `id` | `u64` | Auto-incremented ticket ID. |
| `event_id` | `u64` | Owning event ID. |
| `owner` | `Address` | Current ticket holder. |
| `minted_at` | `u64` | Unix timestamp of purchase. |
| `purchase_price` | `i128` | Amount paid on primary purchase (for cancellation refunds). |

### `SwapRoute`

DEX swap route details (`types.rs#L650-L654`).

| Field | Type | Description |
|-------|------|-------------|
| `router` | `Address` | DEX router contract address. |
| `path` | `Vec<Address>` | Ordered list of token addresses for the swap path. |

### `PlatformFeeSplit`

Computed fee split for a payment (`types.rs#L658-L663`).

| Field | Type | Description |
|-------|------|-------------|
| `gross_amount` | `i128` | Total payment amount. |
| `platform_fee` | `i128` | Fee amount deducted for the platform. |
| `merchant_amount` | `i128` | Amount credited to the merchant. |
| `fee_bps_applied` | `i128` | Effective fee rate in basis points. |

### `PaymentPayload`

Payment routing parameters (`types.rs#L676-L681`).

| Field | Type | Description |
|-------|------|-------------|
| `input_token` | `Address` | Token the payer sends. |
| `settlement_token` | `Address` | Token the merchant receives. |
| `route` | `PaymentRoute` | Direct or swap routing. |
| `max_slippage_bps` | `Option<u32>` | Maximum acceptable slippage for swap routes. |

### `CrossChainBridgePayload`

Cross-chain bridge payload (`types.rs#L713-L723`).

| Field | Type | Description |
|-------|------|-------------|
| `invoice_id` | `u64` | Linked invoice ID. |
| `merchant` | `Address` | Merchant address. |
| `payer` | `Option<Address>` | Payer address. |
| `source_chain` | `String` | Origin chain identifier. |
| `destination_chain` | `String` | Target chain identifier. |
| `token` | `Address` | Token address. |
| `amount` | `i128` | Amount to bridge. |
| `destination_recipient` | `String` | Recipient on the destination chain. |
| `memo` | `Option<String>` | Optional memo. |

### `CrossChainPledge`

Cross-chain pledge record (`types.rs#L696-L709`).

| Field | Type | Description |
|-------|------|-------------|
| `id` | `u64` | Auto-incremented pledge ID. |
| `source_chain` | `String` | Origin chain identifier. |
| `source_pledge_id` | `u64` | Pledge ID on the source chain. |
| `destination_chain` | `String` | Target chain identifier. |
| `merchant` | `Address` | Merchant address. |
| `payer` | `Address` | Payer address. |
| `token` | `Address` | Token address. |
| `amount` | `i128` | Pledge amount. |
| `status` | `CrossChainPledgeStatus` | Current status. |
| `created_at` | `u64` | Unix timestamp of creation. |
| `updated_at` | `u64` | Unix timestamp of last update. |
| `memo` | `Option<String>` | Optional memo. |

### `BridgeDeposit`

Recorded external-chain deposit (`types.rs#L732-L741`).

| Field | Type | Description |
|-------|------|-------------|
| `id` | `u64` | Auto-incremented deposit ID. |
| `source_chain` | `String` | Origin chain identifier. |
| `source_tx_id` | `BytesN<32>` | 32-byte transaction hash on the origin chain (idempotency key). |
| `listener` | `Address` | Authorized bridge listener that recorded this. |
| `token` | `Address` | Token address. |
| `amount` | `i128` | Deposited amount. |
| `recipient` | `Address` | Credited recipient. |
| `timestamp` | `u64` | Unix timestamp. |

### `PlatformFeeSplit`

Computed fee split (also documented above under Structs).

### `TicketListing`

Active secondary-market ticket listing (`types.rs#L1570-L1575`).

| Field | Type | Description |
|-------|------|-------------|
| `ticket_id` | `u64` | Listed ticket's ID. |
| `seller` | `Address` | Listing seller's address. |
| `price` | `i128` | Asking price in the event's token base units. |

### `WithdrawalProposal`

Multi-sig withdrawal proposal (`types.rs#L1425-L1437`).

| Field | Type | Description |
|-------|------|-------------|
| `id` | `u64` | Auto-incremented proposal ID. |
| `merchant` | `Address` | Proposing merchant. |
| `token` | `Address` | Withdrawal token. |
| `amount` | `i128` | Withdrawal amount. |
| `recipient` | `Address` | Fund recipient. |
| `approvals` | `u32` | Number of signer approvals received. |
| `status` | `WithdrawalProposalStatus` | Current status. |
| `created_at` | `u64` | Unix timestamp of creation. |
| `updated_at` | `u64` | Unix timestamp of last status change. |
| `note` | `String` | Human-readable note. |

### `MultiSigConfig`

Multi-sig guard configuration (`types.rs#L1442-L1450`).

| Field | Type | Description |
|-------|------|-------------|
| `threshold` | `i128` | Minimum withdrawal amount triggering multi-sig (0 = disabled). |
| `signers` | `Vec<Address>` | Authorized signer addresses. |
| `quorum` | `u32` | Approvals required to execute. |

### `GovState`

DAO governance singleton (`types.rs#L1458-L1463`).

| Field | Type | Description |
|-------|------|-------------|
| `voting_period` | `u64` | Voting window in seconds (0 = not configured). |
| `quorum_bps` | `u32` | Approval quorum in basis points. |
| `member_count` | `u32` | Number of council members. |
| `proposal_count` | `u64` | Total proposals created. |

### `UpgradeProposal`

DAO upgrade proposal (`types.rs#L1481-L1490`).

| Field | Type | Description |
|-------|------|-------------|
| `id` | `u64` | Auto-incremented proposal ID. |
| `proposer` | `Address` | Proposing council member. |
| `wasm_hash` | `BytesN<32>` | WASM hash to upgrade to. |
| `created_at` | `u64` | Unix timestamp of creation. |
| `voting_ends_at` | `u64` | Unix timestamp when voting closes. |
| `approvals` | `u32` | Number of approval votes. |
| `rejections` | `u32` | Number of rejection votes. |
| `status` | `ProposalStatus` | Current status. |

### `PageInfo`

Keyset pagination cursor (`types.rs#L1539-L1547`).

| Field | Type | Description |
|-------|------|-------------|
| `count` | `u32` | Items returned in this page. |
| `next_cursor` | `u64` | ID of the last item; 0 = no more pages. |
| `has_next_page` | `bool` | Whether more pages exist. |

### `InvoicePage`

Paginated invoice results (`types.rs#L1551-L1555`).

| Field | Type | Description |
|-------|------|-------------|
| `items` | `Vec<Invoice>` | Invoice records in this page. |
| `page_info` | `PageInfo` | Pagination metadata. |

### `MerchantPage`

Paginated merchant results (`types.rs#L1559-L1563`).

| Field | Type | Description |
|-------|------|-------------|
| `items` | `Vec<Merchant>` | Merchant records in this page. |
| `page_info` | `PageInfo` | Pagination metadata. |

### `SubscriptionPlanFilter`

Filter for subscription plan queries (`types.rs#L1496-L1501`).

| Field | Type | Description |
|-------|------|-------------|
| `merchant` | `Option<Address>` | Filter by merchant. |
| `active` | `Option<bool>` | Filter by active status. |
| `token` | `Option<Address>` | Filter by billing token. |

### `SubscriptionFilter`

Filter for subscription queries (`types.rs#L1505-L1511`).

| Field | Type | Description |
|-------|------|-------------|
| `plan_id` | `Option<u64>` | Filter by plan ID. |
| `customer` | `Option<Address>` | Filter by customer address. |
| `status` | `Option<u32>` | Filter by `SubscriptionStatus` as `u32`. |

### `EventFilter`

Filter for ticketing event queries (`types.rs#L1515-L1524`).

| Field | Type | Description |
|-------|------|-------------|
| `merchant` | `Option<Address>` | Filter by merchant. |
| `cancelled` | `Option<bool>` | `true` = only cancelled events; `false` = only active. |
| `start_date` | `Option<u64>` | Start of date range. |
| `end_date` | `Option<u64>` | End of date range. |
| `min_available` | `Option<u32>` | Minimum remaining seats. |

### `WithdrawalProposalFilter`

Filter for withdrawal proposal queries (`types.rs#L1528-L1535`).

| Field | Type | Description |
|-------|------|-------------|
| `merchant` | `Option<Address>` | Filter by merchant. |
| `status` | `Option<u32>` | Filter by `WithdrawalProposalStatus` as `u32`. |
| `token` | `Option<Address>` | Filter by token. |
| `created_after` | `Option<u64>` | Only proposals created after this timestamp. |

← [Back to reference](README.md)
