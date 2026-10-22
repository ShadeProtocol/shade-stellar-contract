# Resource costs and Soroban limits

Soroban transactions are bounded by per-transaction limits on CPU instructions, memory, ledger entry reads and writes, written bytes, and event size. Batch operations grow linearly with batch size and hit those limits first. This page publishes measured costs and the largest safe batch size, and explains how to regenerate and check them.

> **Status:** `process_billing_cycle` is measured on current `main`. `register_merchant`, `create_invoice`, `pay_invoice`, `pay_invoices_batch` and `get_invoices` are measured on a **snapshot** of the `shade` contract (commit `4c97ca0`, the last commit before the crate stopped compiling) because the `shade` crate does not build on current `main`: KYC and campaign-royalty code was merged without its `DataKey`, `ContractError` and event definitions, and several vesting structs and module declarations are duplicated. `purchase_tickets_bulk` is not measured. See [Measurement caveats](#measurement-caveats) and [Unmeasured operations](#unmeasured-operations).

## Limits used

Per-transaction limits assumed below (Stellar network settings, Protocol 23). They are network configuration, not contract constants, and validators can vote to change them. Confirm the current values with `stellar network settings` or the network's `getNetworkConfig` before relying on them.

| Resource | Limit |
|---|---|
| CPU instructions | 100,000,000 |
| Memory | 41,943,040 bytes |
| Ledger entries read | 100 |
| Ledger entries written | 50 |
| Bytes written | 132,096 |
| Contract event bytes | 16,384 |

A batch is "safe" only if it is within **every** limit.

## How the numbers were measured

Measurements come from the Soroban test environment: `env.cost_estimate().resources()` after a single top-level invocation, with the budget reset to unlimited so that over-limit batches can still be measured. The test is [`contracts/subscription/src/test_resource_costs.rs`](../../contracts/subscription/src/test_resource_costs.rs).

> **Warning:** The test environment runs the contract as native Rust, not compiled WASM, and uses the built-in token contract. Instruction and memory counts for a deployed WASM contract are higher, so treat these numbers as a lower bound for CPU and memory. Entry counts, written bytes and event bytes do not depend on native vs WASM execution and are directly comparable. Always confirm a chosen batch size with a testnet simulation.

## `process_billing_cycle`

Every subscription in the batch is charged successfully (the first charge of a new subscription, funded and approved). Plan interval is 30 days, amount 1,000.

| Batch size | Instructions | Memory (bytes) | Read entries | Write entries | Bytes written | Event bytes | Within limits |
|---|---|---|---|---|---|---|---|
| 1 | 390,786 | 56,971 | 8 | 4 | 1,160 | 472 | yes |
| 5 | 2,168,477 | 332,299 | 20 | 16 | 4,904 | 2,360 | yes |
| 10 | 4,738,881 | 815,284 | 35 | 31 | 9,584 | 4,720 | yes |
| 16 | 8,342,406 | 1,598,476 | 53 | 49 | 15,200 | 7,552 | yes |
| 17 | 8,997,681 | 1,750,603 | 56 | 52 | 16,136 | 8,024 | **no** (writes) |
| 20 | 11,066,591 | 2,244,004 | 65 | 61 | 18,944 | 9,440 | **no** (writes) |
| 40 | 27,181,378 | 6,952,444 | 125 | 121 | 37,664 | 18,880 | **no** (reads, writes, events) |
| 80 | 73,958,742 | 23,773,324 | 245 | 241 | 75,104 | 37,760 | **no** (reads, writes, events) |

Costs per additional charged subscription are roughly 3 write entries, 3 read entries, 470 event bytes and about 0.5M instructions in this configuration. Write entries grow as `3n + 1`, so they are the first limit reached.

**Maximum safe batch size: 16 subscriptions per call** in this configuration. The next limits reached are read entries (about 31) and event bytes (about 34), then instructions on native execution (about 100 subscriptions). Because deployed WASM costs more CPU than the native figure, a margin below 16 is prudent when subscriptions use more expensive tokens, plans or paths (for example a pending downgrade, which loads a second plan). Size batches for the all-charged case.

## Shade hub operations

Measured with [`contracts/shade/tests/resource_costs.rs`](../../contracts/shade/tests/resource_costs.rs) on snapshot `4c97ca0` (see [Measurement caveats](#measurement-caveats)). Each row is one top-level call on fresh state. The token is a built-in Stellar asset with a 5% fee, invoices are 1,000 units, and the merchant has a payout account.

### Single operations

| Operation | Instructions | Memory (bytes) | Read entries | Write entries | Bytes written | Event bytes |
|---|---|---|---|---|---|---|
| `register_merchant` | 93,560 | 14,575 | 7 | 4 | 644 | 220 |
| `create_invoice` | 218,055 | 33,323 | 13 | 3 | 672 | 276 |
| `pay_invoice` | 564,523 | 90,086 | 15 | 6 | 1,480 | 948 |

Each fits in one transaction with a wide margin. `pay_invoice` is the most expensive: the token transfer, the fee split and the paid event account for most of it.

### `pay_invoices_batch`

Every invoice in the batch is paid in full by the same payer.

| Batch size | Instructions | Memory (bytes) | Read entries | Write entries | Bytes written | Event bytes | Within limits |
|---|---|---|---|---|---|---|---|
| 1 | 564,812 | 90,086 | 15 | 6 | 1,480 | 948 | yes |
| 5 | 2,758,433 | 397,290 | 19 | 10 | 3,656 | 4,740 | yes |
| 10 | 5,570,899 | 831,380 | 24 | 15 | 6,376 | 9,480 | yes |
| 15 | 8,546,998 | 1,321,120 | 29 | 20 | 9,096 | 14,220 | yes |
| 16 | 9,086,135 | 1,425,746 | 30 | 21 | 9,640 | 15,168 | yes |
| 17 | 9,773,306 | 1,532,598 | 31 | 22 | 10,184 | 16,116 | yes |
| 18 | 10,395,883 | 1,641,676 | 32 | 23 | 10,728 | 17,064 | **no** (events) |
| 20 | 11,651,708 | 1,866,510 | 34 | 25 | 11,816 | 18,960 | **no** (events) |
| 30 | 18,423,058 | 3,124,240 | 44 | 35 | 17,256 | 28,440 | **no** (events) |
| 50 | 32,928,005 | 6,307,500 | 64 | 55 | 28,136 | 47,400 | **no** (events, writes) |

Each additional invoice adds about 1 read entry, 1 write entry, 544 bytes written, 948 event bytes and 0.58M instructions. Event bytes are the first limit reached (948 per payment against the 16,384 cap), then write entries (about 45 invoices), then read entries (about 86).

**Maximum safe batch size: 17 invoices per call** with a built-in token and a single merchant. Payments to different merchants write more entries per invoice, and a token contract with heavier transfers costs more CPU, so size production batches below 17 and confirm with a simulation.

### `get_invoices`

The query walks every stored invoice from 1 to `InvoiceCount` regardless of the filter, so cost depends on the total number of invoices on the contract, not on the number returned. The rows below use an empty filter.

| Stored invoices | Instructions | Memory (bytes) | Read entries | Write entries | Within limits |
|---|---|---|---|---|---|
| 1 | 54,985 | 6,781 | 3 | 0 | yes |
| 10 | 357,752 | 37,480 | 12 | 0 | yes |
| 50 | 1,955,763 | 206,260 | 52 | 0 | yes |
| 75 | 3,128,173 | 338,560 | 77 | 0 | yes |
| 98 | 4,298,007 | 478,492 | 100 | 0 | yes |
| 99 | 4,348,657 | 484,972 | 101 | 0 | **no** (reads) |
| 150 | 7,286,963 | 859,210 | 152 | 0 | **no** (reads) |

Read entries equal the invoice count plus 2, so the 100-entry read limit is reached at **98 stored invoices**. After that `get_invoices` cannot succeed on-chain, even though CPU and memory stay far below their limits. A deployment with more than about 98 invoices needs an off-chain index (for example event-based) for listing and filtering.

## Measurement caveats

- **Snapshot, not `main`.** The Shade rows come from commit `4c97ca0` with `scripts/resource-costs/snapshot-batch-auth.patch` applied. At that commit `pay_invoices_batch` called `require_auth` for the payer once per invoice on top of the batch-level call, which the host rejects, so any batch failed. Current `main` fixed this by splitting out `pay_invoice_inner`; the patch applies the same change to the snapshot. `main` also resolves each invoice amount through `resolve_invoice_amount` before paying, which can add a few read entries per payment. Re-measure on `main` as soon as the crate builds again, and treat the batch size above as an upper bound until then.
- **Native execution.** The test environment runs contracts as native Rust, not compiled WASM, so instruction and memory counts are a lower bound for a deployed contract. Entry counts, bytes written and event bytes do not depend on that and are directly comparable.
- **Built-in token.** The tests use the built-in Stellar asset contract. Other token contracts cost more.

## Unmeasured operations

| Operation | Contract | Batch? | Status |
|---|---|---|---|
| `purchase_tickets_bulk` | `shade` | Quantity, not a loop | Not measured: it does not exist at snapshot `4c97ca0` and the crate does not build on `main` |

`purchase_tickets_bulk` performs one token transfer and one event update regardless of `quantity`, so its cost should be roughly flat in `quantity` and there is no batch size to cap. This is an expectation from reading the code, not a measurement. To measure it, add a test to `contracts/shade/tests/resource_costs.rs` once the `shade` crate builds on `main`, following the existing pattern (build state, `env.cost_estimate().budget().reset_unlimited()`, invoke once, read `env.cost_estimate().resources()`).

## Regenerating the numbers

Subscription batch:

```bash
cargo test -p subscription resource_costs -- --nocapture
```

Shade hub operations:

```bash
scripts/resource-costs/measure-shade.sh
```

The script runs `contracts/shade/tests/resource_costs.rs` in place when the `shade` crate builds. When it does not, it measures snapshot `4c97ca0` in a temporary git worktree and removes it afterwards. Set `CARGO_TARGET_DIR` to an existing build directory to avoid compiling every dependency again.

Both print the tables above in Markdown and fail only if a single operation does not fit in one transaction. Copy the printed rows into this page, and update the limit constants at the top of each test file when the network settings change.

## Estimating fees

Fees depend on the resources above and on the network's current fee settings, so they cannot be derived from the test environment. To estimate a real transaction:

1. Build the transaction (for example with the Stellar CLI or an SDK).
2. Simulate it (`stellar contract invoke ... --sim-only`, or the RPC `simulateTransaction` method). The response contains the exact resource footprint and the minimum resource fee.
3. Compare the simulated resources to the limits above. If simulation fails with a budget or limit error, reduce the batch size.
4. Add an inclusion fee on top of the resource fee and a safety margin for ledger-state drift between simulation and submission.

No fee range is published here because it has not been measured on testnet, and resource figures from the test environment cannot be converted to a fee without the current network fee settings.

## Most expensive operations

By CPU, `pay_invoice` is the heaviest single call (about 0.56M instructions, versus 0.22M for `create_invoice` and 0.09M for `register_merchant`). Batches are limited by the output they produce rather than by CPU: `pay_invoices_batch` runs out of event bytes at 18 invoices, `process_billing_cycle` out of write entries at 17 subscriptions, and `get_invoices` out of read entries at 99 stored invoices. The `get_invoices` full scan is the one operation that gets worse as the platform grows, with no batch size to tune.

Open optimisation work: [Write Resource Optimization Guidelines for Contract Contributors (#531)](https://github.com/ShadeProtocol/shade-stellar-contract/issues/531).

## Related pages

- [Subscriptions and recurring billing](../concepts/subscriptions.md)
- [Payment flows: full, partial, and batch](../concepts/payments.md)
