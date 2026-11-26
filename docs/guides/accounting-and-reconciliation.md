# Merchant Accounting and Reconciliation Guide

> **Disclaimer:** This guide describes technical patterns for mapping Shade on-chain events and contract state into an accounting ledger. It is **not** tax, legal, or accounting advice. Consult a qualified accountant or tax advisor for jurisdiction-specific rules, revenue recognition, and reporting obligations.

Merchants using Shade receive funds through the core `shade` contract into their dedicated `account` contract, with protocol/platform fees routed separately and refunds or withdrawals moving funds back out. This guide shows how to turn Shade's event stream and contract reads into a complete, auditable double-entry ledger.

---

## 1. Event-to-Accounting Mapping

Every money-moving operation emits one or more Soroban `#[contractevent]` records from `contracts/shade/src/events.rs` or `contracts/account/src/events.rs`.

| Event | Contract | Accounting Role | Debit / Credit Treatment (Merchant Books) | Key Fields |
| :--- | :--- | :--- | :--- | :--- |
| `InvoicePaidEvent` | `shade` | **Gross payment, protocol fee, net revenue** | **Debit** `Merchant Account Asset` (`merchant_amount`) + **Debit** `Protocol Fee Expense` (`fee`) / **Credit** `Gross Sales Revenue` (`amount`) | `invoice_id`, `merchant_id`, `merchant_account`, `payer`, `amount`, `fee`, `merchant_amount`, `token`, `timestamp` |
| `PaymentSplitRoutedEvent` | `shade` | **Split confirmation** (merchants vs platform) | Informational confirmation matching `InvoicePaidEvent` (`merchant_amount + platform_amount == gross`) | `invoice_id`, `merchant_account`, `platform_account`, `merchant_amount`, `platform_amount`, `token`, `timestamp` |
| `PlatformFeeRoutedEvent` | `shade` | **Detailed fee routing** (includes BPS rate) | Confirms `gross_amount = platform_fee + merchant_amount` and records `fee_bps_applied` for audit | `route_kind`, `ref_id`, `merchant_id`, `gross_amount`, `platform_fee`, `merchant_amount`, `fee_bps_applied`, `token`, `timestamp` |
| `InvoiceRefundedEvent` | `shade` | **Full refund** | **Debit** `Refunds / Contra-Revenue` (`amount`) / **Credit** `Merchant Account Asset` (or settlement source) | `invoice_id`, `merchant`, `amount`, `timestamp` |
| `InvoicePartiallyRefundedEvent` | `shade` | **Partial refund** | **Debit** `Refunds / Contra-Revenue` (`amount`) / **Credit** `Merchant Account Asset` (`total_amount_refunded` tracks cumulative refunded) | `invoice_id`, `merchant`, `amount`, `total_amount_refunded`, `timestamp` |
| `InvoiceExpiredRefundEvent` | `shade` | **Expired invoice refund** to buyer | **Debit** `Refunds / Contra-Revenue` (`amount`) / **Credit** `Merchant Account Asset` | `invoice_id`, `buyer`, `amount`, `token`, `timestamp` |
| `RefundProcessedEvent` | `account` | **Vault outflow for refund** | Confirms token outflow from the merchant `account` contract to `recipient` | `token`, `amount`, `recipient`, `timestamp` |
| `WithdrawalToEvent` | `account` | **Direct merchant withdrawal** | **Debit** `Treasury / External Wallet` (`amount`) / **Credit** `Merchant Account Asset` (`amount`) | `token`, `merchant`, `recipient`, `amount`, `timestamp` |
| `WithdrawalExecutedEvent` | `account` | **Multi-sig / threshold withdrawal executed** | Join with `PendingWithdrawalCreatedEvent` by `id` for `token`, `amount`, `recipient`, then **Debit** `Treasury` / **Credit** `Merchant Account Asset` | `id`, `timestamp` |
| `AutoWithdrawalTriggeredEvent` | `shade` | **Automated threshold sweep** | **Debit** `Treasury / Recipient Wallet` (`amount`) / **Credit** `Merchant Account Asset` (`amount`) | `merchant_id`, `token`, `amount`, `recipient` |
| `AutoWithdrawalFailedEvent` | `shade` | **Failed sweep attempt** | **No balance movement.** Log as an operational alert only; do not post a ledger entry. | `merchant_id`, `token`, `amount`, `recipient` |

> **Important:** `InvoiceCreatedEvent`, `InvoiceCancelledEvent`, `InvoiceAmendedEvent`, and `PendingWithdrawalCreatedEvent` do **not** move tokens on their own. Record them as sub-ledger state changes, not cash movements.

---

## 2. Reconciling `get_merchant_analytics`, Events, and `get_balances`

