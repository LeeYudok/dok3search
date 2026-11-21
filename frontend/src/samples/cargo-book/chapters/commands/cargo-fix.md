# cargo-fix(1)

## NAME

cargo-fix — Automatically fix lint warnings reported by rustc

## SYNOPSIS

`cargo fix` \[*options*\]

## DESCRIPTION

This Cargo subcommand will automatically take rustc’s suggestions from diagnostics like warnings and apply them to your source code. This is intended to help automate tasks that rustc itself already knows how to tell you to fix!

Executing `cargo fix` will under the hood execute [cargo-check(1)](cargo-check.md). Any warnings applicable to your crate will be automatically fixed (if possible) and all remaining warnings will be displayed when the check process is finished. For example if you’d like to apply all fixes to the current package, you can run:

    cargo fix

which behaves the same as `cargo check --all-targets`.

`cargo fix` is only capable of fixing code that is normally compiled with `cargo check`. If code is conditionally enabled with optional features, you will need to enable those features for that code to be analyzed:

    cargo fix --features foo

Similarly, other `cfg` expressions like platform-specific code will need to pass `--target` to fix code for the given target.

    cargo fix --target x86_64-pc-windows-gnu

If you encounter any problems with `cargo fix` or otherwise have any questions or feature requests please don’t hesitate to file an issue at .

### Edition migration

