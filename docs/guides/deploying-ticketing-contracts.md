# Deploying ticketing contracts

Use `ticketing_factory` to deploy a separate `ticketing` contract for an organizer, create events and tiers in that instance, and issue tickets. This guide covers factory administration, organizer calls, and how an indexer can discover each deployment.

## Prerequisites

- A deployed `ticketing_factory` contract, a network, and funded [admin](../glossary.md#admin) and organizer accounts.
- An optimized `ticketing` WASM built for the network's Soroban protocol version. See [Building the contracts](../getting-started/building.md) and [Prerequisites](../getting-started/prerequisites.md).
- The Stellar CLI (`stellar`) and, for the SDK example, generated TypeScript clients for the factory and ticketing WASMs. See [Generating and using contract bindings](contract-bindings.md).
- The factory contract ID as `$FACTORY_ID`; use the event contract ID returned by the factory as `$TICKETING_ID`.

The standalone `ticketing` contract is independent of Shade's hub ticketing feature. Its tier price is stored as metadata; `ticketing` does not collect ticket payments.

## Configure and deploy through the factory

### 1. Initialize the factory

Initialize once with the administrator address. The admin must authorize the call; a second initialization fails with `AlreadyInitialized`.

```bash
stellar contract invoke \
  --id $FACTORY_ID \
  --source admin \
  --network testnet \
  -- initialize \
  --admin $(stellar keys address admin)
```

### 2. Install the ticketing WASM and set its hash

Install the optimized `ticketing` WASM and save the hash printed by the CLI:

```bash
stellar contract install \
  --wasm <path-to-ticketing-optimized.wasm> \
  --source admin \
  --network testnet
```

`<path-to-ticketing-optimized.wasm>` is a placeholder for your built artifact. Set the resulting 32-byte WASM hash on the factory:

```bash
stellar contract invoke \
  --id $FACTORY_ID \
  --source admin \
  --network testnet \
  -- set_ticketing_wasm_hash \
  --admin $(stellar keys address admin) \
  --wasm_hash <ticketing-wasm-hash>
```

Only the stored admin can change this hash. Updating it affects **future deployments only**: existing ticketing contracts are not upgraded automatically, and the factory has no function to upgrade them.

### 3. Have the organizer deploy a contract

The organizer authorizes its own deployment. The factory deploys the currently configured WASM, assigns the next sequential `ref_id`, records an `EventRef`, publishes `EventContractDeployedEvent`, and returns the `EventRef`.

```bash
stellar contract invoke \
  --id $FACTORY_ID \
  --source organizer \
  --network testnet \
  -- deploy_event_contract \
  --organizer $(stellar keys address organizer)
```

Save the returned `contract` address as `$TICKETING_ID`. The address is derived from on-chain randomness, so do not try to predict it before deployment. The factory does not restrict an organizer to one deployment; each call creates another contract and registry entry.

## Organizer flow: create an event and issue a ticket

Use the address returned by the factory for every call below. Each ticketing instance starts with its own event and tier IDs. The examples assume the first event and first tier in this new instance; use the IDs returned by your actual calls.

### 1. Create an event

The organizer must authorize the call. Times are Unix timestamps in seconds, and `max_capacity` is optional.

```bash
stellar contract invoke \
  --id $TICKETING_ID \
  --source organizer \
  --network testnet \
  -- create_event \
  --organizer $(stellar keys address organizer) \
  --name "Summer Showcase" \
  --description "Annual community event" \
  --start_time <start-unix-seconds> \
  --end_time <end-unix-seconds> \
  --max_capacity 100
```

Record the returned `event_id`.

### 2. Add a tier

`price` is an `i128` amount in the token's base units, but it is informational: this contract does not transfer tokens.

```bash
stellar contract invoke \
  --id $TICKETING_ID \
  --source organizer \
  --network testnet \
  -- add_tier \
  --organizer $(stellar keys address organizer) \
  --event_id 1 \
  --name "General admission" \
  --price 5000000 \
  --max_supply 100
```

Record the returned `tier_id`. The tier's supply cannot exceed the event's remaining overall capacity when the event has a capacity limit.

### 3. Issue a tiered ticket

Generate a cryptographically random, unique 32-byte QR hash off-chain for each ticket. Pass its 64-character hexadecimal representation as `qr_hash`.

```bash
stellar contract invoke \
  --id $TICKETING_ID \
  --source organizer \
  --network testnet \
  -- issue_tiered_ticket \
  --organizer $(stellar keys address organizer) \
  --event_id 1 \
  --holder $(stellar keys address attendee) \
  --qr_hash <unique-32-byte-hex-hash> \
  --tier_id 1
```

The returned value is the `ticket_id`. `issue_tiered_ticket` checks that the organizer owns the event, the tier belongs to that event, neither event nor tier is at capacity, and the QR hash has not already been used.

## Verify it worked

Read the factory record and event back from their respective contracts:

```bash
stellar contract invoke --id $FACTORY_ID --network testnet -- get_event_ref --ref_id 1
stellar contract invoke --id $TICKETING_ID --network testnet -- get_event --event_id 1
```

The first result should contain the organizer, deployed contract address, and deployment timestamp. The second should contain the event details and `Active` state. Use the IDs returned by your calls if they are not `1`.

## Troubleshooting

| Problem | Cause | Fix |
|---|---|---|
| `NotInitialized` | An admin-only factory call ran before initialization. | Call `initialize` with the intended admin. |
| `AlreadyInitialized` | `initialize` was called more than once. | Do not initialize the deployed factory again. |
| `WasmHashNotSet` | An organizer deployed before the admin configured a ticketing WASM hash. | Install the intended WASM and call `set_ticketing_wasm_hash`. |
| `NotAuthorized` | The signer is not the stored factory admin or the organizer address supplied to the ticketing call. | Use the matching admin or organizer identity and authorize the transaction. |
| `EventRefNotFound` | The requested factory reference ID does not exist. | Check `get_event_ref_count` and use a recorded `ref_id`. |

## TypeScript SDK example

Generate clients from the factory and ticketing WASMs as described in [Generating and using contract bindings](contract-bindings.md). This example assumes the factory is initialized and its WASM hash is configured. Provide `signTransaction` through your wallet or secure signer; do not put a secret key in application source.

```typescript
import { randomBytes } from "node:crypto";
import { Client as FactoryClient } from "./bindings/ticketing_factory/src/index.js";
import { Client as TicketingClient } from "./bindings/ticketing/src/index.js";

declare const signTransaction: (xdr: string) => Promise<string>;
declare const rpcUrl: string;
declare const networkPassphrase: string;
declare const organizer: string;
declare const attendee: string;

const factory = new FactoryClient({
  contractId: process.env.FACTORY_ID!,
  rpcUrl,
  networkPassphrase,
});

const deployTx = await factory.deploy_event_contract({ organizer });
const { result: eventRef } = await deployTx.signAndSend({ signTransaction });

const ticketing = new TicketingClient({
  contractId: eventRef.contract,
  rpcUrl,
  networkPassphrase,
});

const eventTx = await ticketing.create_event({
  organizer,
  name: "Summer Showcase",
  description: "Annual community event",
  start_time: 1_800_000_000n,
  end_time: 1_800_003_600n,
  max_capacity: 100n,
});
const { result: eventId } = await eventTx.signAndSend({ signTransaction });

const tierTx = await ticketing.add_tier({
  organizer,
  event_id: eventId,
  name: "General admission",
  price: 5_000_000n,
  max_supply: 100n,
});
const { result: tierId } = await tierTx.signAndSend({ signTransaction });

const ticketTx = await ticketing.issue_tiered_ticket({
  organizer,
  event_id: eventId,
  holder: attendee,
  qr_hash: randomBytes(32),
  tier_id: tierId,
});
const { result: ticketId } = await ticketTx.signAndSend({ signTransaction });

console.log({ contract: eventRef.contract, eventId, tierId, ticketId });
```

The timestamps and prices above are examples; set times appropriate to your event and prices in the intended base units. `randomBytes(32)` creates the unique QR hash. Generated-client write calls simulate first and then sign and submit through `signAndSend`.

## Factory interface and registry

The factory implementation is in `example/ticketing_factory/src/lib.rs`; the organizer-facing ticketing methods are in `example/ticketing/src/lib.rs`. The factory's public functions are:

| Function | Behavior |
|---|---|
| `initialize(env: Env, admin: Address)` | Sets the admin once; the admin must authorize the call. |
| `set_ticketing_wasm_hash(env: Env, admin: Address, wasm_hash: BytesN<32>)` | Admin-only. Sets the WASM hash used by later deployments. |
| `deploy_event_contract(env: Env, organizer: Address) -> EventRef` | Requires organizer authorization and a configured WASM hash; deploys and records a contract. |
| `get_event_ref(env: Env, ref_id: u64) -> EventRef` | Returns one record; an unknown ID fails with `EventRefNotFound`. |
| `get_event_ref_count(env: Env) -> u64` | Returns the total number of recorded deployments, or zero before the first deployment. |
| `get_all_event_refs(env: Env) -> Vec<EventRef>` | Returns records in insertion order. |

Each `EventRef` contains:

| Field | Meaning |
|---|---|
| `ref_id: u64` | Sequential, one-based factory reference ID. |
| `contract: Address` | Deployed ticketing contract address. |
| `organizer: Address` | Address that authorized the deployment. |
| `deployed_at: u64` | Ledger timestamp at deployment. |

`EventContractDeployedEvent` is published by the factory for each deployment with fields `ref_id`, `contract`, `organizer`, and `timestamp`. The event's `timestamp` is the ledger timestamp; the corresponding `EventRef` names that field `deployed_at`.

## Discovering deployments as an indexer

Filter contract events by the factory contract address and the `event_contract_deployed_event` topic. Decode the event fields and persist at least `ref_id`, `contract`, `organizer`, `timestamp`, and the emitting ledger/transaction position. Use the event stream to discover deployments and maintain the organizer-to-contract mapping; event IDs are created later inside each ticketing instance and are not part of this factory event.

For an initial backfill or recovery, read `get_event_ref_count` and fetch IDs from `1` through the returned count with `get_event_ref`, or call `get_all_event_refs` for a small registry. Process event records idempotently by factory address and `ref_id`, and checkpoint the last fully processed ledger so you can resume event consumption without missing deployments.

`get_all_event_refs` scans every ID from `1` to the current count and returns the entire vector. Its work and response grow linearly with the registry; there is no pagination. As the registry grows, a full read can become expensive in execution resources and response size, and eventually exceed network limits. Prefer event indexing for ongoing discovery; for backfills, use the count plus individual lookups and avoid repeatedly fetching the whole registry.

## Related pages

- [Event indexing guide](indexing-events.md)
- [Generating and using contract bindings](contract-bindings.md)
- [Ticketing contract reference](../../example/docs/contracts/ticketing.md)