A healthy ledger must reconcile across three independent sources:

1. **Indexed Event Log** (historical inflows, fees, refunds, withdrawals).
2. **`shade.get_merchant_analytics(merchant, token)`** (and `get_merchant_analytics_summary(merchant)`).
3. **`account.get_balances()`** on the merchant's `account` contract.

### 2.1 Payment Volume & Fee Reconciliation (`shade` vs Events)

In [`contracts/shade/src/components/analytics.rs`](../../contracts/shade/src/components/analytics.rs), `record_merchant_payment` is invoked on every routed payment (`platform_fee.rs`). Note two critical invariants:

- **`total_volume` is cumulative gross volume:** Refunds do **not** decrement `MerchantAnalytics.total_volume`, `total_fees`, or `transaction_count`.
- **Per-token checks:** For each `(merchant, token)` pair up to ledger close `T`:

```text
SUM(InvoicePaidEvent.amount)          == get_merchant_analytics(merchant, token).total_volume
SUM(InvoicePaidEvent.fee)             == get_merchant_analytics(merchant, token).total_fees
COUNT(InvoicePaidEvent)               == get_merchant_analytics(merchant, token).transaction_count
```

Across all tokens for a merchant:

```text
SUM_tokens(MerchantAnalytics.total_volume) == get_merchant_analytics_summary(merchant).total_volume
SUM_tokens(MerchantAnalytics.total_fees)   == get_merchant_analytics_summary(merchant).total_fees
```

If any of these differ, your event indexer missed a ledger range, double-counted a re-indexed transaction, or queried an inconsistent block height.

### 2.2 Vault Balance Reconciliation (`account.get_balances` vs Events)

The merchant `account` contract holds net proceeds (`merchant_amount`) until withdrawn or refunded. For each tracked token returned by `account.get_balances()`:

```text
Expected Balance(token) =
    SUM(InvoicePaidEvent.merchant_amount where merchant_account == account)
  - SUM(account::RefundProcessedEvent.amount where token == token)
  - SUM(account::WithdrawalToEvent.amount where token == token)
  - SUM(executed account::PendingWithdrawalCreatedEvent.amount where token == token)
  - SUM(shade::AutoWithdrawalTriggeredEvent.amount where token == token)
```

Verify that:

```text
Expected Balance(token) == account.get_balances().find(b => b.token == token).balance
```

> **Note:** Check whether an `AutoWithdrawalTriggeredEvent` and an `account` withdrawal event overlap in the same transaction hash (`tx_hash`). Deduplicate outflows by `(tx_hash, event_index)` or ledger sequence so a single transfer is never subtracted twice.

---

## 3. Handling Fiat-Denominated Invoices

When an invoice uses `PricingMode::FixedFiat` (see [Fiat Pricing and Oracles](../concepts/fiat-pricing-and-oracles.md)):

- The invoice stores `FiatPricing { currency, amount, decimals }` (for example `USD`, `10000`, `2` for `$100.00`).
- At resolution/payment time, the contract queries the configured `OracleConfig` and emits `FiatInvoicePricedEvent { invoice_id, token, resolved_amount, timestamp }`.
- Settlement occurs in `token` base units (`resolved_amount`), which is what `InvoicePaidEvent.amount` records.

### Recording the Exchange Rate Used at Payment Time

To maintain accurate fiat-equivalent books without relying on retroactive oracle lookups:

1. **Capture `FiatInvoicePricedEvent`** alongside `InvoicePaidEvent` (or read `get_invoice(invoice_id)` at payment time for `fiat_pricing`).
2. **Compute and store the effective execution rate** at `timestamp`:
   ```text
   fiat_value        = fiat_pricing.amount / (10 ^ fiat_pricing.decimals)
   token_units       = resolved_amount / (10 ^ token_decimals)
   effective_rate    = fiat_value / token_units   // e.g., USD per 1 USDC or XLM
   ```
3. **Freeze the fiat value** on the ledger entry (`fiat_currency`, `fiat_amount`, `effective_rate`, `rate_timestamp`). Never re-price historical payments using today's spot rate; use today's rate only for end-of-period balance revaluation if required by your accounting standard.

---

## 4. CSV Export Schema

An indexer or reconciliation script should export one row per atomic accounting event, ordered deterministically by `(ledger_sequence, tx_hash, event_index)`.

### Column Specification

