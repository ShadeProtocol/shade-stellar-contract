# Test Fixtures

This directory contains committed WASM binaries used by shade's test suite to verify contract mechanics like account factory deployment and upgrade compatibility.

## Files

- **`account.wasm`** — The compiled merchant account contract, deployed by tests that verify account instantiation and permissions. Rebuild with `make fixtures` after any change to `contracts/account/src/`.

- **`upgrade_v2_contract.wasm`** — A reference Shade contract used to verify that upgrades preserve storage and state across WASM swaps. Treated as a fixed baseline; regenerate only when deliberately resetting the upgrade test.

## Usage

**To use these fixtures:** They are included via `include_bytes!` in the test modules that need them. No manual action required.

**To update `account.wasm`:** Run `make fixtures` after editing anything under `contracts/account/src/`. This rebuilds the account contract and places the new WASM here.

**To update `upgrade_v2_contract.wasm`:** Copy a freshly-built `shade.wasm` here when testing or validating upgrade behavior. See [`docs/getting-started/running-tests.md`](../../../../../docs/getting-started/running-tests.md#upgrade_v2_contractwasm-fixture) for context.

## For more information

See [`docs/getting-started/running-tests.md` § Test fixtures](../../../../../docs/getting-started/running-tests.md#test-fixtures) for a complete explanation of what each fixture is, why it matters, when to rebuild it, and how to recognize when a fixture is stale.

For upgrade-related testing and storage compatibility rules, see [`docs/architecture/upgradeability.md`](../../../../../docs/architecture/upgradeability.md).

For the stale-fixture detection strategy, see [Issue 364](https://github.com/ShadeProtocol/shade-stellar-contract/issues/364).
