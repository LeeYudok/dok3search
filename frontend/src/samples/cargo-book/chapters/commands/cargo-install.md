# cargo-install(1)

## NAME

cargo-install — Build and install a Rust binary

## SYNOPSIS

`cargo install` \[*options*\] *crate*\[@*version*\]…\
`cargo install` \[*options*\] `--path` *path*\
`cargo install` \[*options*\] `--git` *url* \[*crate*…\]\
`cargo install` \[*options*\] `--list`

## DESCRIPTION

This command manages Cargo’s local set of installed binary crates. Only packages which have executable `[[bin]]` or `[[example]]` targets can be installed, and all executables are installed into the installation root’s `bin` folder. By default only binaries, not examples, are installed.

The installation root is determined, in order of precedence:

- `--root` option
- `CARGO_INSTALL_ROOT` environment variable
- `install.root` Cargo [config value](../reference/config.md)
- `CARGO_HOME` environment variable
- `$HOME/.cargo`

There are multiple sources from which a crate can be installed. The default source location is crates.io but the `--git`, `--path`, and `--registry` flags can change this source. If the source contains more than one package (such as crates.io or a git repository with multiple crates) the *crate* argument is required to indicate which crate should be installed.

Crates from crates.io can optionally specify the version they wish to install via the `--version` flags, and similarly packages from git repositories can optionally specify the branch, tag, or revision that should be installed. If a crate has multiple binaries, the `--bin` argument can selectively install only one of them, and if you’d rather install examples the `--example` argument can be used as well.

If the package is already installed, Cargo will reinstall it if the installed version does not appear to be up-to-date. If any of the following values change, then Cargo will reinstall the package:

- The package version and source.
- The set of binary names installed.
- The chosen features.
- The profile (`--profile`).
- The target (`--target`).

Installing with `--path` will always build and install, unless there are conflicting binaries from another package. The `--force` flag may be used to force Cargo to always reinstall the package.

If the source is crates.io or `--git` then by default the crate will be built in a temporary target directory. To avoid this, the target directory can be specified by setting the `CARGO_TARGET_DIR` environment variable to a path. In particular, this can be useful for caching build artifacts on continuous integration systems.

### Dealing with the Lockfile

By default, Cargo uses the `Cargo.lock` file included with the package if it is available. This ensures that the installed executable uses the dependency versions selected when the package was published. If no lockfile is available, including for packages published with versions of Cargo before 1.37, Cargo resolves the dependencies normally.

The `--locked` flag does not currently change dependency resolution for `cargo install`. If no `Cargo.lock` is available, using `--locked` emits a warning.

### Configuration Discovery

