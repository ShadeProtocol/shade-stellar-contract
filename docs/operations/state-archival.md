# Runbook: State archival and TTL extension

Use this runbook to keep Shade contract state available and to restore archived entries before retrying a failed contract call.

> **Warning:** Extending or restoring state incurs network fees and rent. Temporary entries are permanently deleted at TTL expiry and cannot be restored. Do not rely on restoration as a substitute for monitoring.

## When to use this

- An alert shows a contract-data, contract-instance, or contract-code TTL approaching expiry.
- A simulated Soroban invocation reports archived entries and cannot proceed without restoration.
- You are preparing a deployment, upgrade, or recovery and need to confirm that the relevant contract state will remain live.

## Preconditions

- Use the Stellar CLI configured for the correct network and RPC endpoint. Confirm the network before signing; this procedure can submit paid transactions.
- Have a funded operator identity authorized to sign the transactions.
- Record the contract ID, contract WASM hash, the full ledger-key XDR for RPC monitoring, the `ScVal` key XDR for CLI data-entry operations, and the last closed ledger sequence.
- Check the target network's live TTL and rent parameters in [Stellar Lab network limits](https://lab.stellar.org/network-limits). Network values can change through protocol upgrades.

## Storage inventory and criticality

Soroban has three contract-data durability types. TTL is measured in ledgers; an entry is live through its `liveUntilLedgerSeq`. Once the current ledger sequence exceeds that value, the entry is no longer live.

| Type | TTL and expiry behavior | Shade production usage |
|---|---|---|
| Instance | All instance data shares the contract-instance TTL. Expiry archives the instance and its instance data; restore it before invoking the contract. | No production `instance()` storage calls. The contract instance still has its own lifecycle. |
| Persistent | Each entry has its own TTL. Expiry archives the entry; it cannot be read or recreated until restored. | All production contract-data entries in `shade` and `account` use persistent storage. |
| Temporary | Each entry has its own TTL. Expiry permanently deletes it; there is no restore procedure. | No production `temporary()` storage calls. |

