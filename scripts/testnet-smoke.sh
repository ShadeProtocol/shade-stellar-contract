#!/usr/bin/env bash
set -euo pipefail

# End-to-end Stellar Testnet smoke test for Shade Protocol.
# Deploys a fresh stack with funded throwaway identities (or an optional
# TESTNET_DEPLOYER_SECRET env var, never printed) and exercises:
#   register_merchant -> deploy_merchant_account -> create_invoice ->
#   pay_invoice -> balance & fee routing checks -> refund_invoice_partial ->
#   analytics reads.

NETWORK="testnet"
INVOICE_AMOUNT="10000000"   # 1 XLM in stroops
FEE_BPS="500"               # 5% platform fee
EXPECTED_FEE="500000"       # 5% of 10_000_000
EXPECTED_NET="9500000"      # 10_000_000 - 500_000
REFUND_AMOUNT="2000000"     # 0.2 XLM partial refund
EXPECTED_AFTER_REFUND="7500000"

RUN_ID="$(date +%s)-$$"
ADMIN_ALIAS="smoke-admin-${RUN_ID}"
PLATFORM_ALIAS="smoke-platform-${RUN_ID}"
MERCHANT_ALIAS="smoke-merchant-${RUN_ID}"
PAYER_ALIAS="smoke-payer-${RUN_ID}"

SHADE_WASM="target/wasm32v1-none/release/shade.wasm"
ACCOUNT_WASM="target/wasm32v1-none/release/account.wasm"
SHADE_WASM_OPT="target/wasm32v1-none/release/shade.optimized.wasm"
ACCOUNT_WASM_OPT="target/wasm32v1-none/release/account.optimized.wasm"

cleanup() {
  for alias in "$ADMIN_ALIAS" "$PLATFORM_ALIAS" "$MERCHANT_ALIAS" "$PAYER_ALIAS"; do
    stellar keys rm "$alias" >/dev/null 2>&1 || true
  done
}
trap cleanup EXIT

strip_quotes() {
  local val="$1"
  val="${val%\"}"
  val="${val#\"}"
  printf '%s' "$val"
}

fund_with_retry() {
  local alias="$1"
  local attempts=5
  local delay=3
  for ((i = 1; i <= attempts; i++)); do
    if stellar keys fund "$alias" --network "$NETWORK" >/dev/null 2>&1; then
      return 0
    fi
    echo "Friendbot funding attempt ${i}/${attempts} for ${alias} failed; retrying in ${delay}s..."
    sleep "$delay"
    delay=$((delay * 2))
  done
  echo "Error: Failed to fund ${alias} on ${NETWORK} after ${attempts} attempts." >&2
  return 1
}

assert_eq() {
  local actual="$1"
  local expected="$2"
  local label="$3"
  if [[ "$actual" != "$expected" ]]; then
    echo "Assertion failed (${label}): expected '${expected}', got '${actual}'" >&2
    return 1
  fi
  echo "OK: ${label} = ${actual}"
}

echo "=== 1. Building and optimizing contracts ==="
cargo build --workspace --target wasm32v1-none --release
stellar contract optimize --wasm "$SHADE_WASM"
stellar contract optimize --wasm "$ACCOUNT_WASM"

echo "=== 2. Setting up funded testnet identities ==="
if [[ -n "${TESTNET_DEPLOYER_SECRET:-}" ]]; then
  # Import provided secret without echoing it to logs.
  set +x
  STELLAR_SECRET_KEY="$TESTNET_DEPLOYER_SECRET" stellar keys add "$ADMIN_ALIAS" --secret-key >/dev/null
  unset STELLAR_SECRET_KEY
  fund_with_retry "$ADMIN_ALIAS" || true
else
  stellar keys generate "$ADMIN_ALIAS" --network "$NETWORK" >/dev/null
  fund_with_retry "$ADMIN_ALIAS"
fi

for alias in "$PLATFORM_ALIAS" "$MERCHANT_ALIAS" "$PAYER_ALIAS"; do
  stellar keys generate "$alias" --network "$NETWORK" >/dev/null
  fund_with_retry "$alias"
done

ADMIN_ADDR="$(stellar keys address "$ADMIN_ALIAS")"
PLATFORM_ADDR="$(stellar keys address "$PLATFORM_ALIAS")"
MERCHANT_ADDR="$(stellar keys address "$MERCHANT_ALIAS")"
PAYER_ADDR="$(stellar keys address "$PAYER_ALIAS")"

echo "Admin:    ${ADMIN_ADDR}"
echo "Platform: ${PLATFORM_ADDR}"
echo "Merchant: ${MERCHANT_ADDR}"
echo "Payer:    ${PAYER_ADDR}"

