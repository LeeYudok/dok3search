#!/usr/bin/env bash
# Idempotent deploy of dok3search to the doksam Kubernetes node.
#
#   deploy/doksam/scripts/deploy.sh
#
# Steps: build linux/amd64 image -> import into doksam containerd -> apply the
# manifest with the commit-SHA image -> record the previous image for rollback
# -> wait for rollout -> verify (200 public endpoints, 405 GET /mcp). Re-running
# with the same commit is a no-op; a new commit produces a new immutable tag and
# rolls the Deployment once. Operational data is never touched.
set -euo pipefail

# shellcheck source-path=SCRIPTDIR
# shellcheck source=lib.sh
. "$(cd "$(dirname "$0")" && pwd)/lib.sh"

IMAGE="$(image_ref)"
MANIFEST="$DEPLOY_DIR/k3s/dok3search.yaml"
REMOTE_TAR="/tmp/dok3search-image.$$.tar"

echo "== ensure namespace =="
on_doksam_args kubectl create namespace "$NAMESPACE" --dry-run=client -o yaml \
  | on_doksam_args kubectl apply -f - >/dev/null

echo "== image =="
CONTAINER_ENGINE="$CONTAINER_ENGINE" bash "$DEPLOY_DIR/scripts/build-image.sh"

echo "== export + import image into doksam containerd =="
SAVE_TAR="$(mktemp -t dok3search-image.XXXXXX.tar)"
cleanup() {
  rm -f "$SAVE_TAR"
}
trap cleanup EXIT

"${CONTAINER_ENGINE_CMD[@]}" save "$IMAGE" -o "$SAVE_TAR"
copy_to_doksam "$SAVE_TAR" "$REMOTE_TAR"
run_sudo_on_doksam ctr -n k8s.io images import "$REMOTE_TAR" >/dev/null
on_doksam_args rm -f "$REMOTE_TAR"

echo "== record previous image (rollback anchor) =="
if on_doksam_args kubectl -n "$NAMESPACE" get deploy "$DEPLOYMENT" >/dev/null 2>&1; then
  CURRENT="$(on_doksam_args kubectl -n "$NAMESPACE" get deploy "$DEPLOYMENT" -o 'jsonpath={.spec.template.spec.containers[0].image}')" || {
    echo "error: cannot read the current Deployment image; refusing to replace the rollback anchor" >&2
    exit 1
  }
else
  CURRENT=""
fi
echo "   running image: ${CURRENT:-<none>}"
echo "   target  image: $IMAGE"

echo "== apply manifest with $IMAGE =="
sed "s|image: dok3search:doksam|image: $IMAGE|" "$MANIFEST" | on_doksam "kubectl apply -f -"

on_doksam "kubectl -n $NAMESPACE annotate deploy $DEPLOYMENT dok3search.doksam/deployed-image='$IMAGE' --overwrite >/dev/null"
if [ -n "$CURRENT" ] && [ "$CURRENT" != "$IMAGE" ] && [ "$CURRENT" != "dok3search:doksam" ]; then
  on_doksam "kubectl -n $NAMESPACE annotate deploy $DEPLOYMENT dok3search.doksam/previous-image='$CURRENT' --overwrite >/dev/null"
  echo "   previous-image annotation set to $CURRENT (rollback target)"
elif [ -z "$CURRENT" ] || [ "$CURRENT" = "dok3search:doksam" ]; then
  on_doksam "kubectl -n $NAMESPACE annotate deploy $DEPLOYMENT dok3search.doksam/previous-image='$IMAGE' --overwrite >/dev/null"
  echo "   initial deployment: previous-image annotation seeded with $IMAGE"
fi

echo "== wait for rollout =="
on_doksam "kubectl -n $NAMESPACE rollout status deployment/$DEPLOYMENT --timeout=180s"

echo "== resources =="
on_doksam "kubectl -n $NAMESPACE get deploy,svc,ingress,pod -o wide"

echo "== verify =="
bash "$DEPLOY_DIR/scripts/verify.sh"
