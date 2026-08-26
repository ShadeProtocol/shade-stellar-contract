# Stretch Goals Feature - Reconnaissance (Crowdfund)

## Entrypoints & Functions
- `set_stretch_goals(env: Env, milestones: Vec<i128>)` (Auth: Organizer)
  - Validates `milestones` are strictly increasing.
  - Stores them in `DataKey::StretchGoals`.
  - Bug found: It currently immediately publishes `StretchGoalReachedEvent` for all milestones when setting them!
- `check_stretch_goals(env: &Env, new_raised: i128)` (Internal)
  - Called automatically during pledging.
  - Checks if `new_raised >= threshold` for each milestone.
  - Marks `DataKey::StretchTriggered(idx)` as true.
  - Publishes `StretchGoalReachedEvent` for newly crossed milestones.

## State Transitions
- `DataKey::StretchGoals`: Set by `set_stretch_goals`.
- `DataKey::StretchTriggered(u32)`: Set to `true` by `check_stretch_goals` when crossed.

## Events
- `StretchGoalReachedEvent { milestone_index: u32, threshold: i128 }`
  - Emitted (incorrectly) in `set_stretch_goals`.
  - Emitted correctly in `check_stretch_goals` when threshold is met.

## Panics / Errors
- `NotInitialized`: `set_stretch_goals` called before contract is initialized (Organizer missing).
- `NotAuthorized`: caller is not the organizer.
- `InvalidGoal`: if milestones are not strictly increasing (`m <= prev`).

## Overflow Surfaces
- `milestones` elements are `i128`. Check against `i128::MAX`.
- `new_raised` could be large, boundary values at exact threshold, threshold-1, threshold+1.

## Plan
Create `contracts/crowdfund/src/tests/test_feature_205.rs`.
Test happy path (set goals, pledge to cross them, check state/events).
Test auth checks (non-organizer setting goals).
Test boundary conditions (overflow/underflow, off-by-one).
Test storage rollback on panic.
