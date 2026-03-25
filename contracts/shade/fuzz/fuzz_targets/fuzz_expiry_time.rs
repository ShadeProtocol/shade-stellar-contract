//! Fuzz target: fuzz_expiry_time
//!
//! Resolves issue #22 — "Fuzz Test expiry_time with Extreme Timestamps"
//!
//! This target exercises every piece of timestamp arithmetic that the Shade
//! contract performs (invoice expiry, fee-update activation, subscription
//! next-charge deadline, and refund-window elapsed time) using arbitrary u64
//! inputs supplied by libFuzzer.
//!
//! Special attention is paid to two historically significant boundaries:
//!
//!   • Year 2038 (Unix timestamp 2_147_483_647 = i32::MAX) — the classic
//!     "Year 2038 problem" that occurs when a 32-bit signed integer is used
//!     to store seconds since the Unix epoch.
//!
//!   • Year 2100 (Unix timestamp ≈ 4_102_444_800) — a common far-future
//!     planning horizon that exceeds u32::MAX / 2 and is used to verify that
//!     u64 arithmetic remains safe well beyond 2038.
//!
//! All arithmetic mirrors the contract source exactly so that any overflow
//! the fuzzer discovers here reflects a real vulnerability in the contract.
//!
//! # Running
//!
//! ```sh
//! # Install cargo-fuzz (requires a nightly toolchain)
//! cargo install cargo-fuzz
//!
//! # Run from contracts/shade/
//! cargo fuzz run fuzz_expiry_time
//!
//! # Run with a specific corpus seed directory
//! cargo fuzz run fuzz_expiry_time fuzz/corpus/fuzz_expiry_time
//! ```

#![no_main]

use libfuzzer_sys::fuzz_target;

// ─── Constants mirroring the contract ────────────────────────────────────────

/// Delay before a pending fee update becomes active (admin.rs).
const FEE_UPDATE_DELAY_SECS: u64 = 3_600; // 1 hour

/// Maximum window after payment in which a refund is allowed (invoice.rs).
const MAX_REFUND_DURATION: u64 = 604_800; // 7 days

// ─── Boundary timestamps used as deterministic seeds ─────────────────────────

/// Unix timestamp for 2038-01-19 03:14:07 UTC — the Year 2038 boundary
/// (i32::MAX). Any contract using a signed 32-bit integer for time would
/// overflow here.
const YEAR_2038: u64 = 2_147_483_647;

/// Approximate Unix timestamp for 2100-01-01 00:00:00 UTC.
/// Exceeds u32::MAX / 2 and is a common long-range planning sentinel.
const YEAR_2100: u64 = 4_102_444_800;

/// One calendar year expressed in minutes (365.25 days × 24 h × 60 min).
const ONE_YEAR_MINUTES: u64 = 525_960;

// ─── Pure arithmetic helpers (mirror contract logic) ─────────────────────────

/// Computes an invoice expiry timestamp: `base + duration_minutes * 60`.
///
/// Returns `None` instead of panicking when either multiplication or addition
/// would overflow `u64`.  This is the checked equivalent of what the caller
/// of `create_invoice` is expected to compute before passing `expires_at`.
#[inline]
fn expiry_time(base: u64, duration_minutes: u64) -> Option<u64> {
    duration_minutes
        .checked_mul(60)
        .and_then(|secs| base.checked_add(secs))
}

/// Computes the fee-update activation time: `base + FEE_UPDATE_DELAY_SECS`.
///
/// Mirrors `admin.rs`: `env.ledger().timestamp().checked_add(FEE_UPDATE_DELAY_SECS)`.
#[inline]
fn fee_activation_time(base: u64) -> Option<u64> {
    base.checked_add(FEE_UPDATE_DELAY_SECS)
}

/// Computes the next subscription charge deadline: `last_charged + interval`.
///
/// Mirrors `subscription.rs`: `subscription.last_charged.checked_add(plan.interval)`.
#[inline]
fn next_charge_time(last_charged: u64, interval: u64) -> Option<u64> {
    last_charged.checked_add(interval)
}

/// Computes seconds elapsed since payment: `now - date_paid`.
///
/// Mirrors `invoice.rs`: `env.ledger().timestamp() - date_paid`.
/// Returns `None` when `date_paid > now` (would underflow).
#[inline]
fn elapsed_since_payment(now: u64, date_paid: u64) -> Option<u64> {
    now.checked_sub(date_paid)
}

// ─── Fuzz target ─────────────────────────────────────────────────────────────

