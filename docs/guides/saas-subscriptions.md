# End-to-end SaaS subscription integration guide

Use the standalone `subscription` contract to create recurring plans, enroll customers, process charges, and synchronize subscription state with your SaaS access controls. This guide follows the full [merchant](../glossary.md#merchant) and customer journey using the generated TypeScript bindings.

> **Note:** The standalone contract in `example/subscription/` is separate from the Shade hub's `charge_subscription` feature. They have different interfaces and lifecycle behavior. This guide covers the standalone contract.

## Prerequisites

- Deploy and initialize the standalone `subscription` contract. Its source is in [`example/subscription/src/lib.rs`](../../example/subscription/src/lib.rs); this example contract is separate from the production `contracts/` workspace.
- Generate TypeScript bindings for the deployed contract. See [Generating and using contract bindings](contract-bindings.md).
- Have the contract ID, network passphrase, RPC URL, a merchant address, and at least one supported SEP-41 token.
- Configure the contract's [admin](../glossary.md#admin) and merchant signers. Each write below must be signed by the address the contract requires.

The snippets use `client` as the generated contract `Client` and `signTransaction` as the appropriate wallet or server-side signer. Keep signing keys in a wallet or secret manager, not in application source.

## Create a plan

The admin first accepts each billing token. A plan's token must already be an [accepted token](../glossary.md#accepted-token). The merchant then creates the plan with its amount in token base units, interval and optional trial period. Set a grace period explicitly: new plans default to zero, which means a failed charge immediately terminates the subscription.

```typescript
const token = "C..."; // accepted SEP-41 token contract

const acceptTokenTx = await client.add_accepted_token({ token });
await acceptTokenTx.signAndSend({ signTransaction: adminSigner });

const createPlanTx = await client.create_plan({
  merchant: merchantAddress,
  description: "Pro monthly",
  token,
  amount: 5_0000000n, // 5 tokens at 7 decimals, in base units
  interval: 2_592_000n, // 30 days, in seconds
  creator: undefined,
  trial_period: 0n,
});
const { result: planId } = await createPlanTx.signAndSend({
  signTransaction: merchantSigner,
});

const graceTx = await client.set_plan_grace_period({
  merchant: merchantAddress,
  plan_id: planId,
  grace_period: 604_800n, // 7 days, in seconds
});
await graceTx.signAndSend({ signTransaction: merchantSigner });
```

`amount` is an `i128`, so use a JavaScript `bigint` and confirm the token's decimals before choosing it. The merchant must authorize both plan calls. The grace period applies to every subscription on the plan; when a due charge lacks sufficient allowance, the subscription enters `PastDue` for that many seconds before it can be terminated.

## Enroll and authorize a customer

Enrollment and payment authorization are separate contract calls in one customer signup journey. `subscribe` creates an `Active` record and returns its ID, but it does not transfer tokens or grant an allowance. The customer must then authorize billing before the keeper can charge.

```typescript
const subscribeTx = await client.subscribe({
  customer: customerAddress,
  plan_id: planId,
});
const { result: subscriptionId } = await subscribeTx.signAndSend({
  signTransaction: customerSigner,
});

const authorizeTx = await client.authorize_billing({
  customer: customerAddress,
  sub_id: subscriptionId,
  cycles: 12,
});
await authorizeTx.signAndSend({ signTransaction: customerSigner });

const { result: allowance } = await client.get_billing_allowance({
  customer: customerAddress,
  sub_id: subscriptionId,
});
```

Check `allowance` before confirming setup to the customer. `authorize_billing` approves the current plan amount multiplied by `cycles`; a later price increase does not increase this allowance. Reauthorizing replaces the existing allowance. The expiry is based on ledger sequence and cycle count, not the plan interval, so size the authorization with your billing schedule in mind. Confirm the selected plan, first-charge timing, and recurring approval amount before the customer signs.

## Process recurring charges

The contract does not run a scheduler. Run an off-chain keeper that finds due subscriptions and calls `process_billing_cycle` regularly. This forgiving batch entry point returns one `ChargeOutcome` for each ID, in the same order. Prefer it to strict `charge` for scheduled billing: `charge` panics on insufficient allowance and does not drive the grace-period state machine.

```typescript
const chargeTx = await client.process_billing_cycle({
  sub_ids: dueSubscriptionIds,
});
const { result: outcomes } = await chargeTx.signAndSend({
  signTransaction: keeperSigner,
});

for (let i = 0; i < outcomes.length; i++) {
  const subscriptionId = dueSubscriptionIds[i];
  const outcome = outcomes[i];
  // Persist the outcome and update dunning/access state as described below.
}
```

The keeper pays transaction fees; it does not need customer authorization. Reconcile through `get_subscription` as needed, and keep `PastDue` IDs in later sweeps so a restored allowance can recover the subscription or the grace period can expire. See the [Billing Keeper Operator Guide](../../example/docs/operations/billing-keeper.md) for scheduling, batch sizing, retries, and monitoring.

## Map billing state to SaaS access

Enforce access in your product backend; the contract does not know about user accounts, roles, or SaaS entitlements. Persist the on-chain `subscriptionId` against the product user and use the subscription status and keeper outcomes to drive a policy such as:

| Contract state or outcome | Suggested product action |
|---|---|
| `Active` | Grant or maintain the plan's access, subject to your signup/trial policy. `subscribe` creates an `Active` record before the first charge, so status alone does not prove a payment has succeeded. |
| `EnteredGrace` / `PastDue` | Start dunning and apply your configured grace access (for example, retain or restrict access until the plan's grace deadline). |
| `Charged` | Record the successful cycle and maintain access. |
| `Recovered` | Clear dunning and restore normal access after a successful recovery charge. |
| `Terminated` or status `Terminated` | Revoke access and stop scheduling charges. This state is terminal. |
| Status `Cancelled` | Stop future billing and apply your published cancellation/access policy. This state is terminal. |
| `NotDueYet` | No change; schedule the next check based on the plan interval or trial. |
| `Skipped` | Re-read the subscription status and remove the terminal subscription from future keeper batches. |

Use `ChargeOutcome` from the keeper transaction for immediate billing reactions and `get_subscription` for the current status. `EnteredGrace` has no corresponding event. `SubExpired` is emitted for cancellation as well as termination, so do not treat that event alone as proof of which occurred; read the subscription record. The contract also does not emit a subscription-created event, so save the ID returned by `subscribe`.

## Change plan terms

The merchant can update a plan's amount or interval. These values are read live when calculating and processing existing subscribers' next charges; the contract does not snapshot them at signup.

```typescript
const amountTx = await client.update_plan_amount({
  merchant: merchantAddress,
  plan_id: planId,
  new_amount: 7_0000000n,
});
await amountTx.signAndSend({ signTransaction: merchantSigner });

const intervalTx = await client.update_plan_interval({
  merchant: merchantAddress,
  plan_id: planId,
  new_interval: 2_592_000n,
});
await intervalTx.signAndSend({ signTransaction: merchantSigner });
```

An amount change applies to the next charge, not retroactively to a completed charge. Existing allowances are unchanged: after a price increase, a customer's allowance may cover fewer cycles or be too small for the next charge. Notify affected customers of the new price and effective cycle, and prompt them to review the terms and reauthorize before billing; `authorize_billing` replaces, rather than adds to, the allowance.

The interval is used with `last_charged` to compute the next due time. Changing it can therefore move the next due date for existing subscribers. For a subscription that has never been charged, the first-charge check instead uses `created_at + trial_period`. Plan updates do not emit a dedicated event; track your own change or poll `get_plan`, and tell customers about the new billing date before the changed terms take effect.

For individual customer moves between plans, the contract also provides `upgrade_subscription` and `downgrade_subscription`. Upgrades take effect immediately; downgrades are applied on the next charge. Both require an active subscription and a target plan using the same token. See the [subscription contract reference](../../example/docs/contracts/subscription.md#lifecycle).

## Cancel a subscription

`cancel_subscription` stops future charges immediately and accepts either an `Active` or `PastDue` subscription. The customer or the plan's merchant may call it. A customer-initiated cancellation also sets that customer's token allowance to zero; a merchant-initiated cancellation cannot revoke the customer's token allowance, but the cancelled status still prevents further charges.

```typescript
const cancelTx = await client.cancel_subscription({
  caller: customerAddress,
  sub_id: subscriptionId,
});
await cancelTx.signAndSend({ signTransaction: customerSigner });

const { result: subscription } = await client.get_subscription({
  sub_id: subscriptionId,
});
```

Tell the customer when the cancellation is submitted and confirmed, that no future cycles will be charged, and what happens to access. Cancellation does not refund an already-paid cycle; state whether access ends immediately or remains available through a paid-through date maintained by your product.

### Cancel with a prorated refund

When your terms promise a refund for unused time in the current cycle, first preview it with `quote_prorated_refund`. The refund is rounded down in token base units and is zero if the subscription was never charged or the cycle has ended.

```typescript
const { result: refundQuote } = await client.quote_prorated_refund({
  sub_id: subscriptionId,
});
if (refundQuote > 0n) {
  // Present the estimate before asking both parties to authorize cancellation.
}
```

Then call `cancel_with_prorated_refund`. Both the caller and the other party must authorize the same transaction. The refund is pulled from the plan's creator, if set, or otherwise its merchant; that source must have approved enough tokens for the refund. The quote is only an estimate because the final amount is computed at execution time.

Use a signing flow that can collect the required authorization from both the customer and the refund source. In the generated-client pattern below, `multiAuthSignTransaction` is your app's callback for that transaction; the generated client does not provide the multi-party signing flow.

```typescript
const refundTx = await client.cancel_with_prorated_refund({
  caller: customerAddress,
  sub_id: subscriptionId,
});
await refundTx.signAndSend({
  signTransaction: multiAuthSignTransaction,
});
```

In the customer flow, collect both signatures before submission and clearly display the quoted amount, token, and that it may change slightly before execution. If the quote is zero, use `cancel_subscription` instead; the refund call fails when there is nothing to refund. Unlike customer-initiated `cancel_subscription`, the refund path does not zero the customer's allowance, so tell the customer how to revoke any remaining approval if they want it removed.

## Verify the integration

Before enabling billing for a live product, verify that:

- The plan reads back with the expected token, amount, interval, and grace period.
- Signup returns a subscription ID and the customer's `get_billing_allowance` is sufficient for the intended charges.
- A keeper charge returns the expected `ChargeOutcome` and updates `get_subscription`.
- Your backend grants, limits, and revokes access for active, past-due, cancelled, and terminated states according to your published policy.

## Related pages

- [Billing Keeper Operator Guide](../../example/docs/operations/billing-keeper.md) — run the off-chain charging process.
- [Subscription contract reference](../../example/docs/contracts/subscription.md) — full interface, states, and outcomes.
- [Subscription billing authorization and allowances](../../example/docs/concepts/subscription-billing-authorization.md) — allowance details and customer authorization flow.
- [Subscription grace periods and past-due handling](../../example/docs/concepts/subscription-grace-periods.md) — state transitions and event behavior.
- [Subscription prorated cancellation refunds](../../example/docs/concepts/subscription-prorated-refunds.md) — refund calculation and authorization requirements.
- [Generating and using contract bindings](contract-bindings.md) — generate and use the TypeScript client.

← [Back to guides](README.md)
