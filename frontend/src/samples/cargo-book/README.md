# The Cargo Book

An offline Markdown copy of the Cargo Book installed with the Rust 1.100.0 toolchain (`0ef1bc386`, 2026-11-09). Chapters follow the local mdBook table of contents, and referenced local pages and images are bundled for offline reading.

The source is [The Cargo Book](https://doc.rust-lang.org/1.100.0/cargo/) maintained in [rust-lang/cargo](https://github.com/rust-lang/cargo). `provenance.json` records the toolchain and SHA-256 digests of the HTML and image inputs. Credential-shaped examples in the source text are redacted while preserving the surrounding documentation. The material is distributed under either MIT or Apache License 2.0, at the recipient's option; see `COPYRIGHT`, `LICENSE-MIT`, and `LICENSE-APACHE`.

Use the `scripts/import-rust-official-components.py` command documented in the Rust Reference README to rebuild both official Rust corpora from an installed `rust-docs` component.
