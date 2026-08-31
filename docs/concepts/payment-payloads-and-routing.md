# Payment payloads, swap routing, and the cross-chain bridge placeholder

`PaymentPayload`, `SwapRoute`, and `PaymentRoute` describe pay-in and settle-out tokens with routing paths and slippage tolerance, validated by `validate_payment_payload`. `CrossChainBridgePayload` and `emit_bridge_placeholder` mark the intended cross-chain integration point. This page documents both and is explicit about what is implemented today versus reserved for future work.

## Why it exists

Shade supports payments where the payer's token differs from the [merchant](../glossary.md#merchant)'s settlement token. The payload types encode the routing intent so the contract can validate path correctness and slippage bounds before settlement. The bridge placeholder defines the event shape for future cross-chain flows.

## How it works

### Payment routes

A `PaymentPayload` describes how a payment should be routed:

- **Direct payment.** The input token equals the settlement token. No swap is needed.
- **Swap payment.** The input token differs from the settlement token. A `SwapRoute` specifies the router contract and the hop path.

### Validation

`validate_payment_payload` enforces correctness for both route types. The function does not execute a swap — it only validates that the payload is well-formed.

**Direct route rules:**

1. `input_token` must equal `settlement_token`. If not → error `InvalidSwapPath`.
2. `max_slippage_bps` must be `None`. If set → error `InvalidSlippage`.

**Swap route rules:**

1. `route.path.len()` must be ≥ 2. If not → error `InvalidSwapPath`.
2. First hop must equal `input_token`. If not → error `InvalidSwapPath`.
3. Last hop must equal `settlement_token`. If not → error `InvalidSwapPath`.
4. `input_token` must not equal `settlement_token`. If equal → error `InvalidSwapPath`.
5. `max_slippage_bps` must be `Some`. If `None` → error `InvalidSlippage`.
6. `max_slippage_bps` value must be ≤ 10,000 (100%). If exceeded → error `InvalidSlippage`.

### Cross-chain bridge placeholder

`CrossChainBridgePayload` encodes a cross-chain transfer intent. `emit_bridge_placeholder` publishes a `BridgePlaceholderEvent` with the caller, payload, and timestamp. **No bridging or fund movement occurs on-chain.** The event is intended for off-chain indexers and future bridge contract consumers.

## Relevant types and storage

| Type / key | Defined in | Purpose |
|---|---|---|
| `PaymentPayload` | [`contracts/shade/src/types.rs#L676-L681`](../../../contracts/shade/src/types.rs#L676-L681) | Describes input token, settlement token, route, and slippage |
| `PaymentRoute` | [`contracts/shade/src/types.rs#L644-L647`](../../../contracts/shade/src/types.rs#L644-L647) | Enum: `Direct` or `Swap(SwapRoute)` |
| `SwapRoute` | [`contracts/shade/src/types.rs#L651-L654`](../../../contracts/shade/src/types.rs#L651-L654) | Router address and hop path |
| `CrossChainBridgePayload` | [`contracts/shade/src/types.rs#L713-L723`](../../../contracts/shade/src/types.rs#L713-L723) | Cross-chain transfer intent fields |
| `BridgePlaceholderEvent` | [`contracts/shade/src/events.rs#L729-L748`](../../../contracts/shade/src/events.rs#L729-L748) | Event emitted by `emit_bridge_placeholder` |

### PaymentPayload fields

| Field | Type | Description |
|---|---|---|
| `input_token` | `Address` | Token the payer is sending |
| `settlement_token` | `Address` | Token the [merchant](../glossary.md#merchant) receives |
| `route` | `PaymentRoute` | `Direct` or `Swap` routing |
| `max_slippage_bps` | `Option<u32>` | Maximum slippage in basis points (required for swaps, must be `None` for direct) |

### SwapRoute fields

| Field | Type | Description |
|---|---|---|
| `router` | `Address` | Address of the swap router contract |
| `path` | `Vec<Address>` | Ordered list of token addresses forming the swap hop path |

### CrossChainBridgePayload fields

| Field | Type | Description |
|---|---|---|
| `invoice_id` | `u64` | Associated invoice ID |
| `merchant` | `Address` | [Merchant](../glossary.md#merchant) address |
| `payer` | `Option<Address>` | Payer address (may be absent for third-party payments) |
| `source_chain` | `String` | Origin chain identifier |
| `destination_chain` | `String` | Target chain identifier |
| `token` | `Address` | Token to bridge |
| `amount` | `i128` | Amount to bridge |
| `destination_recipient` | `String` | Recipient on the destination chain |
| `memo` | `Option<String>` | Optional memo |

## Relevant functions

| Function | Defined in | Purpose |
|---|---|---|
| `validate_payment_payload` | [`contracts/shade/src/components/payment.rs#L7-L48`](../../../contracts/shade/src/components/payment.rs#L7-L48) | Validate payload correctness without executing swaps |
| `emit_bridge_placeholder` | [`contracts/shade/src/shade.rs#L511-L515`](../../../contracts/shade/src/shade.rs#L511-L515) | Publish bridge placeholder event |

## What is implemented today

- **Payload validation.** `validate_payment_payload` is fully implemented and enforced.
- **Direct payments.** Payer sends the settlement token directly; no swap occurs.
- **Bridge placeholder events.** `emit_bridge_placeholder` publishes events but does not move funds.

## What is reserved for future work

- **On-chain swap execution.** The `SwapRoute` path is validated but no swap is executed on-chain. An integrator must perform the swap off-chain or through a separate swap contract before calling the Shade payment function.
- **Cross-chain bridging.** `CrossChainBridgePayload` is a descriptive event shape. No bridging, locking, or fund movement occurs. An integrator must handle cross-chain transfers externally.

> **Warning:** Do not assume that a swap route in a `PaymentPayload` will execute a token swap on-chain. The contract only validates the payload. You must execute the swap off-chain and ensure the payer sends the correct settlement token.

## Forward-compatibility contract

Integrators can rely on:

- The `PaymentPayload` struct and `validate_payment_payload` behaviour will remain stable.
- The `BridgePlaceholderEvent` shape will be emitted when `emit_bridge_placeholder` is called.

What may change when real swap/bridge support lands:

- New validation rules may be added for swap execution.
- `CrossChainBridgePayload` may gain additional fields or move to a dedicated bridge contract.
- The `SwapRoute` type may be extended with execution parameters (deadline, minimum output).

> **Note:** Slippage is represented in basis points where 10,000 = 100%. A `max_slippage_bps` of 250 means 2.5% maximum slippage.

## Constraints and edge cases

- **Direct route slippage.** Must be `None`; providing a value is rejected with `InvalidSlippage`.
- **Swap route slippage.** Must be `Some` and ≤ 10,000. Values above 10,000 are rejected with `InvalidSlippage`.
- **Single-hop swap.** A path of length 1 is rejected with `InvalidSwapPath` (minimum 2 hops).
- **Bridge is event-only.** `emit_bridge_placeholder` requires caller auth and publishes an event; no funds move.

## Related pages

- [Event ticketing, dynamic pricing, and resale](./event-ticketing.md)
- [Auto-withdrawal and merchant settlement](./auto-withdrawal.md)
