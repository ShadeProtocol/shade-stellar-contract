# Pausable emergency-stop mechanism

This page documents the contract-wide pause functionality: what it blocks, who may toggle it, and how it should be used operationally during an incident.

## Overview

The pausable component (`components/pausable.rs`) provides a global emergency stop. When paused, most state-changing functions revert with `ContractPaused` (error code 9). Read-only queries and admin-critical functions remain callable.

## Public functions

### `pause`

```rust
fn pause(env: Env, admin: Address);
```

- **Authorization:** `admin.require_auth()` + `assert_admin`.
- **Effect:** Sets `DataKey::Paused = true` in persistent storage.
- **Idempotency:** Panics with `ContractNotPaused` if already paused — pausing is not idempotent.
- **Event:** Emits `ContractPausedEvent` with the admin address and timestamp.

### `unpause`

```rust
fn unpause(env: Env, admin: Address);
```

- **Authorization:** `admin.require_auth()` + `assert_admin`.
- **Effect:** Sets `DataKey::Paused = false`.
- **Idempotency:** Panics with `ContractNotPaused` if already unpaused.
- **Event:** Emits `ContractUnpausedEvent` with the admin address and timestamp.

### `is_paused`

```rust
fn is_paused(env: Env) -> bool;
```

- **Authorization:** None (read-only).
- Returns `false` if no pause state has ever been set.

## Internal assertions

### `assert_not_paused`

Called at the top of state-changing functions. Panics with `ContractPaused` if the contract is paused. This is the primary gate that blocks operations.

### `assert_paused`

Panics with `ContractNotPaused` if the contract is NOT paused. Used by `unpause` to prevent double-unpause.

## Blocked vs allowed while paused

Functions are marked as **blocked** if they call `pausable_component::assert_not_paused` in `shade.rs`.

| Function | Paused status |
|----------|--------------|
| `initialize` | Allowed (one-time only; no pause check) |
| `get_admin` | Allowed |
| `add_accepted_token` | Blocked |
| `add_accepted_tokens` | Blocked |
| `remove_accepted_token` | Blocked |
| `is_accepted_token` | Allowed |
| `set_account_wasm_hash` | Allowed |
| `set_fee` | Blocked |
| `get_fee` | Allowed |
| `set_platform_account` | Blocked |
| `get_platform_account` | Allowed |
| `set_token_oracle` | Blocked |
| `get_token_oracle` | Allowed |
| `propose_fee` | Blocked |
| `execute_fee` | Blocked |
| `get_pending_fee` | Allowed |
| `register_merchant` | Blocked |
| `get_merchant` | Allowed |
| `get_merchants` | Allowed |
| `is_merchant` | Allowed |
| `set_merchant_status` | Allowed |
| `is_merchant_active` | Allowed |
| `verify_merchant` | Allowed |
| `is_merchant_verified` | Allowed |
| `create_invoice` | Blocked |
| `create_fiat_invoice` | Blocked |
| `create_invoice_draft` | Blocked |
| `finalize_invoice` | Blocked |
| `create_invoice_signed` | Blocked |
| `get_invoice` | Allowed |
| `resolve_invoice_amount` | Allowed |
| `refund_invoice` | Blocked |
| `claim_refund` | Blocked |
| `set_merchant_key` | Allowed |
| `get_merchant_key` | Allowed |
| `grant_role` | Allowed |
| `revoke_role` | Allowed |
| `has_role` | Allowed |
| `get_invoices` | Allowed |
| `refund_invoice_partial` | Blocked |
| `pause` | Allowed |
| `unpause` | Allowed |
| `is_paused` | Allowed |
| `upgrade` | Allowed |
| `restrict_merchant_account` | Allowed |
| `calculate_fee` | Allowed |
| `compute_platform_fee_split` | Allowed |
| `set_merchant_platform_fee` | Blocked |
| `get_merchant_platform_fee` | Allowed |
| `clear_merchant_platform_fee` | Blocked |
| `pay_invoice` | Blocked |
| `pay_invoices_batch` | Blocked |
| `pay_invoice_partial` | Blocked |
| `validate_payment_payload` | Allowed |
| `void_invoice` | Blocked |
| `amend_invoice` | Blocked |
| `propose_admin_transfer` | Allowed |
| `accept_admin_transfer` | Allowed |
| `create_subscription_plan` | Blocked |
| `get_subscription_plan` | Allowed |
| `subscribe` | Blocked |
| `get_subscription` | Allowed |
| `charge_subscription` | Blocked |
| `cancel_subscription` | Blocked |
| `deactivate_plan` | Blocked |
| `get_user_transactions` | Allowed |
| `emit_bridge_placeholder` | Blocked |
| `register_bridge_listener` | Blocked |
| `remove_bridge_listener` | Blocked |
| `is_bridge_listener` | Allowed |
| `record_bridge_deposit` | Blocked |
| `get_bridge_deposit` | Allowed |
| `is_bridge_deposit_processed` | Allowed |
| `get_bridge_deposit_count` | Allowed |
| `get_bridge_credit` | Allowed |
| `add_gov_member` | Blocked |
| `remove_gov_member` | Blocked |
| `is_gov_member` | Allowed |
| `get_gov_member_count` | Allowed |
| `set_governance_config` | Blocked |
| `propose_upgrade` | Blocked |
| `vote_on_upgrade` | Blocked |
| `finalize_upgrade` | Blocked |
| `get_upgrade_proposal` | Allowed |
| `has_voted_on_upgrade` | Allowed |
| `create_event` | Blocked |
| `purchase_ticket` | Blocked |
| `configure_dynamic_pricing` | Blocked |
| `get_current_ticket_price` | Allowed |
| `cancel_event_and_batch_refund` | Blocked |
| `resell_ticket` | Blocked |
| `get_event` | Allowed |
| `get_ticket` | Allowed |
| `get_event_tickets` | Allowed |
| `get_user_tickets` | Allowed |
| `purchase_tickets_bulk` | Blocked |
| `create_nft_collection` | Blocked |
| `mint_nft` | Blocked |
| `batch_mint_nfts` | Blocked |
| `transfer_nft` | Blocked |
| `burn_nft` | Blocked |
| `claim_nft_reward` | Blocked |
| `deactivate_nft_collection` | Blocked |
| `get_nft_collection` | Allowed |
| `get_nft` | Allowed |
| `get_collection_nfts` | Allowed |
| `get_user_nfts` | Allowed |
| `set_multisig_threshold` | Blocked |
| `get_multisig_threshold` | Allowed |
| `configure_multisig` | Blocked |
| `propose_withdrawal` | Blocked |
| `approve_withdrawal` | Blocked |
| `cancel_withdrawal` | Blocked |
| `get_withdrawal_proposal` | Allowed |
| `has_approved_withdrawal` | Allowed |
| `get_withdrawal_proposal_count` | Allowed |
| `create_escrow` | Blocked |
| `get_escrow` | Allowed |
| `fund_escrow` | Blocked |
| `release_escrow` | Blocked |
| `refund_escrow` | Blocked |
| `create_campaign_category` | Blocked |
| `update_campaign_category` | Blocked |
| `get_campaign_category` | Allowed |
| `get_campaign_categories` | Allowed |
| `create_campaign_tag` | Blocked |
| `get_campaign_tag` | Allowed |
| `get_campaign_tags` | Allowed |
| `create_campaign` | Blocked |
| `update_campaign` | Blocked |
| `set_campaign_active` | Blocked |
| `add_campaign_tag` | Blocked |
| `remove_campaign_tag` | Blocked |
| `record_campaign_contribution` | Blocked |
| `get_campaign` | Allowed |
| `get_campaigns` | Allowed |
| `create_backer_campaign` | Blocked |
| `pledge_to_campaign` | Blocked |
| `select_backer_reward_tier` | Blocked |
| `fulfill_backer_reward` | Blocked |
| `claim_backer_perk` | Blocked |
| `create_stretch_goal` | Blocked |
| `unlock_stretch_goal` | Blocked |
| `cancel_stretch_goal` | Blocked |
| `grant_stretch_goal_reward` | Blocked |
| `claim_stretch_goal_reward` | Blocked |