echo "=== 3. Deploying fresh Shade stack ==="
SHADE_ID="$(strip_quotes "$(stellar contract deploy \
  --wasm "$SHADE_WASM_OPT" \
  --network "$NETWORK" \
  --source-account "$ADMIN_ALIAS")")"
echo "Shade contract ID: ${SHADE_ID}"

stellar contract invoke \
  --id "$SHADE_ID" \
  --network "$NETWORK" \
  --source-account "$ADMIN_ALIAS" \
  -- \
  initialize \
  --admin "$ADMIN_ADDR" >/dev/null

ACCOUNT_WASM_HASH="$(strip_quotes "$(stellar contract install \
  --wasm "$ACCOUNT_WASM_OPT" \
  --network "$NETWORK" \
  --source-account "$ADMIN_ALIAS")")"
echo "Account WASM hash: ${ACCOUNT_WASM_HASH}"

stellar contract invoke \
  --id "$SHADE_ID" \
  --network "$NETWORK" \
  --source-account "$ADMIN_ALIAS" \
  -- \
  set_account_wasm_hash \
  --admin "$ADMIN_ADDR" \
  --wasm_hash "$ACCOUNT_WASM_HASH" >/dev/null

TOKEN_ID="$(strip_quotes "$(stellar contract asset id --asset native --network "$NETWORK")")"
stellar contract asset deploy --asset native --network "$NETWORK" --source-account "$ADMIN_ALIAS" >/dev/null 2>&1 || true
echo "Test token (native XLM SAC): ${TOKEN_ID}"

stellar contract invoke \
  --id "$SHADE_ID" \
  --network "$NETWORK" \
  --source-account "$ADMIN_ALIAS" \
  -- \
  add_accepted_token \
  --admin "$ADMIN_ADDR" \
  --token "$TOKEN_ID" >/dev/null

stellar contract invoke \
  --id "$SHADE_ID" \
  --network "$NETWORK" \
  --source-account "$ADMIN_ALIAS" \
  -- \
  set_platform_account \
  --admin "$ADMIN_ADDR" \
  --account "$PLATFORM_ADDR" >/dev/null

stellar contract invoke \
  --id "$SHADE_ID" \
  --network "$NETWORK" \
  --source-account "$ADMIN_ALIAS" \
  -- \
  set_fee \
  --admin "$ADMIN_ADDR" \
  --token "$TOKEN_ID" \
  --fee "$FEE_BPS" >/dev/null

echo "=== 4. Registering merchant and deploying merchant account ==="
stellar contract invoke \
  --id "$SHADE_ID" \
  --network "$NETWORK" \
  --source-account "$MERCHANT_ALIAS" \
  -- \
  register_merchant \
  --merchant "$MERCHANT_ADDR" >/dev/null

MERCHANT_ACCOUNT_ID="$(strip_quotes "$(stellar contract invoke \
  --id "$SHADE_ID" \
  --network "$NETWORK" \
  --source-account "$MERCHANT_ALIAS" \
  -- \
  deploy_merchant_account \
  --merchant "$MERCHANT_ADDR")")"
echo "Merchant account ID: ${MERCHANT_ACCOUNT_ID}"

echo "=== 5. Creating and paying invoice ==="
INVOICE_ID="$(strip_quotes "$(stellar contract invoke \
  --id "$SHADE_ID" \
  --network "$NETWORK" \
  --source-account "$MERCHANT_ALIAS" \
  -- \
  create_invoice \
  --merchant "$MERCHANT_ADDR" \
  --description "testnet smoke test" \
  --amount "$INVOICE_AMOUNT" \
  --token "$TOKEN_ID")")"
echo "Created invoice ID: ${INVOICE_ID}"

PLATFORM_BAL_BEFORE="$(strip_quotes "$(stellar contract invoke \
  --id "$TOKEN_ID" \
  --network "$NETWORK" \
  -- \
  balance \
  --id "$PLATFORM_ADDR")")"

stellar contract invoke \
  --id "$SHADE_ID" \
  --network "$NETWORK" \
  --source-account "$PAYER_ALIAS" \
  -- \
  pay_invoice \
  --payer "$PAYER_ADDR" \
  --invoice_id "$INVOICE_ID" >/dev/null

echo "=== 6. Verifying merchant balance and fee routing ==="
MERCHANT_BAL="$(strip_quotes "$(stellar contract invoke \
  --id "$MERCHANT_ACCOUNT_ID" \
  --network "$NETWORK" \
  -- \
  get_balance \
  --token "$TOKEN_ID")")"
assert_eq "$MERCHANT_BAL" "$EXPECTED_NET" "merchant account balance after payment"