Contract WASM is a separate `ContractCode` ledger entry with its own TTL. It is not a `DataKey` value. Instance and code lifetimes must both be monitored for each deployment; [merchant](../glossary.md#merchant) account instances also rely on their deployed `account` WASM.

The following inventory is based on `contracts/shade/src/types.rs`, `contracts/account/src/types.rs`, and the production storage calls in both crates. The [admin](../glossary.md#admin), merchant, and [invoice](../glossary.md#invoice) state is critical to normal payment processing. Criticality describes operational impact if the state becomes unavailable, not a protocol-enforced priority.

| Contract and entries | Storage | Criticality | Impact if unavailable |
|---|---|---|---|
| `shade`: `Admin`, `PendingAdmin`, `Role`, `Paused`, `ReentrancyStatus`, `ContractInfo` | Persistent | Critical | Blocks authorization, administration, or safe operation of payment paths. |
| `shade`: `AcceptedTokens`, `AccountWasmHash`, `PlatformAccount`, `TokenFee`, `PendingTokenFee`, `MerchantPlatformFee`, `TokenOracle` | Persistent | Critical | Changes or blocks payment acceptance, fee routing, oracle-priced invoices, or merchant-account deployment. |
| `shade`: `Merchant`, `MerchantId`, `MerchantCount`, `MerchantKey`, `MerchantTokens`, `MerchantAccount` | Persistent | Critical | Blocks merchant lookup, signed invoice verification, token checks, and payment/refund routing. |
| `shade`: `Invoice`, `InvoiceCount`, `UsedNonce` | Persistent | Critical | Blocks invoice lookup and lifecycle operations; nonce state protects signed-invoice replay checks. |
| `shade`: `UserTransactions`, `MerchantVolume`, `MerchantAnalytics`, `MerchantAnalyticsSummary`, `TokenAnalytics`, `TokenVolume` | Persistent | Important | Breaks or degrades historical queries and reporting, but does not itself route funds. |
| Each deployed `account`: `Manager`, `Merchant`, `AccountInfo`, `Verified`, `Restricted`, `TrackedTokens`, `Threshold`, `WithdrawalRequest`, `WithdrawalCount`, `WithdrawalAnalytics` | Persistent | Critical | May block account authorization, restrictions, refunds, withdrawals, and access to account state. |
| `shade` and `account` contract instances | Instance ledger entries | Critical | Contract calls are unavailable until the instance is restored. |
| `shade` and `account` WASM | Contract-code ledger entries | Critical | The corresponding contract code cannot be invoked or used to deploy new instances until available again. |

The contract-specific key inventory is also documented in the [storage model](../architecture/storage-model.md). There are no production instance- or temporary-storage calls, and no in-contract `extend_ttl`, `bump_ttl`, or equivalent TTL renewal calls.

## Network TTL parameters

Mainnet values below are the latest values stated in Stellar's state-archival documentation when this runbook was written. Recheck the live network settings before each operation; the network can change configuration.

| Parameter | Mainnet value | Operational meaning |
|---|---:|---|
| `minPersistentTTL` | 2,073,600 ledgers (about 120 days), as of 2026-11-02 | Minimum TTL assigned when a persistent entry is created or restored. Restoration does not automatically give the entry a longer TTL. |
| `minTemporaryTTL` | Check Stellar Lab | Minimum TTL for temporary entries; Shade currently has none. |
| `maxEntryTTL` | Check Stellar Lab | Maximum TTL an entry can be extended to from the current ledger. The Stellar CLI reads the active network maximum and rejects an over-limit extension. |

The Stellar Lab [resource-limits table](https://lab.stellar.org/network-limits) is authoritative for the target network's current values. Do not copy a testnet value into a mainnet operation.

## Monitoring

Monitor each critical `ContractData` entry, the contract-instance entry, and each required `ContractCode` entry through a Soroban RPC or an indexer that exposes `liveUntilLedgerSeq`. For a known ledger-key XDR, `getLedgerEntries` returns the entry's live-until ledger. Compare it with the latest ledger sequence and alert with enough lead time to extend before expiry.

```bash
curl "$SOROBAN_RPC_URL" \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","id":1,"method":"getLedgerEntries","params":{"keys":["<BASE64_LEDGER_KEY_XDR>"]}}'
```

`<BASE64_LEDGER_KEY_XDR>` is the base64-encoded XDR for the full ledger key, not the human-readable contract key or just the key value. Shade's `DataKey` values are contract-specific, so keep an inventory of their ledger-key XDRs in the monitoring system. Track current ledger sequence separately and alert when the remaining TTL reaches your operational threshold.

## Procedure

### 1. Extend live entries

Run `stellar contract extend` for each entry that needs a longer TTL. `--ledgers-to-extend` is a duration from the current ledger, not an absolute ledger number. The CLI simulates and submits the extension; review the target network and estimated resource cost before signing.

Extend the `shade` contract instance:

```bash
stellar contract extend \
  --contract-id "$SHADE_ID" \
  --ledgers-to-extend "$LEDGERS" \
  --network mainnet \
  --source-account ttl-operator \
  --cost
```

Extend a specific persistent contract-data entry. Use the base64-encoded XDR for the key value (`ScVal`), not the full ledger key; the CLI's `--key` option only supports symbol keys, while Shade keys use `DataKey` variants.

```bash
stellar contract extend \
  --contract-id "$SHADE_ID" \
  --key-xdr "$BASE64_DATA_KEY_XDR" \
  --durability persistent \
  --ledgers-to-extend "$LEDGERS" \
  --network mainnet \
  --source-account ttl-operator \
  --cost
```

Extend a contract-code entry by its installed WASM hash:

```bash
stellar contract extend \
  --wasm-hash "$SHADE_WASM_HASH" \
  --ledgers-to-extend "$LEDGERS" \
  --network mainnet \
  --source-account ttl-operator \
  --cost
```

Repeat for the `account` contract instance and its WASM hash as needed. For an instance that is close to expiry, prioritize extending it before its code or data entries.

### 2. Restore archived entries

Starting with Protocol 23, a normal invocation submitted through RPC simulation can automatically restore archived entries identified by the simulation. Retry through the usual client or CLI invocation path and review the simulation's restore requirements and fees. A manually constructed transaction must include the correct restore list.

Use `stellar contract restore` when you need to restore an entry separately, for example to control which transaction pays the restoration cost or when automatic restoration cannot fit in the invocation transaction. Restore a persistent Shade data entry with its key value's base64-encoded XDR (`ScVal`):

```bash
stellar contract restore \
  --contract-id "$SHADE_ID" \
  --key-xdr "$BASE64_DATA_KEY_XDR" \
  --durability persistent \
  --network mainnet \
  --source-account ttl-operator \
  --cost
```

For an archived contract instance, omit `--key-xdr` and target the contract ID:

```bash
stellar contract restore \
  --contract-id "$SHADE_ID" \
  --network mainnet \
  --source-account ttl-operator \
  --cost
```

For an archived contract-code entry, target its installed WASM hash:

```bash
stellar contract restore \
  --wasm-hash "$SHADE_WASM_HASH" \
  --network mainnet \
  --source-account ttl-operator \
  --cost
```

Restoration gives the entry the network's minimum persistent TTL. Extend it to the required operational horizon before retrying the original call. Temporary entries cannot be restored; recreate them only through the application's normal flow if doing so is safe. If restoration fails or the required ledger key cannot be identified, stop and escalate rather than recreating critical state.

### 3. Verify and retry

Query the same ledger key again and confirm that `liveUntilLedgerSeq` is in the future and leaves the required operational margin. Confirm the instance and WASM entries separately. Then retry the original invocation through simulation and verify its expected result.

## Cost estimates

There is no reliable fixed XLM price for a TTL operation: rent depends on the entry's serialized size, the number of ledgers requested, current network rent parameters, and the transaction's resource and inclusion fees. Restoring multiple or large entries costs more than restoring one small entry, and the restored minimum TTL may still need a separate extension. Use `--cost` and transaction simulation against the target network immediately before submission; treat the result as an estimate, set an operator fee budget, and never override the simulated resource fee without an explicit review.

## Rollback

TTL extensions and restorations do not change Shade's application data, but their rent and transaction fees cannot be recovered. An extension cannot be undone. If the wrong entry was targeted, stop further operations, verify the affected ledger keys, and escalate.

## Escalation

If a critical entry is already archived, the expected entry is missing, or restoration exceeds the approved fee budget, pause automated retries and contact the contract operations maintainers. Preserve the transaction hash, contract ID, network, ledger key, simulation result, and RPC error for investigation.

## Follow-up and related pages

- In-contract TTL renewal is tracked in [#567: Extend Storage TTLs for the Contract Instance and Persistent Entries](https://github.com/ShadeProtocol/shade-stellar-contract/issues/567). This runbook is an interim operational mitigation, not a replacement for that work.
- [Storage model](../architecture/storage-model.md)
- [Deployment runbook](deployment.md)
- [Contract upgrade runbook](upgrade-runbook.md)
- [Stellar state archival documentation](https://developers.stellar.org/docs/learn/fundamentals/contract-development/storage/state-archival)
