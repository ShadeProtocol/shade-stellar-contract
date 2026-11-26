# Running the test suite

How the Shade Protocol test suite is organized, how to run all of it or any subset, how the generated `test_snapshots/` directories work, and the conventions to follow when you add a test module.

This page assumes you have a working toolchain — see [Prerequisites and Local Toolchain Setup](prerequisites.md) if `cargo test` does not yet run for you.

## Running tests

Tests compile and run against the **native** build, not WASM. You do not need the `wasm32v1-none` target to run them.

### Everything

```bash
cargo test --workspace --all-features
```

This is exactly what CI runs ([`.github/workflows/ci.yml`](../../.github/workflows/ci.yml)). The `example/` workspace is not included. `--all-features` matters: each contract crate enables the SDK's `testutils` feature through its `[dev-dependencies]`, and helpers like `Address::generate` and `env.mock_all_auths()` do not exist without it.

The [`Makefile`](../../Makefile) wraps this as `make test`.

## Code coverage

Workspace code coverage is measured with [`cargo-llvm-cov`](https://github.com/taiki-e/cargo-llvm-cov) and reported in CI on every push and pull request.

### Running coverage locally

Install the LLVM tools component and `cargo-llvm-cov` once:

```bash
rustup component add llvm-tools-preview
cargo install --locked cargo-llvm-cov
```

Then run:

```bash
make coverage
```

This rebuilds the `account` WASM fixture, runs `cargo llvm-cov --workspace --all-features --lcov --output-path target/lcov.info`, and prints a per-file and workspace summary.

### Baseline coverage

Recorded on `contrib` (`1.100.0` toolchain, stable `cargo-llvm-cov` — region coverage tracks conditional/branch execution at the MIR region level because LLVM MC/DC branch coverage requires nightly):

| Crate | Line Coverage | Region Coverage | Function Coverage |
|---|---|---|---|
| `account` | 96.83% (366/378) | 95.18% (316/332) | 82.98% (39/47) |
| `shade` | 88.16% (2845/3227) | 84.98% (2530/2977) | 80.32% (249/310) |
| **Workspace Total** | **89.07% (3211/3605)** | **86.01% (2846/3309)** | **80.67% (288/357)** |

CI uploads `target/lcov.info` as the `lcov-coverage-report` workflow artifact, posts a coverage delta summary on pull requests, and enforces a non-blocking **85.0%** soft line coverage threshold (warning only) until coverage is raised further.

### A single crate

```bash
cargo test -p shade
cargo test -p account
```

`-p <crate-name>` works from anywhere in the workspace and takes the `[package].name` from that crate's `Cargo.toml`. The equivalent long form is:

```bash
cargo test --manifest-path contracts/shade/Cargo.toml
```

### A single module

Everything after the crate selection is a substring filter on the full test path, so a module name selects every test inside it:

```bash
cargo test -p shade tests::test_invoice
cargo test -p shade tests::test_upgrade
cargo test -p shade tests::test_analytics_aggregation
```

> **Warning:** The filter is a substring match, not a prefix match. `cargo test -p shade tests::test_invoice` also runs `tests::test_invoice_expiry`, `tests::test_invoice_void`, and every other module whose path contains that string. Add `::` and the test name, or `-- --exact`, when you need precision.

### A single test by name

```bash
cargo test -p shade tests::test_upgrade::test_state_persists_after_upgrade -- --exact
```

`--exact` (passed through to the test harness after `--`) requires the full path to match rather than merely contain the filter.

### A single integration test file

For crates using the `tests/` directory layout, each file is its own binary and `--test` selects it by file stem:

```bash
```

### Useful flags

| Flag | Effect |
|---|---|
| `-- --nocapture` | Prints `std::println!`/`eprintln!` output from passing tests, including the SDK's snapshot-file notices. |
| `-- --exact` | Treats the filter as a full test path rather than a substring. |
| `-- --ignored` | Runs only tests marked `#[ignore]`. |
| `--no-fail-fast` | Keeps going after a crate fails, so you see every failure in one run. |
| `-- --test-threads=1` | Serializes tests; useful when you are reading interleaved output. |

## Test layout

Tests live inside each crate.

### In-crate unit tests (`src/tests/mod.rs`)

Test modules live inside the crate's own source tree and are compiled as part of the crate under `#[cfg(test)]`. They can reach private items, internal components, and storage keys directly.

Both crates use this layout.

It has three levels. The crate root declares the module:

```rust
// contracts/shade/src/lib.rs
#[cfg(test)]
pub mod tests;
```

[`contracts/shade/src/tests/mod.rs`](../../contracts/shade/src/tests/mod.rs) then declares every test file:

```rust
pub mod test;
pub mod test_accepted_tokens;
pub mod test_access_control;
// … one line per file in contracts/shade/src/tests/
```

And each file opens with an inner `#![cfg(test)]` attribute, then imports through `crate::`:

```rust
// contracts/shade/src/tests/test_upgrade.rs
#![cfg(test)]
use crate::shade::{Shade, ShadeClient};
use crate::types::DataKey;
use soroban_sdk::testutils::{Address as _, Events as _};
use soroban_sdk::{Address, BytesN, Env, Map, Symbol, TryIntoVal, Val, Vec};
```

Because the module is inside the crate, this test can read raw storage through `crate::types::DataKey` — something an integration test cannot do:

```rust
let stored_admin: Address = env.as_contract(&contract_id, || {
    env.storage().persistent().get(&DataKey::Admin).unwrap()
});
```

## Test fixtures

The `shade` test suite depends on two committed WASM fixtures in `contracts/shade/src/tests/fixtures/`:

### `account.wasm`

**Purpose:** The real, deployed `account` contract code that `shade`'s factory tests instantiate and interact with.

**Used by:**
- `test_account_factory.rs` — tests the factory flow: registering merchants, creating accounts, and setting the account WASM hash.

**How to regenerate:** After any change under `contracts/account/`, run:

```bash
make fixtures
```

This builds the account contract and copies it to `contracts/shade/src/tests/fixtures/account.wasm`. CI automatically runs this step before every test.

**When it's required:** You must rebuild the fixture *before testing* any time you change:
- `contracts/account/src/` — the account contract code
- `contracts/account/Cargo.toml` — account dependencies or feature flags

If you skip this step and run tests, the fixture will be stale and factory tests may behave unexpectedly or fail in ways unrelated to your changes.

### `upgrade_v2_contract.wasm`

**Purpose:** A minimal contract used to test the shade contract's upgrade mechanism. It represents a hypothetical v2 of the shade contract and is uploaded during upgrade tests to verify that code can be replaced.

**Used by:**
- `test_upgrade.rs` — tests that the upgrade function accepts new WASM and that state persists across the upgrade.

**How to regenerate:** This fixture was produced by compiling a minimal Soroban contract to WASM. To regenerate it, you would:

1. Modify or rebuild the source that produced this fixture (the source is not currently in this repository).
2. Compile to WASM with `cargo build --target wasm32v1-none --release`.
3. Replace the file at `contracts/shade/src/tests/fixtures/upgrade_v2_contract.wasm` with the new binary.

Currently, there is no automated `make` target for this — changes to the fixture are manual and rare (only when the upgrade test itself needs updating).

**When it's required:** You need to rebuild this fixture *only* if you are modifying the upgrade test itself or the upgrade mechanism in `shade`. Ordinary contract changes do not affect this fixture.

### Symptoms of a stale fixture

If a fixture becomes out of date, tests that use it may fail in confusing ways:

- **Stale `account.wasm`:**
  - `test_account_factory.rs` tests fail because the factory is trying to instantiate outdated account code.
  - Errors may reference unexpected storage keys, missing methods, or changed interface versions.
  - The fix: `make fixtures`.

- **Stale `upgrade_v2_contract.wasm`:**
  - `test_upgrade.rs` may fail to upload the WASM or fail when calling the upgraded contract.
  - This is rare and usually indicates the upgrade test itself changed and the fixture needs regeneration.

## Test snapshots

### What generates them

Nothing in this repository writes snapshots explicitly — the SDK does it. Every `Env` created with `testutils` enabled writes a JSON snapshot when it is dropped at the end of a test, as long as that `Env` captured something meaningful. `Env::default()` sets `capture_snapshot_at_drop: true`.

Files land under a `test_snapshots/` directory at the crate root, with the test's module path expanded into directories:

```text
contracts/shade/test_snapshots/tests/test_upgrade/test_state_persists_after_upgrade.1.json
```

The trailing number distinguishes multiple `Env` instances created within one test. Running with `-- --nocapture` prints a `Writing test snapshot file for test …` line per file written.

### What they capture

A snapshot is the observable state of the test's Soroban host at the moment the `Env` was dropped:

- **`ledger.entries`** — every ledger entry the test created or touched: contract instances, uploaded WASM, contract data, and their TTLs.
- **`events`** — the contract and system events emitted during the test.
- **`auth`** — the authorization tree that was required and satisfied.

A snapshot with none of those three is skipped, so a test that only reads a getter produces no file.

### Whether to commit them

**Not in this repository.** [`.gitignore`](../../.gitignore) ignores them workspace-wide:

```text
# Test snapshots
test_snapshots/
```

The SDK's own guidance is to commit snapshots so that behavior changes show up as diffs across contract edits, SDK upgrades, and protocol upgrades. Shade has deliberately not adopted that: the suite spans hundreds of tests across nine crates, and the snapshots would dominate every diff. Behavior is asserted explicitly in test bodies instead — see [Asserting on events](#asserting-on-events) below.

The practical consequences:

- A `test_snapshots/` directory appearing in your working tree after `cargo test` is expected. Leave it alone; git already ignores it.
- Never `git add -f` a snapshot file.
- If you want a durable snapshot for one specific investigation, write it somewhere explicit with `env.to_snapshot_file("path.json")` rather than relying on the drop behavior.

### Regenerating and reading them

There is no separate regeneration command and no "accept new snapshot" step — the files are rewritten from scratch on every run:

```bash
# Refresh everything
cargo test --workspace --all-features

# Refresh just one crate's snapshots
rm -rf contracts/shade/test_snapshots
cargo test -p shade
```

To compare behavior before and after a change, copy the directory aside, make the change, re-run, and diff:

```bash
cargo test -p shade
cp -r contracts/shade/test_snapshots /tmp/snapshots-before
# … make your change …
cargo test -p shade
diff -r /tmp/snapshots-before contracts/shade/test_snapshots
```

To suppress snapshot writing entirely in a test — worth doing for a test that creates many `Env`s in a loop — build the `Env` with an explicit config:

```rust
use soroban_sdk::testutils::EnvTestConfig;
use soroban_sdk::Env;

let env = Env::new_with_config(EnvTestConfig {
    capture_snapshot_at_drop: false,
});
```

`env.set_config(…)` changes the setting on an `Env` you already hold.

## Writing a new test module

### Naming and registration

1. Name the file `test_<subject>.rs` — `test_invoice_expiry.rs`, `test_platform_fee.rs`. Feature-numbered names (`test_feature_211.rs`) exist for work tracked to a specific issue; prefer a descriptive name for anything else.
2. Put it in `src/tests/`.
3. Start an in-crate file with `#![cfg(test)]` as its first line.
4. **Register it.** In `src/tests/`, add a `pub mod test_<subject>;` line to that crate's `mod.rs`. An unregistered file compiles as nothing and silently runs zero tests. Files under `tests/` need no registration.
5. Name each test function `test_<behavior>` and make the name describe the assertion, not the call: `test_state_persists_after_upgrade`, not `test_upgrade_2`.

> **Warning:** Declaring the same module twice in a `mod.rs` is a compile error, and so is declaring a module whose file you later delete. If a crate stops compiling right after you touch a test, check `mod.rs` first.

### Setting up the environment

Almost every test starts the same way — construct the `Env`, register the contract, wrap it in the generated client:

```rust
let env = Env::default();
let contract_id = env.register(Shade, ());
let client = ShadeClient::new(&env, &contract_id);
```

Crates with a lot of shared setup factor it into a private helper, which is the pattern to follow:

```rust
// contracts/shade/src/tests/test_access_control.rs
fn setup_test(env: &Env) -> (ShadeClient<'_>, Address) {
    let contract_id = env.register(Shade, ());
    let client = ShadeClient::new(env, &contract_id);
    let admin = Address::generate(env);
    client.initialize(&admin);
    (client, admin)
}
```

### `mock_all_auths` versus explicit auth

Every state-changing entry point in Shade calls `require_auth()` on the acting address, so a test must either satisfy or deliberately withhold that authorization.

**Use `env.mock_all_auths()` when authorization is not what you are testing.** It approves every `require_auth` in the test, letting you focus on the behavior under test:

```rust
let env = Env::default();
env.mock_all_auths();
let (client, admin) = setup_test(&env);

client.grant_role(&admin, &user, &Role::Manager);
assert!(client.has_role(&user, &Role::Manager));
```

**Omit it, or use `mock_auths` with an explicit list, when authorization *is* what you are testing.** `mock_auths` grants exactly the authorizations you name and nothing else, so an unauthorized caller genuinely fails:

```rust
// contracts/shade/src/tests/test_admin_transfer.rs
use soroban_sdk::testutils::{MockAuth, MockAuthInvoke};

let env = Env::default();                       // note: no mock_all_auths
let (client, _admin) = setup_test(&env);

env.mock_auths(&[MockAuth {
    address: &malicious,
    invoke: &MockAuthInvoke {
        contract: &client.address,
        fn_name: "propose_admin_transfer",
        args: (&malicious, &new_admin).into_val(&env),
        sub_invokes: &[],
    },
}]);

let result = client.try_propose_admin_transfer(&malicious, &new_admin);
assert!(result.is_err());
```

Here `malicious` really does sign the call, so the test proves the contract's own `assert_admin` check rejects it — not merely that a signature was missing. That distinction is the reason to reach for `mock_auths`.

> **Security:** A negative authorization test written under `mock_all_auths()` proves nothing about access control if the contract never called `require_auth` at all. When the assertion is "this caller must be rejected," write it with `mock_auths` or with no mocking.

### Asserting on expected panics and errors

Two forms, and they are not interchangeable.

**`#[should_panic]`** asserts the whole call traps, matching on the rendered host error. The number is the discriminant from that crate's error enum ([`contracts/shade/src/errors.rs`](../../contracts/shade/src/errors.rs)):

```rust
#[should_panic(expected = "HostError: Error(Contract, #2)")]
#[test]
fn test_initialize_twice() {
    let env = Env::default();
    let contract_id = env.register(Shade, ());
    let client = ShadeClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    client.initialize(&admin);
    client.initialize(&admin);
}
```

**`try_*` plus a typed comparison** is the stronger form: every generated client method has a `try_` twin that returns a `Result` instead of trapping, so the test can name the error by its variant rather than by a magic number, and can keep asserting afterwards:

```rust
// contracts/shade/src/tests/test_accepted_tokens.rs
use crate::errors::ContractError;

let expected_error =
    soroban_sdk::Error::from_contract_error(ContractError::NotAuthorized as u32);

let add_result = client.try_add_accepted_token(&non_admin, &token);
assert!(matches!(add_result, Err(Ok(err)) if err == expected_error));

client.add_accepted_token(&admin, &token);
let remove_result = client.try_remove_accepted_token(&non_admin, &token);
assert!(matches!(remove_result, Err(Ok(err)) if err == expected_error));
assert!(client.is_accepted_token(&token));
```

The nesting reads: the outer `Err` means the invocation failed; the inner `Ok(err)` means it failed with a recognized contract error rather than an unrecognized host error. Prefer this form — it survives renumbering of the error enum, and it lets one test cover several rejection paths.

### Asserting on events

Import the `Events` testutils trait and read `env.events().all()`. Events are appended in order, so the event you just triggered is the last entry:

```rust
// contracts/shade/src/tests/test_upgrade.rs
use soroban_sdk::testutils::Events as _;

fn assert_latest_upgrade_event(
    env: &Env,
    contract_id: &Address,
    expected_hash: &BytesN<32>,
    expected_timestamp: u64,
) {
    let events = env.events().all();
    assert!(!events.is_empty());

    let (event_contract_id, topics, data) = events.get(events.len() - 1).unwrap();
    assert_eq!(event_contract_id, contract_id.clone());
    assert_eq!(topics.len(), 1);

    let event_name: Symbol = topics.get(0).unwrap().try_into_val(env).unwrap();
    assert_eq!(event_name, Symbol::new(env, "contract_upgraded_event"));

    let data_map: Map<Symbol, Val> = data.try_into_val(env).unwrap();
    let hash_val = data_map.get(Symbol::new(env, "new_wasm_hash")).unwrap();
    let timestamp_val = data_map.get(Symbol::new(env, "timestamp")).unwrap();

    let hash_in_event: BytesN<32> = hash_val.try_into_val(env).unwrap();
    let timestamp_in_event: u64 = timestamp_val.try_into_val(env).unwrap();

    assert_eq!(hash_in_event, expected_hash.clone());
    assert_eq!(timestamp_in_event, expected_timestamp);
}
```

Three things to copy from this helper:

- Each tuple is `(contract_id, topics, data)`. Assert on the contract id too — a cross-contract call emits events from more than one address.
- The topic is the event name as a `Symbol`. `#[contractevent]` derives it from the struct name in snake case, so `ContractUpgradedEvent` publishes under `contract_upgraded_event`.
- The payload is a `Map<Symbol, Val>` keyed by the struct's field names. Convert each field with `try_into_val` and compare against a typed value, never against a string.

Factor this into a `fn assert_latest_*_event` helper at the top of the module when more than one test in the file checks the same event.

### Checklist for adding a test module

- [ ] File is named `test_<subject>.rs` and lives in the layout the crate uses.
- [ ] In-crate files begin with `#![cfg(test)]`.
- [ ] `pub mod test_<subject>;` added to the crate's `src/tests/mod.rs` (in-crate layout only).
- [ ] Shared setup is a private `fn setup_*` helper, not copy-pasted per test.
- [ ] `env.mock_all_auths()` used only where authorization is not the subject; negative auth tests use `mock_auths` or no mocking.
- [ ] Failure cases assert a named `ContractError` via `try_*`, or a `#[should_panic]` string with the matching discriminant.
- [ ] Emitted events are asserted by name and field, not just counted.
- [ ] `cargo test -p <crate>` passes, and `cargo fmt --all` and `cargo clippy --workspace --all-features -- -D warnings` are clean.
- [ ] No `test_snapshots/` file staged in the commit.

## Test file naming

- One test file per component or feature, named `test_<component>.rs` (for example `test_fees.rs`, `test_refund.rs`).
- Add new cases to the existing file for that feature. Don't create a second file such as `test_<feature>_v2.rs` or `test_invoice_<feature>.rs`.
- Shared setup (deploying tokens, initializing the hub, registering merchants and accounts) goes in `tests/support.rs`, not in individual test files.
- Put a short comment above each test describing the case, and declare new test files in `tests/mod.rs`.

## Next steps

- [Analytics Test Suite](analytics-tests.md) — full inventory and commands for the analytics aggregation tests.
- [Building the Contracts](building.md) — turning a passing test run into a deployable WASM.
- [Contributing guidelines](../../CONTRIBUTING.md) — commit format and the PR checklist.

← [Back to Getting Started](README.md)
