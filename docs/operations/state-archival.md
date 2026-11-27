# Runbook: State Archival and TTL Extension

Use this runbook to monitor and maintain the availability of Shade contract state, and to restore archived entries before retrying a failed operation.

> **Warning:** An expired persistent entry or contract instance is archived and unavailable to contract calls until restored. Temporary entries expire permanently. Shade currently has no production `extend_ttl` calls, so operators must monitor and maintain deployed contracts manually.

## When to use this

- As part of routine maintenance for deployed `shade` and merchant-account contracts.
- When a transaction or contract query reports an archived or expired entry.
- Before relying on old invoices, merchant records, or contract code after a period of inactivity.

## Preconditions

- A Stellar CLI version that supports `stellar contract extend` and `stellar contract restore`.
- The contract ID, network, and funded source account for each deployment.
- For individual data entries, the key's XDR encoding and durability. The contract's Rust `DataKey` variant alone is not enough; encode the complete key (including any variant fields) as XDR.
- For contract code, the deployed WASM hash.

Use a trusted operator account with enough XLM for restoration, TTL extension, and transaction fees. Confirm the selected network before signing any operation.

## TTLs and archival

Soroban stores contract data in three durability tiers. TTL is measured in ledgers; at an approximately five-second ledger close time, 4,096 ledgers is about 5.7 hours, 16 ledgers is about 80 seconds, and 6,312,000 ledgers is about one year.

| State | TTL and archival behavior | Shade use |
|---|---|---|
| Contract instance | Has its own TTL. When archived, the contract instance and its instance storage cannot be used until restored. The protocol minimum persistent TTL is 4,096 ledgers. | Every deployed Shade and merchant-account contract depends on an active instance, even though production code does not explicitly use `env.storage().instance()`. |
| Persistent contract data | Long-lived entries have TTLs and accrue rent. Expired entries are archived, not readable by contract execution until restored. The protocol minimum TTL for a newly live persistent entry is 4,096 ledgers. | All production contract data listed below uses persistent storage. |
| Temporary contract data | Short-lived entries expire and are deleted; they are not restorable after expiry. The protocol minimum TTL is 16 ledgers. | No temporary storage is used by Shade production code. |
| Contract WASM | Contract code is stored separately from contract instance/data and has its own TTL. Archived code must be restored by its WASM hash before contracts using it can execute. | The `shade` and account WASM code entries are required by their deployed instances. |

The protocol's commonly documented maximum TTL is 6,312,000 ledgers. Network settings can differ: for example, Stellar Core's testnet settings have configured a 3,110,400-ledger maximum. These limits can change with network upgrades; check the target network's current state-archival configuration before choosing an extension horizon. An extension cannot exceed that network's maximum. Ledger counts are approximate time conversions, not time guarantees.

### Shade storage inventory

The production `shade` contract's `DataKey` values are all written to **persistent** storage. Criticality describes operational impact if the entry is unavailable; entries may be critical to only particular contract functions.

| Entries | Criticality | Impact if unavailable |
|---|---|---|
| `Admin`, `PendingAdmin`, `Role`, `Paused`, `AcceptedTokens`, `AccountWasmHash`, `PlatformAccount`, `TokenFee`, `PendingTokenFee`, `MerchantPlatformFee`, `TokenOracle` | Critical configuration | Authorization, pause controls, accepted-token checks, account deployment, payment routing, fee calculation, and oracle-based pricing can fail or use documented defaults. |
| `Merchant`, `MerchantId`, `MerchantKey`, `MerchantAccount`, `MerchantTokens`, `MerchantCount` | Critical merchant state | Merchant lookup, signature verification, payment routing, account access, token checks, and merchant enumeration can fail. |
| `Invoice`, `InvoiceCount` | Critical payment state | Invoice lookup, payment, void, refund, and invoice enumeration can fail. |
| `ContractInfo` | Initialization metadata | Initialization metadata may be unavailable; it is not the admin authorization source. |
| `UsedNonce` | Security-critical replay protection | A missing nonce record can affect replay protection for signed invoice creation. |
| `MerchantVolume`, `UserTransactions`, `MerchantAnalytics`, `MerchantAnalyticsSummary`, `TokenAnalytics`, `TokenVolume` | Operationally important history/analytics | Analytics and transaction-history queries may fail or return incomplete results. |
| Contract instance | Critical | Contract calls are unavailable while the instance is archived. |
| Shade contract WASM | Critical | The instance cannot execute while the required code entry is archived. |

### Merchant-account storage inventory

The per-merchant `account` contract also writes its `DataKey` entries to **persistent** storage.

| Entries | Criticality | Impact if unavailable |
|---|---|---|
| `Merchant`, `Manager`, `AccountInfo` | Critical identity and authority | Merchant identity, manager authorization, or account initialization checks can fail. |
| `Verified`, `Restricted` | Critical controls | Verification state and account restrictions may be unavailable. |
| `TrackedTokens`, `Threshold` | Critical to configured operations | Token handling and withdrawal-trigger configuration may be unavailable. |
| `WithdrawalRequest`, `WithdrawalCount` | Critical funds/workflow state | Withdrawal authorization and request tracking can fail. |
| `WithdrawalAnalytics` | Operational history | Withdrawal analytics may be unavailable. |
| Contract instance and account WASM | Critical | The account cannot execute while its instance or required code is archived. |

