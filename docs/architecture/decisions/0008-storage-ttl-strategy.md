# 0008. Storage TTL Strategy for Instance and Persistent Entries

- **Status:** Accepted
- **Date:** 2026-11-24

## Context

On Soroban, both contract instance storage and persistent storage entries are subject to state archival under the network's rent and Time-To-Live (TTL) mechanics. Entries that are not renewed before their TTL reaches zero are archived by validators and become inaccessible to contract execution until an explicit restoration transaction is submitted and paid for.

Previously, as documented in [`docs/architecture/storage-model.md`](../storage-model.md), no contract in the Shade workspace invoked `extend_ttl` or related host functions. All core state—including contract instance metadata (`Admin`, `Paused`, `AcceptedTokens`), merchant registries (`Merchant`, `MerchantId`, `MerchantAccount`), and invoices (`Invoice`)—relied exclusively on default write-time TTLs without in-contract upkeep.

If a contract instance or persistent entry expires, mission-critical operations (such as processing invoice payments, managing merchant payouts, or executing administrative controls) fail unexpectedly. Offloading all TTL maintenance to external operator scripts is operationally fragile and introduces single points of failure. Conversely, extending storage unconditionally across every read query would impose state-modification fees on read-only consumers and break standard read-only RPC semantics.

A clear architectural decision is required to balance automated on-chain survival, predictable transaction fees, and long-term storage sustainability.

## Decision

We adopt an automated in-contract TTL extension strategy that protects contract vitality along active execution paths while keeping read operations zero-cost:

1. **Contract Instance TTL Extension on State Changes**:
   - Both `shade` and per-merchant `account` contracts automatically extend their instance storage TTL on every state-changing (write) invocation (e.g., via a shared helper in `core` triggered on execution).
   - **Threshold:** `276,480` ledgers (~16 days at 5-second ledger closes).
   - **Extend-to:** `518,400` ledgers (~30 days at 5-second ledger closes), bounded safely below the network maximum TTL.

2. **Selective Persistent Entry Extension on Hot Paths**:
   - Persistent entries touched during active payment and lifecycle workflows are extended when accessed on hot paths:
     - Core merchant records: `DataKey::Merchant(id)`, `DataKey::MerchantId(address)`, and `DataKey::MerchantAccount(id)`.
     - Active invoice records: `DataKey::Invoice(id)` during invoice creation, payment, voiding, or refunding.
     - Configuration entries: `DataKey::TokenFee(token)`, `DataKey::MerchantPlatformFee(id, token)`, and `DataKey::MerchantTokens(address)`.
   - The same threshold (`276,480` ledgers) and extend-to (`518,400` ledgers) values apply to persistent entry extensions.

3. **Read-Only Paths Do Not Extend TTLs**:
   - Read-only queries (such as `get_invoice`, `get_merchant`, `get_fee`, and list views) do not invoke `extend_ttl`. View calls remain strictly non-mutating and incur no state-write gas or signature requirements.

## Consequences

- **Implementation status**: This strategy is accepted but not yet implemented. No production contract currently calls `extend_ttl`; until implementation is complete, active state is not automatically protected and operators must follow the [State Archival and TTL Extension Runbook](../../operations/state-archival.md).
- **Intended availability after implementation**: Active merchants, recurring payment pathways, and contract administration should remain live through state-changing calls, subject to successful transactions and network limits.
- **Expected rent overhead after implementation**: Write calls will incur additional resource and rent fees for TTL extensions; actual costs depend on entry size, extension horizon, and network parameters.
- **Natural Archival of Inactive Historical Records**: Completed, cancelled, or refunded invoices that are never accessed again will eventually reach their TTL and be archived by the network, saving persistent state rent across the ecosystem.
- **Operator Responsibilities**: Operators must still maintain an off-chain monitoring runbook to restore archived historical invoices if a client requests retro-active audit data, and ensure operational funded accounts exist for administrative bumps if contracts experience prolonged dormancy.

## Alternatives considered

- **Operator-only extension via external keeper bots**:
  - *Why rejected:* Relying entirely on external keepers creates a severe availability hazard. If the keeper experiences downtime, network partition, or wallet drainage, the entire payment gateway could lock up due to instance archival.
- **Extending on every access, including read-only queries**:
  - *Why rejected:* Turning read-only calls into state-mutating transactions requires callers to sign transactions and pay transaction fees just to inspect an invoice or verify a merchant, violating idiomatic Web3 integration patterns and increasing RPC costs.
- **Unbounded archival without contract-level extension**:
  - *Why rejected:* Forcing users or merchants to manually encounter and resolve entry archival errors during routine checkout or settlement introduces unacceptable user friction and operational overhead.
