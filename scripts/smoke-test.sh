#!/usr/bin/env bash
# Smoke tests for surfcontext.org
# Usage: bash scripts/smoke-test.sh <base-url>
set -euo pipefail

BASE="${1:?Usage: smoke-test.sh <base-url>}"
PASS=0
FAIL=0

check() {
  local desc="$1" url="$2" expected_status="${3:-200}" body_contains="${4:-}"
  local status

  status=$(curl -s -o /tmp/smoke-body -w '%{http_code}' "$url")

  if [ "$status" != "$expected_status" ]; then
    echo "FAIL: $desc — expected $expected_status, got $status"
    FAIL=$((FAIL + 1))
    return
  fi

  if [ -n "$body_contains" ] && ! grep -qi -- "$body_contains" /tmp/smoke-body; then
    echo "FAIL: $desc — body missing '$body_contains'"
    FAIL=$((FAIL + 1))
    return
  fi

  echo "PASS: $desc"
  PASS=$((PASS + 1))
}

echo "=== Smoke tests: $BASE ==="
echo ""

# Public pages
check "Landing page" "$BASE/" 200 "SurfContext"
check "Spec page" "$BASE/spec" 200 "ARDS"
check "Getting started" "$BASE/getting-started" 200 "SurfContext"
check "What's new" "$BASE/whats-new" 200
check "Talks index" "$BASE/talks" 200 "ARDS in Ten Minutes"
check "Talk deck" "$BASE/talks/ards-in-ten-minutes" 200 "data-slides"
check "Unknown talk 404" "$BASE/talks/no-such-talk" 404

# Static assets
check "CSS loads" "$BASE/static/css/app.css" 200 "surfdoc"
check "surf-ui CSS loads" "$BASE/static/css/surf-ui.css" 200

# 404 handling
check "404 on bad path" "$BASE/nonexistent-page-xyz" 404

echo ""
echo "=== Results: $PASS passed, $FAIL failed ==="

if [ "$FAIL" -gt 0 ]; then
  exit 1
fi