No production Shade or account code currently writes instance or temporary contract data directly. Soroban still creates and maintains each contract instance and code entry outside those explicit contract storage calls.

## Monitoring

1. Maintain an inventory of each deployed `shade` contract ID, its merchant-account IDs, the WASM hash, network, and the operations account.
2. Use Stellar RPC `getLedgerEntries` to inspect each live ledger entry and its `liveUntilLedgerSeq`. Compare that value with the current ledger sequence and alert well before expiry. Track instance, persistent data, and code entries separately.
3. Keep the relevant ledger keys (including the contract instance and code keys) in your monitoring inventory.
4. Schedule extensions early enough to allow for transaction delays and failed submissions.

The CLI's `stellar contract extend --ttl-ledger-only` option still performs an extension; it only changes the command's output to the resulting TTL ledger. Do not use it as a read-only monitoring command. For large deployments, automate RPC monitoring and alerting rather than querying entries manually.

## Extending live entries

Extend a live contract instance before its TTL becomes too low:

```bash
stellar contract extend \
  --contract-id <CONTRACT_ID> \
  --ledgers-to-extend <LEDGERS> \
  --source-account <SOURCE_ACCOUNT> \
  --network <NETWORK>
```

Extend one persistent entry by supplying its XDR key and durability. Repeat for each critical key; extending the instance does not extend every persistent entry:

```bash
stellar contract extend \
  --contract-id <CONTRACT_ID> \
  --key-xdr <BASE64_XDR_KEY> \
  --durability persistent \
  --ledgers-to-extend <LEDGERS> \
  --source-account <SOURCE_ACCOUNT> \
  --network <NETWORK>
```

Extend contract code by its deployed WASM hash:

```bash
stellar contract extend \
  --wasm-hash <WASM_HASH> \
  --ledgers-to-extend <LEDGERS> \
  --source-account <SOURCE_ACCOUNT> \
  --network <NETWORK>
```

The extension horizon is a number of ledgers from the current ledger, not a calendar date. For example, 518,400 ledgers is about 30 days at five seconds per ledger. Choose a value below the target network's maximum and re-check the live TTL after the transaction confirms.

By default, the CLI simulates, signs, and submits the extension transaction using the configured source identity. The returned transaction result includes the new TTL ledger. The `--build-only` option only prints a transaction envelope; it does not submit it. Confirm the CLI output and the resulting TTL before considering an extension complete.

### Cost estimates

There is no fixed XLM cost per entry: rent depends on the entry's encoded size, the number of ledgers added, network rent parameters, and transaction/resource fees. Larger values and longer extensions cost more. The CLI simulates the intended operation before submission; review the reported fee and compare it with the source account's available balance. Do not treat a testnet estimate as a mainnet quote. Record actual fees from confirmed transactions to inform future budgets.

## Restoring archived state

Restore the contract instance if the instance itself is archived:

```bash
stellar contract restore \
  --contract-id <CONTRACT_ID> \
  --source-account <SOURCE_ACCOUNT> \
  --network <NETWORK>
```

Restore an archived persistent data entry by its XDR key:

```bash
stellar contract restore \
  --contract-id <CONTRACT_ID> \
  --key-xdr <BASE64_XDR_KEY> \
  --durability persistent \
  --source-account <SOURCE_ACCOUNT> \
  --network <NETWORK>
```

Restore archived contract code by its WASM hash:

```bash
stellar contract restore \
  --wasm-hash <WASM_HASH> \
  --source-account <SOURCE_ACCOUNT> \
  --network <NETWORK>
```

Restoration only makes the selected entry live; it does not extend its TTL to a chosen horizon. After restoring, extend critical entries as needed, verify their TTLs, and retry the original operation. A failed operation may depend on multiple archived entries; restore each entry identified by the error or transaction footprint. Temporary entries that have expired cannot be restored.

## Current risk and follow-up

No production contract in this workspace currently calls `extend_ttl` or an equivalent TTL-bump function. Contract instances, persistent state, and contract code therefore rely on network-assigned TTLs and operator monitoring/restoration. Infrequently accessed admin/configuration, merchant, invoice, and account data can be archived; routine calls that need them may then fail until an operator restores the entries.

ADR 0008 records the accepted in-contract strategy, including instance renewal on state-changing paths and selective renewal of active persistent entries. That decision is **not yet implemented** in the production contracts. Track the implementation as a follow-up: [Implement in-contract TTL extension on active Shade paths](https://github.com/ShadeProtocol/shade-stellar-contract/issues/new?title=Implement%20in-contract%20TTL%20extension%20on%20active%20Shade%20paths). Replace this issue-creation link with the tracking issue once it is opened.

## Related pages

- [Storage Model](../architecture/storage-model.md)
- [ADR 0008: Storage TTL Strategy](../architecture/decisions/0008-storage-ttl-strategy.md)
- [Address & Hash Registry](address-registry.md)
- [Operations index](README.md)
