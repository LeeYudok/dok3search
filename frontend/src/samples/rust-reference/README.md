# The Rust Reference

An offline Markdown copy of the Rust language reference installed with the Rust 1.100.0 toolchain (`0ef1bc386`, 2026-11-09). The searchable chapters follow the local mdBook table of contents; referenced local pages are included so chapter links resolve without a network connection.

The source is [The Rust Reference](https://doc.rust-lang.org/1.100.0/reference/) maintained in [rust-lang/reference](https://github.com/rust-lang/reference). `provenance.json` records the toolchain and SHA-256 digests of the HTML and image inputs. The material is distributed under either MIT or Apache License 2.0, at the recipient's option; see `COPYRIGHT`, `LICENSE-MIT`, and `LICENSE-APACHE`.

## Rebuild

Use the matching Rust toolchain's installed `rust-docs` component and Pandoc:

```sh
python3 scripts/import-rust-official-components.py \
  --rust-docs-root "$RUST_TOOLCHAIN/share/doc/rust/html" \
  --license-root "$RUST_TOOLCHAIN/share/doc/rust/licenses" \
  --output-root frontend/src/samples \
  --version '1.100.0 (0ef1bc386 2026-11-09)'
```

The importer creates both the Rust Reference and Cargo Book corpora.