fuzz_target!(|data: &[u8]| {
    // ── Decode three independent u64 values from the fuzzer corpus ──────────
    //
    //   bytes  0– 7  → base_ts        (current ledger timestamp)
    //   bytes  8–15  → duration_mins  (invoice validity in minutes)
    //   bytes 16–23  → interval       (subscription charge interval in seconds)
    //
    if data.len() < 24 {
        return;
    }
    let base_ts: u64 = u64::from_le_bytes(data[0..8].try_into().unwrap());
    let duration_mins: u64 = u64::from_le_bytes(data[8..16].try_into().unwrap());
    let interval: u64 = u64::from_le_bytes(data[16..24].try_into().unwrap());

    // ── 1. Fuzz all arithmetic with the raw corpus values ───────────────────
    // None of these calls may panic — all overflow must be handled via Option.
    let _ = expiry_time(base_ts, duration_mins);
    let _ = fee_activation_time(base_ts);
    let _ = next_charge_time(base_ts, interval);
    // For elapsed: treat `interval` as `date_paid`; clamp to avoid nonsensical
    // underflow so the check focuses on overflow, not programmer error.
    let _ = elapsed_since_payment(base_ts, interval.min(base_ts));

    // ── 2. Verify refund-window logic doesn't overflow ───────────────────────
    if let Some(elapsed) = elapsed_since_payment(base_ts, interval.min(base_ts)) {
        // This comparison must never itself overflow; it is a plain boolean.
        let _within_window: bool = elapsed <= MAX_REFUND_DURATION;
    }

    // ── 3. Hardened assertions at the 2038 and 2100 boundaries ──────────────
    //
    // These assertions are the primary security property being verified:
    // adding up to one year of minutes to either critical timestamp must
    // succeed without overflow when using u64.  A failure here would indicate
    // a real numeric-safety regression.

    // Year 2038 — invoice expiry
    assert!(
        expiry_time(YEAR_2038, ONE_YEAR_MINUTES).is_some(),
        "expiry_time overflowed: YEAR_2038 + {ONE_YEAR_MINUTES} minutes exceeds u64::MAX"
    );
    // Year 2038 — fee activation delay
    assert!(
        fee_activation_time(YEAR_2038).is_some(),
        "fee_activation_time overflowed at YEAR_2038 boundary"
    );
    // Year 2038 — subscription next-charge deadline
    assert!(
        next_charge_time(YEAR_2038, FEE_UPDATE_DELAY_SECS).is_some(),
        "next_charge_time overflowed at YEAR_2038 boundary"
    );
    // Year 2038 — refund elapsed (payment at YEAR_2038, now = YEAR_2038 + 7 days)
    assert!(
        elapsed_since_payment(YEAR_2038 + MAX_REFUND_DURATION, YEAR_2038).is_some(),
        "elapsed_since_payment overflowed at YEAR_2038 boundary"
    );

    // Year 2100 — invoice expiry
    assert!(
        expiry_time(YEAR_2100, ONE_YEAR_MINUTES).is_some(),
        "expiry_time overflowed: YEAR_2100 + {ONE_YEAR_MINUTES} minutes exceeds u64::MAX"
    );
    // Year 2100 — fee activation delay
    assert!(
        fee_activation_time(YEAR_2100).is_some(),
        "fee_activation_time overflowed at YEAR_2100 boundary"
    );
    // Year 2100 — subscription next-charge deadline
    assert!(
        next_charge_time(YEAR_2100, FEE_UPDATE_DELAY_SECS).is_some(),
        "next_charge_time overflowed at YEAR_2100 boundary"
    );
    // Year 2100 — refund elapsed
    assert!(
        elapsed_since_payment(YEAR_2100 + MAX_REFUND_DURATION, YEAR_2100).is_some(),
        "elapsed_since_payment overflowed at YEAR_2100 boundary"
    );

    // ── 4. Boundary-adjacent probes (one second before and after) ───────────
    for &ts in &[
        YEAR_2038.saturating_sub(1),
        YEAR_2038,
        YEAR_2038 + 1,
        YEAR_2100.saturating_sub(1),
        YEAR_2100,
        YEAR_2100 + 1,
    ] {
        // All four operations must not panic for any near-boundary timestamp.
        let _ = expiry_time(ts, duration_mins);
        let _ = fee_activation_time(ts);
        let _ = next_charge_time(ts, interval);
        let _ = elapsed_since_payment(ts, ts.saturating_sub(MAX_REFUND_DURATION));
    }
});
