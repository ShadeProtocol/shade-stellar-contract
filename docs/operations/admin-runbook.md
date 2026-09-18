# Runbook: Admin Operations & Incident Response

This runbook establishes standard operating procedures (SOP) for routine protocol administration and emergency incident response for the Shade Stellar contract suite.

> **Warning:** Privileged operations directly impact live payments, merchant settlements, and treasury funds. Always execute administrative transactions using dedicated multisig or hardware-backed signing setups, and verify contract state before and after every state-changing call.

---

## 1. Key Custody & Governance Policy

### Custody Standard
- **Key Storage**: Admin private keys must never reside on hot internet-connected servers, unencrypted environment variables, or developer laptops.
- **Multisig Threshold**: In staging and production environments, the protocol admin address must be a multisig threshold account (e.g., 2-of-3 or 3-of-5) across geographically distributed keyholders.
- **Auditing & Change-Log**: Every administrative action must be recorded in an operational change-log containing:
  1. Transaction hash (`tx_hash`) and ledger sequence.
  2. Operating signer identity and reason for change.
  3. Pre-state snapshot and post-state verification proof.

---

## 2. Routine Operational Procedures

### 2.1 Proposing and Executing a Fee Update (Two-Step Fee Protocol)

Fee modifications use a two-step timelock pattern (`propose_fee` followed by `execute_fee`) to prevent unexpected rate shifts.

#### Step 1: Propose New Fee
```bash
stellar contract invoke \
  --id <SHADE_CONTRACT_ID> \
  --source <ADMIN_SECRET_OR_KEY> \
  --network <NETWORK> \
  -- propose_fee \
  --admin <ADMIN_ADDRESS> \
  --token <TOKEN_CONTRACT_ADDRESS> \
  --fee <FEE_IN_BPS_OR_FIXED_I128>
```

#### Step 2: Verify Pending Fee State
```bash
stellar contract invoke \
  --id <SHADE_CONTRACT_ID> \
  --source <OPERATOR_ADDRESS> \
  --network <NETWORK> \
  -- get_pending_fee \
  --token <TOKEN_CONTRACT_ADDRESS>
```
*Expected output: Returns `PendingFee` struct with the proposed amount and effective timestamp.*

#### Step 3: Execute Fee (After Timelock Expiration)
```bash
stellar contract invoke \
  --id <SHADE_CONTRACT_ID> \
  --source <ADMIN_SECRET_OR_KEY> \
  --network <NETWORK> \
  -- execute_fee \
  --admin <ADMIN_ADDRESS> \
  --token <TOKEN_CONTRACT_ADDRESS>
```

#### Step 4: Verify Active Fee
```bash
stellar contract invoke \
  --id <SHADE_CONTRACT_ID> \
  --source <OPERATOR_ADDRESS> \
  --network <NETWORK> \
  -- get_fee \
  --token <TOKEN_CONTRACT_ADDRESS>
```

---

### 2.2 Token Whitelist Management

#### Listing an Accepted Payment Token
```bash
stellar contract invoke \
  --id <SHADE_CONTRACT_ID> \
  --source <ADMIN_SECRET_OR_KEY> \
  --network <NETWORK> \
  -- add_accepted_token \
  --admin <ADMIN_ADDRESS> \
  --token <NEW_TOKEN_CONTRACT_ADDRESS>
```

#### Batch Token Listing
```bash
stellar contract invoke \
  --id <SHADE_CONTRACT_ID> \
  --source <ADMIN_SECRET_OR_KEY> \
  --network <NETWORK> \
  -- add_accepted_tokens \
  --admin <ADMIN_ADDRESS> \
  --tokens '[ "<TOKEN_1>", "<TOKEN_2>" ]'
```

#### De-listing a Token
```bash
stellar contract invoke \
  --id <SHADE_CONTRACT_ID> \
  --source <ADMIN_SECRET_OR_KEY> \
  --network <NETWORK> \
  -- remove_accepted_token \
  --admin <ADMIN_ADDRESS> \
  --token <TOKEN_CONTRACT_ADDRESS>
```

#### Verification Step
```bash
stellar contract invoke \
  --id <SHADE_CONTRACT_ID> \
  --source <OPERATOR_ADDRESS> \
  --network <NETWORK> \
  -- is_accepted_token \
  --token <TOKEN_CONTRACT_ADDRESS>
# Returns: true (listed) or false (delisted)
```

---

### 2.3 Merchant Lifecycle & Verification

#### Verifying a Merchant Account
```bash
stellar contract invoke \
  --id <SHADE_CONTRACT_ID> \
  --source <ADMIN_SECRET_OR_KEY> \
  --network <NETWORK> \
  -- verify_merchant \
  --admin <ADMIN_ADDRESS> \
  --merchant_id <MERCHANT_ID_U64> \
  --status true
```

