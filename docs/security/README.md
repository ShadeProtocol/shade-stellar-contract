# Security

Threat model, access control, and audit-relevant notes for the Shade Protocol contracts.

- [Security Policy & Vulnerability Disclosure](../../SECURITY.md) — supported versions, private reporting channels, SLAs, bug bounty guidelines, and safe harbor.
- [Security audit readiness package](audit-package.md) — audit scope, architecture, documented risks, test instructions, and findings tracking.
- [Security review checklist for contract PRs](pr-review-checklist.md) — author and reviewer checks for PRs touching `contracts/`.
- [Threat model and security assumptions](threat-model.md) — assets, trust assumptions, attacker classes, attack surfaces, and known limitations.
- [Signed invoices, merchant keys, and signature verification](signatures.md) — off-chain signing scheme, nonce replay protection, and key rotation.
- [Access control and roles](access-control.md) — admin supremacy, Role enum, grant/revoke, permission matrix.
- [Pausable emergency-stop mechanism](pausable.md) — pause/unpause, blocked/allowed functions, operational playbook.
- [Reentrancy protection](reentrancy.md) — enter/exit guard, guarded functions, contributor rules.
- [Admin ownership and two-step admin transfer](admin-and-ownership.md) — initialization, propose/accept flow, key management.
- [Admin key management with Stellar multisig accounts](admin-key-management.md) — multisig setup, signing admin transactions, hardware wallet signers, two-step admin transfer to a multisig account, and signer rotation and loss procedures.
- [Verifying deployed contracts](verifying-deployments.md) — toolchain pinning, reproducible container builds, and on-chain WASM hash verification.
- [Data privacy and on-chain data exposure](data-privacy.md) — every public stored field and event payload, what never to put on-chain, GDPR implications, and linkability mitigations.

← [Back to documentation home](../README.md)
