# Analytics test suite

The analytics tests prove that a successful [invoice](../glossary.md#invoice) payment updates the on-chain analytics views correctly. This page inventories every test in [`test_analytics_aggregation.rs`](../../contracts/shade/src/tests/test_analytics_aggregation.rs), lists related analytics tests elsewhere in the workspace, and gives the commands to run them.

For the behavior being tested, see [Merchant analytics and transaction history](../concepts/analytics-and-history.md). For the token-level metrics and their limits, see [Token analytics and market share metrics](../concepts/token-analytics.md).

## Run the analytics tests

Run the full workspace suite the same way CI does:

```bash
cargo test --workspace --all-features
```

Run only the analytics aggregation module:

```bash
cargo test -p shade tests::test_analytics_aggregation
```

Run a single test by its full path:

```bash
cargo test -p shade tests::test_analytics_aggregation::test_volume_increments_single_merchant_single_token -- --exact
```

Add `-- --nocapture` to any command to see the SDK's snapshot-file notices. Test snapshots are generated under `contracts/shade/test_snapshots/` and are ignored by git; see [Running the test suite](running-tests.md#test-snapshots).

## Fixture setup

Every test in the module starts from the same [`setup`](../../contracts/shade/src/tests/test_analytics_aggregation.rs) helper:

1. `env.mock_all_auths()` and a non-zero ledger timestamp of `1_700_000_000`.
2. A registered and initialized `Shade` contract.
3. Two Stellar asset contracts, with fees set to 10% for token1 and 5% for token2.
4. One registered and verified [merchant](../glossary.md#merchant).
5. A real `MerchantAccount` contract linked to that merchant, so refund flows can move funds.

`setup_multi_merchant` adds a second verified merchant. `fund_payer` mints test tokens to the [payer](../glossary.md#payer) before each payment.

## Test inventory

The module contains 16 tests. Each table row names the test and the assertion it makes.

### Volume and isolation

| Test | What it proves |
|---|---|
| `test_volume_increments_single_merchant_single_token` | An empty analytics record starts at zero. Two payments of 1000 and 2000 accumulate to 3000 volume, 300 fees, and a count of 2, with a positive, non-decreasing `last_updated`. |
| `test_volume_increments_single_merchant_multiple_tokens` | One merchant paying 1000 in token1 and 2000 in token2 keeps separate per-token records (100 and 100 fees), while the summary aggregates to 3000 volume, 200 fees, and 2 payments. |
| `test_volume_increments_multiple_merchants` | Payments of 1500 and 2500 to two merchants stay isolated: each merchant has its own volume, fees, count, and address. The test does not assert a global token total. |
| `test_token_analytics_aggregation` | Token1 totals 3000 volume, 300 fees, and 2 payments across two merchants; token2 totals 1500, 75, and 1. The tokens are tracked separately. It does not assert `unique_merchants`. |

### Time and timestamps

| Test | What it proves |
|---|---|
| `test_time_bucketing_accuracy_daily` | After advancing the ledger by one day, `last_updated` equals each payment's timestamp and the two timestamps are exactly one day apart. Volume and count remain cumulative. |
| `test_time_bucketing_accuracy_weekly` | After advancing by one week, the `last_updated` delta is exactly one week and volume and count remain cumulative (8000 and 2). |
| `test_analytics_timestamp_accuracy` | Three explicit ledger timestamps are written to the merchant, token, and summary analytics views alike. |
| `test_analytics_cross_time_boundaries` | Payments at the start of a day, the end of that day, and the start of the next day aggregate to 4500 volume, 450 fees, and 3 payments. `last_updated` is the most recent timestamp. |

> **Note:** The "time bucketing" tests assert timestamp accuracy and cumulative totals. They do not verify stored daily or weekly buckets; the contract only stores lifetime aggregates.

### Consistency, discounts, and integrity

| Test | What it proves |
|---|---|
| `test_analytics_consistency_across_payment_types` | Despite its name, this test uses five different payment amounts (500–2500), not five different payment entry points. After each payment, cumulative volume, fees, and count match the running expectation. |
| `test_analytics_with_volume_discounts` | A 15000 payment records the base 10% fee (1500). The next 1000 payment crosses the tier-1 threshold and records a discounted 90 fee, for totals of 16000, 1590, and 2 payments. |
| `test_analytics_merchant_volume_discount_integration` | Four payments (5000, 6000, 40000, 10000) cross discount tiers. `calculate_fee` stays within sane bounds, and analytics volume and count stay exact. The test deliberately does not compare quoted fees with recorded fees because a discount uses the volume recorded *before* the payment. |
| `test_analytics_data_integrity` | With two tokens and one merchant, the merchant summary equals the sum of the per-token records, and each global token record matches the merchant's per-token record. |
| `test_analytics_token_market_dominance` | A 10000-to-1000 volume split between token1 and token2 produces the expected counts (2 and 1) and fees (1000 at 10%, 50 at 5%). The test compares raw counters; it does not call `get_token_market_share`. |

### Edge cases

| Test | What it proves |
|---|---|
| `test_zero_amount_payments_analytics` | A zero-amount `create_invoice` call traps with `Error(Contract, #7)` (`InvalidAmount`). No analytics update occurs because the invoice is never created. |
| `test_analytics_aggregation_performance` | Fifty payments of 100 produce 5000 volume, 500 fees, and a count of 50 in both the merchant and token views. This checks repeated-write accuracy, not wall-clock performance. |
| `test_analytics_data_consistency_after_refunds` | After a 2000 payment and a 1000 partial refund, merchant and token analytics still read 2000, 200, and 1. Refunds never decrement the recorded totals. |

## Related analytics tests

Other modules cover adjacent analytics behavior:

| File | Test | Scope |
|---|---|---|
| [`test_fee_discounts.rs`](../../contracts/shade/src/tests/test_fee_discounts.rs) | `test_fee_discounts_based_on_volume` | Fee tiers from 10% down to 7.5% as volume crosses 10k and 50k, asserted through the merchant account balance. |
| [`test_fee_discounts.rs`](../../contracts/shade/src/tests/test_fee_discounts.rs) | `test_volume_tracking_tier_3` | The 50% discount tier once recorded volume reaches 200k. |
| [`test_platform_fee_routing.rs`](../../contracts/shade/src/tests/test_platform_fee_routing.rs) | `test_merchant_analytics_records_platform_fees` | A 5% platform fee is recorded as `total_fees` in merchant analytics. |
| [`test_withdrawal.rs`](../../contracts/account/src/tests/test_withdrawal.rs) | `test_withdraw_to_tracks_analytics` | The account contract tracks withdrawal amount, count, and timestamp per token. |

## What the tests do not prove

- No stored daily or weekly buckets exist. The time tests only check `last_updated` and lifetime totals.
- `unique_merchants` is not asserted as a distinct-merchant count. See the warning in [Token analytics and market share metrics](../concepts/token-analytics.md#tokenanalytics-fields).
- The performance test is not a benchmark. It checks that 50 repeated writes still aggregate correctly.
- Refunds are intentionally excluded from analytics. Compute net figures from refund events instead.
- Subscription analytics are not exercised. `TransactionType::SubscriptionCharge` is defined, but current payment flows record `InvoicePayment`.

## Related pages

- [Running the test suite](running-tests.md) — suite layout, filters, flags, and snapshot behavior.
- [Merchant analytics and transaction history](../concepts/analytics-and-history.md) — what the analytics fields mean.
- [Fees, volume discounts, and time-locked fee changes](../concepts/fees.md) — how fee tiers are configured.