PLATFORM_BAL_AFTER="$(strip_quotes "$(stellar contract invoke \
  --id "$TOKEN_ID" \
  --network "$NETWORK" \
  -- \
  balance \
  --id "$PLATFORM_ADDR")")"
FEE_RECEIVED="$(python3 -c "print(int('${PLATFORM_BAL_AFTER}') - int('${PLATFORM_BAL_BEFORE}'))")"
assert_eq "$FEE_RECEIVED" "$EXPECTED_FEE" "platform fee routed"

INVOICE_JSON="$(stellar contract invoke \
  --id "$SHADE_ID" \
  --network "$NETWORK" \
  -- \
  get_invoice \
  --invoice_id "$INVOICE_ID")"
INVOICE_STATUS="$(python3 -c "import json,sys; d=json.loads(sys.argv[1]); print(d['status'])" "$INVOICE_JSON")"
assert_eq "$INVOICE_STATUS" "Paid" "invoice status after payment"

echo "=== 7. Executing partial refund ==="
stellar contract invoke \
  --id "$SHADE_ID" \
  --network "$NETWORK" \
  --source-account "$MERCHANT_ALIAS" \
  -- \
  refund_invoice_partial \
  --merchant "$MERCHANT_ADDR" \
  --invoice_id "$INVOICE_ID" \
  --amount "$REFUND_AMOUNT" >/dev/null

MERCHANT_BAL_AFTER_REFUND="$(strip_quotes "$(stellar contract invoke \
  --id "$MERCHANT_ACCOUNT_ID" \
  --network "$NETWORK" \
  -- \
  get_balance \
  --token "$TOKEN_ID")")"
assert_eq "$MERCHANT_BAL_AFTER_REFUND" "$EXPECTED_AFTER_REFUND" "merchant balance after partial refund"

INVOICE_AFTER_REFUND_JSON="$(stellar contract invoke \
  --id "$SHADE_ID" \
  --network "$NETWORK" \
  -- \
  get_invoice \
  --invoice_id "$INVOICE_ID")"
REFUND_STATUS="$(python3 -c "import json,sys; d=json.loads(sys.argv[1]); print(d['status'])" "$INVOICE_AFTER_REFUND_JSON")"
AMOUNT_REFUNDED="$(python3 -c "import json,sys; d=json.loads(sys.argv[1]); print(str(d['amount_refunded']))" "$INVOICE_AFTER_REFUND_JSON")"
assert_eq "$REFUND_STATUS" "PartiallyRefunded" "invoice status after partial refund"
assert_eq "$AMOUNT_REFUNDED" "$REFUND_AMOUNT" "invoice amount_refunded"

echo "=== 8. Reading analytics ==="
MERCHANT_ANALYTICS="$(stellar contract invoke \
  --id "$SHADE_ID" \
  --network "$NETWORK" \
  -- \
  get_merchant_analytics \
  --merchant "$MERCHANT_ADDR" \
  --token "$TOKEN_ID")"
MA_VOL="$(python3 -c "import json,sys; d=json.loads(sys.argv[1]); print(str(d['total_volume']))" "$MERCHANT_ANALYTICS")"
MA_FEES="$(python3 -c "import json,sys; d=json.loads(sys.argv[1]); print(str(d['total_fees']))" "$MERCHANT_ANALYTICS")"
MA_TXS="$(python3 -c "import json,sys; d=json.loads(sys.argv[1]); print(str(d['transaction_count']))" "$MERCHANT_ANALYTICS")"
assert_eq "$MA_VOL" "$INVOICE_AMOUNT" "merchant analytics total_volume"
assert_eq "$MA_FEES" "$EXPECTED_FEE" "merchant analytics total_fees"
assert_eq "$MA_TXS" "1" "merchant analytics transaction_count"

MERCHANT_SUMMARY="$(stellar contract invoke \
  --id "$SHADE_ID" \
  --network "$NETWORK" \
  -- \
  get_merchant_analytics_summary \
  --merchant "$MERCHANT_ADDR")"
MS_VOL="$(python3 -c "import json,sys; d=json.loads(sys.argv[1]); print(str(d['total_volume']))" "$MERCHANT_SUMMARY")"
assert_eq "$MS_VOL" "$INVOICE_AMOUNT" "merchant analytics summary total_volume"

TOKEN_ANALYTICS="$(stellar contract invoke \
  --id "$SHADE_ID" \
  --network "$NETWORK" \
  -- \
  get_token_analytics \
  --token "$TOKEN_ID")"
TA_VOL="$(python3 -c "import json,sys; d=json.loads(sys.argv[1]); print(str(d['total_volume']))" "$TOKEN_ANALYTICS")"
assert_eq "$TA_VOL" "$INVOICE_AMOUNT" "token analytics total_volume"

echo "=== Testnet smoke test passed! ==="
