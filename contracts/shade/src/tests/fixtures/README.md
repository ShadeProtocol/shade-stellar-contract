# Test Fixtures

This directory contains committed WASM binaries used by shade's contract tests.

## Overview

Both fixtures below are referenced by `include_bytes!()` macros in test files and must remain committed to the repository. They represent **production code** (account) or **tested upgrade behavior** (the mock), not test-only utilities.

## Fixtures

### `account.wasm`

The compiled `account` contract. Used by shade's factory tests to verify the hub can deploy and manage account contracts.

**Consumers:**
- `contracts/shade/src/tests/test_account_factory.rs`
- `contracts/shade/src/tests/test_reentrancy.rs` (indirectly, via shared setup)

**Rebuild when:** `contracts/account/src/` changes.

**How:** `make fixtures` — see [`docs/getting-started/running-tests.md#test-fixtures`](../../../docs/getting-started/running-tests.md#test-fixtures) for details.

### `upgrade_v2_contract.wasm`

A minimal mock WASM used to test the shade hub's upgrade mechanism.

**Consumers:**
- `contracts/shade/src/tests/test_upgrade.rs`
- Example module tests

**Rebuild when:** Soroban SDK WASM format changes (rare).

**How:** Compile a minimal Soroban contract and copy the binary here — see [`docs/getting-started/running-tests.md#upgrade_v2_contract.wasm`](../../../docs/getting-started/running-tests.md#test-fixtures) for details.

---

For more information, see the **Test fixtures** section in [`docs/getting-started/running-tests.md`](../../../docs/getting-started/running-tests.md).
