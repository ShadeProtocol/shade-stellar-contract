#![cfg(test)]

use soroban_sdk::Env;

use crate::components::reentrancy::{enter, exit};
use crate::errors::ContractError;
use crate::types::DataKey;

/// Test 1: Standard Execution
/// Calls enter() then exit() once — verifies the guard sets and clears correctly.
#[test]
fn test_enter_sets_reentrancy_flag() {
    let env = Env::default();

    enter(&env);

    // After enter, the ReentrancyStatus key should exist in storage
    assert!(
        env.storage().persistent().has(&DataKey::ReentrancyStatus),
        "ReentrancyStatus should be set after enter()"
    );

    exit(&env);
}

/// Test 2: Exit clears the flag
/// Verifies that exit() removes the ReentrancyStatus key from storage.
#[test]
fn test_exit_clears_reentrancy_flag() {
    let env = Env::default();

    enter(&env);
    exit(&env);

    // After exit, the ReentrancyStatus key should be gone
    assert!(
        !env.storage().persistent().has(&DataKey::ReentrancyStatus),
        "ReentrancyStatus should be cleared after exit()"
    );
}

/// Test 3: Blocked Reentrancy
/// Calls enter() twice without exit() in between — verifies the Reentrancy error is triggered.
#[test]
#[should_panic(expected = "Reentrancy")]
fn test_double_enter_triggers_reentrancy_error() {
    let env = Env::default();

    enter(&env); // First enter — should succeed
    enter(&env); // Second enter without exit — should panic with ContractError::Reentrancy
}

/// Test 4: State Reset After Successful Execution
/// Verifies that after a full enter -> exit cycle, the guard is reset
/// and a subsequent enter() call succeeds.
#[test]
fn test_sequential_calls_succeed_after_reset() {
    let env = Env::default();

    // First guarded call
    enter(&env);
    exit(&env);

    // Guard should be reset — second call should succeed without panic
    enter(&env);
    exit(&env);

    assert!(
        !env.storage().persistent().has(&DataKey::ReentrancyStatus),
        "ReentrancyStatus should be cleared after second exit()"
    );
}

/// Test 5: Multiple sequential calls all succeed
/// Runs enter/exit three times in a row to confirm the guard resets reliably.
#[test]
fn test_multiple_sequential_calls_succeed() {
    let env = Env::default();

    for _ in 0..3 {
        enter(&env);
        assert!(
            env.storage().persistent().has(&DataKey::ReentrancyStatus),
            "ReentrancyStatus should be set inside guarded section"
        );
        exit(&env);
        assert!(
            !env.storage().persistent().has(&DataKey::ReentrancyStatus),
            "ReentrancyStatus should be cleared after exit"
        );
    }
}

/// Test 6: Error Propagation / Storage Integrity
/// Verifies the ContractError::Reentrancy value is 4 as defined,
/// confirming the error enum is correctly wired.
#[test]
fn test_reentrancy_error_value() {
    assert_eq!(ContractError::Reentrancy as u32, 4);
}