# Stellar CLI cookbook for Shade

Copy-pasteable `stellar contract invoke` recipes covering routine administration, merchant management, payments, and read-only inspection on the Shade payment hub contract.

Every recipe in this cookbook targets the `shade` contract deployed on Stellar testnet using the official `stellar` CLI.

---

## Toolchain and testnet verification

| Parameter | Specification | Notes |
| :--- | :--- | :--- |
| **CLI Binary** | `stellar` (formerly `soroban`) | Verified with `stellar 23.0.0` (Stellar CLI 23.x series). |
| **Protocol / SDK** | Soroban Protocol 23 / `soroban-sdk 23.5.3` | Matches [`Cargo.lock`](../../Cargo.lock). |
| **Target Network** | Stellar Testnet | RPC: `https://soroban-testnet.stellar.org:443` |
| **Network Passphrase** | `"Test SDF Network ; September 2015"` | Default testnet passphrase. |
| **Interface Source** | [`contracts/shade/src/shade_interface.rs`](../../contracts/shade/src/shade_interface.rs) | Contract entry points and signatures. |

> **Note:** The `stellar` CLI was previously distributed as `soroban-cli`. All commands below use the modern `stellar` command syntax (`stellar contract invoke`).

---

## Network and identity setup

Configure your testnet network profile and identities once before running any recipe.

### 1. Configure the testnet network profile

If the `testnet` network profile is not already configured in your global CLI config, add it:

```bash
stellar network add \
  --global testnet \
  --rpc-url https://soroban-testnet.stellar.org:443 \
  --network-passphrase "Test SDF Network ; September 2015"
```

### 2. Generate and fund role identities

