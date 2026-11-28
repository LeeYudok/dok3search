#!/usr/bin/env bash
# Verify the dok3search deployment through the public edge on doksam.
#
#   deploy/doksam/scripts/verify.sh
#
# Checks:
#   1. unauthenticated health/SPA/search paths -> 200
#   2. GET /mcp -> 401
#   3. TLS certificate validates against the system trust store
set -euo pipefail

# shellcheck source-path=SCRIPTDIR
# shellcheck source=lib.sh
. "$(cd "$(dirname "$0")" && pwd)/lib.sh"

URL="https://$PUBLIC_HOST"
MCP_EXPECTED_STATUS="${MCP_EXPECTED_STATUS:-401}"
RESOLVE=("--resolve" "$PUBLIC_HOST:$HTTPS_PORT:$NODE_IP")

TLS_ARGS=()
if [ "${TLS_INSECURE:-0}" = 1 ]; then
  TLS_ARGS=(-k)
fi

status() {
  # status <curl args...>  -> prints only the HTTP status code
  curl -sS "${TLS_ARGS[@]}" -o /dev/null -m 15 -w '%{http_code}\n' "$@"
}

echo "== verify via $URL (resolved to $NODE_IP:$HTTPS_PORT) =="

s_ready="$(status "${RESOLVE[@]}" "$URL/health/ready")"
echo "GET  /health/ready         -> $s_ready (expect 200)"

s_health="$(status "${RESOLVE[@]}" "$URL/health")"
echo "GET  /health               -> $s_health (expect 200)"

s_root="$(status "${RESOLVE[@]}" "$URL/")"
echo "GET  /  (static React)     -> $s_root (expect 200)"

s_spa="$(status "${RESOLVE[@]}" "$URL/documents/deep-link")"
echo "GET  /documents/deep-link  -> $s_spa (expect 200)"

s_search="$(status "${RESOLVE[@]}" \
  -H 'content-type: application/json' \
  -d '{"query":"rust","filters":{},"page":1,"limit":5,"include_snippets":true}' \
  "$URL/api/v1/search")"
echo "POST /api/v1/search        -> $s_search (expect 200)"

s_mcp="$(status "${RESOLVE[@]}" "$URL/mcp")"
echo "GET  /mcp                  -> $s_mcp (expect $MCP_EXPECTED_STATUS)"

fail=0
[ "$s_ready" = "200" ] || fail=1
[ "$s_health" = "200" ] || fail=1
[ "$s_root" = "200" ] || fail=1
[ "$s_spa" = "200" ] || fail=1
[ "$s_search" = "200" ] || fail=1
[ "$s_mcp" = "$MCP_EXPECTED_STATUS" ] || fail=1
if [ "$fail" -ne 0 ]; then
  echo "verify FAILED" >&2
  exit 1
fi
echo "verify OK"