#### Deactivating a Malicious or Inactive Merchant
```bash
stellar contract invoke \
  --id <SHADE_CONTRACT_ID> \
  --source <ADMIN_SECRET_OR_KEY> \
  --network <NETWORK> \
  -- set_merchant_status \
  --admin <ADMIN_ADDRESS> \
  --merchant_id <MERCHANT_ID_U64> \
  --status false
```

#### Verification Steps
```bash
# Check verified badge
stellar contract invoke \
  --id <SHADE_CONTRACT_ID> \
  --source <OPERATOR_ADDRESS> \
  --network <NETWORK> \
  -- is_merchant_verified \
  --merchant_id <MERCHANT_ID_U64>

# Check operational status
stellar contract invoke \
  --id <SHADE_CONTRACT_ID> \
  --source <OPERATOR_ADDRESS> \
  --network <NETWORK> \
  -- is_merchant_active \
  --merchant_id <MERCHANT_ID_U64>
```

---

### 2.4 Role Management (Access Control)

#### Granting an Operator or Auditor Role
```bash
stellar contract invoke \
  --id <SHADE_CONTRACT_ID> \
  --source <ADMIN_SECRET_OR_KEY> \
  --network <NETWORK> \
  -- grant_role \
  --admin <ADMIN_ADDRESS> \
  --target <OPERATOR_OR_AGENT_ADDRESS> \
  --role <ROLE_NAME_OR_ENUM>
```

#### Revoking a Role
```bash
stellar contract invoke \
  --id <SHADE_CONTRACT_ID> \
  --source <ADMIN_SECRET_OR_KEY> \
  --network <NETWORK> \
  -- revoke_role \
  --admin <ADMIN_ADDRESS> \
  --target <OPERATOR_OR_AGENT_ADDRESS> \
  --role <ROLE_NAME_OR_ENUM>
```

---

### 2.5 Rotating the Platform Treasury Account

#### Update Treasury Account
```bash
stellar contract invoke \
  --id <SHADE_CONTRACT_ID> \
  --source <ADMIN_SECRET_OR_KEY> \
  --network <NETWORK> \
  -- set_platform_account \
  --admin <ADMIN_ADDRESS> \
  --account <NEW_TREASURY_STELLAR_ADDRESS>
```

#### Verification Step
```bash
stellar contract invoke \
  --id <SHADE_CONTRACT_ID> \
  --source <OPERATOR_ADDRESS> \
  --network <NETWORK> \
  -- get_platform_account
# Confirm output matches NEW_TREASURY_STELLAR_ADDRESS exactly
```

---

### 2.6 Admin Ownership Transfer (Two-Step Handshake)

Ownership transfer requires explicit acceptance by the recipient to prevent accidental burning of admin control.

#### Step 1: Initiate Transfer
```bash
stellar contract invoke \
  --id <SHADE_CONTRACT_ID> \
  --source <CURRENT_ADMIN_SECRET> \
  --network <NETWORK> \
  -- transfer_admin \
  --admin <CURRENT_ADMIN_ADDRESS> \
  --new_admin <NEW_ADMIN_ADDRESS>
```

#### Step 2: Accept Admin Ownership
```bash
stellar contract invoke \
  --id <SHADE_CONTRACT_ID> \
  --source <NEW_ADMIN_SECRET> \
  --network <NETWORK> \
  -- accept_admin \
  --new_admin <NEW_ADMIN_ADDRESS>
```

#### Step 3: Verify Admin State
```bash
stellar contract invoke \
  --id <SHADE_CONTRACT_ID> \
  --source <OPERATOR_ADDRESS> \
  --network <NETWORK> \
  -- get_admin
# Confirms active admin is now NEW_ADMIN_ADDRESS
```

---

## 3. Incident Response Procedure

### 3.1 Severity Matrix & Escalation SLA

| Severity | Definition | Initial Triage SLA | Mitigation / Pause SLA | Escalation Target |
| :--- | :--- | :--- | :--- | :--- |
| **SEV-1 (Critical)** | Active exploit, invariant violation, unauthorized fund drain, cryptographic failure | **< 15 minutes** | **< 30 minutes** | Security Council, Core Signers, Executive Team |
| **SEV-2 (High)** | Core payment flow broken, Oracle price stale/divergent, merchant settlements blocked | **< 30 minutes** | **< 2 hours** | Lead Smart Contract Engineer, Operations Lead |
| **SEV-3 (Medium)** | Non-critical feature failure (e.g. analytics discrepancy, secondary indexing lag) | **< 4 hours** | **< 24 hours** | On-Call Backend Engineer |
| **SEV-4 (Low)** | Cosmetic UI issue, documentation error, minor non-blocking telemetry gap | Next sprint | Scheduled release | Engineering Backlog |

---

### 3.2 Detection Signals

