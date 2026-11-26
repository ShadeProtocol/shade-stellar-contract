# Test Fixtures

This directory contains WASM binaries used by the shade contract's test suite.

For detailed documentation on when and how to use and regenerate these fixtures, see [Test fixtures](../../../../../docs/getting-started/running-tests.md#test-fixtures) in the running-tests guide.

## Files

- **`account.wasm`** — The deployed `account` contract. Rebuild with `make fixtures` after changes to `contracts/account/`.
- **`upgrade_v2_contract.wasm`** — A minimal contract used to test the shade contract's upgrade mechanism.
