# ShadeTrait function reference

Complete API reference for the `shade` contract's public interface — every function in `ShadeTrait` with its signature, parameters, return value, authorization, errors, and emitted events.

## Alphabetical index

| Function | Domain |
|----------|--------|
| [`accept_admin_transfer`](#accept_admin_transfer) | Admin |
| [`add_accepted_token`](#add_accepted_token) | Admin |
| [`add_accepted_tokens`](#add_accepted_tokens) | Admin |
| [`add_campaign_tag`](#add_campaign_tag) | Campaigns |
| [`add_gov_member`](#add_gov_member) | Governance |
| [`amend_invoice`](#amend_invoice) | Invoice |
| [`approve_withdrawal`](#approve_withdrawal) | Multi-sig |
| [`burn_nft`](#burn_nft) | NFT |
| [`calculate_fee`](#calculate_fee) | Fee |
| [`cancel_event_and_batch_refund`](#cancel_event_and_batch_refund) | Ticketing |
| [`cancel_subscription`](#cancel_subscription) | Subscription |
| [`cancel_withdrawal`](#cancel_withdrawal) | Multi-sig |
| [`charge_subscription`](#charge_subscription) | Subscription |
| [`claim_nft_reward`](#claim_nft_reward) | NFT |
| [`claim_refund`](#claim_refund) | Invoice |
| [`clear_merchant_platform_fee`](#clear_merchant_platform_fee) | Fee |
| [`configure_dynamic_pricing`](#configure_dynamic_pricing) | Ticketing |
| [`configure_multisig`](#configure_multisig) | Multi-sig |
| [`compute_platform_fee_split`](#compute_platform_fee_split) | Fee |
| [`create_campaign`](#create_campaign) | Campaigns |
| [`create_campaign_category`](#create_campaign_category) | Campaigns |
| [`create_campaign_tag`](#create_campaign_tag) | Campaigns |
| [`create_escrow`](#create_escrow) | Escrow |
| [`create_event`](#create_event) | Ticketing |
| [`create_fiat_invoice`](#create_fiat_invoice) | Invoice |
| [`create_invoice`](#create_invoice) | Invoice |
| [`create_invoice_draft`](#create_invoice_draft) | Invoice |
| [`create_invoice_signed`](#create_invoice_signed) | Invoice |
| [`create_nft_collection`](#create_nft_collection) | NFT |
| [`create_subscription_plan`](#create_subscription_plan) | Subscription |
| [`deactivate_nft_collection`](#deactivate_nft_collection) | NFT |
| [`deactivate_plan`](#deactivate_plan) | Subscription |
| [`emit_bridge_placeholder`](#emit_bridge_placeholder) | Bridge |
| [`execute_fee`](#execute_fee) | Fee |
| [`finalize_invoice`](#finalize_invoice) | Invoice |
| [`finalize_upgrade`](#finalize_upgrade) | Governance |
| [`find_merchant_id`](#find_merchant_id) | Merchant |
| [`fund_escrow`](#fund_escrow) | Escrow |
| [`get_admin`](#get_admin) | Admin |
| [`get_auto_withdrawal_recipient`](#get_auto_withdrawal_recipient) | Merchant |
| [`get_auto_withdrawal_threshold`](#get_auto_withdrawal_threshold) | Merchant |
| [`get_bridge_credit`](#get_bridge_credit) | Bridge |
| [`get_bridge_deposit`](#get_bridge_deposit) | Bridge |
| [`get_bridge_deposit_count`](#get_bridge_deposit_count) | Bridge |
| [`get_current_ticket_price`](#get_current_ticket_price) | Ticketing |
| [`get_event`](#get_event) | Ticketing |
| [`get_event_tickets`](#get_event_tickets) | Ticketing |
| [`get_fee`](#get_fee) | Fee |
| [`get_gov_member_count`](#get_gov_member_count) | Governance |
| [`get_invoice`](#get_invoice) | Invoice |
| [`get_invoices`](#get_invoices) | Invoice |
| [`get_merchant`](#get_merchant) | Merchant |
| [`get_merchant_account`](#get_merchant_account) | Merchant |
| [`get_merchant_accepted_tokens`](#get_merchant_accepted_tokens) | Merchant |
| [`get_merchant_analytics`](#get_merchant_analytics) | Analytics |
| [`get_merchant_analytics_summary`](#get_merchant_analytics_summary) | Analytics |
| [`get_merchant_key`](#get_merchant_key) | Security |
| [`get_merchant_platform_fee`](#get_merchant_platform_fee) | Fee |
| [`get_merchant_volume`](#get_merchant_volume) | Analytics |
| [`get_merchant_webhook`](#get_merchant_webhook) | Merchant |
| [`get_merchants`](#get_merchants) | Merchant |
| [`get_multisig_threshold`](#get_multisig_threshold) | Multi-sig |
| [`get_nft`](#get_nft) | NFT |
| [`get_nft_collection`](#get_nft_collection) | NFT |
| [`get_pending_fee`](#get_pending_fee) | Fee |
| [`get_platform_account`](#get_platform_account) | Admin |
| [`get_subscription`](#get_subscription) | Subscription |
| [`get_subscription_plan`](#get_subscription_plan) | Subscription |
| [`get_ticket`](#get_ticket) | Ticketing |
| [`get_token_analytics`](#get_token_analytics) | Analytics |
| [`get_token_dominance_metrics`](#get_token_dominance_metrics) | Analytics |
| [`get_token_market_share`](#get_token_market_share) | Analytics |
| [`get_token_oracle`](#get_token_oracle) | Admin |
| [`get_token_volume`](#get_token_volume) | Analytics |
| [`get_top_tokens_by_volume`](#get_top_tokens_by_volume) | Analytics |
| [`get_upgrade_proposal`](#get_upgrade_proposal) | Governance |
| [`get_user_nfts`](#get_user_nfts) | NFT |
| [`get_user_tickets`](#get_user_tickets) | Ticketing |
| [`get_user_transactions`](#get_user_transactions) | Analytics |
| [`get_withdrawal_proposal`](#get_withdrawal_proposal) | Multi-sig |
| [`get_withdrawal_proposal_count`](#get_withdrawal_proposal_count) | Multi-sig |
| [`grant_role`](#grant_role) | Access control |
| [`has_approved_withdrawal`](#has_approved_withdrawal) | Multi-sig |
| [`has_role`](#has_role) | Access control |
| [`has_voted_on_upgrade`](#has_voted_on_upgrade) | Governance |
| [`initialize`](#initialize) | Admin |
| [`is_accepted_token`](#is_accepted_token) | Admin |
| [`is_bridge_deposit_processed`](#is_bridge_deposit_processed) | Bridge |
| [`is_bridge_listener`](#is_bridge_listener) | Bridge |
| [`is_gov_member`](#is_gov_member) | Governance |
| [`is_merchant`](#is_merchant) | Merchant |
| [`is_merchant_active`](#is_merchant_active) | Merchant |
| [`is_merchant_verified`](#is_merchant_verified) | Merchant |
| [`is_paused`](#is_paused) | Admin |
| [`is_token_accepted_for_merchant`](#is_token_accepted_for_merchant) | Merchant |
| [`mint_nft`](#mint_nft) | NFT |
| [`batch_mint_nfts`](#batch_mint_nfts) | NFT |
| [`pay_invoice`](#pay_invoice) | Invoice |
| [`pay_invoice_partial`](#pay_invoice_partial) | Invoice |
| [`pay_invoices_batch`](#pay_invoices_batch) | Invoice |
| [`pause`](#pause) | Admin |
| [`propose_admin_transfer`](#propose_admin_transfer) | Admin |
| [`propose_fee`](#propose_fee) | Fee |
| [`propose_upgrade`](#propose_upgrade) | Governance |
| [`propose_withdrawal`](#propose_withdrawal) | Multi-sig |
| [`purchase_ticket`](#purchase_ticket) | Ticketing |
| [`purchase_tickets_bulk`](#purchase_tickets_bulk) | Ticketing |
| [`record_bridge_deposit`](#record_bridge_deposit) | Bridge |
| [`record_campaign_contribution`](#record_campaign_contribution) | Campaigns |
| [`register_bridge_listener`](#register_bridge_listener) | Bridge |
| [`register_merchant`](#register_merchant) | Merchant |
| [`refund_escrow`](#refund_escrow) | Escrow |
| [`refund_invoice`](#refund_invoice) | Invoice |
| [`refund_invoice_partial`](#refund_invoice_partial) | Invoice |
| [`remove_accepted_token`](#remove_accepted_token) | Admin |
| [`remove_bridge_listener`](#remove_bridge_listener) | Bridge |
| [`remove_campaign_tag`](#remove_campaign_tag) | Campaigns |
| [`remove_gov_member`](#remove_gov_member) | Governance |
| [`remove_merchant_accepted_token`](#remove_merchant_accepted_token) | Merchant |
| [`resell_ticket`](#resell_ticket) | Ticketing |
| [`restrict_merchant_account`](#restrict_merchant_account) | Access control |
| [`revoke_role`](#revoke_role) | Access control |
| [`search_events`](#search_events) | Search |
| [`search_invoices_paginated`](#search_invoices_paginated) | Search |
| [`search_merchants_paginated`](#search_merchants_paginated) | Search |
| [`search_subscription_plans`](#search_subscription_plans) | Search |
| [`search_subscriptions`](#search_subscriptions) | Search |
| [`search_withdrawal_proposals`](#search_withdrawal_proposals) | Search |
| [`set_accepted_tokens`](#set_accepted_tokens) | Admin |
| [`set_account_wasm_hash`](#set_account_wasm_hash) | Admin |
| [`set_auto_withdrawal_recipient`](#set_auto_withdrawal_recipient) | Merchant |
| [`set_auto_withdrawal_threshold`](#set_auto_withdrawal_threshold) | Merchant |
| [`set_campaign_active`](#set_campaign_active) | Campaigns |
| [`set_fee`](#set_fee) | Fee |
| [`set_governance_config`](#set_governance_config) | Governance |
| [`set_merchant_account`](#set_merchant_account) | Merchant |
| [`set_merchant_accepted_tokens`](#set_merchant_accepted_tokens) | Merchant |
| [`set_merchant_key`](#set_merchant_key) | Security |
| [`set_merchant_platform_fee`](#set_merchant_platform_fee) | Fee |
| [`set_merchant_status`](#set_merchant_status) | Merchant |
| [`set_merchant_webhook`](#set_merchant_webhook) | Merchant |
| [`set_multisig_threshold`](#set_multisig_threshold) | Multi-sig |
| [`set_platform_account`](#set_platform_account) | Admin |
| [`set_token_oracle`](#set_token_oracle) | Admin |
| [`subscribe`](#subscribe) | Subscription |
| [`transfer_nft`](#transfer_nft) | NFT |
| [`unpause`](#unpause) | Admin |
| [`update_campaign`](#update_campaign) | Campaigns |
| [`update_campaign_category`](#update_campaign_category) | Campaigns |
| [`upgrade`](#upgrade) | Admin |
| [`validate_payment_payload`](#validate_payment_payload) | Invoice |
| [`void_invoice`](#void_invoice) | Invoice |
| [`vote_on_upgrade`](#vote_on_upgrade) | Governance |
| [`create_backer_campaign`](#create_backer_campaign) | Cross-chain |
| [`update_cross_chain_pledge_status`](#update_cross_chain_pledge_status) | Cross-chain |
| [`get_cross_chain_pledge`](#get_cross_chain_pledge) | Cross-chain |
| [`get_cross_chain_pledge_by_source`](#get_cross_chain_pledge_by_source) | Cross-chain |
| [`get_all_cross_chain_pledges`](#get_all_cross_chain_pledges) | Cross-chain |

## Admin / configuration

### `initialize`

```rust
fn initialize(env: Env, admin: Address);
```

Initialize the contract. Must be called exactly once.

| Parameter | Type | Auth | Description |
|-----------|------|------|-------------|
| `admin` | `Address` | — | Initial admin address. |

**Returns:** nothing. **Events:** `InitalizedEvent`. **Errors:** `AlreadyInitialized` (2).

### `get_admin`

```rust
fn get_admin(env: Env) -> Address;
```

Returns the current admin address.

**Returns:** `Address`. **Auth:** none. **Errors:** none.

### `add_accepted_token`

```rust
fn add_accepted_token(env: Env, admin: Address, token: Address);
```

Add a single token to the accepted-tokens whitelist.

| Parameter | Type | Auth | Description |
|-----------|------|------|-------------|
| `admin` | `Address` | Admin | Admin address. |
| `token` | `Address` | — | Token contract address to accept. |

**Events:** `TokenAddedEvent`. **Errors:** `NotAuthorized` (1).

### `add_accepted_tokens`

```rust
fn add_accepted_tokens(env: Env, admin: Address, tokens: Vec<Address>);
```

Add multiple tokens to the whitelist in one call.

| Parameter | Type | Auth | Description |
|-----------|------|------|-------------|
| `admin` | `Address` | Admin | Admin address. |
| `tokens` | `Vec<Address>` | — | Token addresses to accept. |

**Events:** `TokenAddedEvent` per token. **Errors:** `NotAuthorized` (1).

### `remove_accepted_token`

```rust
fn remove_accepted_token(env: Env, admin: Address, token: Address);
```

Remove a token from the accepted-tokens whitelist.

| Parameter | Type | Auth | Description |
|-----------|------|------|-------------|
| `admin` | `Address` | Admin | Admin address. |
| `token` | `Address` | — | Token to remove. |

**Events:** `TokenRemovedEvent`. **Errors:** `NotAuthorized` (1).

### `is_accepted_token`

```rust
fn is_accepted_token(env: Env, token: Address) -> bool;
```

Whether the token is in the accepted whitelist.

**Returns:** `bool`. **Auth:** none.

### `set_account_wasm_hash`

```rust
fn set_account_wasm_hash(env: Env, admin: Address, wasm_hash: soroban_sdk::BytesN<32>);
```

Set the WASM hash used to deploy merchant account contracts.

| Parameter | Type | Auth | Description |
|-----------|------|------|-------------|
| `admin` | `Address` | Admin | Admin address. |
| `wasm_hash` | `BytesN<32>` | — | WASM hash for account contracts. |

**Events:** `AccountWasmHashSetEvent`. **Errors:** `NotAuthorized` (1).

### `set_platform_account`

```rust
fn set_platform_account(env: Env, admin: Address, account: Address);
```

Set the address that receives platform fees.

| Parameter | Type | Auth | Description |
|-----------|------|------|-------------|
| `admin` | `Address` | Admin | Admin address. |
| `account` | `Address` | — | Platform fee recipient address. |

**Events:** `PlatformAccountSetEvent`. **Errors:** `NotAuthorized` (1).

### `get_platform_account`

```rust
fn get_platform_account(env: Env) -> Address;
```

Returns the platform fee recipient address.

**Returns:** `Address`. **Auth:** none.

### `set_token_oracle`

```rust
fn set_token_oracle(env: Env, admin: Address, token: Address, oracle: OracleConfig);
```

Configure a price oracle for a token.

| Parameter | Type | Auth | Description |
|-----------|------|------|-------------|
| `admin` | `Address` | Admin | Admin address. |
| `token` | `Address` | — | Token to configure. |
| `oracle` | `OracleConfig` | — | Oracle config (contract, price_decimals, token_decimals). |

**Events:** `TokenOracleSetEvent`. **Errors:** `NotAuthorized` (1).

### `get_token_oracle`

```rust
fn get_token_oracle(env: Env, token: Address) -> OracleConfig;
```

Returns the oracle config for a token.

**Returns:** `OracleConfig`. **Errors:** `OracleNotConfigured` (34).

### `pause` / `unpause` / `is_paused`

```rust
fn pause(env: Env, admin: Address);
fn unpause(env: Env, admin: Address);
fn is_paused(env: Env) -> bool;
```

Emergency pause controls. When paused, most state-mutating functions panic with `ContractPaused`.

**Events:** `ContractPausedEvent` / `ContractUnpausedEvent`. **Errors:** `NotAuthorized` (1), `ContractNotPaused` (10), `ContractPaused` (9).

### `upgrade`

```rust
fn upgrade(env: Env, new_wasm_hash: BytesN<32>);
```

Replace the contract's WASM. Requires admin auth. Takes effect immediately.

| Parameter | Type | Auth | Description |
|-----------|------|------|-------------|
| `new_wasm_hash` | `BytesN<32>` | Admin | New WASM hash. |

**Events:** `ContractUpgradedEvent`. **Errors:** `NotAuthorized` (1), `WasmHashNotSet` (18).

> **Warning:** This bypasses DAO governance. Coordinate upgrades through `propose_upgrade`/`vote_on_upgrade`/`finalize_upgrade` in production.

### `propose_admin_transfer` / `accept_admin_transfer`

```rust
fn propose_admin_transfer(env: Env, admin: Address, new_admin: Address);
fn accept_admin_transfer(env: Env, new_admin: Address);
```

Two-step admin handover. Step 1 proposes; step 2 accepts.

**Events:** `AdminTransferProposedEvent`, `AdminTransferAcceptedEvent`. **Errors:** `NotAuthorized` (1).

## Fee management

### `set_fee`

```rust
fn set_fee(env: Env, admin: Address, token: Address, fee: i128);
```

Set the platform fee for a token (in basis points). Applies immediately.

| Parameter | Type | Auth | Description |
|-----------|------|------|-------------|
| `admin` | `Address` | Admin | Admin address. |
| `token` | `Address` | — | Token address. |
| `fee` | `i128` | — | Fee in basis points (10_000 = 100%). |

**Events:** `FeeSetEvent`. **Errors:** `NotAuthorized` (1).

### `get_fee`

```rust
fn get_fee(env: Env, token: Address) -> i128;
```

Returns the current fee for a token.

**Returns:** `i128`. **Errors:** none (returns 0 if not set).

### `propose_fee` / `execute_fee` / `get_pending_fee`

```rust
fn propose_fee(env: Env, admin: Address, token: Address, fee: i128);
fn execute_fee(env: Env, admin: Address, token: Address);
fn get_pending_fee(env: Env, token: Address) -> PendingFee;
```

Time-locked fee update. `propose_fee` stages the change; `execute_fee` applies it after the time-lock.

**Events:** `FeeProposedEvent`, `FeeSetEvent`. **Errors:** `FeeUpdateTooEarly` (42), `NoPendingFeeUpdate` (43).

### `set_merchant_platform_fee` / `get_merchant_platform_fee` / `clear_merchant_platform_fee`

```rust
fn set_merchant_platform_fee(env: Env, caller: Address, merchant_id: u64, token: Address, fee_bps: i128);
fn get_merchant_platform_fee(env: Env, merchant_id: u64, token: Address) -> Option<i128>;
fn clear_merchant_platform_fee(env: Env, caller: Address, merchant_id: u64, token: Address);
```

Per-merchant fee overrides. Requires admin auth for set/clear.

**Events:** `MerchantPlatformFeeSetEvent`, `PlatformFeeClearedEvent`.

### `calculate_fee`

```rust
fn calculate_fee(env: Env, merchant: Address, token: Address, amount: i128) -> i128;
```

Compute the fee for a given amount. Accounts for volume discounts and per-merchant overrides.

**Returns:** `i128` fee amount.

### `compute_platform_fee_split`

```rust
fn compute_platform_fee_split(env: Env, merchant: Address, token: Address, amount: i128) -> PlatformFeeSplit;
```

Compute the full fee split (gross, platform fee, merchant amount, effective bps).

**Returns:** `PlatformFeeSplit`.

## Merchant management

### `register_merchant`

```rust
fn register_merchant(env: Env, merchant: Address);
```

Register a new merchant. Requires the caller to authorize.

| Parameter | Type | Auth | Description |
|-----------|------|------|-------------|
| `merchant` | `Address` | Merchant | Merchant's address. |

**Returns:** nothing. **Events:** `MerchantRegisteredEvent`, `MerchantAccountDeployedEvent`. **Errors:** `MerchantAlreadyRegistered` (5).

### `get_merchant`

```rust
fn get_merchant(env: Env, merchant_id: u64) -> Merchant;
```

Returns a merchant by ID.

**Returns:** `Merchant`. **Errors:** `MerchantNotFound` (6).

### `get_merchants`

```rust
fn get_merchants(env: Env, filter: MerchantFilter) -> Vec<Merchant>;
```

List merchants matching the filter.

**Returns:** `Vec<Merchant>`.

### `is_merchant`

```rust
fn is_merchant(env: Env, merchant: Address) -> bool;
```

Whether the address is registered as a merchant.

**Returns:** `bool`.

### `set_merchant_status`

```rust
fn set_merchant_status(env: Env, admin: Address, merchant_id: u64, status: bool);
```

Enable or disable a merchant. Requires admin auth.

**Events:** `MerchantStatusChangedEvent`. **Errors:** `MerchantNotFound` (6).

### `is_merchant_active` / `is_merchant_verified`

```rust
fn is_merchant_active(env: Env, merchant_id: u64) -> bool;
fn is_merchant_verified(env: Env, merchant_id: u64) -> bool;
```

Check merchant status flags.

### `verify_merchant`

```rust
fn verify_merchant(env: Env, admin: Address, merchant_id: u64, status: bool);
```

Set the verified flag on a merchant. Requires admin auth.

**Events:** `MerchantVerifiedEvent`.

### `set_merchant_account` / `get_merchant_account`

```rust
fn set_merchant_account(env: Env, merchant: Address, account: Address);
fn get_merchant_account(env: Env, merchant_id: u64) -> Address;
```

Manage the merchant's account contract address.

### `set_merchant_webhook` / `get_merchant_webhook`

```rust
fn set_merchant_webhook(env: Env, merchant: Address, webhook: String);
fn get_merchant_webhook(env: Env, merchant_id: u64) -> String;
```

Manage the merchant's webhook URL.

**Events:** `MerchantWebhookSetEvent`.

### `set_merchant_accepted_tokens` / `get_merchant_accepted_tokens` / `remove_merchant_accepted_token` / `is_token_accepted_for_merchant`

```rust
fn set_merchant_accepted_tokens(env: Env, merchant: Address, tokens: Vec<Address>);
fn get_merchant_accepted_tokens(env: Env, merchant: Address) -> Vec<Address>;
fn remove_merchant_accepted_token(env: Env, merchant: Address, token: Address);
fn is_token_accepted_for_merchant(env: Env, merchant: Address, token: Address) -> bool;
```

Per-merchant token whitelist management.

**Events:** `MerchantTokensSetEvent`, `MerchantTokenRemovedEvent`.

### `set_merchant_key` / `get_merchant_key`

```rust
fn set_merchant_key(env: Env, merchant: Address, key: BytesN<32>);
fn get_merchant_key(env: Env, merchant: Address) -> BytesN<32>;
```

Register or retrieve the ed25519 public key for signed invoices.

**Events:** `MerchantKeySetEvent`. **Errors:** `MerchantKeyNotFound` (11).

### `restrict_merchant_account`

```rust
fn restrict_merchant_account(env: Env, caller: Address, merchant_address: Address, status: bool);
```

Restrict or unrestrict a merchant account. Requires admin auth.

**Events:** `AccountRestrictedEvent`.

### `find_merchant_id`

```rust
fn find_merchant_id(env: Env, address: Address) -> u64;
```

Look up a merchant ID from an address. Returns 0 if not registered.

**Returns:** `u64`.

### Auto-withdrawal

```rust
fn set_auto_withdrawal_threshold(env: Env, merchant: Address, token: Address, threshold: i128);
fn get_auto_withdrawal_threshold(env: Env, merchant_id: u64, token: Address) -> Option<i128>;
fn set_auto_withdrawal_recipient(env: Env, merchant: Address, recipient: Address);
fn get_auto_withdrawal_recipient(env: Env, merchant_id: u64) -> Option<Address>;
```

Configure auto-withdrawal thresholds and recipients.

**Events:** `AutoWithdrawThresholdEvent`, `AutoWithdrawRecipientEvent`.

## Invoice management

### `create_invoice`

```rust
fn create_invoice(env: Env, merchant: Address, description: String, amount: i128, token: Address, expires_at: Option<u64>) -> u64;
```

Create a payable invoice. Requires merchant auth.

| Parameter | Type | Auth | Description |
|-----------|------|------|-------------|
| `merchant` | `Address` | Merchant | Merchant address. |
| `description` | `String` | — | Invoice description. |
| `amount` | `i128` | — | Amount in token base units. |
| `token` | `Address` | — | Accepting token. |
| `expires_at` | `Option<u64>` | — | Optional expiration timestamp. |

**Returns:** `u64` invoice ID. **Events:** `InvoiceCreatedEvent`. **Errors:** `InvalidAmount` (7), `TokenNotAccepted` (12), `MerchantNotActive` (32).

### `create_fiat_invoice`

```rust
fn create_fiat_invoice(env: Env, merchant: Address, description: String, fiat_amount: i128, fiat_currency: String, fiat_decimals: u32, token: Address, expires_at: Option<u64>) -> u64;
```

Create an invoice denominated in fiat. Converted to crypto at payment time via oracle.

**Returns:** `u64` invoice ID. **Events:** `InvoiceCreatedEvent`.

### `create_invoice_draft`

```rust
fn create_invoice_draft(env: Env, merchant: Address, description: String, amount: i128, token: Address, expires_at: Option<u64>) -> u64;
```

Create a draft invoice (not yet finalized for payment). Requires merchant auth.

**Returns:** `u64` invoice ID. **Events:** `InvoiceCreatedEvent`.

### `finalize_invoice`

```rust
fn finalize_invoice(env: Env, merchant: Address, invoice_id: u64);
```

Finalize a draft invoice, making it payable. Requires merchant auth.

**Errors:** `InvalidInvoiceStatus` (16).

### `create_invoice_signed`

```rust
fn create_invoice_signed(env: Env, caller: Address, merchant: Address, description: String, amount: i128, token: Address, nonce: BytesN<32>, signature: BytesN<64>) -> u64;
```

Create an invoice from an off-chain signed payload. The signature is verified against the merchant's registered ed25519 key.

| Parameter | Type | Auth | Description |
|-----------|------|------|-------------|
| `caller` | `Address` | Caller | Address submitting the transaction. |
| `merchant` | `Address` | — | Merchant address. |
| `description` | `String` | — | Invoice description. |
| `amount` | `i128` | — | Amount in token base units. |
| `token` | `Address` | — | Accepting token. |
| `nonce` | `BytesN<32>` | — | Unique 32-byte nonce (single-use). |
| `signature` | `BytesN<64>` | — | Ed25519 signature. |

**Returns:** `u64` invoice ID. **Events:** `InvoiceCreatedEvent`, `NonceInvalidatedEvent`. **Errors:** `MerchantKeyNotFound` (11), `NonceAlreadyUsed` (14).

See [Signed invoices](../security/signatures.md) for the message construction and signing scheme.

### `get_invoice`

```rust
fn get_invoice(env: Env, invoice_id: u64) -> Invoice;
```

Returns an invoice by ID.

**Errors:** `InvoiceNotFound` (8).

### `resolve_invoice_amount`

```rust
fn resolve_invoice_amount(env: Env, invoice_id: u64) -> i128;
```

Resolve the payment amount for an invoice. For fiat-priced invoices, converts via oracle.

**Returns:** `i128`.

### `get_invoices`

```rust
fn get_invoices(env: Env, filter: InvoiceFilter) -> Vec<Invoice>;
```

List invoices matching the filter.

**Returns:** `Vec<Invoice>`.

### `pay_invoice`

```rust
fn pay_invoice(env: Env, payer: Address, invoice_id: u64);
```

Pay an invoice in full. Transfers the exact amount from the payer.

| Parameter | Type | Auth | Description |
|-----------|------|------|-------------|
| `payer` | `Address` | Payer | Payer address. |
| `invoice_id` | `u64` | — | Invoice to pay. |

**Events:** `InvoicePaidEvent`, `PlatformFeeRoutedEvent`. **Errors:** `InvoiceNotFound` (8), `InvalidInvoiceStatus` (16), `InvoiceExpired` (27), `TokenNotAcceptedByMerchant` (41), `InsufficientBalance` (30).

### `pay_invoice_partial`

```rust
fn pay_invoice_partial(env: Env, payer: Address, invoice_id: u64, amount: i128);
```

Make a partial payment toward an invoice.

**Events:** `InvoicePaidEvent`. **Errors:** same as `pay_invoice` plus `InvalidAmount` (7).

### `pay_invoices_batch`

```rust
fn pay_invoices_batch(env: Env, payer: Address, invoice_ids: Vec<u64>);
```

Pay multiple invoices in a single call. Each invoice is paid individually.

**Events:** `InvoicePaidEvent` per invoice. **Errors:** per-invoice errors.

### `refund_invoice`

```rust
fn refund_invoice(env: Env, merchant: Address, invoice_id: u64);
```

Fully refund a paid invoice. Requires merchant auth.

**Events:** `InvoiceRefundedEvent`. **Errors:** `InvalidInvoiceStatus` (16), `RefundPeriodExpired` (17).

### `refund_invoice_partial`

```rust
fn refund_invoice_partial(env: Env, merchant: Address, invoice_id: u64, amount: i128);
```

Partially refund a paid invoice. Requires merchant auth.

**Events:** `InvoicePartiallyRefundedEvent`.

### `claim_refund`

```rust
fn claim_refund(env: Env, buyer: Address, invoice_id: u64);
```

Claim a refund for a refunded invoice. Requires buyer auth.

**Errors:** `InvalidInvoiceStatus` (16).

### `void_invoice`

```rust
fn void_invoice(env: Env, merchant: Address, invoice_id: u64);
```

Cancel (void) a pending invoice. Requires merchant auth.

**Events:** `InvoiceCancelledEvent`. **Errors:** `InvalidInvoiceStatus` (16).

### `amend_invoice`

```rust
fn amend_invoice(env: Env, merchant: Address, invoice_id: u64, new_amount: Option<i128>, new_description: Option<String>);
```

Amend an unfinalized invoice's amount and/or description. Requires merchant auth.

**Events:** `InvoiceAmendedEvent`.

### `validate_payment_payload`

```rust
fn validate_payment_payload(env: Env, payload: PaymentPayload);
```

Validate a payment payload structure without executing the payment.

**Errors:** `InvalidSwapPath` (44), `InvalidSlippage` (45).

## Subscription engine

### `create_subscription_plan`

```rust
fn create_subscription_plan(env: Env, merchant: Address, description: String, token: Address, amount: i128, interval: u64) -> u64;
```

Create a recurring billing plan. Requires merchant auth.

**Returns:** `u64` plan ID. **Events:** `SubscriptionPlanCreatedEvent`. **Errors:** `InvalidInterval` (21), `TokenNotAcceptedByMerchant` (41).

### `get_subscription_plan`

```rust
fn get_subscription_plan(env: Env, plan_id: u64) -> SubscriptionPlan;
```

Returns a plan by ID. **Errors:** `PlanNotFound` (22).

### `subscribe`

```rust
fn subscribe(env: Env, customer: Address, plan_id: u64) -> u64;
```

Subscribe a customer to a plan. The customer must have approved sufficient SEP-41 allowance.

**Returns:** `u64` subscription ID. **Events:** `SubscribedEvent`. **Errors:** `PlanNotFound` (22), `PlanNotActive` (23).

### `get_subscription`

```rust
fn get_subscription(env: Env, subscription_id: u64) -> Subscription;
```

Returns a subscription by ID. **Errors:** `SubscriptionNotFound` (24).

### `charge_subscription`

```rust
fn charge_subscription(env: Env, subscription_id: u64);
```

Trigger a charge for a subscription. Callable by anyone. Panics if the billing interval has not elapsed.

**Events:** `SubscriptionChargedEvent`, `PlatformFeeRoutedEvent`. **Errors:** `SubscriptionNotFound` (24), `SubscriptionNotActive` (25), `ChargeTooEarly` (26).

### `cancel_subscription`

```rust
fn cancel_subscription(env: Env, caller: Address, subscription_id: u64);
```

Cancel a subscription. Either the customer or the merchant may call this.

**Events:** `SubscriptionCancelledEvent`. **Errors:** `SubscriptionNotFound` (24).

### `deactivate_plan`

```rust
fn deactivate_plan(env: Env, caller: Address, plan_id: u64);
```

Deactivate a plan so no new customers can enroll. Only the plan owner may call.

**Events:** `PlanDeactivatedEvent`. **Errors:** `PlanNotFound` (22), `NotCampaignMerchant` (209).

## Access control

### `grant_role` / `revoke_role` / `has_role`

```rust
fn grant_role(env: Env, admin: Address, user: Address, role: Role);
fn revoke_role(env: Env, admin: Address, user: Address, role: Role);
fn has_role(env: Env, user: Address, role: Role) -> bool;
```

Manage access-control roles. `grant_role` and `revoke_role` require admin auth.

**Events:** `RoleGrantedEvent`, `RoleRevokedEvent`. **Errors:** `NotAuthorized` (1).

## Analytics

### `get_merchant_volume`

```rust
fn get_merchant_volume(env: Env, merchant: Address, token: Address) -> i128;
```

Cumulative volume for a merchant-token pair.

### `get_merchant_analytics`

```rust
fn get_merchant_analytics(env: Env, merchant: Address, token: Address) -> MerchantAnalytics;
```

Detailed analytics for a merchant-token pair.

### `get_merchant_analytics_summary`

```rust
fn get_merchant_analytics_summary(env: Env, merchant: Address) -> MerchantAnalyticsSummary;
```

Aggregate analytics across all tokens for a merchant.

### `get_token_analytics`

```rust
fn get_token_analytics(env: Env, token: Address) -> TokenAnalytics;
```

Aggregate analytics for a specific token.

### `get_token_volume`

```rust
fn get_token_volume(env: Env, token: Address) -> i128;
```

Total volume for a specific token.

### `get_token_dominance_metrics`

```rust
fn get_token_dominance_metrics(env: Env, tokens: Vec<Address>) -> Vec<(Address, i128)>;
```

Volume-sorted token dominance metrics.

### `get_top_tokens_by_volume`

```rust
fn get_top_tokens_by_volume(env: Env, limit: u32) -> Vec<(Address, i128)>;
```

Top tokens by volume with a limit.

### `get_token_market_share`

```rust
fn get_token_market_share(env: Env, token: Address) -> i128;
```

Market share of a token in basis points (10_000 = 100%).

### `get_user_transactions`

```rust
fn get_user_transactions(env: Env, user: Address) -> Vec<Transaction>;
```

All transactions executed by a specific address.

## Escrow

### `create_escrow`

```rust
fn create_escrow(env: Env, seller: Address, buyer: Address, token: Address, amount: i128, invoice_id: Option<u64>) -> u64;
```

Create an escrow for physical goods. Requires seller auth.

**Returns:** `u64` escrow ID. **Errors:** `InvalidAmount` (7), `MerchantNotFound` (6).

### `get_escrow`

```rust
fn get_escrow(env: Env, escrow_id: u64) -> Escrow;
```

Returns an escrow by ID. **Errors:** `EscrowNotFound` (46).

### `fund_escrow`

```rust
fn fund_escrow(env: Env, buyer: Address, escrow_id: u64);
```

Fund an escrow. Requires buyer auth.

**Errors:** `EscrowNotFound` (46), `InvalidEscrowStatus` (47).

### `release_escrow`

```rust
fn release_escrow(env: Env, buyer: Address, escrow_id: u64);
```

Release escrow to seller. Requires buyer auth.

**Errors:** `EscrowNotFound` (46), `InvalidEscrowStatus` (47).

### `refund_escrow`

```rust
fn refund_escrow(env: Env, seller: Address, escrow_id: u64);
```

Refund escrow to buyer. Requires seller auth.

**Errors:** `EscrowNotFound` (46), `InvalidEscrowStatus` (47).

## Event ticketing

### `create_event`

```rust
fn create_event(env: Env, merchant: Address, name: String, ticket_price: i128, token: Address, capacity: u32, event_date: u64, royalty_bps: u32) -> u64;
```

Create a ticketing event. Requires merchant auth.

**Returns:** `u64` event ID. **Events:** `EventCreatedEvent`. **Errors:** `InvalidCapacity` (122), `InvalidEventDate` (123), `InvalidRoyaltyBps` (124).

### `purchase_ticket`

```rust
fn purchase_ticket(env: Env, event_id: u64, buyer: Address) -> u64;
```

Purchase a single ticket. Requires buyer auth.

**Returns:** `u64` ticket ID. **Events:** `TicketPurchasedEvent`. **Errors:** `EventNotFound` (120), `EventSoldOut` (121).

### `purchase_tickets_bulk`

```rust
fn purchase_tickets_bulk(env: Env, event_id: u64, buyer: Address, quantity: u32, shade_token: Address, merchant_account: Address);
```

Purchase multiple tickets with automatic group discount (5–9: 5%, 10–19: 10%, 20+: 15%).

### `configure_dynamic_pricing`

```rust
fn configure_dynamic_pricing(env: Env, merchant: Address, event_id: u64, early_bird_end: u64, early_bird_discount_bps: u32, late_markup_bps: u32);
```

Configure dynamic pricing tiers. Requires merchant auth.

### `get_current_ticket_price`

```rust
fn get_current_ticket_price(env: Env, event_id: u64) -> i128;
```

Returns the current ticket price after applying dynamic pricing.

### `cancel_event_and_batch_refund`

```rust
fn cancel_event_and_batch_refund(env: Env, merchant: Address, event_id: u64);
```

Cancel an event and refund all ticket holders. Requires merchant auth.

### `resell_ticket`

```rust
fn resell_ticket(env: Env, seller: Address, buyer: Address, ticket_id: u64, resale_price: i128);
```

Resell a ticket on the secondary market. Enforces price bounds (0.5x–2x original).

**Events:** `TicketResoldEvent`. **Errors:** `TicketNotFound` (125), `NotTicketOwner` (126), `InvalidResalePrice` (127).

### `get_event` / `get_ticket` / `get_event_tickets` / `get_user_tickets`

```rust
fn get_event(env: Env, event_id: u64) -> Event;
fn get_ticket(env: Env, ticket_id: u64) -> Ticket;
fn get_event_tickets(env: Env, event_id: u64) -> Vec<u64>;
fn get_user_tickets(env: Env, user: Address) -> Vec<u64>;
```

Event and ticket lookups.

## Campaigns (categories, tags, fundraising)

### `create_campaign_category`

```rust
fn create_campaign_category(env: Env, admin: Address, name: String, description: String) -> u64;
```

Create a campaign category. Admin-only.

**Returns:** `u64` category ID. **Errors:** `CampaignCategoryAlreadyExists` (202).

### `update_campaign_category`

```rust
fn update_campaign_category(env: Env, admin: Address, category_id: u64, name: Option<String>, description: Option<String>, active: Option<bool>);
```

Update a campaign category. Admin-only.

### `get_campaign_category` / `get_campaign_categories`

```rust
fn get_campaign_category(env: Env, category_id: u64) -> CampaignCategory;
fn get_campaign_categories(env: Env) -> Vec<CampaignCategory>;
```

Category lookups.

### `create_campaign_tag`

```rust
fn create_campaign_tag(env: Env, creator: Address, name: String) -> u64;
```

Create a campaign tag. Admin or merchant.

**Returns:** `u64` tag ID. **Errors:** `CampaignTagAlreadyExists` (205).

### `get_campaign_tag` / `get_campaign_tags`

```rust
fn get_campaign_tag(env: Env, tag_id: u64) -> CampaignTag;
fn get_campaign_tags(env: Env) -> Vec<CampaignTag>;
```

Tag lookups.

### `create_campaign`

```rust
fn create_campaign(env: Env, merchant: Address, title: String, description: String, category_id: u64, tags: Vec<u64>, goal_amount: i128, token: Address, deadline: u64) -> u64;
```

Create a fundraising campaign. Requires merchant auth.

**Returns:** `u64` campaign ID. **Events:** `CampaignCreatedEvent`. **Errors:** `InvalidCampaignGoal` (206), `InvalidCampaignDeadline` (207), `CampaignCategoryNotFound` (201), `CampaignTagNotFound` (204).

### `update_campaign`

```rust
fn update_campaign(env: Env, merchant: Address, campaign_id: u64, title: Option<String>, description: Option<String>);
```

Update title/description. Owner-only. Goal/token/deadline are immutable.

### `set_campaign_active`

```rust
fn set_campaign_active(env: Env, merchant: Address, campaign_id: u64, active: bool);
```

Toggle a campaign's active flag. Owner-only.

### `add_campaign_tag` / `remove_campaign_tag`

```rust
fn add_campaign_tag(env: Env, merchant: Address, campaign_id: u64, tag_id: u64);
fn remove_campaign_tag(env: Env, merchant: Address, campaign_id: u64, tag_id: u64);
```

Attach/detach tags. Owner-only. De-duplicated.

### `record_campaign_contribution`

```rust
fn record_campaign_contribution(env: Env, campaign_id: u64, contributor: Address, amount: i128);
```

Record a contribution. Open to any caller.

### `get_campaign` / `get_campaigns`

```rust
fn get_campaign(env: Env, campaign_id: u64) -> Campaign;
fn get_campaigns(env: Env, filter: CampaignFilter) -> Vec<Campaign>;
```

Campaign lookups.

## NFT minting and distribution

### `create_nft_collection`

```rust
fn create_nft_collection(env: Env, merchant: Address, name: String, base_uri: String, max_supply: u64, royalty_bps: u32) -> u64;
```

Create an NFT collection. Requires merchant auth.

**Returns:** `u64` collection ID.

### `mint_nft`

```rust
fn mint_nft(env: Env, merchant: Address, collection_id: u64, recipient: Address, token_uri: String) -> u64;
```

Mint a single NFT. Requires merchant auth.

**Returns:** `u64` NFT ID.

### `batch_mint_nfts`

```rust
fn batch_mint_nfts(env: Env, merchant: Address, collection_id: u64, recipients: Vec<Address>, token_uris: Vec<String>) -> Vec<u64>;
```

Mint NFTs to multiple recipients in one call.

**Returns:** `Vec<u64>` NFT IDs.

### `transfer_nft`

```rust
fn transfer_nft(env: Env, from: Address, to: Address, nft_id: u64);
```

Transfer an NFT. Requires `from` auth.

### `burn_nft`

```rust
fn burn_nft(env: Env, owner: Address, nft_id: u64);
```

Permanently destroy an NFT. Requires owner auth.

### `claim_nft_reward`

```rust
fn claim_nft_reward(env: Env, claimer: Address, nft_id: u64);
```

Claim a reward NFT. Requires claimer auth.

### `deactivate_nft_collection`

```rust
fn deactivate_nft_collection(env: Env, merchant: Address, collection_id: u64);
```

Deactivate a collection so no further minting is possible. Requires merchant auth.

### `get_nft_collection` / `get_nft` / `get_collection_nfts` / `get_user_nfts`

```rust
fn get_nft_collection(env: Env, collection_id: u64) -> NftCollection;
fn get_nft(env: Env, nft_id: u64) -> Nft;
fn get_collection_nfts(env: Env, collection_id: u64) -> Vec<u64>;
fn get_user_nfts(env: Env, user: Address) -> Vec<u64>;
```

NFT lookups.

## Bridge / cross-chain

### `emit_bridge_placeholder`

```rust
fn emit_bridge_placeholder(env: Env, caller: Address, payload: CrossChainBridgePayload);
```

Emit a bridge placeholder event. Requires caller auth.

**Events:** `BridgePlaceholderEvent`.

### `register_bridge_listener` / `remove_bridge_listener` / `is_bridge_listener`

```rust
fn register_bridge_listener(env: Env, admin: Address, listener: Address);
fn remove_bridge_listener(env: Env, admin: Address, listener: Address);
fn is_bridge_listener(env: Env, listener: Address) -> bool;
```

Manage authorized bridge listeners. Admin-only.

**Events:** `BridgeListenerRegisteredEvent`, `BridgeListenerRemovedEvent`.

### `record_bridge_deposit`

```rust
fn record_bridge_deposit(env: Env, listener: Address, source_chain: String, source_tx_id: BytesN<32>, token: Address, amount: i128, recipient: Address) -> u64;
```

Record a confirmed external-chain deposit. Listener-only. De-duplicated on `source_tx_id`.

**Returns:** `u64` deposit ID. **Events:** `BridgeDepositRecordedEvent`. **Errors:** `BridgeDepositProcessed` (49).

### `get_bridge_deposit` / `is_bridge_deposit_processed` / `get_bridge_deposit_count` / `get_bridge_credit`

```rust
fn get_bridge_deposit(env: Env, deposit_id: u64) -> Option<BridgeDeposit>;
fn is_bridge_deposit_processed(env: Env, source_tx_id: BytesN<32>) -> bool;
fn get_bridge_deposit_count(env: Env) -> u64;
fn get_bridge_credit(env: Env, recipient: Address, token: Address) -> i128;
```

Bridge deposit lookups.

### Cross-chain pledges

```rust
fn create_backer_campaign(env: Env, source_chain: String, source_pledge_id: u64, destination_chain: String, merchant: Address, payer: Address, token: Address, amount: i128, memo: Option<String>) -> u64;
fn update_cross_chain_pledge_status(env: Env, pledge_id: u64, new_status: CrossChainPledgeStatus);
fn get_cross_chain_pledge(env: Env, pledge_id: u64) -> CrossChainPledge;
fn get_cross_chain_pledge_by_source(env: Env, source_chain: String, source_pledge_id: u64) -> CrossChainPledge;
fn get_all_cross_chain_pledges(env: Env) -> Vec<CrossChainPledge>;
```

Cross-chain pledge management.

**Events:** `CrossChainPledgeCreatedEvent`, `CrossChainPledgeUpdatedEvent`.

## DAO governance

### `add_gov_member` / `remove_gov_member` / `is_gov_member` / `get_gov_member_count`

```rust
fn add_gov_member(env: Env, admin: Address, member: Address);
fn remove_gov_member(env: Env, admin: Address, member: Address);
fn is_gov_member(env: Env, member: Address) -> bool;
fn get_gov_member_count(env: Env) -> u32;
```

Manage governance council members. Admin-only.

**Events:** `GovMemberAddedEvent`, `GovMemberRemovedEvent`.

### `set_governance_config`

```rust
fn set_governance_config(env: Env, admin: Address, voting_period: u64, quorum_bps: u32);
```

Configure voting window and quorum. Admin-only.

**Events:** `GovConfigSetEvent`. **Errors:** `InvalidGovConfig` (102).

### `propose_upgrade`

```rust
fn propose_upgrade(env: Env, proposer: Address, wasm_hash: BytesN<32>) -> u64;
```

Open an upgrade proposal. Member-only.

**Returns:** `u64` proposal ID. **Events:** `UpgradeProposedEvent`. **Errors:** `NotGovMember` (100), `GovNotConfigured` (101).

### `vote_on_upgrade`

```rust
fn vote_on_upgrade(env: Env, voter: Address, proposal_id: u64, approve: bool);
```

Cast a vote. Member-only.

**Events:** `UpgradeVoteCastEvent`. **Errors:** `AlreadyVoted` (107), `VotingClosed` (105), `ProposalNotActive` (104).

### `finalize_upgrade`

```rust
fn finalize_upgrade(env: Env, caller: Address, proposal_id: u64);
```

Finalize a proposal after voting closes. Applies the upgrade if passed.

**Events:** `UpgradeProposalFinalizedEvent`. **Errors:** `VotingStillOpen` (106), `ProposalNotActive` (104).

### `get_upgrade_proposal` / `has_voted_on_upgrade`

```rust
fn get_upgrade_proposal(env: Env, proposal_id: u64) -> Option<UpgradeProposal>;
fn has_voted_on_upgrade(env: Env, proposal_id: u64, member: Address) -> bool;
```

Proposal lookups.

## Multi-sig withdrawal

### `set_multisig_threshold`

```rust
fn set_multisig_threshold(env: Env, admin: Address, token: Address, threshold: i128);
```

Set the per-token withdrawal threshold. Admin-only.

### `get_multisig_threshold`

```rust
fn get_multisig_threshold(env: Env, token: Address) -> i128;
```

Returns the configured threshold. **Errors:** `ThresholdNotSet` (149).

### `configure_multisig`

```rust
fn configure_multisig(env: Env, admin: Address, signers: Vec<Address>, quorum: u32);
```

Replace the signer list and quorum. Admin-only.

**Errors:** `InvalidQuorum` (142).

### `propose_withdrawal`

```rust
fn propose_withdrawal(env: Env, merchant: Address, token: Address, amount: i128, recipient: Address, note: String) -> u64;
```

Open a withdrawal proposal. Amount must be >= threshold.

**Returns:** `u64` proposal ID. **Errors:** `BelowMultiSigThreshold` (140).

### `approve_withdrawal`

```rust
fn approve_withdrawal(env: Env, signer: Address, proposal_id: u64);
```

Approve a pending proposal. Signer-only. Executes automatically when quorum is reached.

**Events:** `WithdrawalApprovedEvent`. **Errors:** `NotASigner` (143), `AlreadyApproved` (144), `QuorumNotReached` (147).

### `cancel_withdrawal`

```rust
fn cancel_withdrawal(env: Env, caller: Address, proposal_id: u64);
```

Cancel a pending proposal. Proposer or admin only.

### `get_withdrawal_proposal` / `has_approved_withdrawal` / `get_withdrawal_proposal_count`

```rust
fn get_withdrawal_proposal(env: Env, proposal_id: u64) -> WithdrawalProposal;
fn has_approved_withdrawal(env: Env, signer: Address, proposal_id: u64) -> bool;
fn get_withdrawal_proposal_count(env: Env) -> u64;
```

Proposal lookups.

## Search and filtering

### `search_invoices_paginated`

```rust
fn search_invoices_paginated(env: Env, caller: Address, filter: InvoiceFilter, cursor: u64, page_size: u32) -> InvoicePage;
```

Paginated invoice search with full filter support. Pass `cursor = 0` for the first page.

**Returns:** `InvoicePage`.

### `search_merchants_paginated`

```rust
fn search_merchants_paginated(env: Env, filter: MerchantFilter, cursor: u64, page_size: u32) -> MerchantPage;
```

Paginated merchant search.

**Returns:** `MerchantPage`.

### `search_subscription_plans`

```rust
fn search_subscription_plans(env: Env, caller: Address, filter: SubscriptionPlanFilter) -> Vec<SubscriptionPlan>;
```

Filter subscription plans.

### `search_subscriptions`

```rust
fn search_subscriptions(env: Env, filter: SubscriptionFilter) -> Vec<Subscription>;
```

Filter subscriptions.

### `search_events`

```rust
fn search_events(env: Env, caller: Address, filter: EventFilter) -> Vec<Event>;
```

Filter on-chain events.

### `search_withdrawal_proposals`

```rust
fn search_withdrawal_proposals(env: Env, caller: Address, filter: WithdrawalProposalFilter) -> Vec<WithdrawalProposal>;
```

Filter withdrawal proposals.

← [Back to reference](README.md)
