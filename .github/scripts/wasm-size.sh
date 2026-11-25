#!/usr/bin/env bash
set -euo pipefail

BUDGET=".github/wasm-budget.json"

# measure <dir-with-wasm> <out.json>: optimize each contract and record its size in bytes
measure() {
  local dir=$1 out=$2 json='{}'
  for name in $(jq -r 'keys[]' "$BUDGET"); do
    stellar contract optimize --wasm "$dir/$name.wasm" >&2
    local size
    size=$(stat -c %s "$dir/$name.optimized.wasm")
    json=$(jq --arg n "$name" --argjson s "$size" '. + {($n): $s}' <<<"$json")
  done
  echo "$json" > "$out"
}

# check <head.json> [base.json]: print a table, fail if over budget
check() {
  local head=$1 base=${2:-} fail=0
  local md="| Contract | Size | Budget | Used | vs base |\n|---|---|---|---|---|\n"
  for name in $(jq -r 'keys[]' "$BUDGET"); do
    local size budget pct delta="n/a"
    size=$(jq -r --arg n "$name" '.[$n]' "$head")
    budget=$(jq -r --arg n "$name" '.[$n]' "$BUDGET")
    pct=$(( size * 100 / budget ))
    if [ -n "$base" ] && [ -f "$base" ]; then
      local b
      b=$(jq -r --arg n "$name" '.[$n] // empty' "$base")
      [ -n "$b" ] && delta=$(printf '%+d B (%+d%%)' $((size - b)) $(( (size - b) * 100 / b )))
    fi
    local status=""
    if [ "$size" -gt "$budget" ]; then status=" ❌ OVER BUDGET"; fail=1; fi
    md+="| $name | $size B | $budget B | ${pct}%$status | $delta |\n"
  done
  printf "### WASM size report\n\n%b" "$md" | tee -a "${GITHUB_STEP_SUMMARY:-/dev/null}"
  if [ "$fail" -eq 1 ]; then
    echo "::error::A contract exceeds its WASM size budget. See docs/wasm-budget.md to raise it deliberately."
    exit 1
  fi
}

"$@"
