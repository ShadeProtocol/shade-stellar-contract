# Verifying Deployed Contracts

Merchants, integrators, and auditors can independently verify that any on-chain `shade` or `account` WASM binary was built from a specific commit in this repository. Because Soroban contracts do not carry a verifiable source version on chain, the **SHA-256 hash of the optimized WASM bytecode** is the sole cryptographic link between a deployed contract and its source code.

## Why pinning is required

Producing byte-identical WASM across different machines requires pinning every layer of the build pipeline:

| Component | Where it is pinned | Pinned value | Why the pin matters |
|---|---|---|---|
| **Rust toolchain** | [`rust-toolchain.toml`](../../rust-toolchain.toml) | `1.96.0` (`wasm32v1-none`) | Different `rustc` and LLVM versions emit different WASM instructions and codegen layouts, changing the output hash even when the Rust source is identical. |
| **Soroban SDK & dependencies** | [`Cargo.toml`](../../Cargo.toml) + [`Cargo.lock`](../../Cargo.lock) | `soroban-sdk` `23.5.3` (via `--locked`) | Minor or patch updates to `soroban-sdk`, `soroban-env-common`, or `stellar-xdr` alter generated contract specs, host bindings, and serialization code. Passing `--locked` ensures Cargo never resolves newer versions. |
| **Stellar CLI (`wasm-opt`)** | [`Dockerfile.reproducible`](../../Dockerfile.reproducible) | `stellar-cli` `28.0.0` | `stellar contract optimize` runs `wasm-opt` passes over the compiled `.wasm`. Different CLI releases bundle different `wasm-opt` versions or pass flags, producing different `.optimized.wasm` bytes. |
| **Release profile** | [`Cargo.toml`](../../Cargo.toml) `[profile.release]` | `opt-level = "z"`, `lto = true`, `codegen-units = 1`, `panic = "abort"`, `strip = "symbols"`, `debug = 0` | Any profile difference (such as `release-with-logs`) changes binary size, debug sections, and instruction layout. |
| **Build paths & locale** | [`Dockerfile.reproducible`](../../Dockerfile.reproducible) | `--remap-path-prefix`, `TZ=UTC`, `LC_ALL=C`, `SOURCE_DATE_EPOCH=0` | Panic strings and metadata can embed absolute filesystem paths (e.g., `/home/alice/...` vs `/Users/bob/...`) or locale-dependent ordering. Remapping `/workspace=/build` and `/usr/local/cargo=/cargo` eliminates host-path leaks. |

---

## Reproducible build

### Option 1: Containerized build (recommended)

The [`Dockerfile.reproducible`](../../Dockerfile.reproducible) image pins Debian Bookworm, Rust `1.96.0`, the `wasm32v1-none` target, `stellar-cli` `28.0.0`, and deterministic environment variables.

1. Check out the exact release tag or commit you want to verify:

   ```bash
   git clone https://github.com/ShadeProtocol/shade-stellar-contract.git
   cd shade-stellar-contract
   git checkout <tag-or-commit-sha>
   ```

2. Run the containerized build script:

   ```bash
   ./scripts/reproducible-build.sh
   ```

   Or invoke Docker directly:

   ```bash
   docker build -f Dockerfile.reproducible -t shade-reproducible-build:latest .
   docker run --rm -v "$PWD:/workspace" -w /workspace shade-reproducible-build:latest
   ```

3. The script writes the optimized WASM binaries and `SHA256SUMS` to `target/wasm32v1-none/release/`:

   - `target/wasm32v1-none/release/shade.optimized.wasm`
   - `target/wasm32v1-none/release/account.optimized.wasm`
   - `target/wasm32v1-none/release/SHA256SUMS`

### Option 2: Native build (matching local toolchain)

If you cannot run Docker, ensure your local environment matches the exact pinned versions (`rustup show` should report `1.96.0` via [`rust-toolchain.toml`](../../rust-toolchain.toml), and `stellar --version` should report `28.0.0`):

```bash
RUSTFLAGS="--remap-path-prefix=$PWD=/build --remap-path-prefix=${CARGO_HOME:-$HOME/.cargo}=/cargo" \
  TZ=UTC LC_ALL=C SOURCE_DATE_EPOCH=0 \
  cargo build --locked --target wasm32v1-none --release -p shade -p account

stellar contract optimize --wasm target/wasm32v1-none/release/shade.wasm
stellar contract optimize --wasm target/wasm32v1-none/release/account.wasm

sha256sum target/wasm32v1-none/release/*.optimized.wasm | sort -k2 > target/wasm32v1-none/release/SHA256SUMS
cat target/wasm32v1-none/release/SHA256SUMS
```

