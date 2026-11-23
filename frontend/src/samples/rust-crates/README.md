# Rust Workspace & Dependency Crates (`cargo doc`)

Offline Markdown generated from `backend/rust/target/doc` (1.100.0 (0ef1bc386 2026-11-09)).

## Rebuild

```sh
cargo doc --manifest-path backend/rust/Cargo.toml --workspace --no-deps
python3 scripts/import-rustdoc-and-errors.py --output-root frontend/src/samples
```