Generate dedicated CLI key aliases for the [admin](../glossary.md#admin), [merchant](../glossary.md#merchant), and [payer](../glossary.md#payer), then fund them via Friendbot on testnet:

```bash
# Generate key aliases
stellar keys generate admin --network testnet
stellar keys generate merchant --network testnet
stellar keys generate payer --network testnet

# Fund keys using testnet Friendbot
stellar keys fund admin --network testnet
stellar keys fund merchant --network testnet
stellar keys fund payer --network testnet
```

### 3. Set environment variables

Export the contract ID, network, and account addresses into your shell environment. All subsequent recipes reference these variables:

```bash
# Target network profile
export NETWORK="testnet"

# Shade hub contract address (deployed C... address)
export SHADE_ID="CA3D5KRYMCMCZVACWNGLXGVCCC25DHNVJNBDW5ICWKO2WDNKGMA74QGW"

# Resolved public addresses for CLI key aliases
export ADMIN_ADDRESS="$(stellar keys address admin)"
export MERCHANT_ADDRESS="$(stellar keys address merchant)"
export PAYER_ADDRESS="$(stellar keys address payer)"

# Accepted payment token contract address (e.g. Testnet USDC or SAC token)
export TOKEN_ADDRESS="CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC"

# Price oracle contract address (for fiat-denominated invoices)
export ORACLE_CONTRACT="CA7QIPRF2WPE372W5EPITJ7GF5L3FYIZCLU4J27YQLT5DGYJ7GHR636W"
```

---

## Encoding complex arguments

Stellar CLI parses function arguments according to the Soroban XDR ABI specifications defined in [`contracts/shade/src/types.rs`](../../contracts/shade/src/types.rs).

### Summary of type encodings

| Type | CLI syntax | Example |
| :--- | :--- | :--- |
| **Primitive (`u64`, `u32`, `i128`)** | Plain numeric literal | `--invoice_id 42 --amount 100000000` |
| **Boolean (`bool`)** | `true` or `false` | `--status true` |
| **String (`String`)** | Quoted UTF-8 string | `--description "Software License"` |
| **Address (`Address`)** | `G...` (account) or `C...` (contract) | `--admin "$ADMIN_ADDRESS"` |
| **Vector (`Vec<T>`)** | JSON array | `--tokens '["CDLZ...", "CB64..."]'` |
| **Option (`Option<T>`)** | Pass `--flag <val>` for `Some`, omit flag for `None` | `--expires_at 1764000000` *(or omit flag)* |
| **Struct** | JSON object string | `--oracle '{"contract":"CA...","price_decimals":8,"token_decimals":7}'` |
| **Unit Enum** | Variant name string or JSON | `--role Admin` *(or `--role '{"Admin":[]}'`)* |
| **Tuple / Data Enum** | JSON object with variant key | `--route '{"Direct":[]}'` |
| **Fixed Bytes (`BytesN<32>`)** | 64-character lowercase hex string | `--key 0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef` |
| **Signature Bytes (`BytesN<64>`)** | 128-character lowercase hex string | `--signature <128-char-hex>` |

### Vectors (`Vec<T>`)
Pass vectors as JSON arrays wrapped in single quotes:
```bash
--tokens '["CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC"]'
--invoice_ids '[101, 102, 103]'
```

### Options (`Option<T>`)
- **Top-level CLI argument:** Omit the flag completely to pass `None`. Supply the flag with a value to pass `Some(val)`.
- **Nested inside JSON struct:** Use `null` or omit the field:
  ```json
  {"status": 1, "merchant": null, "min_amount": null, "max_amount": null, "start_date": null, "end_date": null}
  ```

### Structs
Provide fields as a JSON object matching the Rust struct definition:
```bash
# OracleConfig struct: { contract: Address, price_decimals: u32, token_decimals: u32 }
--oracle '{"contract":"'$ORACLE_CONTRACT'","price_decimals":8,"token_decimals":7}'

# MerchantFilter struct: { is_active: Option<bool>, is_verified: Option<bool> }
--filter '{"is_active":true,"is_verified":true}'
```

### Enums
- **Unit variants (`Role`):** Pass the variant name directly:
  ```bash
  --role Admin
  # Or: --role Manager
  # Or: --role Operator
  ```
- **Data variants (`PaymentRoute`):**
  ```bash
  # PaymentRoute::Direct
  --route '{"Direct":[]}'

  # PaymentRoute::Swap(SwapRoute)
  --route '{"Swap":{"router":"'$ROUTER_ADDRESS'","path":["'$TOKEN_A'","'$TOKEN_B'"]}}'
  ```

---

## Admin recipes

Admin commands require administrator authorization. They must be invoked with `--source-account admin` (or an account holding the `Admin` role).

### Initialize contract

Initialize the Shade hub contract immediately after deployment. Can only be called once:

```bash
stellar contract invoke \
  --id "$SHADE_ID" \
  --network "$NETWORK" \
  --source-account admin \
  -- \
  initialize \
  --admin "$ADMIN_ADDRESS"
```

### Whitelist an accepted payment token

Whitelist a token so merchants can accept it and payers can use it:

```bash
stellar contract invoke \
  --id "$SHADE_ID" \
  --network "$NETWORK" \
  --source-account admin \
  -- \
  add_accepted_token \
  --admin "$ADMIN_ADDRESS" \
  --token "$TOKEN_ADDRESS"
```

### Whitelist multiple accepted tokens in batch

```bash
stellar contract invoke \
  --id "$SHADE_ID" \
  --network "$NETWORK" \
  --source-account admin \
  -- \
  add_accepted_tokens \
  --admin "$ADMIN_ADDRESS" \
  --tokens '["CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC"]'
```

### Remove an accepted token

Remove a token from the accepted token list. Existing invoices remain valid, but new invoices cannot use this token:

```bash
stellar contract invoke \
  --id "$SHADE_ID" \
  --network "$NETWORK" \
  --source-account admin \
  -- \
  remove_accepted_token \
  --admin "$ADMIN_ADDRESS" \
  --token "$TOKEN_ADDRESS"
```

### Set token fee immediately

Set the protocol fee rate for a token in basis points (`500` = 5.00%, `10_000` = 100%):

```bash
stellar contract invoke \
  --id "$SHADE_ID" \
  --network "$NETWORK" \
  --source-account admin \
  -- \
  set_fee \
  --admin "$ADMIN_ADDRESS" \
  --token "$TOKEN_ADDRESS" \
  --fee 250
```

### Propose a time-locked fee change

Propose a fee change that requires a 48-hour time lock before execution:

```bash
stellar contract invoke \
  --id "$SHADE_ID" \
  --network "$NETWORK" \
  --source-account admin \
  -- \
  propose_fee \
  --admin "$ADMIN_ADDRESS" \
  --token "$TOKEN_ADDRESS" \
  --fee 300
```

### Execute a time-locked fee change

Execute a pending fee proposal after the 48-hour delay (`172_800` seconds) has elapsed:

```bash
stellar contract invoke \
  --id "$SHADE_ID" \
  --network "$NETWORK" \
  --source-account admin \
  -- \
  execute_fee \
  --admin "$ADMIN_ADDRESS" \
  --token "$TOKEN_ADDRESS"
```

### Set per-merchant platform fee override

Set a customized fee rate for a specific merchant (callable by Admin or Manager):

```bash
stellar contract invoke \
  --id "$SHADE_ID" \
  --network "$NETWORK" \
  --source-account admin \
  -- \
  set_merchant_platform_fee \
  --caller "$ADMIN_ADDRESS" \
  --merchant_id 1 \
  --token "$TOKEN_ADDRESS" \
  --fee_bps 150
```

### Clear per-merchant platform fee override

Revert a merchant back to the global token fee:

```bash
stellar contract invoke \
  --id "$SHADE_ID" \
  --network "$NETWORK" \
  --source-account admin \
  -- \
  clear_merchant_platform_fee \
  --caller "$ADMIN_ADDRESS" \
  --merchant_id 1 \
  --token "$TOKEN_ADDRESS"
```

### Set platform treasury account

Set the address where protocol fees are collected:

```bash
stellar contract invoke \
  --id "$SHADE_ID" \
  --network "$NETWORK" \
  --source-account admin \
  -- \
  set_platform_account \
  --admin "$ADMIN_ADDRESS" \
  --account "$ADMIN_ADDRESS"
```

### Configure token oracle

Configure the oracle contract for fiat conversion calculations:

```bash
stellar contract invoke \
  --id "$SHADE_ID" \
  --network "$NETWORK" \
  --source-account admin \
  -- \
  set_token_oracle \
  --admin "$ADMIN_ADDRESS" \
  --token "$TOKEN_ADDRESS" \
  --oracle '{"contract":"'$ORACLE_CONTRACT'","price_decimals":8,"token_decimals":7}'
```

### Pause contract (emergency stop)

Halt all payment processing, merchant registrations, and invoice operations:

```bash
stellar contract invoke \
  --id "$SHADE_ID" \
  --network "$NETWORK" \
  --source-account admin \
  -- \
  pause \
  --admin "$ADMIN_ADDRESS"
```

### Unpause contract

Resume normal contract operations:

```bash
stellar contract invoke \
  --id "$SHADE_ID" \
  --network "$NETWORK" \
  --source-account admin \
  -- \
  unpause \
  --admin "$ADMIN_ADDRESS"
```

### Grant a role

Assign an administrative role (`Admin`, `Manager`, or `Operator`) to an address:

```bash
stellar contract invoke \
  --id "$SHADE_ID" \
  --network "$NETWORK" \
  --source-account admin \
  -- \
  grant_role \
  --admin "$ADMIN_ADDRESS" \
  --user "$MERCHANT_ADDRESS" \
  --role Manager
```

### Revoke a role

```bash
stellar contract invoke \
  --id "$SHADE_ID" \
  --network "$NETWORK" \
  --source-account admin \
  -- \
  revoke_role \
  --admin "$ADMIN_ADDRESS" \
  --user "$MERCHANT_ADDRESS" \
  --role Manager
```

### Propose two-step admin transfer (Step 1)

Initiate an admin ownership transfer to a new address:

```bash
export NEW_ADMIN_ADDRESS="GBZXN7PIRZGNMHGA72XZUFTVPTQK4F43B776CVSTH645JCSGCLANQDT3"

stellar contract invoke \
  --id "$SHADE_ID" \
  --network "$NETWORK" \
  --source-account admin \
  -- \
  propose_admin_transfer \
  --admin "$ADMIN_ADDRESS" \
  --new_admin "$NEW_ADMIN_ADDRESS"
```

### Accept two-step admin transfer (Step 2)

Must be signed and submitted by the nominated new admin:

```bash
stellar contract invoke \
  --id "$SHADE_ID" \
  --network "$NETWORK" \
  --source-account new_admin \
  -- \
  accept_admin_transfer \
  --new_admin "$NEW_ADMIN_ADDRESS"
```

### Set merchant active status

Deactivate or activate an existing merchant:

```bash
stellar contract invoke \
  --id "$SHADE_ID" \
  --network "$NETWORK" \
  --source-account admin \
  -- \
  set_merchant_status \
  --admin "$ADMIN_ADDRESS" \
  --merchant_id 1 \
  --status true
```

### Verify merchant

Mark a merchant as verified in the protocol registry:

```bash
stellar contract invoke \
  --id "$SHADE_ID" \
  --network "$NETWORK" \
  --source-account admin \
  -- \
  verify_merchant \
  --admin "$ADMIN_ADDRESS" \
  --merchant_id 1 \
  --status true
```

### Restrict merchant account

Restrict or unrestrict withdrawals from a merchant's account contract:

```bash
stellar contract invoke \
  --id "$SHADE_ID" \
  --network "$NETWORK" \
  --source-account admin \
  -- \
  restrict_merchant_account \
  --caller "$ADMIN_ADDRESS" \
  --merchant_address "$MERCHANT_ADDRESS" \
  --status true
```

### Set account WASM hash

Register the compiled WASM hash for merchant account deployments:

```bash
export ACCOUNT_WASM_HASH="8d3a1f81cf0567fbf53267d64380aa5de4d0b16ee979c5bdf14d027f6e0b3c66"

stellar contract invoke \
  --id "$SHADE_ID" \
  --network "$NETWORK" \
  --source-account admin \
  -- \
  set_account_wasm_hash \
  --admin "$ADMIN_ADDRESS" \
  --wasm_hash "$ACCOUNT_WASM_HASH"
```

### Upgrade contract WASM

Upgrade the contract logic to a newly installed WASM hash:

```bash
export NEW_SHADE_WASM_HASH="04ef7d8b51d8b2d18447814db36e788eb5a43922de1924151b14a9386d9f7ad9"

stellar contract invoke \
  --id "$SHADE_ID" \
  --network "$NETWORK" \
  --source-account admin \
  -- \
  upgrade \
  --new_wasm_hash "$NEW_SHADE_WASM_HASH"
```

---

## Merchant recipes

Merchant operations manage registration, identity keys, account contracts, webhook endpoints, and invoice lifecycles.

### Register as a merchant

Register the caller address as a new merchant. Automatically generates a sequential merchant ID:

```bash
stellar contract invoke \
  --id "$SHADE_ID" \
  --network "$NETWORK" \
  --source-account merchant \
  -- \
  register_merchant \
  --merchant "$MERCHANT_ADDRESS"
```

### Set merchant public key

Set a 32-byte public key used for off-chain message verification and signing:

```bash
export MERCHANT_KEY_HEX="0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"

stellar contract invoke \
  --id "$SHADE_ID" \
  --network "$NETWORK" \
  --source-account merchant \
  -- \
  set_merchant_key \
  --merchant "$MERCHANT_ADDRESS" \
  --key "$MERCHANT_KEY_HEX"
```

### Set merchant webhook URL

Set the HTTPS webhook endpoint for off-chain payment notifications:

```bash
stellar contract invoke \
  --id "$SHADE_ID" \
  --network "$NETWORK" \
  --source-account merchant \
  -- \
  set_merchant_webhook \
  --merchant "$MERCHANT_ADDRESS" \
  --webhook "https://api.example.com/webhooks/shade"
```

### Deploy dedicated merchant account contract

Deploy an isolated smart account contract for holding merchant funds:

```bash
stellar contract invoke \
  --id "$SHADE_ID" \
  --network "$NETWORK" \
  --source-account merchant \
  -- \
  deploy_merchant_account \
  --merchant "$MERCHANT_ADDRESS"
```

### Configure merchant accepted tokens

Restrict accepted tokens for this merchant to a specific subset of globally accepted tokens:

```bash
stellar contract invoke \
  --id "$SHADE_ID" \
  --network "$NETWORK" \
  --source-account merchant \
  -- \
  set_merchant_accepted_tokens \
  --merchant "$MERCHANT_ADDRESS" \
  --tokens '["CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC"]'
```

### Remove a merchant accepted token

```bash
stellar contract invoke \
  --id "$SHADE_ID" \
  --network "$NETWORK" \
  --source-account merchant \
  -- \
  remove_merchant_accepted_token \
  --merchant "$MERCHANT_ADDRESS" \
  --token "$TOKEN_ADDRESS"
```

### Create a standard crypto invoice

Create a fixed-crypto invoice with an optional expiration timestamp (Unix seconds). Amount is in base units (`100_0000000` = 100 tokens with 7 decimals):

```bash
# With expiration timestamp (e.g. 1764000000)
stellar contract invoke \
  --id "$SHADE_ID" \
  --network "$NETWORK" \
  --source-account merchant \
  -- \
  create_invoice \
  --merchant "$MERCHANT_ADDRESS" \
  --description "Monthly SaaS Subscription" \
  --amount 100000000 \
  --token "$TOKEN_ADDRESS" \
  --expires_at 1764000000
```

To create an invoice without expiration, omit `--expires_at`:

```bash
stellar contract invoke \
  --id "$SHADE_ID" \
  --network "$NETWORK" \
  --source-account merchant \
  -- \
  create_invoice \
  --merchant "$MERCHANT_ADDRESS" \
  --description "Lifetime Access Pass" \
  --amount 500000000 \
  --token "$TOKEN_ADDRESS"
```

### Create a fiat-denominated invoice

Create an invoice priced in fiat (e.g. 49.99 USD, `fiat_amount: 4999`, `fiat_decimals: 2`) converted via oracle at payment time:

```bash
stellar contract invoke \
  --id "$SHADE_ID" \
  --network "$NETWORK" \
  --source-account merchant \
  -- \
  create_fiat_invoice \
  --merchant "$MERCHANT_ADDRESS" \
  --description "Order #98241" \
  --fiat_amount 4999 \
  --fiat_currency "USD" \
  --fiat_decimals 2 \
  --token "$TOKEN_ADDRESS" \
  --expires_at 1764000000
```

### Create an invoice draft

Create a draft invoice that is not yet payable:

```bash
stellar contract invoke \
  --id "$SHADE_ID" \
  --network "$NETWORK" \
  --source-account merchant \
  -- \
  create_invoice_draft \
  --merchant "$MERCHANT_ADDRESS" \
  --description "Draft Consulting Retainer" \
  --amount 250000000 \
  --token "$TOKEN_ADDRESS" \
  --expires_at 1764000000
```

### Finalize an invoice draft

Finalize a draft invoice to transition its status to `Pending` so it can be paid:

```bash
stellar contract invoke \
  --id "$SHADE_ID" \
  --network "$NETWORK" \
  --source-account merchant \
  -- \
  finalize_invoice \
  --merchant "$MERCHANT_ADDRESS" \
  --invoice_id 1
```

### Void an invoice

Cancel an unpaid `Pending` or `Draft` invoice. Voided invoices cannot be paid:

```bash
stellar contract invoke \
  --id "$SHADE_ID" \
  --network "$NETWORK" \
  --source-account merchant \
  -- \
  void_invoice \
  --merchant "$MERCHANT_ADDRESS" \
  --invoice_id 1
```

### Amend an invoice

Update the amount and/or description of a `Pending` unpaid invoice:

```bash
stellar contract invoke \
  --id "$SHADE_ID" \
  --network "$NETWORK" \
  --source-account merchant \
  -- \
  amend_invoice \
  --merchant "$MERCHANT_ADDRESS" \
  --invoice_id 1 \
  --new_amount 120000000 \
  --new_description "Monthly SaaS Subscription (Updated Plan)"
```

### Refund an invoice in full

Refund 100% of a paid invoice back to the payer:

```bash
stellar contract invoke \
  --id "$SHADE_ID" \
  --network "$NETWORK" \
  --source-account merchant \
  -- \
  refund_invoice \
  --merchant "$MERCHANT_ADDRESS" \
  --invoice_id 1
```

### Refund an invoice partially

Refund a specific portion of a paid invoice:

```bash
stellar contract invoke \
  --id "$SHADE_ID" \
  --network "$NETWORK" \
  --source-account merchant \
  -- \
  refund_invoice_partial \
  --merchant "$MERCHANT_ADDRESS" \
  --invoice_id 1 \
  --amount 30000000
```

### Configure auto-withdrawal recipient

Set a cold wallet or alternate address for automatic fund sweeps:

```bash
export COLD_STORAGE_ADDRESS="GCALF2Y54PVR43J23TOC2R36K7L735J4JCY5Q6U7GOC5T6F66G2M2ZJ6"

stellar contract invoke \
  --id "$SHADE_ID" \
  --network "$NETWORK" \
  --source-account merchant \
  -- \
  set_auto_withdrawal_recipient \
  --merchant "$MERCHANT_ADDRESS" \
  --recipient "$COLD_STORAGE_ADDRESS"
```

### Configure auto-withdrawal balance threshold

Set the balance threshold in base units above which incoming funds are automatically swept to the recipient address:

```bash
stellar contract invoke \
  --id "$SHADE_ID" \
  --network "$NETWORK" \
  --source-account merchant \
  -- \
  set_auto_withdrawal_threshold \
  --merchant "$MERCHANT_ADDRESS" \
  --token "$TOKEN_ADDRESS" \
  --threshold 1000000000
```

---

## Payer recipes

Payer operations cover token allowances, invoice payments, partial payments, and batch payments.

### 1. Approve token allowance (Prerequisite)

Before paying invoices, the payer must authorize the `shade` contract to transfer tokens on their behalf by calling `approve` on the token contract:

```bash
# Approve Shade contract to spend up to 1,000 tokens until ledger sequence 2,500,000
stellar contract invoke \
  --id "$TOKEN_ADDRESS" \
  --network "$NETWORK" \
  --source-account payer \
  -- \
  approve \
  --from "$PAYER_ADDRESS" \
  --spender "$SHADE_ID" \
  --amount 10000000000 \
  --live_until_ledger 2500000
```

### 2. Pay an invoice in full

Pay the full outstanding amount for an invoice:

```bash
stellar contract invoke \
  --id "$SHADE_ID" \
  --network "$NETWORK" \
  --source-account payer \
  -- \
  pay_invoice \
  --payer "$PAYER_ADDRESS" \
  --invoice_id 1
```

### 3. Pay an invoice partially

Pay a partial amount toward an invoice (transitions status to `PartiallyPaid`):

```bash
stellar contract invoke \
  --id "$SHADE_ID" \
  --network "$NETWORK" \
  --source-account payer \
  -- \
  pay_invoice_partial \
  --payer "$PAYER_ADDRESS" \
  --invoice_id 1 \
  --amount 50000000
```

### 4. Batch pay multiple invoices

Pay multiple invoices atomically in a single transaction:

```bash
stellar contract invoke \
  --id "$SHADE_ID" \
  --network "$NETWORK" \
  --source-account payer \
  -- \
  pay_invoices_batch \
  --payer "$PAYER_ADDRESS" \
  --invoice_ids '[1, 2, 3]'
```

### 5. Claim a buyer refund

Claim an authorized refund as the invoice buyer:

```bash
stellar contract invoke \
  --id "$SHADE_ID" \
  --network "$NETWORK" \
  --source-account payer \
  -- \
  claim_refund \
  --buyer "$PAYER_ADDRESS" \
  --invoice_id 1
```

### 6. Validate payment payload

Verify routing parameters and slippage limits before executing complex payments:

```bash
stellar contract invoke \
  --id "$SHADE_ID" \
  --network "$NETWORK" \
  --source-account payer \
  -- \
  validate_payment_payload \
  --payload '{"input_token":"'$TOKEN_ADDRESS'","settlement_token":"'$TOKEN_ADDRESS'","route":{"Direct":[]},"max_slippage_bps":100}'
```

---

## Read-only query recipes

Read-only queries simulate transaction execution on the RPC node. They do not consume transaction fees and can be called without `--source-account`.

### Invoice queries

#### Query a single invoice by ID
```bash
stellar contract invoke \
  --id "$SHADE_ID" \
  --network "$NETWORK" \
  -- \
  get_invoice \
  --invoice_id 1
```

#### Resolve current payable amount for an invoice
Calculates the exact token amount due (evaluating live oracle prices for fiat invoices):
```bash
stellar contract invoke \
  --id "$SHADE_ID" \
  --network "$NETWORK" \
  -- \
  resolve_invoice_amount \
  --invoice_id 1
```

#### Query invoices matching a filter
Filter by status (`0` = Pending, `1` = Paid, `2` = Cancelled, `3` = Refunded, etc.):
```bash
stellar contract invoke \
  --id "$SHADE_ID" \
  --network "$NETWORK" \
  -- \
  get_invoices \
  --filter '{"status":0,"merchant":null,"min_amount":null,"max_amount":null,"start_date":null,"end_date":null}'
```

#### Paginated search for invoices
Keyset-paginated search with cursor and page size:
```bash
stellar contract invoke \
  --id "$SHADE_ID" \
  --network "$NETWORK" \
  -- \
  search_invoices_paginated \
  --caller "$ADMIN_ADDRESS" \
  --filter '{"status":null,"merchant":null,"min_amount":null,"max_amount":null,"start_date":null,"end_date":null}' \
  --cursor 0 \
  --page_size 10
```

---

### Merchant queries

#### Query merchant profile by ID
```bash
stellar contract invoke \
  --id "$SHADE_ID" \
  --network "$NETWORK" \
  -- \
  get_merchant \
  --merchant_id 1
```

#### Look up merchant ID from address
```bash
stellar contract invoke \
  --id "$SHADE_ID" \
  --network "$NETWORK" \
  -- \
  find_merchant_id \
  --address "$MERCHANT_ADDRESS"
```

#### Check if an address is a registered merchant
```bash
stellar contract invoke \
  --id "$SHADE_ID" \
  --network "$NETWORK" \
  -- \
  is_merchant \
  --merchant "$MERCHANT_ADDRESS"
```

#### Check if a merchant is active
```bash
stellar contract invoke \
  --id "$SHADE_ID" \
  --network "$NETWORK" \
  -- \
  is_merchant_active \
  --merchant_id 1
```

#### Check if a merchant is verified
```bash
stellar contract invoke \
  --id "$SHADE_ID" \
  --network "$NETWORK" \
  -- \
  is_merchant_verified \
  --merchant_id 1
```

#### Query merchants matching filter
```bash
stellar contract invoke \
  --id "$SHADE_ID" \
  --network "$NETWORK" \
  -- \
  get_merchants \
  --filter '{"is_active":true,"is_verified":true}'
```

#### Paginated search for merchants
```bash
stellar contract invoke \
  --id "$SHADE_ID" \
  --network "$NETWORK" \
  -- \
  search_merchants_paginated \
  --filter '{"is_active":null,"is_verified":null}' \
  --cursor 0 \
  --page_size 10
```

#### Query merchant verification key
```bash
stellar contract invoke \
  --id "$SHADE_ID" \
  --network "$NETWORK" \
  -- \
  get_merchant_key \
  --merchant "$MERCHANT_ADDRESS"
```

#### Query merchant webhook URL
```bash
stellar contract invoke \
  --id "$SHADE_ID" \
  --network "$NETWORK" \
  -- \
  get_merchant_webhook \
  --merchant_id 1
```

#### Query merchant account contract address
```bash
stellar contract invoke \
  --id "$SHADE_ID" \
  --network "$NETWORK" \
  -- \
  get_merchant_account \
  --merchant_id 1
```

#### Query merchant accepted tokens list
```bash
stellar contract invoke \
  --id "$SHADE_ID" \
  --network "$NETWORK" \
  -- \
  get_merchant_accepted_tokens \
  --merchant "$MERCHANT_ADDRESS"
```

#### Check if token is accepted for a specific merchant
```bash
stellar contract invoke \
  --id "$SHADE_ID" \
  --network "$NETWORK" \
  -- \
  is_token_accepted_for_merchant \
  --merchant "$MERCHANT_ADDRESS" \
  --token "$TOKEN_ADDRESS"
```

#### Query merchant auto-withdrawal recipient
```bash
stellar contract invoke \
  --id "$SHADE_ID" \
  --network "$NETWORK" \
  -- \
  get_auto_withdrawal_recipient \
  --merchant_id 1
```

#### Query merchant auto-withdrawal threshold
```bash
stellar contract invoke \
  --id "$SHADE_ID" \
  --network "$NETWORK" \
  -- \
  get_auto_withdrawal_threshold \
  --merchant_id 1 \
  --token "$TOKEN_ADDRESS"
```

---

### Protocol and fee queries

#### Query current contract administrator
```bash
stellar contract invoke \
  --id "$SHADE_ID" \
  --network "$NETWORK" \
  -- \
  get_admin
```

#### Check if contract is paused
```bash
stellar contract invoke \
  --id "$SHADE_ID" \
  --network "$NETWORK" \
  -- \
  is_paused
```

#### Check if token is accepted globally
```bash
stellar contract invoke \
  --id "$SHADE_ID" \
  --network "$NETWORK" \
  -- \
  is_accepted_token \
  --token "$TOKEN_ADDRESS"
```

#### Query global fee rate for token
Returns fee in basis points (`100` = 1.00%):
```bash
stellar contract invoke \
  --id "$SHADE_ID" \
  --network "$NETWORK" \
  -- \
  get_fee \
  --token "$TOKEN_ADDRESS"
```

#### Query pending time-locked fee update
```bash
stellar contract invoke \
  --id "$SHADE_ID" \
  --network "$NETWORK" \
  -- \
  get_pending_fee \
  --token "$TOKEN_ADDRESS"
```

#### Query per-merchant platform fee override
```bash
stellar contract invoke \
  --id "$SHADE_ID" \
  --network "$NETWORK" \
  -- \
  get_merchant_platform_fee \
  --merchant_id 1 \
  --token "$TOKEN_ADDRESS"
```

#### Query platform fee treasury account
```bash
stellar contract invoke \
  --id "$SHADE_ID" \
  --network "$NETWORK" \
  -- \
  get_platform_account
```

#### Query token oracle configuration
```bash
stellar contract invoke \
  --id "$SHADE_ID" \
  --network "$NETWORK" \
  -- \
  get_token_oracle \
  --token "$TOKEN_ADDRESS"
```

#### Check if user has an assigned role
```bash
stellar contract invoke \
  --id "$SHADE_ID" \
  --network "$NETWORK" \
  -- \
  has_role \
  --user "$MERCHANT_ADDRESS" \
  --role Manager
```

#### Calculate fee for an amount
Calculates the fee amount in base units for a given payment:
```bash
stellar contract invoke \
  --id "$SHADE_ID" \
  --network "$NETWORK" \
  -- \
  calculate_fee \
  --merchant "$MERCHANT_ADDRESS" \
  --token "$TOKEN_ADDRESS" \
  --amount 100000000
```

#### Compute platform fee split
Computes the breakdown between gross amount, platform fee, merchant payout, and applied fee bps:
```bash
stellar contract invoke \
  --id "$SHADE_ID" \
  --network "$NETWORK" \
  -- \
  compute_platform_fee_split \
  --merchant "$MERCHANT_ADDRESS" \
  --token "$TOKEN_ADDRESS" \
  --amount 100000000
```

---

### Analytics and history queries

#### Query merchant volume for a token
```bash
stellar contract invoke \
  --id "$SHADE_ID" \
  --network "$NETWORK" \
  -- \
  get_merchant_volume \
  --merchant "$MERCHANT_ADDRESS" \
  --token "$TOKEN_ADDRESS"
```

#### Query merchant analytics record for a token
```bash
stellar contract invoke \
  --id "$SHADE_ID" \
  --network "$NETWORK" \
  -- \
  get_merchant_analytics \
  --merchant "$MERCHANT_ADDRESS" \
  --token "$TOKEN_ADDRESS"
```

#### Query merchant summary analytics across all tokens
```bash
stellar contract invoke \
  --id "$SHADE_ID" \
  --network "$NETWORK" \
  -- \
  get_merchant_analytics_summary \
  --merchant "$MERCHANT_ADDRESS"
```

#### Query historical transactions for a user
```bash
stellar contract invoke \
  --id "$SHADE_ID" \
  --network "$NETWORK" \
  -- \
  get_user_transactions \
  --user "$PAYER_ADDRESS"
```

#### Query token analytics
Returns total volume, collected fees, transaction count, and unique merchant count:
```bash
stellar contract invoke \
  --id "$SHADE_ID" \
  --network "$NETWORK" \
  -- \
  get_token_analytics \
  --token "$TOKEN_ADDRESS"
```

#### Query total token volume
```bash
stellar contract invoke \
  --id "$SHADE_ID" \
  --network "$NETWORK" \
  -- \
  get_token_volume \
  --token "$TOKEN_ADDRESS"
```

#### Query top tokens by volume
```bash
stellar contract invoke \
  --id "$SHADE_ID" \
  --network "$NETWORK" \
  -- \
  get_top_tokens_by_volume \
  --limit 5
```

#### Query token dominance metrics
Compare volumes across a set of tokens:
```bash
stellar contract invoke \
  --id "$SHADE_ID" \
  --network "$NETWORK" \
  -- \
  get_token_dominance_metrics \
  --tokens '["CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC"]'
```

#### Query token market share
Returns the percentage share of protocol volume in basis points:
```bash
stellar contract invoke \
  --id "$SHADE_ID" \
  --network "$NETWORK" \
  -- \
  get_token_market_share \
  --token "$TOKEN_ADDRESS"
```

---

## Troubleshooting and common CLI errors

| Error Message / Code | Cause | Solution |
| :--- | :--- | :--- |
| `HostError: Error(Contract, #1)` | `NotInitialized` (`Error::NotInitialized`) | Call `initialize` with the admin address before invoking protected functions. |
| `HostError: Error(Contract, #3)` | `Unauthorized` (`Error::Unauthorized`) | The `--source-account` does not match the required signer (e.g. not the admin or invoice merchant). |
| `HostError: Error(Contract, #6)` | `TokenNotAccepted` (`Error::TokenNotAccepted`) | The token address has not been whitelisted via `add_accepted_token`. |
| `HostError: Error(Contract, #11)` | `ContractPaused` (`Error::ContractPaused`) | Contract is currently paused. Admin must invoke `unpause` to resume transactions. |
| `HostError: Error(Contract, #12)` | `FeeUpdateTooEarly` (`Error::FeeUpdateTooEarly`) | Exactly 48 hours must pass between `propose_fee` and `execute_fee`. |
| `HostError: Error(Auth, InvalidAction)` | Missing signature or signer mismatch | Verify that the `--source-account` corresponds to the address passed in auth parameters (e.g. `--admin`, `--merchant`, `--payer`). |
| `HostError: Error(Contract, #...)` on token `transfer_from` | Insufficient allowance or balance | Payer must call `approve` on the token contract granting `shade` sufficient allowance and verify token balance. |
| `error: invalid value '<val>' for '--<arg>'` | CLI argument formatting mismatch | Check the [Encoding complex arguments](#encoding-complex-arguments) section above for correct JSON/hex string syntax. |

For a complete reference of error variants and numeric error codes, see [`docs/reference/errors.md`](errors.md).
