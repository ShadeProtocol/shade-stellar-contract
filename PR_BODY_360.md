# feat(crowdfund): financial penalties for malicious campaigns (#360)

Closes #360.

## Summary

Implements an opt-in, self-imposed financial-penalty mechanism for the `crowdfund`
contract that lets backers recover a slice of a successful campaign's payout if
the organizer is later judged to have acted maliciously. The organizer
voluntarily commits to a slashable rate (≤ 50%) before any pledge lands; backers
can then open a 7-day, pledge-weighted governance vote to accuse the campaign;
on majority approval, the slashed slice is detained on-chain at the moment of
`execute_campaign` / `release_milestone` and distributed pro-rata to backers
under claim — with a 6-month unclaimed sweep as a fallback.

## Why

Successful crowdfund campaigns have no recourse today if the organizer
disappears, never delivers, or acts in bad faith. Existing commitment devices
(refunds on failed goals, milestone voting) only protect against *delivery*
risk, not *malice* risk. This PR adds a credible threat of partial recovery
that does not require a centralized arbiter.

## Design highlights

- **Organizer signal, not enforcement.** `PenaltyBps` is set voluntarily by the
  organizer. It locks automatically after the first pledge lands so the
  disclosed rate is guaranteed for the entire campaign.
- **Anti-griefing floor.** Opening a malice report requires the reporter to
  hold at least 1 % of `Raised`, so a single sock-puppet cannot spam votes.
- **Front-running protection.** `execute_campaign` and `release_milestone`
  refuse to run while a malice vote window is still open — the organizer
  cannot race to withdraw funds before backers finalize the vote.
- **Majority rule, pledge-weighted.** Approval requires `votes > raised / 2`,
  using each backer's pledge as weight. Same governance model as the existing
  milestone voting.
- **Snapshot-based pro-rata.** `Raised` is frozen into `PenaltySnapshotRaised`
  at resolution, so claim math stays correct even if the campaign somehow
  accrues more pledges after the report.
- **Reentrancy-safe claims.** Storage is mutated *before* the cross-contract
  token transfer in `claim_penalty_refund`.
- **Sweep fallback.** 6 months after resolution, unclaimed penalty can be
  routed to a configurable recipient (treasury / governance); no funds are
  permanently stuck.

## Constants

| Constant | Value | Reason |
|---|---|---|
| `MAX_PENALTY_BPS` | 5 000 bps (50 %) | Self-imposed cap so an organizer cannot accidentally over-commit. |
| `PENALTY_VOTE_WINDOW` | 604 800 s (7 days) | Long enough for global backers to vote; short enough to keep capital actionable. |
| `MIN_REPORTER_PLEDGE_BPS` | 100 bps (1 %) | Low griefing floor. |
| `PENALTY_SWEEP_AFTER_SECS` | 15 778 800 s (~6 months) | Comfortable margin for slow backers before unclaimed funds are reusable. |

## Public API

### Configuration & status
| Function | Notes |
|---|---|
| `set_penalty_bps(bps)` | Organizer-only. ≤ `MAX_PENALTY_BPS`. Locked after first pledge. |
| `get_penalty_bps()` | Current rate. |
| `penalty_locked()` | True once a pledge has landed. |
| `is_malice_report_active()` | True when a vote window is open. |
| `is_penalty_approved()` | True once `resolve_malice_report` resolved in favour of backers. |
| `penalty_pool_balance()` | On-chain slashed balance awaiting backer claims. |
| `penalty_snapshot_raised()` | `Raised` at resolution time — claim denominator. |
| `malice_vote_start() / deadline() / reporter() / reason()` | Active-proposal metadata. |
| `penalty_vote_counts()` → `(approval, rejection)` | Live tally. |
| `backer_penalty_claimed(backer)` | Per-backer bookkeeping. |
| `penalty_sweep_unlock()` | Timestamp after which sweep becomes permissible. |
| `max_penalty_bps() / penalty_vote_window() / penalty_sweep_after()` | Static config. |

### Report, vote, resolution
| Function | Notes |
|---|---|
| `report_malicious(reporter, reason)` | Opens the 7-day vote window. Reporter must hold ≥ 1 % of `Raised` and may not be the organizer. |
| `vote_on_malice(voter, approve)` | One vote per backer per active window. Weight = backer pledge. |
| `resolve_malice_report()` | Callable by anyone once the window closes. Approved iff `approval > raised / 2`. Snapshots `Raised` and arms the sweep timer. |