This command operates on system or user level, not project level. This means that the local [configuration discovery](../reference/config.md#hierarchical-structure) is ignored. Instead, the configuration discovery begins at `$CARGO_HOME/config.toml`. If the package is installed with `--path $PATH`, the local configuration will be used, beginning discovery at `$PATH/.cargo/config.toml`.

## OPTIONS

### Install Options

[--vers version](#option-cargo-install---vers)\
[--version version](#option-cargo-install---version)  
Specify a version to install. This may be a [version requirement](../reference/specifying-dependencies.md), like `~1.2`, to have Cargo select the newest version from the given requirement. If the version does not have a requirement operator (such as `^` or `~`), then it must be in the form *MAJOR.MINOR.PATCH*, and will install exactly that version; it is *not* treated as a caret requirement like Cargo dependencies are.

[--git url](#option-cargo-install---git)  
Git URL to install the specified crate from.

[--branch branch](#option-cargo-install---branch)  
Branch to use when installing from git.

[--tag tag](#option-cargo-install---tag)  
Tag to use when installing from git.

[--rev sha](#option-cargo-install---rev)  
Specific commit to use when installing from git.

[--path path](#option-cargo-install---path)  
Filesystem path to local crate to install from.

[--list](#option-cargo-install---list)  
List all installed packages and their versions.

[-n](#option-cargo-install--n)\
[--dry-run](#option-cargo-install---dry-run)  
(unstable) Perform all checks without installing.

[-f](#option-cargo-install--f)\
[--force](#option-cargo-install---force)  
Force overwriting existing crates or binaries. This can be used if a package has installed a binary with the same name as another package. This is also useful if something has changed on the system that you want to rebuild with, such as a newer version of `rustc`.

[--no-track](#option-cargo-install---no-track)  
By default, Cargo keeps track of the installed packages with a metadata file stored in the installation root directory. This flag tells Cargo not to use or create that file. With this flag, Cargo will refuse to overwrite any existing files unless the `--force` flag is used. This also disables Cargo’s ability to protect against multiple concurrent invocations of Cargo installing at the same time.

[--bin name…](#option-cargo-install---bin)  
Install only the specified binary.

[--bins](#option-cargo-install---bins)  
Install all binaries. This is the default behavior.

[--example name…](#option-cargo-install---example)  
Install only the specified example.

[--examples](#option-cargo-install---examples)  
Install all examples.

[--root dir](#option-cargo-install---root)  
Directory to install packages into.

[--registry registry](#option-cargo-install---registry)  
Name of the registry to use. Registry names are defined in [Cargo config files](../reference/config.md). If not specified, the default registry is used, which is defined by the `registry.default` config key which defaults to `crates-io`.

[--index index](#option-cargo-install---index)  
The URL of the registry index to use.

### Feature Selection

The feature flags allow you to control which features are enabled. When no feature options are given, the `default` feature is activated for every selected package.

See [the features documentation](../reference/features.md#command-line-feature-options) for more details.

[-F features](#option-cargo-install--F)\
[--features features](#option-cargo-install---features)  
Space or comma separated list of features to activate. Features of workspace members may be enabled with `package-name/feature-name` syntax. This flag may be specified multiple times, which enables all specified features.

[--all-features](#option-cargo-install---all-features)  
Activate all available features of all selected packages.

[--no-default-features](#option-cargo-install---no-default-features)  
Do not activate the `default` feature of the selected packages.

### Compilation Options

[--target tuple](#option-cargo-install---target)  
Install for the specified target architecture. The default is the host architecture. The general format of the tuple is `<arch><sub>-<vendor>-<sys>-<abi>`.

Possible values:

- Any supported target in `rustc --print target-list`.
- `"host-tuple"`, which will internally be substituted by the host’s target. This can be particularly useful if you’re cross-compiling some crates, and don’t want to specify your host’s machine as a target (for instance, an `xtask` in a shared project that may be worked on by many hosts).
- A path to a custom target specification. See [Custom Target Lookup Path](../../rustc/targets/custom.html#custom-target-lookup-path) for more information.

This may also be specified with the `build.target` [config value](../reference/config.md).

Note that specifying this flag makes Cargo run in a different mode where the target artifacts are placed in a separate directory. See the [build cache](../reference/build-cache.md) documentation for more details.

[--target-dir directory](#option-cargo-install---target-dir)  
Directory for all generated artifacts and intermediate files. May also be specified with the `CARGO_TARGET_DIR` environment variable, or the `build.target-dir` [config value](../reference/config.md). Defaults to a new temporary folder located in the temporary directory of the platform.

When using `--path`, by default it will use `target` directory in the workspace of the local crate unless `--target-dir` is specified.

[--debug](#option-cargo-install---debug)  
Build with the `debug` profile instead of the `release` profile. See also the `--profile` option for choosing a specific profile by name.

[--profile name](#option-cargo-install---profile)  
Install with the given profile. See [the reference](../reference/profiles.md) for more details on profiles.

[--timings](#option-cargo-install---timings)  
Output information how long each compilation takes, and track concurrency information over time.

A file `cargo-timing.html` will be written to the `target/cargo-timings` directory at the end of the build. An additional report with a timestamp in its filename is also written if you want to look at a previous run. These reports are suitable for human consumption only, and do not provide machine-readable timing data.

### Manifest Options

[--ignore-rust-version](#option-cargo-install---ignore-rust-version)  
Ignore `rust-version` specification in packages.

[--locked](#option-cargo-install---locked)  
Asserts that the exact same dependencies and versions are used as when the existing `Cargo.lock` file was originally generated. Cargo will exit with an error when either of the following scenarios arises:

- The lock file is missing.
- Cargo attempted to change the lock file due to a different dependency resolution.

It may be used in environments where deterministic builds are desired, such as in CI pipelines.

[--offline](#option-cargo-install---offline)  
Prevents Cargo from accessing the network for any reason. Without this flag, Cargo will stop with an error if it needs to access the network and the network is not available. With this flag, Cargo will attempt to proceed without the network if possible.

Beware that this may result in different dependency resolution than online mode. Cargo will restrict itself to crates that are downloaded locally, even if there might be a newer version as indicated in the local copy of the index. See the [cargo-fetch(1)](cargo-fetch.md) command to download dependencies before going offline.

May also be specified with the `net.offline` [config value](../reference/config.md).

[--frozen](#option-cargo-install---frozen)  
Equivalent to specifying both `--locked` and `--offline`.

### Miscellaneous Options

[-j N](#option-cargo-install--j)\
[--jobs N](#option-cargo-install---jobs)  
Number of parallel jobs to run. May also be specified with the `build.jobs` [config value](../reference/config.md). Defaults to the number of logical CPUs. If negative, it sets the maximum number of parallel jobs to the number of logical CPUs plus provided value. If a string `default` is provided, it sets the value back to defaults. Should not be 0.

[--keep-going](#option-cargo-install---keep-going)  
Build as many crates in the dependency graph as possible, rather than aborting the build on the first one that fails to build.

For example if the current package depends on dependencies `fails` and `works`, one of which fails to build, `cargo install -j1` may or may not build the one that succeeds (depending on which one of the two builds Cargo picked to run first), whereas `cargo install -j1 --keep-going` would definitely run both builds, even if the one run first fails.

### Display Options

[-v](#option-cargo-install--v)\
[--verbose](#option-cargo-install---verbose)  
Use verbose output. May be specified twice for “very verbose” output which includes extra output such as dependency warnings and build script output. May also be specified with the `term.verbose` [config value](../reference/config.md).

[-q](#option-cargo-install--q)\
[--quiet](#option-cargo-install---quiet)  
Do not print cargo log messages. May also be specified with the `term.quiet` [config value](../reference/config.md).

[--color when](#option-cargo-install---color)  
Control when colored output is used. Valid values:

- `auto` (default): Automatically detect if color support is available on the terminal.
- `always`: Always display colors.
- `never`: Never display colors.

May also be specified with the `term.color` [config value](../reference/config.md).

[--message-format fmt](#option-cargo-install---message-format)  
The output format for diagnostic messages. Can be specified multiple times and consists of comma-separated values. Valid values:

- `human` (default): Display in a human-readable text format. Conflicts with `short` and `json`.
- `short`: Emit shorter, human-readable text messages. Conflicts with `human` and `json`.
- `json`: Emit JSON messages to stdout. See [the reference](../reference/external-tools.md#json-messages) for more details. Conflicts with `human` and `short`.
- `json-diagnostic-short`: Ensure the `rendered` field of JSON messages contains the “short” rendering from rustc. Cannot be used with `human` or `short`.
- `json-diagnostic-rendered-ansi`: Ensure the `rendered` field of JSON messages contains embedded ANSI color codes for respecting rustc’s default color scheme. Cannot be used with `human` or `short`.
- `json-render-diagnostics`: Instruct Cargo to not include rustc diagnostics in JSON messages printed, but instead Cargo itself should render the JSON diagnostics coming from rustc. Cargo’s own JSON diagnostics and others coming from rustc are still emitted. Cannot be used with `human` or `short`.

### Common Options

[+toolchain](#option-cargo-install-+toolchain)  
If Cargo has been installed with rustup, and the first argument to `cargo` begins with `+`, it will be interpreted as a rustup toolchain name (such as `+stable` or `+nightly`). See the [rustup documentation](https://rust-lang.github.io/rustup/overrides.html) for more information about how toolchain overrides work.

[--config KEY=VALUE or PATH](#option-cargo-install---config)  
Overrides a Cargo configuration value. The argument should be in TOML syntax of `KEY=VALUE`, or provided as a path to an extra configuration file. This flag may be specified multiple times. See the [command-line overrides section](../reference/config.md#command-line-overrides) for more information.

[-C PATH](#option-cargo-install--C)  
Changes the current working directory before executing any specified operations. This affects things like where cargo looks by default for the project manifest (`Cargo.toml`), as well as the directories searched for discovering `.cargo/config.toml`, for example. This option must appear before the command name, for example `cargo -C path/to/my-project build`.

This option is only available on the [nightly channel](https://doc.rust-lang.org/book/appendix-07-nightly-rust.html) and requires the `-Z unstable-options` flag to enable (see [\#10098](https://github.com/rust-lang/cargo/issues/10098)).

[-h](#option-cargo-install--h)\
[--help](#option-cargo-install---help)  
Prints help information.

[-Z flag](#option-cargo-install--Z)  
Unstable (nightly-only) flags to Cargo. Run `cargo -Z help` for details.

## ENVIRONMENT

See [the reference](../reference/environment-variables.md) for details on environment variables that Cargo reads.

## EXIT STATUS

- `0`: Cargo succeeded.
- `101`: Cargo failed to complete.

## EXAMPLES

1.  Install or upgrade a package from crates.io:

        cargo install ripgrep

2.  Install or reinstall the package in the current directory:

        cargo install --path .

3.  View the list of installed packages:

        cargo install --list

## SEE ALSO

[cargo(1)](cargo.md), [cargo-uninstall(1)](cargo-uninstall.md), [cargo-search(1)](cargo-search.md), [cargo-publish(1)](cargo-publish.md)