| Column | Type | Description |
| :--- | :--- | :--- |
| `ledger_sequence` | `u64` | Stellar ledger sequence number |
| `timestamp_utc` | `ISO-8601` | Ledger close timestamp in UTC (`2026-11-25T08:45:00Z`) |
| `tx_hash` | `string` | 64-char hex Stellar transaction hash |
| `event_index` | `u32` | Zero-based event index within the transaction |
| `entry_type` | `enum` | `PAYMENT_GROSS`, `PROTOCOL_FEE`, `PAYMENT_NET`, `REFUND`, `WITHDRAWAL` |
| `contract_source` | `enum` | `shade` or `account` |
| `merchant_id` | `u64` | Shade merchant ID (nullable for standalone `account` events) |
| `merchant_account` | `string` | Merchant `account` contract address (`C...`) |
| `invoice_id` | `u64` | Invoice ID (empty for withdrawals) |
| `counterparty` | `string` | Payer or withdrawal recipient Stellar address (`G...` / `C...`) |
| `token_address` | `string` | SAC / token contract address (`C...`) |
| `token_symbol` | `string` | Human symbol resolved off-chain (e.g., `USDC`, `XLM`) |
| `amount_raw` | `i128` | Signed base-unit delta (`+` inflow to merchant account, `-` outflow/fee) |
| `token_decimals` | `u8` | Token decimal places (typically `7` on Stellar) |
| `amount_decimal` | `string` | Signed human-readable decimal string |
| `fiat_currency` | `string` | Quoted fiat currency (`USD`, `EUR`, or empty if `FixedCrypto`) |
| `fiat_amount` | `string` | Fiat amount in quote currency (empty if `FixedCrypto`) |
| `effective_rate` | `string` | Fiat per 1 token unit at payment time (empty if `FixedCrypto`) |

### Sample CSV Rows

```csv
ledger_sequence,timestamp_utc,tx_hash,event_index,entry_type,contract_source,merchant_id,merchant_account,invoice_id,counterparty,token_address,token_symbol,amount_raw,token_decimals,amount_decimal,fiat_currency,fiat_amount,effective_rate
54120310,2026-11-25T08:45:00Z,a1b2c3d4...,0,PAYMENT_GROSS,shade,42,CABC...123,901,GXYZ...789,CCW6...USDC,USDC,1000000000,7,100.0000000,USD,100.00,1.000000
54120310,2026-11-25T08:45:00Z,a1b2c3d4...,0,PROTOCOL_FEE,shade,42,CABC...123,901,CPLAT...999,CCW6...USDC,USDC,-5000000,7,-0.5000000,USD,-0.50,1.000000
54120310,2026-11-25T08:45:00Z,a1b2c3d4...,0,PAYMENT_NET,shade,42,CABC...123,901,GXYZ...789,CCW6...USDC,USDC,995000000,7,99.5000000,USD,99.50,1.000000
54120890,2026-11-25T09:15:12Z,e5f6a7b8...,1,REFUND,shade,42,CABC...123,901,GXYZ...789,CCW6...USDC,USDC,-200000000,7,-20.0000000,USD,-20.00,1.000000
54121500,2026-11-25T10:00:05Z,99887766...,0,WITHDRAWAL,account,42,CABC...123,,GTREAS...456,CCW6...USDC,USDC,-795000000,7,-79.5000000,,,
```

---

## 5. Off-Chain Data Not Stored On-Chain

Shade keeps personally identifiable information (PII) and tax/business metadata off-chain by design. The following fields **are not on-chain** and must be joined from your merchant backend or ERP:

- **Customer identity & billing details:** Legal name, email, billing address, VAT/GST registration ID.
- **Tax jurisdiction & tax breakdown:** Sales tax / VAT rate collected, tax-exempt status, withholding amounts.
- **Merchant internal references:** Order ID, SKU list, internal invoice/PO number, cost center.
- **Off-chain settlement costs:** Bank wire fees, off-ramp spread, FX conversion fees when cashing out stablecoins to fiat.

### How to Join On-Chain and Off-Chain Records

1. Store Shade's `invoice_id` (returned by `create_invoice` / `create_fiat_invoice`) on your internal order record at checkout creation time.
2. Join `InvoicePaidEvent.invoice_id`, `InvoiceRefundedEvent.invoice_id`, and `FiatInvoicePricedEvent.invoice_id` directly to your internal `orders.shade_invoice_id`.
3. For withdrawals (`WithdrawalToEvent`, `AutoWithdrawalTriggeredEvent`), join via `tx_hash` and `recipient` address to your treasury/off-ramp transfer records.

---

## Related Documentation

- [Event Indexing Guide](indexing-events.md)
- [Fiat Pricing and Oracles](../concepts/fiat-pricing-and-oracles.md)
- [Merchant Account Withdrawals](../concepts/merchant-account-withdrawals.md)
- [Analytics and History](../concepts/analytics-and-history.md)
- [Events Reference](../reference/events.md)
