# cargo-rustc(1)

## NAME

cargo-rustc — Compile the current package, and pass extra options to the compiler

## SYNOPSIS

`cargo rustc` \[*options*\] \[`--` *args*\]

## DESCRIPTION

The specified target for the current package (or package specified by `-p` if provided) will be compiled along with all of its dependencies. The specified *args* will all be passed to the final compiler invocation, not any of the dependencies. Note that the compiler will still unconditionally receive arguments such as `-L`, `--extern`, and `--crate-type`, and the specified *args* will simply be added to the compiler invocation.

See  for documentation on rustc flags.

This command requires that only one target is being compiled when additional arguments are provided. If more than one target is available for the current package the filters of `--lib`, `--bin`, etc, must be used to select which target is compiled.

To pass flags to all compiler processes spawned by Cargo, use the `RUSTFLAGS` [environment variable](../reference/environment-variables.md) or the `build.rustflags` [config value](../reference/config.md).

## OPTIONS

### Package Selection

By default, the package in the current working directory is selected. The `-p` flag can be used to choose a different package in a workspace.

[-p spec](#option-cargo-rustc--p)\
[--package spec](#option-cargo-rustc---package)  
The package to build. See [cargo-pkgid(1)](cargo-pkgid.md) for the SPEC format.

### Target Selection

When no target selection options are given, `cargo rustc` will build all binary and library targets of the selected package.

Binary targets are automatically built if there is an integration test or benchmark being selected to build. This allows an integration test to execute the binary to exercise and test its behavior. The `CARGO_BIN_EXE_<name>` [environment variable](../reference/environment-variables.md#environment-variables-cargo-sets-for-crates) is set when the integration test is built and run so that it can use the [`env` macro](https://doc.rust-lang.org/std/macro.env.html) or the [`var` function](https://doc.rust-lang.org/std/env/fn.var.html) to locate the executable.

Passing target selection flags will build only the specified targets.

Note that `--bin`, `--example`, `--test` and `--bench` flags also support common Unix glob patterns like `*`, `?` and `[]`. However, to avoid your shell accidentally expanding glob patterns before Cargo handles them, you must use single quotes or double quotes around each glob pattern.

[--lib](#option-cargo-rustc---lib)  
Build the package’s library.

[--bin name…](#option-cargo-rustc---bin)  
Build the specified binary. This flag may be specified multiple times and supports common Unix glob patterns.

[--bins](#option-cargo-rustc---bins)  
Build all binary targets.

[--example name…](#option-cargo-rustc---example)  
Build the specified example. This flag may be specified multiple times and supports common Unix glob patterns.

[--examples](#option-cargo-rustc---examples)  
Build all example targets.

[--test name…](#option-cargo-rustc---test)  
Build the specified integration test. This flag may be specified multiple times and supports common Unix glob patterns.

[--tests](#option-cargo-rustc---tests)  
Build all targets that have the `test = true` manifest flag set. By default this includes the library and binaries built as unittests, and integration tests. Be aware that this will also build any required dependencies, so the lib target may be built twice (once as a unittest, and once as a dependency for binaries, integration tests, etc.). Targets may be enabled or disabled by setting the `test` flag in the manifest settings for the target.

[--bench name…](#option-cargo-rustc---bench)  
Build the specified benchmark. This flag may be specified multiple times and supports common Unix glob patterns.

[--benches](#option-cargo-rustc---benches)  
Build all targets that have the `bench = true` manifest flag set. By default this includes the library and binaries built as benchmarks, and bench targets. Be aware that this will also build any required dependencies, so the lib target may be built twice (once as a benchmark, and once as a dependency for binaries, benchmarks, etc.). Targets may be enabled or disabled by setting the `bench` flag in the manifest settings for the target.

[--all-targets](#option-cargo-rustc---all-targets)  
Build all targets. This is equivalent to specifying `--lib --bins --tests --benches --examples`.

### Feature Selection

The feature flags allow you to control which features are enabled. When no feature options are given, the `default` feature is activated for every selected package.

See [the features documentation](../reference/features.md#command-line-feature-options) for more details.

[-F features](#option-cargo-rustc--F)\
[--features features](#option-cargo-rustc---features)  
Space or comma separated list of features to activate. Features of workspace members may be enabled with `package-name/feature-name` syntax. This flag may be specified multiple times, which enables all specified features.

[--all-features](#option-cargo-rustc---all-features)  
Activate all available features of all selected packages.

[--no-default-features](#option-cargo-rustc---no-default-features)  
Do not activate the `default` feature of the selected packages.

### Compilation Options

[--target tuple](#option-cargo-rustc---target)  
Build for the specified target architecture. Flag may be specified multiple times. The default is the host architecture. The general format of the tuple is `<arch><sub>-<vendor>-<sys>-<abi>`.

Possible values:

- Any supported target in `rustc --print target-list`.
- `"host-tuple"`, which will internally be substituted by the host’s target. This can be particularly useful if you’re cross-compiling some crates, and don’t want to specify your host’s machine as a target (for instance, an `xtask` in a shared project that may be worked on by many hosts).
- A path to a custom target specification. See [Custom Target Lookup Path](https://doc.rust-lang.org/1.100.0/rustc/targets/custom.html#custom-target-lookup-path) for more information.

This may also be specified with the `build.target` [config value](../reference/config.md).

Note that specifying this flag makes Cargo run in a different mode where the target artifacts are placed in a separate directory. See the [build cache](../reference/build-cache.md) documentation for more details.

[-r](#option-cargo-rustc--r)\
[--release](#option-cargo-rustc---release)  
Build optimized artifacts with the `release` profile. See also the `--profile` option for choosing a specific profile by name.

[--profile name](#option-cargo-rustc---profile)  
Build with the given profile.

The `rustc` subcommand will treat the following named profiles with special behaviors:

- `check` — Builds in the same way as the [cargo-check(1)](cargo-check.md) command with the `dev` profile.
- `test` — Builds in the same way as the [cargo-test(1)](cargo-test.md) command, enabling building in test mode which will enable tests and enable the `test` cfg option. See [rustc tests](https://doc.rust-lang.org/rustc/tests/index.html) for more detail.
- `bench` — Builds in the same was as the [cargo-bench(1)](cargo-bench.md) command, similar to the `test` profile.

See [the reference](../reference/profiles.md) for more details on profiles.

[--timings](#option-cargo-rustc---timings)  
Output information how long each compilation takes, and track concurrency information over time.

A file `cargo-timing.html` will be written to the `target/cargo-timings` directory at the end of the build. An additional report with a timestamp in its filename is also written if you want to look at a previous run. These reports are suitable for human consumption only, and do not provide machine-readable timing data.

[--crate-type crate-type](#option-cargo-rustc---crate-type)  
Build for the given crate type. This flag accepts a comma-separated list of 1 or more crate types, of which the allowed values are the same as `crate-type` field in the manifest for configuring a Cargo target. See [`crate-type` field](../reference/cargo-targets.md#the-crate-type-field) for possible values.

If the manifest contains a list, and `--crate-type` is provided, the command-line argument value will override what is in the manifest.

This flag only works when building a `lib` or `example` library target.

### Output Options

[--target-dir directory](#option-cargo-rustc---target-dir)  
Directory for all generated artifacts and intermediate files. May also be specified with the `CARGO_TARGET_DIR` environment variable, or the `build.target-dir` [config value](../reference/config.md). Defaults to `target` in the root of the workspace.

### Display Options

[-v](#option-cargo-rustc--v)\
[--verbose](#option-cargo-rustc---verbose)  
Use verbose output. May be specified twice for “very verbose” output which includes extra output such as dependency warnings and build script output. May also be specified with the `term.verbose` [config value](../reference/config.md).

[-q](#option-cargo-rustc--q)\
[--quiet](#option-cargo-rustc---quiet)  
Do not print cargo log messages. May also be specified with the `term.quiet` [config value](../reference/config.md).

[--color when](#option-cargo-rustc---color)  
Control when colored output is used. Valid values:

- `auto` (default): Automatically detect if color support is available on the terminal.
- `always`: Always display colors.
- `never`: Never display colors.

May also be specified with the `term.color` [config value](../reference/config.md).

[--message-format fmt](#option-cargo-rustc---message-format)  
The output format for diagnostic messages. Can be specified multiple times and consists of comma-separated values. Valid values:

- `human` (default): Display in a human-readable text format. Conflicts with `short` and `json`.
- `short`: Emit shorter, human-readable text messages. Conflicts with `human` and `json`.
- `json`: Emit JSON messages to stdout. See [the reference](../reference/external-tools.md#json-messages) for more details. Conflicts with `human` and `short`.
- `json-diagnostic-short`: Ensure the `rendered` field of JSON messages contains the “short” rendering from rustc. Cannot be used with `human` or `short`.
- `json-diagnostic-rendered-ansi`: Ensure the `rendered` field of JSON messages contains embedded ANSI color codes for respecting rustc’s default color scheme. Cannot be used with `human` or `short`.
- `json-render-diagnostics`: Instruct Cargo to not include rustc diagnostics in JSON messages printed, but instead Cargo itself should render the JSON diagnostics coming from rustc. Cargo’s own JSON diagnostics and others coming from rustc are still emitted. Cannot be used with `human` or `short`.

### Manifest Options

[--manifest-path path](#option-cargo-rustc---manifest-path)  
Path to the `Cargo.toml` file. By default, Cargo searches for the `Cargo.toml` file in the current directory or any parent directory.

[--ignore-rust-version](#option-cargo-rustc---ignore-rust-version)  
Ignore `rust-version` specification in packages.

[--locked](#option-cargo-rustc---locked)  
Asserts that the exact same dependencies and versions are used as when the existing `Cargo.lock` file was originally generated. Cargo will exit with an error when either of the following scenarios arises:

- The lock file is missing.
- Cargo attempted to change the lock file due to a different dependency resolution.

It may be used in environments where deterministic builds are desired, such as in CI pipelines.

[--offline](#option-cargo-rustc---offline)  
Prevents Cargo from accessing the network for any reason. Without this flag, Cargo will stop with an error if it needs to access the network and the network is not available. With this flag, Cargo will attempt to proceed without the network if possible.

Beware that this may result in different dependency resolution than online mode. Cargo will restrict itself to crates that are downloaded locally, even if there might be a newer version as indicated in the local copy of the index. See the [cargo-fetch(1)](cargo-fetch.md) command to download dependencies before going offline.

May also be specified with the `net.offline` [config value](../reference/config.md).

[--frozen](#option-cargo-rustc---frozen)  
Equivalent to specifying both `--locked` and `--offline`.

### Common Options

[+toolchain](#option-cargo-rustc-+toolchain)  
If Cargo has been installed with rustup, and the first argument to `cargo` begins with `+`, it will be interpreted as a rustup toolchain name (such as `+stable` or `+nightly`). See the [rustup documentation](https://rust-lang.github.io/rustup/overrides.html) for more information about how toolchain overrides work.

[--config KEY=VALUE or PATH](#option-cargo-rustc---config)  
Overrides a Cargo configuration value. The argument should be in TOML syntax of `KEY=VALUE`, or provided as a path to an extra configuration file. This flag may be specified multiple times. See the [command-line overrides section](../reference/config.md#command-line-overrides) for more information.

[-C PATH](#option-cargo-rustc--C)  
Changes the current working directory before executing any specified operations. This affects things like where cargo looks by default for the project manifest (`Cargo.toml`), as well as the directories searched for discovering `.cargo/config.toml`, for example. This option must appear before the command name, for example `cargo -C path/to/my-project build`.

This option is only available on the [nightly channel](https://doc.rust-lang.org/book/appendix-07-nightly-rust.html) and requires the `-Z unstable-options` flag to enable (see [\#10098](https://github.com/rust-lang/cargo/issues/10098)).

[-h](#option-cargo-rustc--h)\
[--help](#option-cargo-rustc---help)  
Prints help information.

[-Z flag](#option-cargo-rustc--Z)  
Unstable (nightly-only) flags to Cargo. Run `cargo -Z help` for details.

### Miscellaneous Options

[-j N](#option-cargo-rustc--j)\
[--jobs N](#option-cargo-rustc---jobs)  
Number of parallel jobs to run. May also be specified with the `build.jobs` [config value](../reference/config.md). Defaults to the number of logical CPUs. If negative, it sets the maximum number of parallel jobs to the number of logical CPUs plus provided value. If a string `default` is provided, it sets the value back to defaults. Should not be 0.

[--keep-going](#option-cargo-rustc---keep-going)  
Build as many crates in the dependency graph as possible, rather than aborting the build on the first one that fails to build.

For example if the current package depends on dependencies `fails` and `works`, one of which fails to build, `cargo rustc -j1` may or may not build the one that succeeds (depending on which one of the two builds Cargo picked to run first), whereas `cargo rustc -j1 --keep-going` would definitely run both builds, even if the one run first fails.

[--future-incompat-report](#option-cargo-rustc---future-incompat-report)  
Displays a future-incompat report for any future-incompatible warnings produced during execution of this command

See [cargo-report(1)](cargo-report.md)

## ENVIRONMENT

See [the reference](../reference/environment-variables.md) for details on environment variables that Cargo reads.

## EXIT STATUS

- `0`: Cargo succeeded.
- `101`: Cargo failed to complete.

## EXAMPLES

1.  Check if your package (not including dependencies) uses unsafe code:

        cargo rustc --lib -- -D unsafe-code

2.  Try an experimental flag on the nightly compiler, such as this which prints the size of every type:

        cargo rustc --lib -- -Z print-type-sizes

3.  Override `crate-type` field in Cargo.toml with command-line option:

        cargo rustc --lib --crate-type lib,cdylib

## SEE ALSO

[cargo(1)](cargo.md), [cargo-build(1)](cargo-build.md), [rustc(1)](https://doc.rust-lang.org/rustc/index.html)
