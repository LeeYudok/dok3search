# cargo-rustdoc(1)

## NAME

cargo-rustdoc — Build a package’s documentation, using specified custom flags

## SYNOPSIS

`cargo rustdoc` \[*options*\] \[`--` *args*\]

## DESCRIPTION

The specified target for the current package (or package specified by `-p` if provided) will be documented with the specified *args* being passed to the final rustdoc invocation. Dependencies will not be documented as part of this command. Note that rustdoc will still unconditionally receive arguments such as `-L`, `--extern`, and `--crate-type`, and the specified *args* will simply be added to the rustdoc invocation.

See  for documentation on rustdoc flags.

This command requires that only one target is being compiled when additional arguments are provided. If more than one target is available for the current package the filters of `--lib`, `--bin`, etc, must be used to select which target is compiled.

To pass flags to all rustdoc processes spawned by Cargo, use the `RUSTDOCFLAGS` [environment variable](../reference/environment-variables.md) or the `build.rustdocflags` [config value](../reference/config.md).

## OPTIONS

### Documentation Options

[--open](#option-cargo-rustdoc---open)  
Open the docs in a browser after building them. This will use your default browser unless you define another one in the `BROWSER` environment variable or use the [`doc.browser`](../reference/config.md#docbrowser) configuration option.

### Package Selection

By default, the package in the current working directory is selected. The `-p` flag can be used to choose a different package in a workspace.

[-p spec](#option-cargo-rustdoc--p)\
[--package spec](#option-cargo-rustdoc---package)  
The package to document. See [cargo-pkgid(1)](cargo-pkgid.md) for the SPEC format.

### Target Selection

When no target selection options are given, `cargo rustdoc` will document all binary and library targets of the selected package. The binary will be skipped if its name is the same as the lib target. Binaries are skipped if they have `required-features` that are missing.

Passing target selection flags will document only the specified targets.

Note that `--bin`, `--example`, `--test` and `--bench` flags also support common Unix glob patterns like `*`, `?` and `[]`. However, to avoid your shell accidentally expanding glob patterns before Cargo handles them, you must use single quotes or double quotes around each glob pattern.

[--lib](#option-cargo-rustdoc---lib)  
Document the package’s library.

[--bin name…](#option-cargo-rustdoc---bin)  
Document the specified binary. This flag may be specified multiple times and supports common Unix glob patterns.

[--bins](#option-cargo-rustdoc---bins)  
Document all binary targets.

[--example name…](#option-cargo-rustdoc---example)  
Document the specified example. This flag may be specified multiple times and supports common Unix glob patterns.

[--examples](#option-cargo-rustdoc---examples)  
Document all example targets.

[--test name…](#option-cargo-rustdoc---test)  
Document the specified integration test. This flag may be specified multiple times and supports common Unix glob patterns.

[--tests](#option-cargo-rustdoc---tests)  
Document all targets that have the `test = true` manifest flag set. By default this includes the library and binaries built as unittests, and integration tests. Be aware that this will also build any required dependencies, so the lib target may be built twice (once as a unittest, and once as a dependency for binaries, integration tests, etc.). Targets may be enabled or disabled by setting the `test` flag in the manifest settings for the target.

[--bench name…](#option-cargo-rustdoc---bench)  
Document the specified benchmark. This flag may be specified multiple times and supports common Unix glob patterns.

[--benches](#option-cargo-rustdoc---benches)  
Document all targets that have the `bench = true` manifest flag set. By default this includes the library and binaries built as benchmarks, and bench targets. Be aware that this will also build any required dependencies, so the lib target may be built twice (once as a benchmark, and once as a dependency for binaries, benchmarks, etc.). Targets may be enabled or disabled by setting the `bench` flag in the manifest settings for the target.

[--all-targets](#option-cargo-rustdoc---all-targets)  
Document all targets. This is equivalent to specifying `--lib --bins --tests --benches --examples`.

### Feature Selection

The feature flags allow you to control which features are enabled. When no feature options are given, the `default` feature is activated for every selected package.

See [the features documentation](../reference/features.md#command-line-feature-options) for more details.

[-F features](#option-cargo-rustdoc--F)\
[--features features](#option-cargo-rustdoc---features)  
Space or comma separated list of features to activate. Features of workspace members may be enabled with `package-name/feature-name` syntax. This flag may be specified multiple times, which enables all specified features.

[--all-features](#option-cargo-rustdoc---all-features)  
Activate all available features of all selected packages.

[--no-default-features](#option-cargo-rustdoc---no-default-features)  
Do not activate the `default` feature of the selected packages.

### Compilation Options

[--target tuple](#option-cargo-rustdoc---target)  
Document for the specified target architecture. Flag may be specified multiple times. The default is the host architecture. The general format of the tuple is `<arch><sub>-<vendor>-<sys>-<abi>`.

Possible values:

- Any supported target in `rustc --print target-list`.
- `"host-tuple"`, which will internally be substituted by the host’s target. This can be particularly useful if you’re cross-compiling some crates, and don’t want to specify your host’s machine as a target (for instance, an `xtask` in a shared project that may be worked on by many hosts).
- A path to a custom target specification. See [Custom Target Lookup Path](../../rustc/targets/custom.html#custom-target-lookup-path) for more information.

This may also be specified with the `build.target` [config value](../reference/config.md).

Note that specifying this flag makes Cargo run in a different mode where the target artifacts are placed in a separate directory. See the [build cache](../reference/build-cache.md) documentation for more details.

[-r](#option-cargo-rustdoc--r)\
[--release](#option-cargo-rustdoc---release)  
Document optimized artifacts with the `release` profile. See also the `--profile` option for choosing a specific profile by name.

[--profile name](#option-cargo-rustdoc---profile)  
Document with the given profile. See [the reference](../reference/profiles.md) for more details on profiles.

[--timings](#option-cargo-rustdoc---timings)  
Output information how long each compilation takes, and track concurrency information over time.

A file `cargo-timing.html` will be written to the `target/cargo-timings` directory at the end of the build. An additional report with a timestamp in its filename is also written if you want to look at a previous run. These reports are suitable for human consumption only, and do not provide machine-readable timing data.

### Output Options

[--target-dir directory](#option-cargo-rustdoc---target-dir)  
Directory for all generated artifacts and intermediate files. May also be specified with the `CARGO_TARGET_DIR` environment variable, or the `build.target-dir` [config value](../reference/config.md). Defaults to `target` in the root of the workspace.

### Display Options

[-v](#option-cargo-rustdoc--v)\
[--verbose](#option-cargo-rustdoc---verbose)  
Use verbose output. May be specified twice for “very verbose” output which includes extra output such as dependency warnings and build script output. May also be specified with the `term.verbose` [config value](../reference/config.md).

[-q](#option-cargo-rustdoc--q)\
[--quiet](#option-cargo-rustdoc---quiet)  
Do not print cargo log messages. May also be specified with the `term.quiet` [config value](../reference/config.md).

[--color when](#option-cargo-rustdoc---color)  
Control when colored output is used. Valid values:

- `auto` (default): Automatically detect if color support is available on the terminal.
- `always`: Always display colors.
- `never`: Never display colors.

May also be specified with the `term.color` [config value](../reference/config.md).

[--message-format fmt](#option-cargo-rustdoc---message-format)  
The output format for diagnostic messages. Can be specified multiple times and consists of comma-separated values. Valid values:

- `human` (default): Display in a human-readable text format. Conflicts with `short` and `json`.
- `short`: Emit shorter, human-readable text messages. Conflicts with `human` and `json`.
- `json`: Emit JSON messages to stdout. See [the reference](../reference/external-tools.md#json-messages) for more details. Conflicts with `human` and `short`.
- `json-diagnostic-short`: Ensure the `rendered` field of JSON messages contains the “short” rendering from rustc. Cannot be used with `human` or `short`.
- `json-diagnostic-rendered-ansi`: Ensure the `rendered` field of JSON messages contains embedded ANSI color codes for respecting rustc’s default color scheme. Cannot be used with `human` or `short`.
- `json-render-diagnostics`: Instruct Cargo to not include rustc diagnostics in JSON messages printed, but instead Cargo itself should render the JSON diagnostics coming from rustc. Cargo’s own JSON diagnostics and others coming from rustc are still emitted. Cannot be used with `human` or `short`.

### Manifest Options

[--manifest-path path](#option-cargo-rustdoc---manifest-path)  
Path to the `Cargo.toml` file. By default, Cargo searches for the `Cargo.toml` file in the current directory or any parent directory.

[--ignore-rust-version](#option-cargo-rustdoc---ignore-rust-version)  
Ignore `rust-version` specification in packages.

[--locked](#option-cargo-rustdoc---locked)  
Asserts that the exact same dependencies and versions are used as when the existing `Cargo.lock` file was originally generated. Cargo will exit with an error when either of the following scenarios arises:

- The lock file is missing.
- Cargo attempted to change the lock file due to a different dependency resolution.

It may be used in environments where deterministic builds are desired, such as in CI pipelines.

[--offline](#option-cargo-rustdoc---offline)  
Prevents Cargo from accessing the network for any reason. Without this flag, Cargo will stop with an error if it needs to access the network and the network is not available. With this flag, Cargo will attempt to proceed without the network if possible.

Beware that this may result in different dependency resolution than online mode. Cargo will restrict itself to crates that are downloaded locally, even if there might be a newer version as indicated in the local copy of the index. See the [cargo-fetch(1)](cargo-fetch.md) command to download dependencies before going offline.

May also be specified with the `net.offline` [config value](../reference/config.md).

[--frozen](#option-cargo-rustdoc---frozen)  
Equivalent to specifying both `--locked` and `--offline`.

### Common Options

[+toolchain](#option-cargo-rustdoc-+toolchain)  
If Cargo has been installed with rustup, and the first argument to `cargo` begins with `+`, it will be interpreted as a rustup toolchain name (such as `+stable` or `+nightly`). See the [rustup documentation](https://rust-lang.github.io/rustup/overrides.html) for more information about how toolchain overrides work.

[--config KEY=VALUE or PATH](#option-cargo-rustdoc---config)  
Overrides a Cargo configuration value. The argument should be in TOML syntax of `KEY=VALUE`, or provided as a path to an extra configuration file. This flag may be specified multiple times. See the [command-line overrides section](../reference/config.md#command-line-overrides) for more information.

[-C PATH](#option-cargo-rustdoc--C)  
Changes the current working directory before executing any specified operations. This affects things like where cargo looks by default for the project manifest (`Cargo.toml`), as well as the directories searched for discovering `.cargo/config.toml`, for example. This option must appear before the command name, for example `cargo -C path/to/my-project build`.

This option is only available on the [nightly channel](https://doc.rust-lang.org/book/appendix-07-nightly-rust.html) and requires the `-Z unstable-options` flag to enable (see [\#10098](https://github.com/rust-lang/cargo/issues/10098)).

[-h](#option-cargo-rustdoc--h)\
[--help](#option-cargo-rustdoc---help)  
Prints help information.

[-Z flag](#option-cargo-rustdoc--Z)  
Unstable (nightly-only) flags to Cargo. Run `cargo -Z help` for details.

### Miscellaneous Options

[-j N](#option-cargo-rustdoc--j)\
[--jobs N](#option-cargo-rustdoc---jobs)  
Number of parallel jobs to run. May also be specified with the `build.jobs` [config value](../reference/config.md). Defaults to the number of logical CPUs. If negative, it sets the maximum number of parallel jobs to the number of logical CPUs plus provided value. If a string `default` is provided, it sets the value back to defaults. Should not be 0.

[--keep-going](#option-cargo-rustdoc---keep-going)  
Build as many crates in the dependency graph as possible, rather than aborting the build on the first one that fails to build.

For example if the current package depends on dependencies `fails` and `works`, one of which fails to build, `cargo rustdoc -j1` may or may not build the one that succeeds (depending on which one of the two builds Cargo picked to run first), whereas `cargo rustdoc -j1 --keep-going` would definitely run both builds, even if the one run first fails.

[--output-format](#option-cargo-rustdoc---output-format)  
The output type for the documentation emitted. Valid values:

- `html` (default): Emit the documentation in HTML format.
- `json`: Emit the documentation in the [experimental JSON format](https://doc.rust-lang.org/nightly/nightly-rustc/rustdoc_json_types).

This option is only available on the [nightly channel](https://doc.rust-lang.org/book/appendix-07-nightly-rust.html) and requires the `-Z unstable-options` flag to enable.

## ENVIRONMENT

See [the reference](../reference/environment-variables.md) for details on environment variables that Cargo reads.

## EXIT STATUS

- `0`: Cargo succeeded.
- `101`: Cargo failed to complete.

## EXAMPLES

1.  Build documentation with custom CSS included from a given file:

        cargo rustdoc --lib -- --extend-css extra.css

## SEE ALSO

[cargo(1)](cargo.md), [cargo-doc(1)](cargo-doc.md), [rustdoc(1)](https://doc.rust-lang.org/rustdoc/index.html)
