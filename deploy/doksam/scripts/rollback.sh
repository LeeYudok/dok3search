#!/usr/bin/env bash
# Idempotent rollback of the dok3search image on the doksam node.
#
#   deploy/doksam/scripts/rollback.sh
#
# Scope: only the Deployment container image is reverted to the image recorded
# in the `dok3search.doksam/previous-image` annotation by the last deploy.sh
# run, then the rollout is waited on and verified. Namespace, Service, Ingress,
# Middleware, Secret and the PersistentVolumeClaim are left untouched, so the
# operational data (SQLite journal + Tantivy generations) is preserved.
#
# Running this twice is a no-op: the target is the annotation value and it is
# not mutated here. A forward re-deploy moves the annotation again.
set -euo pipefail

# shellcheck source-path=SCRIPTDIR
# shellcheck source=lib.sh
. "$(cd "$(dirname "$0")" && pwd)/lib.sh"

echo "== inspect current state =="
CURRENT="$(on_doksam "kubectl -n $NAMESPACE get deploy $DEPLOYMENT -o jsonpath='{.spec.template.spec.containers[0].image}' 2>/dev/null || true")"
PREVIOUS="$(on_doksam "kubectl -n $NAMESPACE get deploy $DEPLOYMENT -o jsonpath='{.metadata.annotations.dok3search\\.doksam/previous-image}' 2>/dev/null || true")"

echo "   current  image: ${CURRENT:-<none>}"
echo "   rollback image: ${PREVIOUS:-<none>}"

if [ -z "$PREVIOUS" ]; then
  echo "no previous-image annotation recorded; nothing to roll back."
  exit 0
fi

if [ "$CURRENT" = "$PREVIOUS" ]; then
  echo "already on the rollback image; nothing to do."
else
  echo "== set image to $PREVIOUS =="
  on_doksam "kubectl -n $NAMESPACE set image deployment/$DEPLOYMENT $DEPLOYMENT='$PREVIOUS'"
  on_doksam "kubectl -n $NAMESPACE annotate deploy $DEPLOYMENT dok3search.doksam/deployed-image='$PREVIOUS' --overwrite >/dev/null"
fi

echo "== wait for rollout =="
on_doksam "kubectl -n $NAMESPACE rollout status deployment/$DEPLOYMENT --timeout=180s"

echo "== verify =="
bash "$DEPLOY_DIR/scripts/verify.sh"
