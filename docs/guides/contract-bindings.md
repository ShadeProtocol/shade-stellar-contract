# Generating and Using Contract Bindings and SDKs

This guide explains how client and frontend developers can generate, configure, and consume strongly-typed TypeScript bindings for Shade Protocol smart contracts deployed on the Stellar network (Soroban).

---

## Overview

Soroban smart contracts encode their public interface in custom WASM environment sections. The Stellar CLI (`stellar` / `soroban`) can inspect this contract metadata and emit fully-typed TypeScript client packages, ensuring that data structures, parameters, and error types remain strictly synchronized between on-chain contracts and client applications.

For contract data structure reference, see [Data Types Reference](../reference/data-types.md).  
For the deployed contract architecture, see [Contracts Overview](../contracts/README.md).

---

## Prerequisites

1. **Rust & Soroban Toolchain**:
   ```bash
   rustup target add wasm32-unknown-unknown
   ```
2. **Stellar CLI**:
   ```bash
   cargo install --locked stellar-cli --features opt
   # Or via npm
   npm install -g @stellar/stellar-cli
   ```
3. **Node.js**: v18 or later.

---

## 1. Building the Contract WASM

Before generating bindings, build an optimized contract WASM binary:

```bash
cargo build --target wasm32-unknown-unknown --release --package shade_stellar_contract
```

The compiled WASM will be located at:
```
target/wasm32-unknown-unknown/release/shade_stellar_contract.wasm
```

Optimize the binary for deployment and bindings generation:
```bash
stellar contract optimize \
  --wasm target/wasm32-unknown-unknown/release/shade_stellar_contract.wasm
```

---

## 2. Generating TypeScript Bindings

Use `stellar contract bindings typescript` to generate the client package.

### From Built WASM (Local Development)

```bash
stellar contract bindings typescript \
  --wasm target/wasm32-unknown-unknown/release/shade_stellar_contract.optimized.wasm \
  --output-dir packages/shade-client \
  --contract-name ShadeClient \
  --overwrite
```

### From Deployed Contract ID (Testnet / Mainnet)

```bash
stellar contract bindings typescript \
  --network testnet \
  --contract-id CA...CONTRACT_ID \
  --output-dir packages/shade-client \
  --contract-name ShadeClient \
  --overwrite
```

---

## 3. Package Structure

The generated package in `packages/shade-client` includes:

- `src/index.ts` — Main `Client` class with contract methods
- `src/types.ts` — Strongly-typed TypeScript interfaces mapping directly to [Data Types Reference](../reference/data-types.md)
- `src/constants.ts` — Contract deployment hashes and network defaults
- `package.json` — Pre-configured package dependencies (`@stellar/stellar-sdk`)

Install dependencies in the generated package:
```bash
cd packages/shade-client && npm install && npm run build
```

---

## 4. Consuming the Generated Client

### Initialization

```typescript
import { Client, networks } from 'shade-client';

const client = new Client({
  ...networks.testnet,
  rpcUrl: 'https://soroban-testnet.stellar.org',
  contractId: 'CA7QYQE...YOUR_CONTRACT_ID',
});
```

### Read-Only Invocations

Read-only contract functions do not require signing:

```typescript
import { Client } from 'shade-client';

async function fetchMerchantStatus(client: Client, merchantAddress: string) {
  const isVerified = await client.is_merchant_verified({
    merchant: merchantAddress,
  });
  console.log(`Merchant verification status: ${isVerified}`);
  return isVerified;
}
```

### State-Changing Invocations & Signing

State-changing methods require transaction signing:

```typescript
import { Client } from 'shade-client';
import { Keypair } from '@stellar/stellar-sdk';

async function registerMerchant(client: Client, adminKeypair: Keypair, merchantAddress: string) {
  const tx = await client.register_merchant(
    {
      merchant: merchantAddress,
      tier: 1,
    },
    {
      signAndSend: true,
      publicKey: adminKeypair.publicKey(),
      signTransaction: async (xdr) => {
        // Sign via Keypair or Wallet extension (Freighter / Lobstr)
        const envelope = Keypair.fromSecret(adminKeypair.secret()).sign(xdr);
        return envelope;
      },
    }
  );

  console.log('Transaction hash:', tx.hash);
}
```

---

## 5. Error Handling

Contract execution errors map to typed error codes defined in the contract:

```typescript
try {
  await client.pay_invoice({ invoice_id: 12345n });
} catch (err: any) {
  if (err.message.includes('ContractError(1)')) {
    console.error('Action not authorized');
  } else if (err.message.includes('ContractError(2)')) {
    console.error('Invoice expired or not found');
  } else {
    console.error('Unexpected error:', err);
  }
}
```

---

## 6. Build and Sync Workflow (Keeping Bindings in Sync)

When contract interfaces change during development:

1. **Recompile WASM**:
   ```bash
   cargo build --target wasm32-unknown-unknown --release
   ```
2. **Regenerate Bindings**:
   ```bash
   stellar contract bindings typescript \
     --wasm target/wasm32-unknown-unknown/release/shade_stellar_contract.wasm \
     --output-dir packages/shade-client \
     --overwrite
   ```
3. **Verify Git API Diffs**:
   ```bash
   git diff packages/shade-client/src/types.ts
   ```
4. **Run Client Unit Tests**:
   ```bash
   npm --prefix packages/shade-client test
   ```

---

## References

- [Data Types Reference](../reference/data-types.md)
- [Shade Contract Interface](../reference/shade-interface.md)
- [Admin Operations Runbook](../operations/admin-runbook.md)
- [Stellar Documentation — TypeScript Bindings](https://developers.stellar.org/docs/smart-contracts/getting-started/bindings)
