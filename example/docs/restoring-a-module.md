# Restoring an Example Module into the Shade Hub

This guide provides end-to-end instructions for integrators and developers who need to restore a module that was extracted from the core shade hub contract under [ADR 0007: Limit the workspace to the core payment gateway](../../docs/architecture/decisions/0007-core-gateway-scope.md).

---

## 1. Context and Architecture

Under ADR 0007, peripheral capabilities (such as subscriptions, crowdfunding, ticketing, escrow, and governance) were moved out of contracts/shade into the example/*/src/from_shade/ workspace crates. This architectural decision keeps the core shade gateway focused strictly on payment acceptance, settlement, invoicing, fees, and merchant accounts, maintaining a small audit surface and a WASM binary size of approximately 80 KB.

However, certain integrations require synchronous, atomic execution inside the same contract instance rather than cross-contract invocations. In those scenarios, integrators can restore the extracted module directly into contracts/shade.

This walkthrough uses the **escrow** module as the reference implementation because its rom_shade code is fully self-contained and already compiles under --features from-shade.

---

## 2. End-to-End Walkthrough: Restoring escrow

The reference code for the escrow module is located in:
`
example/escrow/src/from_shade/
├── entrypoints.rs   # Entry point signatures and wiring
├── errors.rs        # Dedicated ContractError variants
├── events.rs        # Contract events
├── escrow.rs        # Core business logic component
├── mod.rs           # Module definitions
├── tests/           # Integration test suite
└── types.rs         # Data types and DataKey storage keys
`

### Step 2.1: Copy Component, Types, Events, and Errors into contracts/shade

1. **Component Logic:**
   Copy example/escrow/src/from_shade/escrow.rs to contracts/shade/src/components/escrow.rs.
   Register the new component in contracts/shade/src/components/mod.rs:
   `
ust
   pub mod escrow;
   `

2. **Data Types & Storage Keys:**
   Copy the Escrow and EscrowStatus structs from example/escrow/src/from_shade/types.rs into contracts/shade/src/types.rs.
   Add the storage keys to DataKey in contracts/shade/src/types.rs:
   `
ust
   #[derive(Clone)]
   #[contracttype]
   pub enum DataKey {
       // ... existing core variants ...

       // --- Escrow Module Keys ---
       EscrowCount,
       Escrow(u64),
   }
   `

3. **Events:**
   Port the escrow event publishing functions from example/escrow/src/from_shade/events.rs into contracts/shade/src/events.rs:
   - escrow_created(env: &Env, escrow_id: u64, seller: &Address, buyer: &Address, token: &Address, amount: i128, invoice_id: Option<u64>)
   - escrow_funded(env: &Env, escrow_id: u64, buyer: &Address, amount: i128)
   - escrow_released(env: &Env, escrow_id: u64, seller: &Address, amount: i128)
   - escrow_refunded(env: &Env, escrow_id: u64, buyer: &Address, amount: i128)
   - escrow_disputed(env: &Env, escrow_id: u64, disputed_by: &Address)
   - escrow_resolved(env: &Env, escrow_id: u64, resolver: &Address, release_to_seller: bool)

---

### Step 2.2: Choose Stable Error Discriminants

Soroban error discriminants are stable on-chain representations. You must ensure new variants do not collide with existing variants in contracts/shade/src/errors.rs.

In contracts/shade/src/errors.rs, ContractError contains existing variants up to discriminant 52 (e.g. MerchantAccountAlreadySet = 52).
Historical variants for escrow in example/escrow/src/from_shade/errors.rs used:
`
ust
EscrowNotFound = 46,
InvalidEscrowStatus = 47,
`
Discriminants 46 and 47 are currently vacant in the hub's ContractError (which skips from 45 to 50), or you may allocate new contiguous discriminants at 53+.

Add the variants to ContractError in contracts/shade/src/errors.rs:
`
ust
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum ContractError {
    // ... existing variants ...
    EscrowNotFound = 46,
    InvalidEscrowStatus = 47,
}
`

> **Important:** Never renumber or change existing error discriminants. Existing clients and indexers rely on stable numerical error codes.

---

### Step 2.3: Expose Entry Points in ShadeTrait and shade.rs

Inspect example/escrow/src/from_shade/entrypoints.rs to review the functions the module exposed on the hub.

1. **Add Signatures to ShadeTrait in contracts/shade/src/shade_interface.rs:**
   `
ust
   // --- Escrow Entrypoints ---
   fn create_escrow(
       env: Env,
       seller: Address,
       buyer: Address,
       token: Address,
       amount: i128,
       invoice_id: Option<u64>,
   ) -> u64;
   fn get_escrow(env: Env, escrow_id: u64) -> Escrow;
   fn fund_escrow(env: Env, buyer: Address, escrow_id: u64);
   fn release_escrow(env: Env, buyer: Address, escrow_id: u64);
   fn refund_escrow(env: Env, seller: Address, escrow_id: u64);
   fn dispute_escrow(env: Env, caller: Address, escrow_id: u64);
   fn resolve_dispute(env: Env, admin: Address, escrow_id: u64, release_to_seller: bool);
   `

2. **Implement Entry Points in contracts/shade/src/shade.rs:**
   Wire each method through the contract implementation, enforcing pause guards where mutating operations occur:
   `
ust
   #[contractimpl]
   impl ShadeTrait for Shade {
       // ... existing methods ...

       fn create_escrow(
           env: Env,
           seller: Address,
           buyer: Address,
           token: Address,
           amount: i128,
           invoice_id: Option<u64>,
       ) -> u64 {
           components::pausable::assert_not_paused(&env);
           components::escrow::create_escrow(&env, &seller, &buyer, &token, amount, invoice_id)
       }

       fn get_escrow(env: Env, escrow_id: u64) -> Escrow {
           components::escrow::get_escrow(&env, escrow_id)
       }

       fn fund_escrow(env: Env, buyer: Address, escrow_id: u64) {
           components::pausable::assert_not_paused(&env);
           components::escrow::fund_escrow(&env, &buyer, escrow_id);
       }

       fn release_escrow(env: Env, buyer: Address, escrow_id: u64) {
           components::pausable::assert_not_paused(&env);
           components::escrow::release_escrow(&env, &buyer, escrow_id);
       }

       fn refund_escrow(env: Env, seller: Address, escrow_id: u64) {
           components::pausable::assert_not_paused(&env);
           components::escrow::refund_escrow(&env, &seller, escrow_id);
       }

       fn dispute_escrow(env: Env, caller: Address, escrow_id: u64) {
           components::pausable::assert_not_paused(&env);
           components::escrow::dispute_escrow(&env, &caller, escrow_id);
       }

       fn resolve_dispute(env: Env, admin: Address, escrow_id: u64, release_to_seller: bool) {
           components::pausable::assert_not_paused(&env);
           components::escrow::resolve_dispute(&env, &admin, escrow_id, release_to_seller);
       }
   }
   `

---

### Step 2.4: Restore Tests, Fixtures, and Snapshots

1. Copy example/escrow/src/from_shade/tests/test_escrow_integration.rs to contracts/shade/src/tests/test_escrow.rs.
2. Register the module in contracts/shade/src/tests/mod.rs:
   `
ust
   mod test_escrow;
   `
3. Update imports within the test file to point to crate::* rather than external dependencies.
4. Run the test suite:
   `ash
   cargo test -p shade --test test_escrow
   `
5. If using budget tracking or snapshot assertions, re-run tests with update flags to regenerate fixtures.

---

## 3. Costs and Trade-offs of Restoring a Module

Before restoring an example module into the hub, evaluate the following costs:

| Factor | Hub Module (Restored) | Standalone Contract |
|---|---|---|
| **Audit Surface** | Expands the core contract surface; any vulnerability impacts the entire payment gateway. | Isolated to the standalone contract; core payment gateway remains unaffected. |
| **WASM Binary Size** | Increases the hub binary beyond ~80 KB; risks approaching Soroban contract size limits. | Hub remains compact; standalone contract has its own independent WASM budget. |
| **Resource & Instruction Budgets** | Shared execution budget. Complex multi-step operations risk hitting transaction budget ceilings (Issue #357). | Metered independently across invocations. |
| **Storage Compatibility** | Must ensure DataKey variants and persistent entries never collide with existing hub state. | Complete storage isolation; zero risk of state namespace collision. |
| **Upgradeability & Maintenance** | Requires upgrading the entire hub contract to patch or modify the module. | Can be upgraded, paused, or retired independently without touching the hub. |

### Technical Risks to Verify:
1. **Budget Checks (Issue #357):** Soroban enforces strict CPU instruction and memory limits per invocation. Restoring large modules increases code size and compilation overhead. Verify that transactions remain well below simulation budget limits (Error(Budget, ExceededLimit)).
2. **Storage Layout:** Ensure DataKey enum discriminant numbering is backward-compatible with deployed hubs. Never reorder or remove existing DataKey variants.

---

## 4. Alternative: Deploying Standalone Example Contracts

In most production environments, deploying the standalone contract (e.g. example/escrow) alongside the hub is the superior architectural choice:

* **When to deploy standalone:** Recommended for almost all integrations. Standalone contracts communicate with shade via client coordination or cross-contract calls (Env::invoke_contract), keeping payment processing decoupled from escrow or subscription state machines.
* **When to restore into the hub:** Only when atomic synchronous state changes within a single contract invocation are strictly required, or when contract-to-contract call latency/gas overhead must be minimized.

---

## 5. References and Related Documentation

- [ADR 0007: Limit the workspace to the core payment gateway](../../docs/architecture/decisions/0007-core-gateway-scope.md)
- [Example Contracts Directory Overview](../README.md)
- [Threat Model and Known Limitations](../../docs/security/threat-model.md#known-limitations)
