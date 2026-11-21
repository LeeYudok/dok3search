# cargo-publish(1)

## NAME

cargo-publish — Upload a package to the registry

## SYNOPSIS

`cargo publish` \[*options*\]

## DESCRIPTION

This command will create a distributable, compressed `.crate` file with the source code of the package in the current directory and upload it to a registry. The default registry is . This performs the following steps:

1.  Performs a few checks, including:
    - Checks the `package.publish` key in the manifest for restrictions on which registries you are allowed to publish to.
2.  Create a `.crate` file by following the steps in [cargo-package(1)](cargo-package.md).
3.  Upload the crate to the registry. The server will perform additional checks on the crate.
4.  The client will poll waiting for the package to appear in the index, and may timeout. In that case, you will need to check for completion manually. This timeout does not affect the upload.

This command requires you to be authenticated using [cargo-login(1)](cargo-login.md) or environment variables of the [`registry.token`](../reference/config.md#registrytoken) and [`registries.<name>.token`](../reference/config.md#registriesnametoken) config fields.

See [the reference](../reference/publishing.md) for more details about packaging and publishing.

## OPTIONS

### Publish Options

[--dry-run](#option-cargo-publish---dry-run)  
Perform all checks without uploading.

[--no-verify](#option-cargo-publish---no-verify)  
Don’t verify the contents by building them.

[--allow-dirty](#option-cargo-publish---allow-dirty)  
Allow working directories with uncommitted VCS changes to be packaged.

[--index index](#option-cargo-publish---index)  
The URL of the registry index to use.

[--registry registry](#option-cargo-publish---registry)  
Name of the registry to publish to. Registry names are defined in [Cargo config files](../reference/config.md). If not specified, and there is a [`package.publish`](../reference/manifest.md#the-publish-field) field in `Cargo.toml` with a single registry, then it will publish to that registry. Otherwise it will use the default registry, which is defined by the [`registry.default`](../reference/config.md#registrydefault) config key which defaults to `crates-io`.

### Package Selection

By default, when no package selection options are given, the packages selected depend on the selected manifest file (based on the current working directory if `--manifest-path` is not given). If the manifest is the root of a workspace then the workspaces default members are selected, otherwise only the package defined by the manifest will be selected.

The default members of a workspace can be set explicitly with the `workspace.default-members` key in the root manifest. If this is not set, a virtual workspace will include all workspace members (equivalent to passing `--workspace`), and a non-virtual workspace will include only the root crate itself.

[-p spec…](#option-cargo-publish--p)\
[--package spec…](#option-cargo-publish---package)  
Publish only the specified packages. See [cargo-pkgid(1)](cargo-pkgid.md) for the SPEC format. This flag may be specified multiple times and supports common Unix glob patterns like `*`, `?` and `[]`. However, to avoid your shell accidentally expanding glob patterns before Cargo handles them, you must use single quotes or double quotes around each pattern.

[--workspace](#option-cargo-publish---workspace)  
Publish all members in the workspace.

[--all](#option-cargo-publish---all)  
Deprecated alias for `--workspace`.

[--exclude SPEC…](#option-cargo-publish---exclude)  
Exclude the specified packages. Must be used in conjunction with the `--workspace` flag. This flag may be specified multiple times and supports common Unix glob patterns like `*`, `?` and `[]`. However, to avoid your shell accidentally expanding glob patterns before Cargo handles them, you must use single quotes or double quotes around each pattern.

### Compilation Options

[--target tuple](#option-cargo-publish---target)  
Publish for the specified target architecture. Flag may be specified multiple times. The default is the host architecture. The general format of the tuple is `<arch><sub>-<vendor>-<sys>-<abi>`.

Possible values:

- Any supported target in `rustc --print target-list`.
- `"host-tuple"`, which will internally be substituted by the host’s target. This can be particularly useful if you’re cross-compiling some crates, and don’t want to specify your host’s machine as a target (for instance, an `xtask` in a shared project that may be worked on by many hosts).
- A path to a custom target specification. See [Custom Target Lookup Path](https://doc.rust-lang.org/1.100.0/rustc/targets/custom.html#custom-target-lookup-path) for more information.

This may also be specified with the `build.target` [config value](../reference/config.md).

Note that specifying this flag makes Cargo run in a different mode where the target artifacts are placed in a separate directory. See the [build cache](../reference/build-cache.md) documentation for more details.

[--target-dir directory](#option-cargo-publish---target-dir)  
Directory for all generated artifacts and intermediate files. May also be specified with the `CARGO_TARGET_DIR` environment variable, or the `build.target-dir` [config value](../reference/config.md). Defaults to `target` in the root of the workspace.

### Feature Selection

The feature flags allow you to control which features are enabled. When no feature options are given, the `default` feature is activated for every selected package.

See [the features documentation](../reference/features.md#command-line-feature-options) for more details.

[-F features](#option-cargo-publish--F)\
[--features features](#option-cargo-publish---features)  
Space or comma separated list of features to activate. Features of workspace members may be enabled with `package-name/feature-name` syntax. This flag may be specified multiple times, which enables all specified features.

[--all-features](#option-cargo-publish---all-features)  
Activate all available features of all selected packages.

[--no-default-features](#option-cargo-publish---no-default-features)  
Do not activate the `default` feature of the selected packages.

### Manifest Options

[--manifest-path path](#option-cargo-publish---manifest-path)  
Path to the `Cargo.toml` file. By default, Cargo searches for the `Cargo.toml` file in the current directory or any parent directory.

[--locked](#option-cargo-publish---locked)  
Asserts that the exact same dependencies and versions are used as when the existing `Cargo.lock` file was originally generated. Cargo will exit with an error when either of the following scenarios arises:

- The lock file is missing.
- Cargo attempted to change the lock file due to a different dependency resolution.

It may be used in environments where deterministic builds are desired, such as in CI pipelines.

[--offline](#option-cargo-publish---offline)  
Prevents Cargo from accessing the network for any reason. Without this flag, Cargo will stop with an error if it needs to access the network and the network is not available. With this flag, Cargo will attempt to proceed without the network if possible.

Beware that this may result in different dependency resolution than online mode. Cargo will restrict itself to crates that are downloaded locally, even if there might be a newer version as indicated in the local copy of the index. See the [cargo-fetch(1)](cargo-fetch.md) command to download dependencies before going offline.

May also be specified with the `net.offline` [config value](../reference/config.md).

[--frozen](#option-cargo-publish---frozen)  
Equivalent to specifying both `--locked` and `--offline`.

### Miscellaneous Options

[-j N](#option-cargo-publish--j)\
[--jobs N](#option-cargo-publish---jobs)  
Number of parallel jobs to run. May also be specified with the `build.jobs` [config value](../reference/config.md). Defaults to the number of logical CPUs. If negative, it sets the maximum number of parallel jobs to the number of logical CPUs plus provided value. If a string `default` is provided, it sets the value back to defaults. Should not be 0.

[--keep-going](#option-cargo-publish---keep-going)  
Build as many crates in the dependency graph as possible, rather than aborting the build on the first one that fails to build.

For example if the current package depends on dependencies `fails` and `works`, one of which fails to build, `cargo publish -j1` may or may not build the one that succeeds (depending on which one of the two builds Cargo picked to run first), whereas `cargo publish -j1 --keep-going` would definitely run both builds, even if the one run first fails.

### Display Options

[-v](#option-cargo-publish--v)\
[--verbose](#option-cargo-publish---verbose)  
Use verbose output. May be specified twice for “very verbose” output which includes extra output such as dependency warnings and build script output. May also be specified with the `term.verbose` [config value](../reference/config.md).

[-q](#option-cargo-publish--q)\
[--quiet](#option-cargo-publish---quiet)  
Do not print cargo log messages. May also be specified with the `term.quiet` [config value](../reference/config.md).

[--color when](#option-cargo-publish---color)  
Control when colored output is used. Valid values:

- `auto` (default): Automatically detect if color support is available on the terminal.
- `always`: Always display colors.
- `never`: Never display colors.

May also be specified with the `term.color` [config value](../reference/config.md).

### Common Options

[+toolchain](#option-cargo-publish-+toolchain)  
If Cargo has been installed with rustup, and the first argument to `cargo` begins with `+`, it will be interpreted as a rustup toolchain name (such as `+stable` or `+nightly`). See the [rustup documentation](https://rust-lang.github.io/rustup/overrides.html) for more information about how toolchain overrides work.

[--config KEY=VALUE or PATH](#option-cargo-publish---config)  
Overrides a Cargo configuration value. The argument should be in TOML syntax of `KEY=VALUE`, or provided as a path to an extra configuration file. This flag may be specified multiple times. See the [command-line overrides section](../reference/config.md#command-line-overrides) for more information.

[-C PATH](#option-cargo-publish--C)  
Changes the current working directory before executing any specified operations. This affects things like where cargo looks by default for the project manifest (`Cargo.toml`), as well as the directories searched for discovering `.cargo/config.toml`, for example. This option must appear before the command name, for example `cargo -C path/to/my-project build`.

This option is only available on the [nightly channel](https://doc.rust-lang.org/book/appendix-07-nightly-rust.html) and requires the `-Z unstable-options` flag to enable (see [\#10098](https://github.com/rust-lang/cargo/issues/10098)).

[-h](#option-cargo-publish--h)\
[--help](#option-cargo-publish---help)  
Prints help information.

[-Z flag](#option-cargo-publish--Z)  
Unstable (nightly-only) flags to Cargo. Run `cargo -Z help` for details.

## ENVIRONMENT

See [the reference](../reference/environment-variables.md) for details on environment variables that Cargo reads.

## EXIT STATUS

- `0`: Cargo succeeded.
- `101`: Cargo failed to complete.

## EXAMPLES

1.  Publish the current package:

        cargo publish

## SEE ALSO

[cargo(1)](cargo.md), [cargo-package(1)](cargo-package.md), [cargo-login(1)](cargo-login.md)
