# Monitoring and alerting

What to monitor on a live `shade` deployment, how to derive metrics from on-chain events and RPC reads, when to alert, and how to run automated RPC health checks. Use this guide alongside the [Admin Runbook](admin-runbook.md) and [Upgrade Runbook](upgrade-runbook.md) to detect failed payments, paused contracts, unauthorized admin actions, unusual volume, and nearly expired state quickly.

> **Note:** This repository does not ship a built-in monitoring stack or alerting daemon. You operate the indexer, metrics exporter, and alerting pipeline off-chain against a `soroban-rpc` endpoint — see the [Event indexing guide](../guides/indexing-events.md) for the ingestion architecture.

## Key metrics

Derive operational metrics from two complementary sources:

- **On-chain events** streamed via Soroban RPC `getEvents` (fast, append-only signal for state transitions).
- **Contract reads** via `simulateTransaction` / `stellar contract invoke` (authoritative point-in-time state for reconciliation and gauges).

> **Note:** Reverted transactions emit no Soroban events (see [Events reference § Precise semantics](../reference/events.md#precise-semantics)). Always measure error rates and payment success rates from RPC transaction status alongside emitted events.

| Metric | Type | Primary source | How to derive |
|---|---|---|---|
| Payment volume | Counter / Rate | `InvoicePaidEvent`, `PlatformFeeRoutedEvent` | Sum `amount` (or `gross_amount`) grouped by `token` and `merchant_id`. Cross-check periodically against `get_token_volume`, `get_token_analytics`, and `get_merchant_volume`. |
| Payment success rate | Ratio (0–100%) | RPC tx results + `InvoicePaidEvent` | `successful_payment_txs / total_payment_txs` over a rolling 5m and 1h window for `pay_invoice`, `pay_invoice_partial`, and `pay_invoices_batch` calls targeting the `shade` contract ID. |
| Refund rate | Ratio (0–100%) | `InvoiceRefundedEvent`, `InvoicePartiallyRefundedEvent` | `sum(refunded_amount) / sum(paid_amount)` (and refund count / payment count) over a rolling 1h and 24h window, globally and per [merchant](../glossary.md#merchant). |
| Active merchants | Gauge | `MerchantRegisteredEvent`, `MerchantStatusChangedEvent` | Count merchants whose latest `active` flag is `true`. Reconcile periodically with `get_merchants` / `is_merchant_active` (and track `is_merchant_verified` separately). |
| Pending fee changes | Gauge + Countdown | `FeeProposedEvent`, `FeeSetEvent`, `get_pending_fee` | Count whitelisted tokens where `get_pending_fee(token)` returns a pending proposal, and track seconds remaining until the 48-hour (`172,800`s) `FEE_UPDATE_DELAY` elapses. |
| Paused state | Gauge (`0` / `1`) | `ContractPausedEvent`, `ContractUnpausedEvent`, `is_paused` | `1` when `is_paused()` returns `true`, `0` when `false`. Also track continuous pause duration in seconds. |
| Withdrawal volume | Counter / Histogram | `AutoWithdrawalTriggeredEvent`, `AutoWithdrawalFailedEvent`, `WithdrawalToEvent` | Track per-event `amount` and rolling 1h sum per `token` and merchant across `shade` auto-withdrawals and merchant `account` withdrawals. |
| Nearly expired state | Gauge | `get_invoices`, RPC `getLedgerEntries` | Count `Pending` [invoices](../glossary.md#invoice) within 1 hour of `expires_at`, and track minimum remaining TTL (in ledgers) for `shade` and `account` contract instance/persistent entries. |
| Subscription grace counts | Gauge / Ratio | Off-chain subscription indexer or satellite contract | Count active subscriptions currently in a payment grace period, ratio of grace-period to active subscriptions, and subscriptions within 24h of grace expiry. *(See note below.)* |

> **Note:** The core `shade` contract in this workspace handles invoices, payments, fees, and merchant accounts (see [ADR-0007: Core gateway scope](../architecture/decisions/0007-core-gateway-scope.md)) and does not store subscription records on-chain. If you run a subscription billing service or satellite contract on top of `shade`, emit and scrape the subscription grace count metrics from that service's database or contract reads.

## Alerts and thresholds

Severity levels match [Admin Runbook § Severity and escalation](admin-runbook.md#severity-and-escalation): **Critical** pages immediately (target response: 15 minutes); **High** pages on-call (target response: 1 hour); **Medium** notifies during business hours (target response: 1 business day).

| Alert | Trigger signal | Threshold | Severity | Runbook action |
|---|---|---|---|---|
| Admin transfer proposed or accepted | `AdminTransferProposedEvent`, `AdminTransferAcceptedEvent`, or `get_admin` mismatch | Any occurrence (`count > 0`) | **Critical** | Verify against change log; if unauthorized, execute [Admin Runbook § Pause decision](admin-runbook.md#pause-decision) and [Admin transfer](admin-runbook.md#admin-transfer). |
| Role granted or revoked | `RoleGrantedEvent`, `RoleRevokedEvent` | Any occurrence (`count > 0`) | **High** (**Critical** if unapproved) | Verify caller and target against change log; revoke unauthorized grants via [Admin Runbook § Role management](admin-runbook.md#role-management). |
| Platform account changed | `PlatformAccountSetEvent` or `get_platform_account` mismatch | Any occurrence (`count > 0`) | **Critical** | Verify against change log; if unauthorized, pause and rotate per [Admin Runbook § Platform account rotation](admin-runbook.md#platform-account-rotation). |
| Time-locked fee proposed | `FeeProposedEvent` | Any occurrence (`count > 0`) | **High** | Confirm token, basis points, and 48h execution window against change log per [Admin Runbook § Routine operation: fees](admin-runbook.md#routine-operation-fees). |
| Immediate fee change (`set_fee`) | `FeeSetEvent` with no matching `FeeProposedEvent` ≥ 48h prior | Any occurrence (`count > 0`) | **Critical** | Confirm emergency approval in change log; if unapproved, pause per [Admin Runbook § Pause decision](admin-runbook.md#pause-decision) and correct via [Routine operation: fees](admin-runbook.md#routine-operation-fees). |
| Contract paused | `ContractPausedEvent` or `is_paused == true` | Any occurrence (`is_paused == true` for > 1m) | **Critical** (unplanned) / **High** (planned) | Confirm planned maintenance or initiate triage; follow [Admin Runbook § Pause procedure](admin-runbook.md#pause-procedure) and [Unpause criteria](admin-runbook.md#unpause-criteria). |
| Contract upgraded | `ContractUpgradedEvent` or deployed WASM hash drift | Any occurrence (`count > 0` or hash mismatch) | **Critical** | If planned, complete [Upgrade Runbook § Verification](upgrade-runbook.md#verification); if unexpected, pause immediately and assess [Upgrade Runbook § Rollback](upgrade-runbook.md#rollback). |
| Unusually large withdrawal | `AutoWithdrawalTriggeredEvent`, `WithdrawalToEvent`, `PendingWithdrawalCreatedEvent` | Single withdrawal > 3× 7-day P95 (or > per-token cap), or 1h volume > 5× 7-day baseline | **High** (**Critical** if unattributed) | Inspect merchant and account state via [Admin Runbook § Triage and diagnosis](admin-runbook.md#triage-and-diagnosis); restrict account if compromised via [Merchant operations](admin-runbook.md#merchant-operations). |
| Auto-withdrawal failures | `AutoWithdrawalFailedEvent` | > 3 failures in 15 minutes for any merchant | **Medium** | Inspect merchant recipient and account state per [Admin Runbook § Merchant operations](admin-runbook.md#merchant-operations). |
| Spike in transaction errors | Failed RPC transactions targeting `shade` (`pay_invoice*`, `create_invoice*`) | Error rate > 5% over 5m (or > 10 failed payments in 5m) → **High**; > 20% over 5m → **Critical** | **High** / **Critical** | Investigate root cause via [Admin Runbook § Triage and diagnosis](admin-runbook.md#triage-and-diagnosis) and evaluate [Pause decision](admin-runbook.md#pause-decision). |
| Refund rate spike | `InvoiceRefundedEvent` + `InvoicePartiallyRefundedEvent` vs. `InvoicePaidEvent` | Refund volume > 10% of settled payment volume over 1h (min. 5 refunds) | **Medium** (**High** if > 25%) | Inspect affected invoices and merchants via [Admin Runbook § Triage and diagnosis](admin-runbook.md#triage-and-diagnosis). |
| RPC health check failed | Synthetic probe failure (unreachable, timeout, or bad response) | 2 consecutive failed probes (≥ 2 minutes) | **High** (**Critical** if all RPC endpoints fail) | Verify RPC provider status and contract state per [Admin Runbook § Triage and diagnosis](admin-runbook.md#triage-and-diagnosis). |
| Contract TTL nearly expired | `getLedgerEntries` `liveUntilLedgerSeq` minus current ledger | < 120,960 ledgers (~7 days at 5s/ledger) → **Medium**; < 34,560 ledgers (~2 days) → **High** | **Medium** / **High** | Extend contract instance/persistent TTL before archival; verify deployment state per [Deployment Runbook](deployment.md). |
| Subscription grace count spike | Off-chain subscription grace-period ratio | > 10% of active subscriptions in grace period, or > 2× 7-day baseline | **Medium** | Check for payment or oracle failures affecting renewals via [Admin Runbook § Triage and diagnosis](admin-runbook.md#triage-and-diagnosis). |

> **Security:** Treat any unapproved `AdminTransferProposedEvent`, `RoleGrantedEvent`, `PlatformAccountSetEvent`, `FeeSetEvent`, or `ContractUpgradedEvent` as a potential [admin](../glossary.md#admin) key compromise. Follow [Admin ownership § Compromised admin key](../security/admin-and-ownership.md#compromised-admin-key--blast-radius) immediately.

## RPC health checks

Run a lightweight synthetic health check every 60 seconds against your primary and fallback Soroban RPC endpoints. A healthy check verifies four invariants:

1. **Contract reachable** — the RPC endpoint responds healthy (`getHealth`) and read-only contract simulations succeed within your latency budget (e.g. < 2 seconds).
2. **`is_paused` is `false`** — confirms payments and invoice creation are not halted.
3. **Admin unchanged** — `get_admin` matches the pinned expected admin address recorded in your [change log](admin-runbook.md#change-log).
4. **WASM hash unchanged** — the deployed contract's executable WASM hash matches the hash recorded in your [deployment artifact record](deployment.md#artifact-recording) (see [Upgradeability § Establishing what is deployed](../architecture/upgradeability.md#establishing-what-is-deployed)).

### Example health-check script

Replace the four configuration variables at the top of the script with your deployment's values before running it in a cron job or monitoring runner:

```bash
#!/usr/bin/env bash
set -euo pipefail

# Replace these placeholders with your live deployment values:
RPC_URL="${RPC_URL:-https://soroban-testnet.stellar.org}"
NETWORK="${NETWORK:-testnet}"
SHADE_ID="${SHADE_ID:?Set SHADE_ID to the deployed shade contract ID}"
EXPECTED_ADMIN="${EXPECTED_ADMIN:?Set EXPECTED_ADMIN to the pinned admin address}"
EXPECTED_WASM_HASH="${EXPECTED_WASM_HASH:?Set EXPECTED_WASM_HASH to the recorded 64-char hex WASM hash}"

# 1. RPC reachability
curl -sf -X POST "$RPC_URL" \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","id":1,"method":"getHealth"}' | grep -q '"status":"healthy"'

# 2. Contract reachable & is_paused == false
PAUSED=$(stellar contract invoke --id "$SHADE_ID" --network "$NETWORK" -- is_paused)
if [[ "$PAUSED" != "false" ]]; then
  echo "ALERT: Contract $SHADE_ID is paused (is_paused=$PAUSED)" >&2
  exit 1
fi

# 3. Admin unchanged
ACTUAL_ADMIN=$(stellar contract invoke --id "$SHADE_ID" --network "$NETWORK" -- get_admin | tr -d '"')
if [[ "$ACTUAL_ADMIN" != "$EXPECTED_ADMIN" ]]; then
  echo "CRITICAL: Admin mismatch! expected=$EXPECTED_ADMIN actual=$ACTUAL_ADMIN" >&2
  exit 1
fi

# 4. WASM hash unchanged
# Inspect the deployed contract instance via Stellar CLI and compare the recorded hash.
ACTUAL_WASM_HASH=$(stellar contract fetch --id "$SHADE_ID" --network "$NETWORK" | sha256sum | awk '{print $1}')
if [[ "$ACTUAL_WASM_HASH" != "$EXPECTED_WASM_HASH" ]]; then
  echo "CRITICAL: WASM hash mismatch! expected=$EXPECTED_WASM_HASH actual=$ACTUAL_WASM_HASH" >&2
  exit 1
fi

echo "OK: $SHADE_ID reachable, unpaused, admin and WASM hash verified."
```

Placeholders to supply via environment variables:

- `RPC_URL`: Your `soroban-rpc` endpoint URL.
- `NETWORK`: Stellar CLI network name (`testnet`, `mainnet`, or custom).
- `SHADE_ID`: The deployed `shade` contract address (`C...`).
- `EXPECTED_ADMIN`: The current authorized admin address from your change log.
- `EXPECTED_WASM_HASH`: The 64-character hex SHA-256 hash of the deployed `.optimized.wasm` from your deployment artifact record.

## Tooling options

Choose tooling that fits your team's existing stack; every option below consumes the same `soroban-rpc` `getEvents` and read-only simulation endpoints.

| Layer | Recommended options | Notes |
|---|---|---|
| Event ingestion & indexing | Custom worker (Rust/TypeScript) polling `getEvents` per [Event indexing guide](../guides/indexing-events.md); Stellar ecosystem indexers (Mercury, Zephyr, Hubble) | Persist events in PostgreSQL (using `numeric` for `i128` amounts) so metrics survive RPC retention windows. |
| Metrics storage | Prometheus, Grafana Cloud, Datadog | Expose counters and gauges from your indexer and health-check runner on a `/metrics` endpoint. |
| Dashboards | Grafana, Datadog Dashboards, Metabase | Grafana pairs cleanly with Prometheus + PostgreSQL; Metabase works directly over the indexer's SQL tables. |
| Alerting & paging | Prometheus Alertmanager + PagerDuty/Opsgenie, Grafana Alerting, Slack/Discord webhooks | Route **Critical** and **High** alerts to paging; route **Medium** alerts to an operations channel. |
| Synthetic RPC probes | Kubernetes `CronJob`, GitHub Actions scheduled workflow, Grafana Synthetic Monitoring | Run the health-check script above from at least two independent RPC providers to avoid false positives from a single node outage. |

## Example dashboard layouts

Organize dashboards into two focused views so on-call operators can assess protocol safety in seconds and drill into payment health without clutter.

### Dashboard 1: Protocol health and security

Designed for on-call triage and executive health status.

```text
+----------------------------------+----------------------------------+----------------------------------+
| Contract Status (Stat)           | Admin & WASM Integrity (Stat)    | RPC Probe Latency (Time Series)  |
| is_paused: FALSE (green)         | Admin: MATCH | WASM: MATCH       | p50 / p95 ms per RPC endpoint    |
+----------------------------------+----------------------------------+----------------------------------+
| Payment Success Rate (Gauge/TS)  | Failed Tx by Error Code (Bar)    | Pending Fee Proposals (Table)    |
| Rolling 5m & 1h %                | Count of failed shade calls/5m   | Token | Proposed bps | ETA (48h) |
+----------------------------------+----------------------------------+----------------------------------+
| Privileged & Security Events Feed (Log / Table)                                                        |
| Timestamp | Event (AdminTransfer, RoleGranted, FeeProposed, FeeSet, Paused, Upgraded) | Tx Hash        |
+--------------------------------------------------------------------------------------------------------+
```

### Dashboard 2: Payments, merchants, and withdrawals

Designed for operations and merchant support.

```text
+----------------------------------+----------------------------------+----------------------------------+
| 24h Payment Volume by Token (TS) | Refund Rate 1h / 24h (Stat + TS) | Active / Verified Merchants      |
| Stacked bar by token symbol      | Refunded / Paid % + count        | Active count | Verified count    |
+----------------------------------+----------------------------------+----------------------------------+
| Withdrawals & Anomalies (TS)     | Auto-Withdrawal Failures (Stat)  | Expiring State & Grace Counts    |
| Per-token withdrawal volume +    | AutoWithdrawalFailedEvent / 15m  | Invoices <1h to expiry | TTL min |
| 3x P95 threshold line            |                                  | Subscription grace count (if any)|
+----------------------------------+----------------------------------+----------------------------------+
| Top Merchants by Volume & Refund Rate (Table)                                                          |
| Merchant ID | 24h Volume | Payment Count | Refund Rate % | Account Restricted?                         |
+--------------------------------------------------------------------------------------------------------+
```

## Verification

Confirm your monitoring setup is working end-to-end on testnet before relying on it for mainnet:

1. Run the [RPC health-check script](#example-health-check-script) against your testnet `shade` contract and confirm it exits `0` with `OK`.
2. Trigger a controlled testnet event (such as `propose_fee` or `pause` followed by `unpause`) and confirm the corresponding alert fires with the expected severity and runbook link.
3. Verify that both dashboard views update within your target ingestion lag (typically < 30 seconds).

## Related pages

- [Admin Operations and Incident Response Runbook](admin-runbook.md) — response procedures, severity levels, and pause/unpause criteria linked from the alerts above.
- [Contract Upgrade Runbook](upgrade-runbook.md) — verification and rollback procedures for `ContractUpgradedEvent` alerts.
- [Deployment Runbook](deployment.md) — artifact recording for pinned admin addresses and WASM hashes.
- [Event indexing guide](../guides/indexing-events.md) — off-chain ingestion, schema design, and reconciliation.
- [Events reference](../reference/events.md) — complete catalog of `shade` and `account` events and their payload fields.

← [Back to Operations](README.md)
