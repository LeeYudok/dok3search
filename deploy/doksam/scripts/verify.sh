#!/usr/bin/env bash
# Verify the dok3search deployment through the public edge on doksam.
#
#   deploy/doksam/scripts/verify.sh
#
# Checks (status codes only, never credential values):
#   1. unauthenticated protected paths -> 401
#   2. authenticated health/SPA/API paths -> expected success
#   3. TLS certificate validates against the system trust store
#
# The password is read from the documented 0600 file on the doksam runtime (or
# from DOK3_BASIC_AUTH_PASS) and passed to curl through a temporary mode-0600
# config file; it is never printed and never appears in curl argv.
set -euo pipefail

# shellcheck source-path=SCRIPTDIR
# shellcheck source=lib.sh
. "$(cd "$(dirname "$0")" && pwd)/lib.sh"

USERNAME="${DOK3_BASIC_AUTH_USER:-dok3search}"
PASS="${DOK3_BASIC_AUTH_PASS:-}"
REMOTE_PASS_FILE="${DOK3_BASIC_AUTH_PASS_FILE:-/etc/dok3search/basic-auth.pass}"
CURL_CONFIG=""

if [ -z "$PASS" ]; then
  PASS="$(run_sudo_on_doksam cat "$REMOTE_PASS_FILE" 2>/dev/null || true)"
fi

if [ -z "$PASS" ]; then
  echo "error: no basicAuth password available; set DOK3_BASIC_AUTH_PASS or $REMOTE_PASS_FILE" >&2
  exit 1
fi

URL="https://$PUBLIC_HOST"
RESOLVE=("--resolve" "$PUBLIC_HOST:$HTTPS_PORT:$NODE_IP")

cleanup() {
  if [ -n "$CURL_CONFIG" ]; then
    rm -f "$CURL_CONFIG"
  fi
}
trap cleanup EXIT

curl_config_escape() {
  local value="$1"
  value="${value//\\/\\\\}"
  value="${value//\"/\\\"}"
  printf '%s' "$value"
}

TLS_ARGS=()
if [ "${TLS_INSECURE:-0}" = 1 ]; then
  TLS_ARGS=(-k)
fi

write_curl_config() {
  CURL_CONFIG="$(mktemp -t dok3search-curl.XXXXXX)"
  chmod 600 "$CURL_CONFIG"
  case "$USERNAME" in
    *$'\n'* | *$'\r'*)
      echo "error: basicAuth username must not contain newline characters" >&2
      exit 1
      ;;
  esac
  case "$PASS" in
    *$'\n'* | *$'\r'*)
      echo "error: basicAuth password must not contain newline characters" >&2
      exit 1
      ;;
  esac
  {
    printf 'user = "%s:%s"\n' \
      "$(curl_config_escape "$USERNAME")" \
      "$(curl_config_escape "$PASS")"
  } > "$CURL_CONFIG"
}

status() {
  # status <curl args...>  -> prints only the HTTP status code
  curl -sS "${TLS_ARGS[@]}" -o /dev/null -m 15 -w '%{http_code}\n' "$@"
}

auth_status() {
  status --config "$CURL_CONFIG" "$@"
}

echo "== verify via $URL (resolved to $NODE_IP:$HTTPS_PORT) =="

unauth_health="$(status "${RESOLVE[@]}" "$URL/health")"
echo "unauth  /health             -> $unauth_health (expect 401)"

unauth_ready="$(status "${RESOLVE[@]}" "$URL/health/ready")"
echo "unauth  /health/ready       -> $unauth_ready (expect 401)"

unauth_root="$(status "${RESOLVE[@]}" "$URL/")"
echo "unauth  /                   -> $unauth_root (expect 401)"

unauth_search="$(status "${RESOLVE[@]}" \
  -H 'content-type: application/json' \
  -d '{"query":"rust","filters":{},"page":1,"limit":5,"include_snippets":true}' \
  "$URL/api/v1/search")"
echo "unauth  POST /api/v1/search -> $unauth_search (expect 401)"

unauth_mcp="$(status "${RESOLVE[@]}" "$URL/mcp")"
echo "unauth  /mcp                -> $unauth_mcp (expect 401)"

unauth_admin="$(status "${RESOLVE[@]}" "$URL/admin")"
echo "unauth  /admin              -> $unauth_admin (expect 401)"

unauth_spa="$(status "${RESOLVE[@]}" "$URL/documents/deep-link")"
echo "unauth  /documents/deep-link -> $unauth_spa (expect 401)"

write_curl_config

auth_health="$(auth_status "${RESOLVE[@]}" "$URL/health/ready")"
echo "auth    /health/ready       -> $auth_health (expect 200)"

auth_health_alias="$(auth_status "${RESOLVE[@]}" "$URL/health")"
echo "auth    /health             -> $auth_health_alias (expect 200)"

auth_static="$(auth_status "${RESOLVE[@]}" "$URL/")"
echo "auth    /  (static React)   -> $auth_static (expect 200)"

auth_spa="$(auth_status "${RESOLVE[@]}" "$URL/documents/deep-link")"
echo "auth    /documents/deep-link -> $auth_spa (expect 200)"

auth_search="$(auth_status "${RESOLVE[@]}" \
  -H 'content-type: application/json' \
  -d '{"query":"rust","filters":{},"page":1,"limit":5,"include_snippets":true}' \
  "$URL/api/v1/search")"
echo "auth    POST /api/v1/search -> $auth_search (expect 200)"

fail=0
[ "$unauth_health" = "401" ] || fail=1
[ "$unauth_ready" = "401" ] || fail=1
[ "$unauth_root" = "401" ] || fail=1
[ "$unauth_search" = "401" ] || fail=1
[ "$unauth_mcp" = "401" ] || fail=1
[ "$unauth_admin" = "401" ] || fail=1
[ "$unauth_spa" = "401" ] || fail=1
[ "$auth_health" = "200" ] || fail=1
[ "$auth_health_alias" = "200" ] || fail=1
[ "$auth_static" = "200" ] || fail=1
[ "$auth_spa" = "200" ] || fail=1
[ "$auth_search" = "200" ] || fail=1
if [ "$fail" -ne 0 ]; then
  echo "verify FAILED" >&2
  exit 1
fi
echo "verify OK"
