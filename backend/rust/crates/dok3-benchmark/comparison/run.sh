#!/bin/sh
# Run the same benchmark harness against a Git baseline and the working tree.
set -eu

if [ "$#" -lt 1 ] || [ "$#" -gt 2 ]; then
    echo "Usage: $0 <baseline-commit> [korean]" >&2
    exit 2
fi

repo=$(git -C "$(dirname "$0")" rev-parse --show-toplevel)
baseline=$(git -C "$repo" rev-parse --verify "$1^{commit}")
comparison="$repo/backend/rust/crates/dok3-benchmark/comparison"
scratch=$(mktemp -d /tmp/dok3-tantivy-comparison.XXXXXX)
trap 'rm -rf "$scratch"' EXIT
suite=${2:-default}
export CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-/tmp/dok3-target-slice-a}

for variant in baseline candidate; do
    tree="$scratch/$variant"
    mkdir "$tree"
    git -C "$repo" archive --output="$tree/source.tar" "$baseline" backend/rust
    tar -xf "$tree/source.tar" -C "$tree"
    workspace="$tree/backend/rust"
    for crate in dok3-benchmark dok3-index dok3-analyzer; do
        if [ "$crate" = dok3-benchmark ] || {
            [ "$variant" = candidate ] && { [ "$suite" != korean ] || [ "$crate" = dok3-analyzer ]; }
        }; then
            rm -rf "$workspace/crates/$crate"
            cp -R "$repo/backend/rust/crates/$crate" "$workspace/crates/$crate"
        fi
    done
    benchmark="$workspace/crates/dok3-benchmark"
    mkdir -p "$benchmark/examples"
    cp "$comparison/tantivy.rs" "$benchmark/examples/tantivy_comparison.rs"
    # The comparison uses a direct Tantivy adapter dependency in this temporary
    # workspace. Cargo only updates the temporary lockfile.
    printf '\ndok3-index.workspace = true\n' >> "$benchmark/Cargo.toml"
    cargo build --quiet --offline --release --manifest-path "$workspace/Cargo.toml" \
        -p dok3-benchmark --example tantivy_comparison
    cp "$CARGO_TARGET_DIR/release/examples/tantivy_comparison" "$scratch/$variant-bin"
done

for variant in baseline candidate; do
    printf '%s\n' "$variant ($baseline)"
    "$scratch/$variant-bin" "$variant" "$suite"
done
