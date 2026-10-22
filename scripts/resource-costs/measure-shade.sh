#!/usr/bin/env bash
# Regenerates the Shade hub rows of docs/reference/resource-costs.md.
#
#   scripts/resource-costs/measure-shade.sh
#
# The harness is contracts/shade/tests/resource_costs.rs. When the `shade`
# crate builds on the current checkout it is run in place. When it does not,
# the script measures a snapshot of the last commit that did build (SNAPSHOT
# below) in a temporary git worktree, with snapshot-batch-auth.patch applied
# so pay_invoices_batch authorizes the payer once, as it does on main.
#
# Set CARGO_TARGET_DIR to reuse an existing build cache (a fresh worktree
# otherwise compiles every dependency from scratch).
set -euo pipefail

SNAPSHOT=4c97ca0
ROOT="$(git rev-parse --show-toplevel)"
HARNESS=contracts/shade/tests/resource_costs.rs
RUN=(cargo test -p shade --test resource_costs -- --nocapture --test-threads=1)

cd "$ROOT"
if cargo check -p shade --lib --quiet 2>/dev/null; then
  echo "# shade builds at $(git rev-parse --short HEAD); measuring in place" >&2
  "${RUN[@]}"
  exit
fi

echo "# shade does not build at $(git rev-parse --short HEAD); measuring snapshot $SNAPSHOT" >&2
WORK="$(mktemp -d)"
trap 'git worktree remove --force "$WORK/wt" 2>/dev/null || true; rm -rf "$WORK"' EXIT
git worktree add --detach --quiet "$WORK/wt" "$SNAPSHOT"
cp Cargo.lock "$WORK/wt/Cargo.lock"
mkdir -p "$WORK/wt/contracts/shade/tests"
cp "$HARNESS" "$WORK/wt/$HARNESS"
git -C "$WORK/wt" apply "$ROOT/scripts/resource-costs/snapshot-batch-auth.patch"
cd "$WORK/wt"
"${RUN[@]}"