The `cargo fix` subcommand can also be used to migrate a package from one [edition](https://doc.rust-lang.org/edition-guide/editions/transitioning-an-existing-project-to-a-new-edition.html) to the next. The general procedure is:

1.  Run `cargo fix --edition`. Consider also using the `--all-features` flag if your project has multiple features. You may also want to run `cargo fix --edition` multiple times with different `--target` flags if your project has platform-specific code gated by `cfg` attributes.
2.  Modify `Cargo.toml` to set the [edition field](../reference/manifest.md#the-edition-field) to the new edition.
3.  Run your project tests to verify that everything still works. If new warnings are issued, you may want to consider running `cargo fix` again (without the `--edition` flag) to apply any suggestions given by the compiler.

And hopefully that’s it! Just keep in mind of the caveats mentioned above that `cargo fix` cannot update code for inactive features or `cfg` expressions. Also, in some rare cases the compiler is unable to automatically migrate all code to the new edition, and this may require manual changes after building with the new edition.

## OPTIONS

### Fix options

[--broken-code](#option-cargo-fix---broken-code)  
Fix code even if it already has compiler errors. This is useful if `cargo fix` fails to apply the changes. It will apply the changes and leave the broken code in the working directory for you to inspect and manually fix.

[--edition](#option-cargo-fix---edition)  
Apply changes that will update the code to the next edition. This will not update the edition in the `Cargo.toml` manifest, which must be updated manually after `cargo fix --edition` has finished.

[--edition-idioms](#option-cargo-fix---edition-idioms)  
Apply suggestions that will update code to the preferred style for the current edition.

[--allow-no-vcs](#option-cargo-fix---allow-no-vcs)  
Fix code even if a VCS was not detected.

[--allow-dirty](#option-cargo-fix---allow-dirty)  
Fix code even if the working directory has changes (including staged changes).

[--allow-staged](#option-cargo-fix---allow-staged)  
Fix code even if the working directory has staged changes.

### Package Selection

By default, when no package selection options are given, the packages selected depend on the selected manifest file (based on the current working directory if `--manifest-path` is not given). If the manifest is the root of a workspace then the workspaces default members are selected, otherwise only the package defined by the manifest will be selected.

The default members of a workspace can be set explicitly with the `workspace.default-members` key in the root manifest. If this is not set, a virtual workspace will include all workspace members (equivalent to passing `--workspace`), and a non-virtual workspace will include only the root crate itself.

[-p spec…](#option-cargo-fix--p)\
[--package spec…](#option-cargo-fix---package)  
Fix only the specified packages. See [cargo-pkgid(1)](cargo-pkgid.md) for the SPEC format. This flag may be specified multiple times and supports common Unix glob patterns like `*`, `?` and `[]`. However, to avoid your shell accidentally expanding glob patterns before Cargo handles them, you must use single quotes or double quotes around each pattern.

[--workspace](#option-cargo-fix---workspace)  
Fix all members in the workspace.

[--all](#option-cargo-fix---all)  
Deprecated alias for `--workspace`.

[--exclude SPEC…](#option-cargo-fix---exclude)  
Exclude the specified packages. Must be used in conjunction with the `--workspace` flag. This flag may be specified multiple times and supports common Unix glob patterns like `*`, `?` and `[]`. However, to avoid your shell accidentally expanding glob patterns before Cargo handles them, you must use single quotes or double quotes around each pattern.

### Target Selection

When no target selection options are given, `cargo fix` will fix all targets (`--all-targets` implied). Binaries are skipped if they have `required-features` that are missing.

Passing target selection flags will fix only the specified targets.

Note that `--bin`, `--example`, `--test` and `--bench` flags also support common Unix glob patterns like `*`, `?` and `[]`. However, to avoid your shell accidentally expanding glob patterns before Cargo handles them, you must use single quotes or double quotes around each glob pattern.

[--lib](#option-cargo-fix---lib)  
Fix the package’s library.

[--bin name…](#option-cargo-fix---bin)  
Fix the specified binary. This flag may be specified multiple times and supports common Unix glob patterns.

[--bins](#option-cargo-fix---bins)  
Fix all binary targets.

[--example name…](#option-cargo-fix---example)  
Fix the specified example. This flag may be specified multiple times and supports common Unix glob patterns.

[--examples](#option-cargo-fix---examples)  
Fix all example targets.

[--test name…](#option-cargo-fix---test)  
Fix the specified integration test. This flag may be specified multiple times and supports common Unix glob patterns.

[--tests](#option-cargo-fix---tests)  
Fix all targets that have the `test = true` manifest flag set. By default this includes the library and binaries built as unittests, and integration tests. Be aware that this will also build any required dependencies, so the lib target may be built twice (once as a unittest, and once as a dependency for binaries, integration tests, etc.). Targets may be enabled or disabled by setting the `test` flag in the manifest settings for the target.

[--bench name…](#option-cargo-fix---bench)  
Fix the specified benchmark. This flag may be specified multiple times and supports common Unix glob patterns.

[--benches](#option-cargo-fix---benches)  
Fix all targets that have the `bench = true` manifest flag set. By default this includes the library and binaries built as benchmarks, and bench targets. Be aware that this will also build any required dependencies, so the lib target may be built twice (once as a benchmark, and once as a dependency for binaries, benchmarks, etc.). Targets may be enabled or disabled by setting the `bench` flag in the manifest settings for the target.

[--all-targets](#option-cargo-fix---all-targets)  
Fix all targets. This is equivalent to specifying `--lib --bins --tests --benches --examples`.

### Feature Selection

The feature flags allow you to control which features are enabled. When no feature options are given, the `default` feature is activated for every selected package.

See [the features documentation](../reference/features.md#command-line-feature-options) for more details.

[-F features](#option-cargo-fix--F)\
[--features features](#option-cargo-fix---features)  
Space or comma separated list of features to activate. Features of workspace members may be enabled with `package-name/feature-name` syntax. This flag may be specified multiple times, which enables all specified features.

[--all-features](#option-cargo-fix---all-features)  
Activate all available features of all selected packages.

[--no-default-features](#option-cargo-fix---no-default-features)  
Do not activate the `default` feature of the selected packages.

### Compilation Options

[--target tuple](#option-cargo-fix---target)  
Fix for the specified target architecture. Flag may be specified multiple times. The default is the host architecture. The general format of the tuple is `<arch><sub>-<vendor>-<sys>-<abi>`.

Possible values:

- Any supported target in `rustc --print target-list`.
- `"host-tuple"`, which will internally be substituted by the host’s target. This can be particularly useful if you’re cross-compiling some crates, and don’t want to specify your host’s machine as a target (for instance, an `xtask` in a shared project that may be worked on by many hosts).
- A path to a custom target specification. See [Custom Target Lookup Path](https://doc.rust-lang.org/1.100.0/rustc/targets/custom.html#custom-target-lookup-path) for more information.

This may also be specified with the `build.target` [config value](../reference/config.md).

Note that specifying this flag makes Cargo run in a different mode where the target artifacts are placed in a separate directory. See the [build cache](../reference/build-cache.md) documentation for more details.

[-r](#option-cargo-fix--r)\
[--release](#option-cargo-fix---release)  
Fix optimized artifacts with the `release` profile. See also the `--profile` option for choosing a specific profile by name.

[--profile name](#option-cargo-fix---profile)  
Fix with the given profile.

As a special case, specifying the `test` profile will also enable checking in test mode which will enable checking tests and enable the `test` cfg option. See [rustc tests](https://doc.rust-lang.org/rustc/tests/index.html) for more detail.

See [the reference](../reference/profiles.md) for more details on profiles.

[--timings](#option-cargo-fix---timings)  
Output information how long each compilation takes, and track concurrency information over time.

A file `cargo-timing.html` will be written to the `target/cargo-timings` directory at the end of the build. An additional report with a timestamp in its filename is also written if you want to look at a previous run. These reports are suitable for human consumption only, and do not provide machine-readable timing data.

### Output Options

[--target-dir directory](#option-cargo-fix---target-dir)  
Directory for all generated artifacts and intermediate files. May also be specified with the `CARGO_TARGET_DIR` environment variable, or the `build.target-dir` [config value](../reference/config.md). Defaults to `target` in the root of the workspace.

### Display Options

[-v](#option-cargo-fix--v)\
[--verbose](#option-cargo-fix---verbose)  
Use verbose output. May be specified twice for “very verbose” output which includes extra output such as dependency warnings and build script output. May also be specified with the `term.verbose` [config value](../reference/config.md).

[-q](#option-cargo-fix--q)\
[--quiet](#option-cargo-fix---quiet)  
Do not print cargo log messages. May also be specified with the `term.quiet` [config value](../reference/config.md).

[--color when](#option-cargo-fix---color)  
Control when colored output is used. Valid values:

- `auto` (default): Automatically detect if color support is available on the terminal.
- `always`: Always display colors.
- `never`: Never display colors.

May also be specified with the `term.color` [config value](../reference/config.md).

[--message-format fmt](#option-cargo-fix---message-format)  
The output format for diagnostic messages. Can be specified multiple times and consists of comma-separated values. Valid values:

- `human` (default): Display in a human-readable text format. Conflicts with `short` and `json`.
- `short`: Emit shorter, human-readable text messages. Conflicts with `human` and `json`.
- `json`: Emit JSON messages to stdout. See [the reference](../reference/external-tools.md#json-messages) for more details. Conflicts with `human` and `short`.
- `json-diagnostic-short`: Ensure the `rendered` field of JSON messages contains the “short” rendering from rustc. Cannot be used with `human` or `short`.
- `json-diagnostic-rendered-ansi`: Ensure the `rendered` field of JSON messages contains embedded ANSI color codes for respecting rustc’s default color scheme. Cannot be used with `human` or `short`.
- `json-render-diagnostics`: Instruct Cargo to not include rustc diagnostics in JSON messages printed, but instead Cargo itself should render the JSON diagnostics coming from rustc. Cargo’s own JSON diagnostics and others coming from rustc are still emitted. Cannot be used with `human` or `short`.

### Manifest Options

[--manifest-path path](#option-cargo-fix---manifest-path)  
Path to the `Cargo.toml` file. By default, Cargo searches for the `Cargo.toml` file in the current directory or any parent directory.

[--ignore-rust-version](#option-cargo-fix---ignore-rust-version)  
Ignore `rust-version` specification in packages.

[--locked](#option-cargo-fix---locked)  
Asserts that the exact same dependencies and versions are used as when the existing `Cargo.lock` file was originally generated. Cargo will exit with an error when either of the following scenarios arises:

- The lock file is missing.
- Cargo attempted to change the lock file due to a different dependency resolution.

It may be used in environments where deterministic builds are desired, such as in CI pipelines.

[--offline](#option-cargo-fix---offline)  
Prevents Cargo from accessing the network for any reason. Without this flag, Cargo will stop with an error if it needs to access the network and the network is not available. With this flag, Cargo will attempt to proceed without the network if possible.

Beware that this may result in different dependency resolution than online mode. Cargo will restrict itself to crates that are downloaded locally, even if there might be a newer version as indicated in the local copy of the index. See the [cargo-fetch(1)](cargo-fetch.md) command to download dependencies before going offline.

May also be specified with the `net.offline` [config value](../reference/config.md).

[--frozen](#option-cargo-fix---frozen)  
Equivalent to specifying both `--locked` and `--offline`.

### Common Options

[+toolchain](#option-cargo-fix-+toolchain)  
If Cargo has been installed with rustup, and the first argument to `cargo` begins with `+`, it will be interpreted as a rustup toolchain name (such as `+stable` or `+nightly`). See the [rustup documentation](https://rust-lang.github.io/rustup/overrides.html) for more information about how toolchain overrides work.

[--config KEY=VALUE or PATH](#option-cargo-fix---config)  
Overrides a Cargo configuration value. The argument should be in TOML syntax of `KEY=VALUE`, or provided as a path to an extra configuration file. This flag may be specified multiple times. See the [command-line overrides section](../reference/config.md#command-line-overrides) for more information.

[-C PATH](#option-cargo-fix--C)  
Changes the current working directory before executing any specified operations. This affects things like where cargo looks by default for the project manifest (`Cargo.toml`), as well as the directories searched for discovering `.cargo/config.toml`, for example. This option must appear before the command name, for example `cargo -C path/to/my-project build`.

This option is only available on the [nightly channel](https://doc.rust-lang.org/book/appendix-07-nightly-rust.html) and requires the `-Z unstable-options` flag to enable (see [\#10098](https://github.com/rust-lang/cargo/issues/10098)).

[-h](#option-cargo-fix--h)\
[--help](#option-cargo-fix---help)  
Prints help information.

[-Z flag](#option-cargo-fix--Z)  
Unstable (nightly-only) flags to Cargo. Run `cargo -Z help` for details.

### Miscellaneous Options

[-j N](#option-cargo-fix--j)\
[--jobs N](#option-cargo-fix---jobs)  
Number of parallel jobs to run. May also be specified with the `build.jobs` [config value](../reference/config.md). Defaults to the number of logical CPUs. If negative, it sets the maximum number of parallel jobs to the number of logical CPUs plus provided value. If a string `default` is provided, it sets the value back to defaults. Should not be 0.

[--keep-going](#option-cargo-fix---keep-going)  
Build as many crates in the dependency graph as possible, rather than aborting the build on the first one that fails to build.

For example if the current package depends on dependencies `fails` and `works`, one of which fails to build, `cargo fix -j1` may or may not build the one that succeeds (depending on which one of the two builds Cargo picked to run first), whereas `cargo fix -j1 --keep-going` would definitely run both builds, even if the one run first fails.

## ENVIRONMENT

See [the reference](../reference/environment-variables.md) for details on environment variables that Cargo reads.

## EXIT STATUS

- `0`: Cargo succeeded.
- `101`: Cargo failed to complete.

## EXAMPLES

1.  Apply compiler suggestions to the local package:

        cargo fix

2.  Update a package to prepare it for the next edition:

        cargo fix --edition

3.  Apply suggested idioms for the current edition:

        cargo fix --edition-idioms

## SEE ALSO

[cargo(1)](cargo.md), [cargo-check(1)](cargo-check.md)
