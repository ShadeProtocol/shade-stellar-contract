# Test Fixtures

This directory contains pre-compiled WASM binaries used by shade's test suite to simulate cross-contract interactions and upgrades.

## Files

- **`account.wasm`** — The real account contract, rebuilt by `make fixtures` after any change to `contracts/account/`. See [docs/getting-started/running-tests.md § Test fixtures](../../../../docs/getting-started/running-tests.md#test-fixtures) for details on when to regenerate it and what happens if it becomes stale.

- **`upgrade_v2_contract.wasm`** — A minimal stub WASM used to test shade's contract upgrade mechanism. It is frozen and not built from source. See [docs/getting-started/running-tests.md § Test fixtures](../../../../docs/getting-started/running-tests.md#test-fixtures) for details on what it's used for and when it needs updating.

## When to run `make fixtures`

After modifying any source code in `contracts/account/`, always run:

```bash
make fixtures
```

This ensures that the fixture stays in sync with the account contract's implementation. See [Running the test suite § Test fixtures](../../../../docs/getting-started/running-tests.md#test-fixtures) in the full documentation for symptoms of a stale fixture and troubleshooting steps.

## Related documentation

- [Running the test suite](../../../../docs/getting-started/running-tests.md) — full details on test organization, test snapshots, and test module conventions
