#!/bin/sh
# Build one air-gapped dependency bundle for the Rust workspace and npm frontend.
set -eu

usage() {
  cat <<'EOF'
Usage: scripts/offline-bundle.sh [--output-dir DIR]

Creates DIR/dok3search-offline-<UTC timestamp>.tar.gz and a matching
.sha256 manifest. DIR defaults to <repo>/offline-bundles.

The bundle contains:
  cargo/vendor/       cargo vendor output for backend/rust/Cargo.lock
  cargo/config.toml   receiver-side vendored-sources example
  npm/cache/          npm cache populated from frontend/package-lock.json
  bundle-info.txt     creator toolchain and platform metadata

Run this script on the same OS/architecture family as the receiver.
EOF
}

die() {
  printf '[offline-bundle] error: %s\n' "$1" >&2
  exit 1
}

REPO_ROOT=$(cd "$(dirname "$0")/.." && pwd)
OUTPUT_DIR="${REPO_ROOT}/offline-bundles"

while [ "$#" -gt 0 ]; do
  case "$1" in
    --output-dir)
      [ "$#" -ge 2 ] || die "--output-dir requires a value"
      OUTPUT_DIR=$2
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      usage >&2
      die "unknown argument: $1"
      ;;
  esac
done

command -v cargo >/dev/null 2>&1 || die "cargo is required"
command -v rustc >/dev/null 2>&1 || die "rustc is required"
command -v npm >/dev/null 2>&1 || die "npm is required"
command -v node >/dev/null 2>&1 || die "node is required"
command -v git >/dev/null 2>&1 || die "git is required"
command -v tar >/dev/null 2>&1 || die "tar is required"
if ! command -v sha256sum >/dev/null 2>&1 && ! command -v shasum >/dev/null 2>&1; then
  die "sha256sum or shasum is required"
fi

mkdir -p "$OUTPUT_DIR"
OUTPUT_DIR=$(cd "$OUTPUT_DIR" && pwd)

TIMESTAMP=$(date -u '+%Y%m%dT%H%M%SZ')
BUNDLE_NAME="dok3search-offline-${TIMESTAMP}"
ARCHIVE_NAME="${BUNDLE_NAME}.tar.gz"
ARCHIVE_PATH="${OUTPUT_DIR}/${ARCHIVE_NAME}"
MANIFEST_PATH="${ARCHIVE_PATH}.sha256"

[ ! -e "$ARCHIVE_PATH" ] || die "archive already exists: $ARCHIVE_PATH"

TMP_DIR=$(mktemp -d "${TMPDIR:-/tmp}/dok3search-offline.XXXXXX")
trap 'rm -rf "$TMP_DIR"' EXIT HUP INT TERM

STAGE_DIR="${TMP_DIR}/${BUNDLE_NAME}"
FRONTEND_STAGE="${TMP_DIR}/frontend-cache-seed"
mkdir -p "${STAGE_DIR}/cargo" "${STAGE_DIR}/npm" "$FRONTEND_STAGE"

printf '[offline-bundle] vendoring Rust dependencies from backend/rust/Cargo.lock\n'
CARGO_TERM_COLOR=never cargo vendor \
  --quiet \
  --locked \
  --manifest-path "${REPO_ROOT}/backend/rust/Cargo.toml" \
  "${STAGE_DIR}/cargo/vendor" >/dev/null

cat >"${STAGE_DIR}/cargo/config.toml" <<'EOF'
[source.crates-io]
replace-with = "vendored-sources"

[source.vendored-sources]
directory = "backend/rust/vendor"
EOF

printf '[offline-bundle] populating npm cache from frontend/package-lock.json\n'
cp "${REPO_ROOT}/frontend/package.json" "${REPO_ROOT}/frontend/package-lock.json" "$FRONTEND_STAGE/"
(
  cd "$FRONTEND_STAGE"
  npm ci \
    --cache "${STAGE_DIR}/npm/cache" \
    --ignore-scripts \
    --no-audit \
    --no-fund \
    --no-update-notifier \
    --loglevel=error
)

cat >"${STAGE_DIR}/bundle-info.txt" <<EOF
format=1
created_utc=${TIMESTAMP}
git_commit=$(git -C "$REPO_ROOT" rev-parse HEAD)
platform=$(uname -s)
arch=$(uname -m)
rustc=$(rustc --version)
cargo=$(cargo --version)
node=$(node --version)
npm=$(npm --version)
EOF

printf '[offline-bundle] creating archive\n'
tar -C "$TMP_DIR" -czf "$ARCHIVE_PATH" "$BUNDLE_NAME"

if command -v sha256sum >/dev/null 2>&1; then
  (
    cd "$OUTPUT_DIR"
    sha256sum "$ARCHIVE_NAME"
  ) >"$MANIFEST_PATH"
elif command -v shasum >/dev/null 2>&1; then
  (
    cd "$OUTPUT_DIR"
    shasum -a 256 "$ARCHIVE_NAME"
  ) >"$MANIFEST_PATH"
else
  die "sha256sum or shasum is required"
fi

printf '[offline-bundle] archive:  %s\n' "$ARCHIVE_PATH"
printf '[offline-bundle] manifest: %s\n' "$MANIFEST_PATH"
