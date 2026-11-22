#!/usr/bin/env bash
# Smoke-test the dok3search container image without exposing a host port.
#
#   deploy/doksam/scripts/smoke-container.sh
#   SKIP_BUILD=1 deploy/doksam/scripts/smoke-container.sh
#   CONTAINER_ENGINE='podman --connection podman-machine-default' \
#     deploy/doksam/scripts/smoke-container.sh
#
# The server binary is the only runtime process under test. curl runs inside the
# freshly started container, so the same script works with a remote podman
# service: no local port-forward or host reachability is required.
set -euo pipefail

# shellcheck source-path=SCRIPTDIR
# shellcheck source=lib.sh
. "$(cd "$(dirname "$0")" && pwd)/lib.sh"

IMAGE="$(image_ref)"
NAME="dok3search-smoke-$$"

cleanup() {
  "${CONTAINER_ENGINE_CMD[@]}" rm -f "$NAME" >/dev/null 2>&1 || true
}
trap cleanup EXIT

if [ "${SKIP_BUILD:-0}" != "1" ]; then
  CONTAINER_ENGINE="$CONTAINER_ENGINE" bash "$DEPLOY_DIR/scripts/build-image.sh"
fi

echo "== starting $NAME from $IMAGE with $CONTAINER_ENGINE =="
"${CONTAINER_ENGINE_CMD[@]}" run --rm -d --name "$NAME" "$IMAGE" >/dev/null

check_path() {
  local path="$1"
  echo "== checking $path =="
  "${CONTAINER_ENGINE_CMD[@]}" exec "$NAME" sh -c "
    i=0
    while [ \"\$i\" -lt 30 ]; do
      if curl -fsS \"http://127.0.0.1:8080$path\" >/dev/null; then
        exit 0
      fi
      i=\$((i + 1))
      sleep 1
    done
    exit 1
  "
}

check_path /health/ready
check_path /health
check_path /

echo "container smoke OK: $IMAGE"
