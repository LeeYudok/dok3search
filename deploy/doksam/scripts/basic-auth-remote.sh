#!/usr/bin/env bash
# Runs ON the doksam runtime. Creates the Traefik basicAuth htpasswd file
# (mode 0600) and the Kubernetes Secret.
#
# The plaintext password lives only in the 0600 file and the Secret `users`
# key. Nothing here echoes the password or the htpasswd hash.
#
# Input contract (non-interactive):
#   - basicAuth password: DOK3_BASIC_AUTH_PASS or the first stdin line.
#   - sudo password:      SUDO_PASS from the environment or the next stdin line.
#
# Invoked by create-basic-auth-secret.sh; not meant to be run directly by hand
# unless you are already on the doksam node.
set -euo pipefail

NAMESPACE="${NAMESPACE:-dok3search}"
USERNAME="${BASIC_AUTH_USER:-dok3search}"
SECRET="${BASIC_AUTH_SECRET:-dok3search-basic-auth}"
KEY="${BASIC_AUTH_KEY:-users}"

DIR="/etc/dok3search"
PASS_FILE="$DIR/basic-auth.pass"
USERS_FILE="$DIR/basic-auth.users"

shell_join() {
  local out=""
  local arg=""
  local quoted=""
  for arg in "$@"; do
    printf -v quoted '%q' "$arg"
    if [ -n "$out" ]; then
      out+=" "
    fi
    out+="$quoted"
  done
  printf '%s' "$out"
}

sudo_run() {
  local sudo_pass="$1"
  shift
  local sudo_command=""
  local script=""

  if [ -z "$sudo_pass" ]; then
    echo "error: SUDO_PASS must be set in the environment or supplied on stdin" >&2
    exit 2
  fi

  sudo_command="$(shell_join "$@")"
  script="$(cat <<'EOF'
set -euo pipefail
IFS= read -r sudo_pass || true
if [ -z "${sudo_pass:-}" ]; then
  echo "error: SUDO_PASS must be set in the environment or supplied on stdin" >&2
  exit 2
fi
printf '%s\n' "$sudo_pass" | sudo -S -p ''
EOF
)"
  # sudo needs a separating space/newline before the command; without one the
  # command text is appended straight after `-p ''` and parsed as sudo flags.
  script+=" ${sudo_command}"
  printf '%s\n' "$sudo_pass" | bash -lc "$script"
}

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

P1="${DOK3_BASIC_AUTH_PASS:-}"
if [ -z "$P1" ]; then
  P1="$(read_secret "basicAuth password for $USERNAME" "")"
fi
[ -n "$P1" ] || { echo "error: password must not be empty" >&2; exit 1; }

if [ -z "${DOK3_BASIC_AUTH_PASS:-}" ] && [ -t 0 ]; then
  P2="$(read_secret "confirm basicAuth password for $USERNAME" "")"
  [ "$P1" = "$P2" ] || { echo "error: passwords do not match" >&2; exit 1; }
fi

SUDO_PASS_VALUE="${SUDO_PASS:-}"
if [ -z "$SUDO_PASS_VALUE" ]; then
  SUDO_PASS_VALUE="$(read_secret "doksam sudo password" "")"
fi
[ -n "$SUDO_PASS_VALUE" ] || {
  echo "error: SUDO_PASS must be set in the environment or supplied on stdin" >&2
  exit 2
}

if ! command -v openssl >/dev/null 2>&1; then
  echo "error: openssl is required to generate the basicAuth htpasswd line" >&2
  exit 1
fi

umask 077
TMP_DIR="$(mktemp -d -t dok3search-basic-auth.XXXXXX)"
TMP_PASS="$TMP_DIR/basic-auth.pass"
TMP_USERS="$TMP_DIR/basic-auth.users"
cleanup() {
  rm -rf "$TMP_DIR"
}
trap cleanup EXIT

printf '%s\n' "$P1" > "$TMP_PASS"
{
  printf '%s:' "$USERNAME"
  printf '%s\n' "$P1" | openssl passwd -apr1 -stdin
} > "$TMP_USERS"
chmod 600 "$TMP_PASS" "$TMP_USERS"
unset P1 P2

sudo_run "$SUDO_PASS_VALUE" install -d -m 700 -o root -g root "$DIR"
sudo_run "$SUDO_PASS_VALUE" install -m 600 -o root -g root "$TMP_PASS" "$PASS_FILE"
sudo_run "$SUDO_PASS_VALUE" install -m 600 -o root -g root "$TMP_USERS" "$USERS_FILE"
unset SUDO_PASS_VALUE

kubectl create namespace "$NAMESPACE" --dry-run=client -o yaml | kubectl apply -f - >/dev/null
kubectl -n "$NAMESPACE" create secret generic "$SECRET" \
  --from-file="$KEY=$TMP_USERS" \
  --dry-run=client -o yaml | kubectl apply -f - >/dev/null

echo "basicAuth secret refreshed: namespace=$NAMESPACE secret=$SECRET key=$KEY"
echo "0600 files: $PASS_FILE $USERS_FILE"