---

## Verifying an on-chain deployment

Once you have built the local `.optimized.wasm` artifacts, fetch the live bytecode from the Stellar network and compare hashes.

### 1. Fetch by contract ID (`shade` or a merchant `account`)

For a deployed contract instance (`C...` address):

```bash
stellar contract fetch \
  --id <CONTRACT_ID> \
  --network mainnet \
  -o onchain.wasm
```

*(Use `--network testnet` for testnet deployments, or pass `--rpc-url` and `--network-passphrase` explicitly.)*

### 2. Fetch by WASM hash (optional)

When verifying an uploaded WASM hash directly — for example, the `account` WASM hash registered in `shade` or a `new_wasm_hash` emitted by `ContractUpgradedEvent`:

```bash
stellar contract fetch \
  --wasm-hash <WASM_HASH> \
  --network mainnet \
  -o onchain.wasm
```

### 3. Compare hashes and bytes

Compute the SHA-256 digest of the fetched WASM and compare it against your local build:

```bash
sha256sum onchain.wasm target/wasm32v1-none/release/shade.optimized.wasm
```

You can also use the Stellar CLI helper or byte-for-byte comparison:

```bash
stellar contract info hash --wasm onchain.wasm
cmp -s onchain.wasm target/wasm32v1-none/release/shade.optimized.wasm && echo "MATCH: byte-identical"
```

If the SHA-256 digests match, the on-chain contract is guaranteed to be executing the exact bytecode compiled from that commit.

### 4. Inspect embedded contract metadata

To inspect the contract interface or any build metadata embedded in the WASM:

```bash
stellar contract info interface --wasm onchain.wasm
stellar contract info meta --wasm onchain.wasm
stellar contract info build --wasm onchain.wasm
```

---

## Published hashes and address registry

Expected WASM SHA-256 hashes for each tagged release and deployed network address are published in:

- [Address & Hash Registry](../operations/address-registry.md) — canonical mapping of release tags, WASM hashes, and network contract IDs.
- [`CHANGELOG.md`](../../CHANGELOG.md) — release history linked to the registry and GitHub Releases (`SHA256SUMS`).

---

## Evaluating Stellar contract source verification services

Several ecosystem tools aim to make Soroban contract verification accessible in block explorers:

| Service / Standard | How it works | Current status & limitations |
|---|---|---|
| **Stellar Expert** | Developers submit a Contract ID, GitHub repository URL, commit/tag, and build configuration through Stellar Expert's verification flow or API. Stellar Expert rebuilds the contract in a container and displays a **Verified** badge if the resulting hash matches the on-chain WASM. | Convenient for end-users browsing an explorer, but availability depends on the explorer supporting the exact Rust/Stellar CLI/SDK toolchain version used by the release. |
| **SEP-58 & Stellar CLI (`stellar contract verify` / `info build`)** | SEP-58 defines standard WASM custom-section metadata (source repository, commit, build environment) so tools can discover and verify source automatically from the on-chain bytecode. | Requires embedding SEP-58 build metadata at compile time and relies on external verifiers supporting the exact container/toolchain environment. |
| **Independent reproducible build (this guide)** | Auditors and merchants check out the tagged commit, run `Dockerfile.reproducible`, and compare `sha256sum` against `stellar contract fetch`. | **Authoritative.** Zero trust in third-party explorers or hosted verification APIs; works offline once the on-chain WASM is fetched. |

### Recommendation

Treat third-party explorer verification (such as Stellar Expert) as an optional convenience layer for public visibility. For security-critical checks — such as auditing a release, verifying an upgrade, or onboarding a high-volume merchant — always perform an independent verification using [`Dockerfile.reproducible`](../../Dockerfile.reproducible) and `stellar contract fetch`.

---

## Related pages

- [Address & Hash Registry](../operations/address-registry.md)
- [Releases Runbook](../operations/releases.md)
- [Deployment Runbook](../operations/deployment.md)
- [Upgradeability § Versioning and identifying a deployed build](../architecture/upgradeability.md#versioning-and-identifying-a-deployed-build)
