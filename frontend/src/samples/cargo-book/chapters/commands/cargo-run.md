# cargo-run(1)

## NAME

cargo-run — Run the current package

## SYNOPSIS

`cargo run` \[*options*\] \[`--` *args*\]

## DESCRIPTION

Run a binary or example of the local package.

All the arguments following the two dashes (`--`) are passed to the binary to run. If you’re passing arguments to both Cargo and the binary, the ones after `--` go to the binary, the ones before go to Cargo.

Unlike [cargo-test(1)](cargo-test.md) and [cargo-bench(1)](cargo-bench.md), `cargo run` sets the working directory of the binary executed to the current working directory, same as if it was executed in the shell directly.

## OPTIONS

### Package Selection

By default, the package in the current working directory is selected. The `-p` flag can be used to choose a different package in a workspace.

[-p spec](#option-cargo-run--p)\
[--package spec](#option-cargo-run---package)  
The package to run. See [cargo-pkgid(1)](cargo-pkgid.md) for the SPEC format.

### Target Selection

When no target selection options are given, `cargo run` will run the binary target. If there are multiple binary targets, you must pass a target flag to choose one. Or, the `default-run` field may be specified in the `[package]` section of `Cargo.toml` to choose the name of the binary to run by default.

[--bin name](#option-cargo-run---bin)  
Run the specified binary.

[--example name](#option-cargo-run---example)  
Run the specified example.

### Feature Selection

The feature flags allow you to control which features are enabled. When no feature options are given, the `default` feature is activated for every selected package.

See [the features documentation](../reference/features.md#command-line-feature-options) for more details.

[-F features](#option-cargo-run--F)\
[--features features](#option-cargo-run---features)  
Space or comma separated list of features to activate. Features of workspace members may be enabled with `package-name/feature-name` syntax. This flag may be specified multiple times, which enables all specified features.

[--all-features](#option-cargo-run---all-features)  
Activate all available features of all selected packages.

[--no-default-features](#option-cargo-run---no-default-features)  
Do not activate the `default` feature of the selected packages.

### Compilation Options

[--target tuple](#option-cargo-run---target)  
Run for the specified target architecture. The default is the host architecture. The general format of the tuple is `<arch><sub>-<vendor>-<sys>-<abi>`.

Possible values:

- Any supported target in `rustc --print target-list`.
- `"host-tuple"`, which will internally be substituted by the host’s target. This can be particularly useful if you’re cross-compiling some crates, and don’t want to specify your host’s machine as a target (for instance, an `xtask` in a shared project that may be worked on by many hosts).
- A path to a custom target specification. See [Custom Target Lookup Path](../../rustc/targets/custom.html#custom-target-lookup-path) for more information.

This may also be specified with the `build.target` [config value](../reference/config.md).

Note that specifying this flag makes Cargo run in a different mode where the target artifacts are placed in a separate directory. See the [build cache](../reference/build-cache.md) documentation for more details.

[-r](#option-cargo-run--r)\
[--release](#option-cargo-run---release)  
Run optimized artifacts with the `release` profile. See also the `--profile` option for choosing a specific profile by name.

[--profile name](#option-cargo-run---profile)  
Run with the given profile. See [the reference](../reference/profiles.md) for more details on profiles.

[--timings](#option-cargo-run---timings)  
Output information how long each compilation takes, and track concurrency information over time.

A file `cargo-timing.html` will be written to the `target/cargo-timings` directory at the end of the build. An additional report with a timestamp in its filename is also written if you want to look at a previous run. These reports are suitable for human consumption only, and do not provide machine-readable timing data.

### Output Options

[--target-dir directory](#option-cargo-run---target-dir)  
Directory for all generated artifacts and intermediate files. May also be specified with the `CARGO_TARGET_DIR` environment variable, or the `build.target-dir` [config value](../reference/config.md). Defaults to `target` in the root of the workspace.

### Display Options

[-v](#option-cargo-run--v)\
[--verbose](#option-cargo-run---verbose)  
Use verbose output. May be specified twice for “very verbose” output which includes extra output such as dependency warnings and build script output. May also be specified with the `term.verbose` [config value](../reference/config.md).

[-q](#option-cargo-run--q)\
[--quiet](#option-cargo-run---quiet)  
Do not print cargo log messages. May also be specified with the `term.quiet` [config value](../reference/config.md).

[--color when](#option-cargo-run---color)  
Control when colored output is used. Valid values:

- `auto` (default): Automatically detect if color support is available on the terminal.
- `always`: Always display colors.
- `never`: Never display colors.

May also be specified with the `term.color` [config value](../reference/config.md).

[--message-format fmt](#option-cargo-run---message-format)  
The output format for diagnostic messages. Can be specified multiple times and consists of comma-separated values. Valid values:

- `human` (default): Display in a human-readable text format. Conflicts with `short` and `json`.
- `short`: Emit shorter, human-readable text messages. Conflicts with `human` and `json`.
- `json`: Emit JSON messages to stdout. See [the reference](../reference/external-tools.md#json-messages) for more details. Conflicts with `human` and `short`.
- `json-diagnostic-short`: Ensure the `rendered` field of JSON messages contains the “short” rendering from rustc. Cannot be used with `human` or `short`.
- `json-diagnostic-rendered-ansi`: Ensure the `rendered` field of JSON messages contains embedded ANSI color codes for respecting rustc’s default color scheme. Cannot be used with `human` or `short`.
- `json-render-diagnostics`: Instruct Cargo to not include rustc diagnostics in JSON messages printed, but instead Cargo itself should render the JSON diagnostics coming from rustc. Cargo’s own JSON diagnostics and others coming from rustc are still emitted. Cannot be used with `human` or `short`.

### Manifest Options

[--manifest-path path](#option-cargo-run---manifest-path)  
Path to the `Cargo.toml` file. By default, Cargo searches for the `Cargo.toml` file in the current directory or any parent directory.

[--ignore-rust-version](#option-cargo-run---ignore-rust-version)  
Ignore `rust-version` specification in packages.

[--locked](#option-cargo-run---locked)  
Asserts that the exact same dependencies and versions are used as when the existing `Cargo.lock` file was originally generated. Cargo will exit with an error when either of the following scenarios arises:

- The lock file is missing.
- Cargo attempted to change the lock file due to a different dependency resolution.

It may be used in environments where deterministic builds are desired, such as in CI pipelines.

[--offline](#option-cargo-run---offline)  
Prevents Cargo from accessing the network for any reason. Without this flag, Cargo will stop with an error if it needs to access the network and the network is not available. With this flag, Cargo will attempt to proceed without the network if possible.

Beware that this may result in different dependency resolution than online mode. Cargo will restrict itself to crates that are downloaded locally, even if there might be a newer version as indicated in the local copy of the index. See the [cargo-fetch(1)](cargo-fetch.md) command to download dependencies before going offline.

May also be specified with the `net.offline` [config value](../reference/config.md).

[--frozen](#option-cargo-run---frozen)  
Equivalent to specifying both `--locked` and `--offline`.

### Common Options

[+toolchain](#option-cargo-run-+toolchain)  
If Cargo has been installed with rustup, and the first argument to `cargo` begins with `+`, it will be interpreted as a rustup toolchain name (such as `+stable` or `+nightly`). See the [rustup documentation](https://rust-lang.github.io/rustup/overrides.html) for more information about how toolchain overrides work.

[--config KEY=VALUE or PATH](#option-cargo-run---config)  
Overrides a Cargo configuration value. The argument should be in TOML syntax of `KEY=VALUE`, or provided as a path to an extra configuration file. This flag may be specified multiple times. See the [command-line overrides section](../reference/config.md#command-line-overrides) for more information.

[-C PATH](#option-cargo-run--C)  
Changes the current working directory before executing any specified operations. This affects things like where cargo looks by default for the project manifest (`Cargo.toml`), as well as the directories searched for discovering `.cargo/config.toml`, for example. This option must appear before the command name, for example `cargo -C path/to/my-project build`.

This option is only available on the [nightly channel](https://doc.rust-lang.org/book/appendix-07-nightly-rust.html) and requires the `-Z unstable-options` flag to enable (see [\#10098](https://github.com/rust-lang/cargo/issues/10098)).

[-h](#option-cargo-run--h)\
[--help](#option-cargo-run---help)  
Prints help information.

[-Z flag](#option-cargo-run--Z)  
Unstable (nightly-only) flags to Cargo. Run `cargo -Z help` for details.

### Miscellaneous Options

[-j N](#option-cargo-run--j)\
[--jobs N](#option-cargo-run---jobs)  
Number of parallel jobs to run. May also be specified with the `build.jobs` [config value](../reference/config.md). Defaults to the number of logical CPUs. If negative, it sets the maximum number of parallel jobs to the number of logical CPUs plus provided value. If a string `default` is provided, it sets the value back to defaults. Should not be 0.

[--keep-going](#option-cargo-run---keep-going)  
Build as many crates in the dependency graph as possible, rather than aborting the build on the first one that fails to build.

For example if the current package depends on dependencies `fails` and `works`, one of which fails to build, `cargo run -j1` may or may not build the one that succeeds (depending on which one of the two builds Cargo picked to run first), whereas `cargo run -j1 --keep-going` would definitely run both builds, even if the one run first fails.

## ENVIRONMENT

See [the reference](../reference/environment-variables.md) for details on environment variables that Cargo reads.

## EXIT STATUS

- `0`: Cargo succeeded.
- `101`: Cargo failed to complete.

## EXAMPLES

1.  Build the local package and run its main target (assuming only one binary):

        cargo run

2.  Run an example with extra arguments:

        cargo run --example exname -- --exoption exarg1 exarg2

## SEE ALSO

[cargo(1)](cargo.md), [cargo-build(1)](cargo-build.md)
