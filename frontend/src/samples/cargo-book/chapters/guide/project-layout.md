# Package Layout

Cargo uses conventions for file placement to make it easy to dive into a new Cargo [package](../appendix/glossary.md#package ""package" (glossary entry)"):

``` text
.
├── Cargo.lock
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── main.rs
│   └── bin/
│       ├── named-executable.rs
│       ├── another-executable.rs
│       └── multi-file-executable/
│           ├── main.rs
│           └── some_module.rs
├── benches/
│   ├── large-input.rs
│   └── multi-file-bench/
│       ├── main.rs
│       └── bench_module.rs
├── examples/
│   ├── simple.rs
│   └── multi-file-example/
│       ├── main.rs
│       └── ex_module.rs
└── tests/
    ├── some-integration-tests.rs
    └── multi-file-test/
        ├── main.rs
        └── test_module.rs
```

- `Cargo.toml` and `Cargo.lock` are stored in the root of your package (*package root*).
- Source code goes in the `src` directory.
- The default library file is `src/lib.rs`.
- The default executable file is `src/main.rs`.
  - Other executables can be placed in `src/bin/`.
- Benchmarks go in the `benches` directory.
- Examples go in the `examples` directory.
- Integration tests go in the `tests` directory.

If a binary, example, bench, or integration test consists of multiple source files, place a `main.rs` file along with the extra [*modules*](../appendix/glossary.md#module ""module" (glossary entry)") within a subdirectory of the `src/bin`, `examples`, `benches`, or `tests` directory. The name of the executable will be the directory name.

> ![](../../assets/inline/project-layout-1.svg)Note
>
> By convention, binaries, examples, benches and integration tests follow `kebab-case` naming style, unless there are compatibility reasons to do otherwise (e.g. compatibility with a pre-existing binary name). Modules within those targets are `snake_case` following the [Rust standard](https://rust-lang.github.io/rfcs/0430-finalizing-naming-conventions.html).

You can learn more about Rust’s module system in [the book](https://doc.rust-lang.org/1.100.0/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html).

See [Configuring a target](../reference/cargo-targets.md#configuring-a-target) for more details on manually configuring targets. See [Target auto-discovery](../reference/cargo-targets.md#target-auto-discovery) for more information on controlling how Cargo automatically infers target names.
