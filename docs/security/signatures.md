# Signed invoices, merchant keys, and signature verification

This page documents the off-chain signing scheme that lets merchants authorize invoice creation from external systems. An integrator building a payment gateway reads this page to produce a valid signature and submit it via `create_invoice_signed`.

## Overview

A merchant can register an ed25519 public key and then sign invoice parameters off-chain. The contract verifies the signature on-chain before creating the invoice. This lets a merchant's backend server authorize invoices without submitting a Soroban transaction itself.

The flow:

1. Merchant calls `set_merchant_key` to register a public key.
2. Merchant's backend builds a message from the invoice parameters and signs it with the corresponding private key.
3. A caller submits `create_invoice_signed` with the signed payload.
4. The contract verifies the signature and creates the invoice.

## Key registration

### `set_merchant_key`

```rust
fn set_merchant_key(env: Env, merchant: Address, key: BytesN<32>);
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `merchant` | `Address` | The merchant's address (requires auth). |
| `key` | `BytesN<32>` | A 32-byte ed25519 public key. |

- **Authorization:** The `merchant` address must authorize the call.
- **Storage:** Stored under `DataKey::MerchantKey(merchant)`.
- **Rotation:** Calling `set_merchant_key` again replaces the previous key. Any payloads signed with the old key will fail verification after rotation.
- **Event:** Emits `MerchantKeySetEvent` with the new key and timestamp.

### `get_merchant_key`

```rust
fn get_merchant_key(env: Env, merchant: Address) -> BytesN<32>;
```

Returns the currently registered public key for the merchant. Panics with `MerchantKeyNotFound` if no key has been set.

## Message construction

The message that the merchant signs is a concatenation of XDR-serialized fields separated by `|` bytes:

```
contract_address | merchant_address | nonce | amount | token_address | description_bytes
```

Concretely, the bytes are built as follows (see `contracts/shade/src/components/signature_util.rs`):

1. **contract_address** — the Shade contract's own address, serialized to XDR.
2. `|` — a single separator byte (`0x7C`).
3. **merchant_address** — the merchant's address, serialized to XDR.
4. `|` — separator.
5. **nonce** — the 32-byte nonce (`BytesN<32>`), raw bytes.
6. `|` — separator.
7. **amount** — the invoice amount as `i128`, big-endian 16 bytes.
8. `|` — separator.
9. **token_address** — the payment token address, serialized to XDR.
10. `|` — separator.
11. **description_bytes** — the invoice description string, serialized to XDR.

> **Note:** The message uses raw XDR serialization of each field. Do not UTF-8-encode addresses or use JSON. The `amount` field is the only one using big-endian byte encoding rather than XDR.

## Nonce and replay protection

Each signed invoice must include a unique 32-byte nonce (`BytesN<32>`). The contract enforces single-use:

1. Before verifying the signature, the contract checks `DataKey::UsedNonce(merchant, nonce)`.
2. If the nonce already exists in storage, the transaction panics with `NonceAlreadyUsed`.
3. If the nonce is fresh, it is stored as `true` and a `NonceInvalidated` event is emitted.

The signer must never reuse a nonce. A common strategy is to use a random 32-byte value or a monotonically increasing counter.

## Verification path

The `verify_invoice_signature` function in `contracts/shade/src/components/signature_util.rs` performs:

1. **Key lookup:** Reads `DataKey::MerchantKey(merchant)` from persistent storage. Panics with `MerchantKeyNotFound` if absent.
2. **Nonce invalidation:** Calls `invalidate_nonce` which checks and marks the nonce as used.
3. **Message construction:** Builds the message bytes as described above.
4. **Signature check:** Calls `env.crypto().ed25519_verify(&key, &message, &signature)`. Panics with a crypto error if the signature is invalid.

## Off-chain signing example

Using the `ed25519` signing scheme, a merchant backend would:

```python
from stellar_sdk import StrKey, KeyPair
from stellar_sdk import xdr as stellar_xdr
import hashlib

# 1. Load the merchant's keypair (private key stored securely off-chain)
keypair = KeyPair.from_secret("S...")  # or load from secure storage

# 2. Build the message (must match the contract's build_message exactly)
contract_address_xdr = contract_account.to_xdr_bytes()
merchant_address_xdr = merchant_account.to_xdr_bytes()
token_address_xdr = token_account.to_xdr_bytes()
description_xdr = stellar_xdr.String(description).to_xdr_bytes()

msg = (
    contract_address_xdr
    + b"|"
    + merchant_address_xdr
    + b"|"
    + nonce_bytes  # 32 bytes, random or sequential
    + b"|"
    + amount.to_bytes(16, byteorder="big")
    + b"|"
    + token_address_xdr
    + b"|"
    + description_xdr
)

# 3. Sign the message
signature = keypair.sign(msg)

# 4. Submit create_invoice_signed with:
#    - caller: the address submitting the transaction
#    - merchant: the merchant address
#    - description, amount, token: the invoice parameters
#    - nonce: the 32-byte nonce used above
#    - signature: the 64-byte ed25519 signature
```

## Error cases

| Error | Code | Cause |
|-------|------|-------|
| `MerchantKeyNotFound` | 11 | The merchant has not called `set_merchant_key`, or the key was never registered. |
| `NonceAlreadyUsed` | 14 | The nonce has been used in a previous `create_invoice_signed` call. |
| Crypto verification error | — | The signature does not match the message for the registered public key. |

## Security considerations

- **Key storage:** The ed25519 private key must be stored in a secure key management system (HSM, KMS, or encrypted vault). Never commit private keys to source control or expose them in logs.
- **Key rotation:** Rotating a key via `set_merchant_key` invalidates all previously valid signatures. Plan rotation during low-traffic periods and ensure the new key is registered before the old one is decommissioned.
- **Nonce uniqueness:** If using random nonces, the 32-byte space (2^256) makes collisions astronomically unlikely. If using sequential counters, the counter must be tracked off-chain and never reset.
- **Description field:** The description is included in the signed message. Changing the description after signing invalidates the signature, which prevents a relayer from tampering with invoice details.

← [Back to security](README.md)
