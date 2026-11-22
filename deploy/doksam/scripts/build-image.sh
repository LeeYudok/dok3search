#!/usr/bin/env bash
# Build the dok3search image for the doksam node (linux/amd64).
#
#   deploy/doksam/scripts/build-image.sh
#
# The tag is derived from the current commit SHA (`dok3search:sha-<git-sha>`) so
# the same commit always produces the same tag and the imported image is
# immutable. Nothing is pushed to a registry. Requires podman (or docker):
#   CONTAINER_ENGINE=docker deploy/doksam/scripts/build-image.sh
# For a remote podman service, pass all engine arguments in one variable:
#   CONTAINER_ENGINE='podman --connection podman-machine-default' \
#     deploy/doksam/scripts/build-image.sh
set -euo pipefail

# shellcheck source-path=SCRIPTDIR
# shellcheck source=lib.sh
. "$(cd "$(dirname "$0")" && pwd)/lib.sh"

IMAGE="$(image_ref)"

if [ -z "${GIT_SHA:-}" ] && [ -n "$(git -C "$REPO_ROOT" status --porcelain --untracked-files=normal)" ]; then
  echo "error: image tags are commit SHA based; commit the worktree or set GIT_SHA for an explicit tag" >&2
  git -C "$REPO_ROOT" status --short >&2
  exit 1
fi

echo "== building $IMAGE for linux/amd64 with $CONTAINER_ENGINE =="
echo "   context: $REPO_ROOT"

"${CONTAINER_ENGINE_CMD[@]}" build --platform linux/amd64 -t "$IMAGE" "$REPO_ROOT"

"${CONTAINER_ENGINE_CMD[@]}" image inspect "$IMAGE" >/dev/null
echo "== built $IMAGE =="
printf '%s\n' "$IMAGE" > "$DEPLOY_DIR/.last-build-image"
