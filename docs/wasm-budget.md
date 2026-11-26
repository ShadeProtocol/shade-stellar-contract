# WASM size budget

CI optimizes each contract (`stellar contract optimize`) and compares its size
against `.github/wasm-budget.json`. A PR fails if any contract is over budget.
Each PR also reports the size change versus the base branch.

## Raising a budget on purpose

1. Edit `.github/wasm-budget.json` in the same PR as the change that needs it.
2. Set the new limit a little (about 3-5%) above the new optimized size.
3. In the PR description, explain why the growth is needed and what you
   considered to avoid it.
4. A maintainer must approve the budget change explicitly.

Budgets should only go down or stay flat unless there is a good reason.
See ADR 0007 for the size target.