## Operational playbook

### When to pause

Pause the contract immediately if you detect:
- Unauthorized fund movement or suspected key compromise.
- A vulnerability being actively exploited.
- Unexpected behavior in payment flows, subscriptions, or escrow.

### What to communicate to merchants

1. Announce the pause with a clear reason and expected investigation timeline.
2. Advise merchants that invoice creation, payments, and subscriptions are frozen.
3. Reassure that read-only queries still work and no funds are at risk from the pause itself.

### What state can be safely inspected while paused

All read-only functions remain callable. Merchants and users can:
- Query invoices, subscriptions, merchants, campaigns, and analytics.
- Verify current fee rates, token acceptlists, and oracle configs.
- Check escrow statuses and bridge deposit records.

### Checklist before unpausing

1. Verify the root cause is fully resolved.
2. Review all transactions executed between pause and unpause (if any admin operations were performed).
3. Confirm no in-flight transactions were left in an inconsistent state.
4. Test the fix on testnet or in a simulation if possible.
5. Unpause via `unpause(admin)` and verify `is_paused` returns `false`.
6. Monitor the first few transactions post-unpause for anomalies.

## Risks of pausing with in-flight state

| Risk | Description |
|------|-------------|
| In-flight subscriptions | Active subscriptions continue to accrue time but cannot be charged while paused. When unpaused, `charge_subscription` may attempt to bill for an overdue period. |
| Expiring invoices | Invoices with `expires_at` may expire while paused, preventing payment even after unpausing. |
| Escrow deadlines | Escrowed funds may reach their deadline while paused, potentially triggering automatic refund logic. |
| Auto-withdrawal thresholds | Merchant auto-withdrawal triggers are frozen. Large balances may accumulate and execute in bulk when unpaused. |
| Governance voting windows | Upgrade proposals may have their voting windows close during a pause, potentially passing or failing without full participation. |

## Known limitations

- The pause is global — there is no per-feature granularity (e.g., pausing payments while leaving subscriptions active). A future enhancement is proposed in the component source.
- Idempotency is not supported: calling `pause` on an already-paused contract panics. The admin must ensure the contract is unpaused before attempting to pause again.

← [Back to security](README.md)
