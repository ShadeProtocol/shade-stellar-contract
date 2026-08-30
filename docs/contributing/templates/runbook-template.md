<!--
Runbook template — use for operational procedures a protocol operator
follows under time pressure or during an incident (e.g. "Pausing the
Contract," "Rotating the Admin Key," "Responding to a Failed Upgrade Vote").
Delete this comment block before publishing the page.
See docs/contributing/documentation-style-guide.md for tone/format rules.
-->

# Runbook: <operational scenario>

One sentence: when to use this runbook and what outcome it produces.

> **Warning:** State any irreversible consequence up front, before the reader starts executing steps (e.g. "Pausing halts all payment functions immediately for every merchant").

## When to use this

Concrete triggers for reaching for this runbook (an alert, a reported symptom, a scheduled maintenance window). Be specific enough that someone under pressure can confirm "yes, this is the right runbook" in a few seconds.

## Preconditions

- Access/authorization required (e.g. "must hold the admin key" or "must be a governance council member").
- State the system must be in before starting.

## Procedure

### 1. <Diagnose / confirm the situation>

```bash
stellar contract invoke --id <contract-id> -- is_paused
```

### 2. <Take the action>

```bash
stellar contract invoke --id <contract-id> -- pause --admin <admin-address>
```

### 3. <Confirm the action took effect>

```bash
stellar contract invoke --id <contract-id> -- is_paused
# true
```

## Rollback

How to reverse this action if it was taken in error, or state clearly if it cannot be reversed (e.g. "unpause with the `unpause` call below" or "this is irreversible; the contract must be redeployed").

```bash
stellar contract invoke --id <contract-id> -- unpause --admin <admin-address>
```

## Escalation

Who to notify and through what channel if the procedure doesn't resolve the situation.

## Related pages

- [Concept page explaining the underlying mechanism](#)
- [Contract reference for the functions used here](#)
