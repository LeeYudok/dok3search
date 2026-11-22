#!/usr/bin/env bash
# Idempotent deploy of dok3search to the doksam Kubernetes node.
#
#   deploy/doksam/scripts/deploy.sh
#
# Steps: build linux/amd64 image -> import into doksam containerd -> apply the
# manifest with the commit-SHA image -> record the previous image for rollback
# -> wait for rollout -> verify (401 unauth / 200 auth). Re-running with the
# same commit is a no-op; a new commit produces a new immutable tag and rolls
# the Deployment once. Operational data is never touched.
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

if on_doksam_args kubectl -n "$NAMESPACE" get secret "$BASIC_AUTH_SECRET" >/dev/null 2>&1; then
  if [ -n "$(on_doksam_args kubectl -n "$NAMESPACE" get secret "$BASIC_AUTH_SECRET" -o "jsonpath={.data.$BASIC_AUTH_KEY}" 2>/dev/null || true)" ]; then
    echo "   basicAuth secret $BASIC_AUTH_SECRET present (key $BASIC_AUTH_KEY)"
  else
    echo "error: basicAuth secret $BASIC_AUTH_SECRET is missing key $BASIC_AUTH_KEY in namespace $NAMESPACE" >&2
    echo "       run deploy/doksam/scripts/create-basic-auth-secret.sh first" >&2
    exit 1
  fi
else
  echo "error: basicAuth secret $BASIC_AUTH_SECRET is missing in namespace $NAMESPACE" >&2
  echo "       run deploy/doksam/scripts/create-basic-auth-secret.sh first" >&2
  exit 1
fi

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
CURRENT="$(on_doksam "kubectl -n $NAMESPACE get deploy $DEPLOYMENT -o jsonpath='{.spec.template.spec.containers[0].image}' 2>/dev/null || true")"
echo "   running image: ${CURRENT:-<none>}"
echo "   target  image: $IMAGE"

echo "== apply manifest with $IMAGE =="
sed "s|image: dok3search:doksam|image: $IMAGE|" "$MANIFEST" | on_doksam "kubectl apply -f -"

on_doksam "kubectl -n $NAMESPACE annotate deploy $DEPLOYMENT dok3search.doksam/deployed-image='$IMAGE' --overwrite >/dev/null"
if [ -n "$CURRENT" ] && [ "$CURRENT" != "$IMAGE" ] && [ "$CURRENT" != "dok3search:doksam" ]; then
  on_doksam "kubectl -n $NAMESPACE annotate deploy $DEPLOYMENT dok3search.doksam/previous-image='$CURRENT' --overwrite >/dev/null"
  echo "   previous-image annotation set to $CURRENT (rollback target)"
fi

echo "== wait for rollout =="
on_doksam "kubectl -n $NAMESPACE rollout status deployment/$DEPLOYMENT --timeout=180s"

echo "== resources =="
on_doksam "kubectl -n $NAMESPACE get deploy,svc,ingress,pod -o wide"

echo "== verify =="
bash "$DEPLOY_DIR/scripts/verify.sh"
