#!/usr/bin/env bash
# Create or refresh the Traefik basicAuth Secret on the doksam cluster.
#
#   BASIC_AUTH_USER=dok3search deploy/doksam/scripts/create-basic-auth-secret.sh
#
# Generation happens on the doksam runtime: the htpasswd file and the plaintext
# 0600 password file stay there, and the Secret is created from the htpasswd
# file. No credential value is written to git or printed to the terminal.
#
# Non-interactive stdin input (use a mode-0600 file):
#   deploy/doksam/scripts/create-basic-auth-secret.sh < /path/to/0600-stdin
#
# The first stdin line is the basicAuth password; the optional second line is
# the doksam sudo password. The script also accepts the documented environment
# variable names, but credential values must not be placed on the command line.
#
# Documented locations (values are never recorded):
#   doksam host: /etc/dok3search/basic-auth.pass   (plaintext, mode 0600)
#   doksam host: /etc/dok3search/basic-auth.users  (htpasswd, mode 0600)
#   cluster:     Secret dok3search-basic-auth (namespace dok3search), key users
#
# Set HOST=local to run the helper directly on a host that already has kubectl
# pointed at the doksam cluster.
set -euo pipefail

# shellcheck source-path=SCRIPTDIR
# shellcheck source=lib.sh
. "$(cd "$(dirname "$0")" && pwd)/lib.sh"

REMOTE_HELPER="$DEPLOY_DIR/scripts/basic-auth-remote.sh"
USERNAME="${BASIC_AUTH_USER:-dok3search}"
PASSWORD="${DOK3_BASIC_AUTH_PASS:-}"
SUDO_PASS_VALUE="${SUDO_PASS:-}"

read_secret() {
  local name="$1"
  local value="$2"
  if [ -n "$value" ]; then
    printf '%s' "$value"
    return
  fi
  if [ -t 0 ]; then
    read -rsp "$name: " value
    echo
  else
    IFS= read -r value || true
  fi
  printf '%s' "$value"
}

if [ -z "$PASSWORD" ]; then
  PASSWORD="$(read_secret "basicAuth password for $USERNAME" "")"
fi
[ -n "$PASSWORD" ] || {
  echo "error: password must not be empty" >&2
  exit 1
}
case "$PASSWORD" in
  *$'\n'* | *$'\r'*)
    echo "error: basicAuth password must not contain newline characters" >&2
    exit 1
    ;;
esac

if [ -z "${DOK3_BASIC_AUTH_PASS:-}" ] && [ -t 0 ]; then
  CONFIRM="$(read_secret "confirm basicAuth password for $USERNAME" "")"
  [ "$PASSWORD" = "$CONFIRM" ] || {
    echo "error: passwords do not match" >&2
    exit 1
  }
  unset CONFIRM
fi

if [ -z "$SUDO_PASS_VALUE" ]; then
  if [ -t 0 ]; then
    SUDO_PASS_VALUE="$(read_secret "doksam sudo password" "")"
  else
    # Non-interactive stdin contract: first line is the basicAuth password,
    # second line (when present) is SUDO_PASS. If the remote environment
    # already provides SUDO_PASS, an EOF here is allowed and the helper will
    # consume the remote value instead.
    IFS= read -r SUDO_PASS_VALUE || true
  fi
fi
case "$SUDO_PASS_VALUE" in
  *$'\n'* | *$'\r'*)
    echo "error: SUDO_PASS must not contain newline characters" >&2
    exit 1
    ;;
esac

DEST="/tmp/dok3search-basic-auth-remote.$$.sh"
copy_to_doksam "$REMOTE_HELPER" "$DEST"

REMOTE_COMMAND="$(shell_join env \
  "NAMESPACE=$NAMESPACE" \
  "BASIC_AUTH_USER=$USERNAME" \
  "BASIC_AUTH_SECRET=$BASIC_AUTH_SECRET" \
  "BASIC_AUTH_KEY=$BASIC_AUTH_KEY" \
  bash "$DEST")"

set +e
{
  printf '%s\n' "$PASSWORD"
  if [ -n "$SUDO_PASS_VALUE" ]; then
    printf '%s\n' "$SUDO_PASS_VALUE"
  fi
} | on_doksam "$REMOTE_COMMAND"
RC=$?
set -e

on_doksam_args rm -f "$DEST" >/dev/null 2>&1 || true
unset PASSWORD SUDO_PASS_VALUE
exit "$RC"
