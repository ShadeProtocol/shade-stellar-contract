<!--
How-to template — use for task-oriented pages that walk through
accomplishing one specific goal (e.g. "How to Register a Merchant,"
"How to Deploy a New Contract to Testnet").
Delete this comment block before publishing the page.
See docs/contributing/documentation-style-guide.md for tone/format rules.
-->

# How to <accomplish a specific task>

One sentence: what you'll accomplish by following this guide.

## Prerequisites

- Tooling, accounts, or prior setup needed before starting (e.g. "a funded Stellar testnet account," "the contract already deployed").
- Link to setup docs rather than repeating setup instructions here.

## Steps

### 1. <First step>

Explain the step, then show the command or code:

```bash
stellar contract invoke --id <contract-id> -- register_merchant --merchant <address>
```

### 2. <Next step>

Continue numbering steps sequentially. Each step should have exactly one clear action.

## Verify it worked

Show a command or check that confirms success, and what the expected output looks like:

```bash
stellar contract invoke --id <contract-id> -- is_merchant --merchant <address>
# true
```

## Troubleshooting

| Problem | Cause | Fix |
|---|---|---|
| Example error message | Why it happens | What to do |

## Related pages

- [Concept page explaining the underlying mechanism](#)
- [Contract reference for the functions used here](#)
