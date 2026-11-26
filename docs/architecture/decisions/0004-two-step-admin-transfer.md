# 0004. Two-step admin transfer

- **Status:** Accepted
- **Date:** 2026-10-22

## Context

The admin controls fees, pausing, upgrades, and whitelists. Transferring admin to a mistyped or unusable address would permanently lock those controls.

## Decision

Admin transfer is a two-step flow: the current admin nominates a pending admin via `propose_admin_transfer`, and the nominee must call `accept_admin_transfer` (with `require_auth`) before ownership changes. See [Admin and ownership](../../security/admin-and-ownership.md) and [Admin key management](../../security/admin-key-management.md) for guidance on holding the admin role in a multisig Stellar account or hardware wallet.

## Consequences

- A transfer cannot complete unless the new address can actually sign.
- The current admin can cancel or overwrite a pending nomination.
- Transfers take two transactions instead of one.
- A nominee that is a multisig account must collect the required signatures to call `accept_admin_transfer`, so the multisig setup must be verified before nomination. The multisig signer configuration, thresholds, and a test signing run should be confirmed as described in [Admin key management](../../security/admin-key-management.md) before `propose_admin_transfer` is called, since a nominee that cannot meet its threshold will leave the pending transfer stuck.

## Alternatives considered

- **Single-call transfer:** one typo away from losing admin permanently.
- **Multisig-only admin:** safer but forces every deployment into a multisig setup.