### Payout integration (existing flows now penalty-aware)
- `execute_campaign()` — `compute_and_lock_penalty` deducts the slashed slice
  from the organizer payout when `is_penalty_approved` is true; no-op
  otherwise.
- `release_milestone(index)` — same `compute_and_lock_penalty` application per
  milestone slice, so a malicious milestone does not break the rest of the
  release schedule.

### Claim & sweep
| Function | Notes |
|---|---|
| `claim_penalty_refund(backer)` | Pro-rata transfer from the on-chain penalty pool. Idempotent per backer. |
| `sweep_unclaimed_penalty(recipient)` | Anyone may trigger after `penalty_sweep_unlock`. |

## Events

- `PenaltyConfiguredEvent { bps }`
- `MaliciousReportFiledEvent { reporter, reason, vote_deadline }`
- `MaliceVoteCastEvent { voter, approve, weight }`
- `MaliceReportResolvedEvent { approved, approval_weight, rejection_weight, snapshot_raised }`
- `PenaltySlashedEvent { amount, source, pool_balance }`
- `PenaltyRefundClaimedEvent { backer, amount, remaining_pool }`
- `PenaltySweptEvent { recipient, amount }`

## Errors added (`contracts/crowdfund/src/errors.rs`)

`PenaltyBpsInvalid`, `PenaltyLocked`, `MaliceReportActive`, `NoMaliceReport`,
`MaliceVoteWindowActive`, `MaliceVoteWindowExpired`, `PenaltyNotApproved`,
`InsufficientPenaltyPool`, `NotOrganizer`, `InsufficientReporterStake`,
`PenaltySweepTooEarly`, `NoPenaltyShareAvailable`, `PenaltyVoteAlreadyCast`.

## Files touched

```
contracts/crowdfund/src/lib.rs      +649 / -4
contracts/crowdfund/src/errors.rs    +26
contracts/crowdfund/src/test.rs     +412
```

## Tests added (`contracts/crowdfund/src/test.rs`)

Happy path:
- `test_set_penalty_bps_records_and_emits`
- `test_penalty_bps_locks_after_contribution`
- `test_report_malicious_opens_vote_window`
- `test_vote_on_malice_records_weight_once`
- `test_resolve_malice_approves_on_majority`
- `test_execute_campaign_keeps_full_payout_when_no_penalty`
- `test_execute_campaign_slashes_after_malice_approval` — full end-to-end flow including pro-rata backer claims
- `test_release_milestone_does_not_apply_penalty_when_unapproved`
- `test_release_milestone_slashes_after_malice_approval`
- `test_claim_penalty_refund_returns_no_share_when_pool_empty`
- `test_sweep_unclaimed_penalty_before_window_panics`

Negative / guard paths:
- `test_set_penalty_bps_above_max_panics`
- `test_set_penalty_bps_blocked_after_first_pledge`
- `test_report_malicious_requires_min_stake`
- `test_report_malicious_cannot_self_report`
- `test_vote_on_malice_double_vote_panics`
- `test_vote_on_malice_after_window_panics`
- `test_resolve_malice_requires_window_close`
- `test_claim_penalty_refund_before_approval_panics`
- `test_execute_campaign_blocked_during_vote_window`

## Security review notes

- All pledge-weighted transitions use `saturating_*` arithmetic — no overflow.
- `compute_and_lock_penalty` is the single source of truth for slashing math
  and is invoked at every payout site, so milestone mode and direct-execute
  mode stay consistent.
- The penalty pool is funded implicitly from the payout transfer; no extra
  `token::transfer` call is needed (the contract already holds the funds),
  so there is no extra cross-contract surface.
- `sweep_unclaimed_penalty` zeroes the pool by setting it to
  `PenaltyTotalClaimed`, not by resetting it to zero — this preserves the
  invariant that `pool >= total_claimed` even after sweep.

## Notes for reviewer

- No contract-upgrade migration is needed beyond a fresh deploy; all storage
  keys are additive under the existing `DataKey` enum.
- Snapshot of `Raised` taken at resolution (`PenaltySnapshotRaised`) keeps
  pro-rata math stable even if more pledges accrue before `execute_campaign`
  / `release_milestone` actually runs.
- Constants are exposed as view functions so off-chain tooling can read them
  without recompilation.