1. **RPC & Invariant Alarms**: Automated alerts on unexpected contract balance decreases or abnormal invoice volume.
2. **Oracle Divergence**: Asset price feed deviations exceeding 5% within a 5-minute rolling window.
3. **On-Chain Event Spikes**: High frequency of contract error codes (`PaymentFailed`, `Unauthorized`, `InvalidState`).
4. **External Disclosures**: Verified submissions from bug bounty programs or security partners.

---

### 3.3 Emergency Pause Protocol

#### When to Pause
- Invariant failure where contract liabilities exceed assets.
- Suspected reentrancy or logic flaw in invoice settlement or escrow release.
- Critical oracle malfunction causing underpriced asset swaps.

#### When NOT to Pause
- Single user client-side wallet connectivity issues.
- Isolated upstream indexing or UI display lags.
- Routine fee adjustments or planned maintenance.

#### Emergency Pause Execution
```bash
stellar contract invoke \
  --id <SHADE_CONTRACT_ID> \
  --source <ADMIN_OR_GUARDIAN_SECRET> \
  --network <NETWORK> \
  -- pause \
  --admin <ADMIN_ADDRESS>
```

#### Verification Step
```bash
stellar contract invoke \
  --id <SHADE_CONTRACT_ID> \
  --source <OPERATOR_ADDRESS> \
  --network <NETWORK> \
  -- is_paused
# Must return: true
```

---

### 3.4 Triage & Root-Cause Diagnosis

1. **Inspect Contract Events**:
   Query Soroban RPC for recent events emitted by `<SHADE_CONTRACT_ID>` filtering by ledger ranges surrounding the incident timestamp:
   ```bash
   stellar events --id <SHADE_CONTRACT_ID> --start-ledger <INCIDENT_LEDGER>
   ```
2. **Inspect Storage Entries**:
   Verify current state entries (escrow balances, pending fee queues, merchant balances) against known valid snapshots.
3. **Reproduce in Local Sandbox**:
   Extract transaction XDR and replay in a local Soroban test harness (`cargo test` with sandbox mocks) to isolate the vulnerability vector.

---

### 3.5 Remediation Options

- **Option A (Parameter Fix)**: If caused by bad configuration, update oracle addresses (`set_token_oracle`) or delist vulnerable tokens (`remove_accepted_token`).
- **Option B (Role Revocation)**: Revoke compromised operator or merchant credentials immediately (`revoke_role`, `set_merchant_status false`).
- **Option C (Emergency WASM Upgrade)**:
  If a contract logic bug is discovered:
  1. Audit and compile patched WASM binary: `cargo build --target wasm32-unknown-unknown --release`.
  2. Install new WASM to Stellar ledger: `stellar contract install --wasm target/wasm32-unknown-unknown/release/shade.wasm`.
  3. Execute contract upgrade via admin upgrade method with the new WASM hash:
     ```bash
     stellar contract invoke \
       --id <SHADE_CONTRACT_ID> \
       --source <ADMIN_SECRET_OR_KEY> \
       -- upgrade \
       --admin <ADMIN_ADDRESS> \
       --new_wasm_hash <NEW_WASM_HASH_BYTES32>
     ```

---

### 3.6 Unpause Protocol & Safe Resumption

Before unpausing, verify all of the following conditions:
- [ ] Root cause identified, documented, and patched.
- [ ] On-chain contract state audited and verified consistent.
- [ ] Unit and integration regression tests pass 100% locally.
- [ ] Core signers and Security Council provide written unpause sign-off.

#### Execution
```bash
stellar contract invoke \
  --id <SHADE_CONTRACT_ID> \
  --source <ADMIN_SECRET_OR_KEY> \
  --network <NETWORK> \
  -- unpause \
  --admin <ADMIN_ADDRESS>
```

#### Verification & Canary Period
```bash
stellar contract invoke \
  --id <SHADE_CONTRACT_ID> \
  --source <OPERATOR_ADDRESS> \
  --network <NETWORK> \
  -- is_paused
# Must return: false
```
*Maintain high-frequency telemetry monitoring for 24 hours following the unpause.*

---

## 4. Post-Incident Process

1. **Timeline Reconstruction**:
   Consolidate ledger sequence numbers, timestamps, transaction hashes, and affected accounts into a unified chronological log.
2. **Merchant & Partner Communication**:
   Publish factual, transparent status updates to affected merchants and partners through verified announcement channels within 2 hours of containment.
3. **Root Cause Analysis (RCA)**:
   Publish a blameless post-mortem report covering:
   - Root technical cause.
   - Attack vector and impact quantification.
   - Response timeline and effectiveness.
   - Preventative engineering commitments.

---

## 5. Related Documentation

- [Pausable Security Pattern](../security/pausable.md)
- [Admin and Ownership Architecture](../security/admin-and-ownership.md)
- [Access Control Model](../security/access-control.md)
- [Contract Upgradeability Guide](../architecture/upgradeability.md)
- [Security Threat Model](../security/threat-model.md)
- [Shade Interface Reference](../reference/shade-interface.md)
