#!/usr/bin/env bash
# Shared helpers for the dok3search doksam deployment scripts.
# Sourced by the other scripts; never prints secrets.
#
# All kubectl / ctr operations run on the doksam runtime. Set HOST to the ssh
# alias for that machine (default `doksam`), or HOST=local to run in place on a
# box that already talks to the doksam cluster.

set -euo pipefail

DEPLOY_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
REPO_ROOT="$(cd "$DEPLOY_DIR/../.." && pwd)"

NAMESPACE="${NAMESPACE:-dok3search}"
DEPLOYMENT="${DEPLOYMENT:-dok3search}"
IMAGE_REPO="${IMAGE_REPO:-dok3search}"

# doksam runtime access. HOST=local runs the commands in the current shell.
HOST="${HOST:-doksam}"
SSH_OPTS=(-o ControlMaster=auto -o "ControlPath=$HOME/.ssh/cm-dok3search-%C" -o ControlPersist=600)

# Public edge + node address used for the Host-based verification.
PUBLIC_HOST="${PUBLIC_HOST:-www.doksam.com}"
NODE_IP="${NODE_IP:-192.168.113.220}"
HTTPS_PORT="${HTTPS_PORT:-443}"

# Container engine used by build-image.sh (podman or docker).
CONTAINER_ENGINE="${CONTAINER_ENGINE:-podman}"
IFS=' ' read -r -a CONTAINER_ENGINE_CMD <<< "$CONTAINER_ENGINE"
if [ "${#CONTAINER_ENGINE_CMD[@]}" -eq 0 ]; then
  echo "error: CONTAINER_ENGINE must name podman or docker" >&2
  exit 2
fi

BASIC_AUTH_SECRET="${BASIC_AUTH_SECRET:-dok3search-basic-auth}"
BASIC_AUTH_KEY="${BASIC_AUTH_KEY:-users}"

# Commit-SHA based, immutable image tag.
image_tag() {
  local sha
  if [ -n "${GIT_SHA:-}" ]; then
    if [[ ! "$GIT_SHA" =~ ^[0-9a-f]{40}$ ]]; then
      echo "error: GIT_SHA must be a 40-character lowercase commit SHA" >&2
      exit 2
    fi
    sha="$GIT_SHA"
  else
    sha="$(git -C "$REPO_ROOT" rev-parse HEAD)"
  fi
  printf 'sha-%s\n' "$sha"
}

image_ref() {
  printf '%s:%s\n' "$IMAGE_REPO" "$(image_tag)"
}

shell_quote() {
  printf '%q' "$1"
}

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

# Run a shell command on the doksam runtime (or locally when HOST=local/empty).
on_doksam() {
  if [ -z "$HOST" ] || [ "$HOST" = "local" ]; then
    bash -lc "$1"
  else
    # The command string is assembled locally and intentionally evaluated remotely.
    # shellcheck disable=SC2029
    ssh "${SSH_OPTS[@]}" "$HOST" "$1"
  fi
}

# Run an argv-style command on the doksam runtime without re-parsing it by hand.
on_doksam_args() {
  local command
  command="$(shell_join "$@")"
  on_doksam "$command"
}

# Run a command on the doksam runtime with a tty (interactive prompts).
on_doksam_tty() {
  if [ -z "$HOST" ] || [ "$HOST" = "local" ]; then
    bash -lc "$1"
  else
    ssh -t "${SSH_OPTS[@]}" "$HOST" "$1"
  fi
}

# Run a root command on the doksam runtime without putting SUDO_PASS in argv.
#
# The password is taken from the local SUDO_PASS environment when present, a
# local tty prompt, or the caller's stdin. For ssh hosts, stdin is forwarded to
# the remote helper; the helper also accepts a SUDO_PASS variable from its own
# environment. No password is echoed or passed as a command-line argument.
run_sudo_on_doksam() {
  local sudo_pass="${SUDO_PASS:-}"
  local sudo_command=""
  local script=""

  if [ "$#" -eq 0 ]; then
    echo "error: run_sudo_on_doksam requires a command" >&2
    return 2
  fi

  sudo_command="$(shell_join "$@")"
  script="$(cat <<'EOF'
set -euo pipefail
if [ -n "${SUDO_PASS:-}" ]; then
  sudo_pass="$SUDO_PASS"
else
  IFS= read -r sudo_pass || true
fi
if [ -z "${sudo_pass:-}" ]; then
  echo "error: SUDO_PASS must be set in the remote environment or supplied on stdin" >&2
  exit 2
fi
printf '%s\n' "$sudo_pass" | sudo -S -p ''
EOF
)"
  # sudo needs a separating space before the command; without one the command
  # text is appended straight after `-p ''` and parsed as sudo flags.
  script+=" ${sudo_command}"

  if [ -z "$sudo_pass" ] && [ -t 0 ]; then
    read -rsp "doksam sudo password: " sudo_pass
    echo
  fi

  if [ -z "$HOST" ] || [ "$HOST" = "local" ]; then
    if [ -n "$sudo_pass" ]; then
      printf '%s\n' "$sudo_pass" | bash -lc "$script"
    else
      bash -lc "$script"
    fi
  else
    if [ -n "$sudo_pass" ]; then
      # The command payload is intentionally evaluated on the remote host.
      # shellcheck disable=SC2029
      printf '%s\n' "$sudo_pass" | ssh "${SSH_OPTS[@]}" "$HOST" "$script"
    else
      # The command payload is intentionally evaluated on the remote host.
      # shellcheck disable=SC2029
      ssh "${SSH_OPTS[@]}" "$HOST" "$script"
    fi
  fi
}

copy_to_doksam() {
  local src="$1" dest="$2"
  if [ -z "$HOST" ] || [ "$HOST" = "local" ]; then
    cp "$src" "$dest"
  else
    scp -q "${SSH_OPTS[@]}" "$src" "$HOST:$dest"
  fi
}
