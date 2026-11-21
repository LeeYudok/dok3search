# cargo-clean(1)

## NAME

cargo-clean — Remove generated artifacts

## SYNOPSIS

`cargo clean` \[*options*\]

## DESCRIPTION

Remove artifacts from the target directory that Cargo has generated in the past.

With no options, `cargo clean` will delete the entire target directory.

## OPTIONS

### Package Selection

When no packages are selected, all packages and all dependencies in the workspace are cleaned.

[-p spec…](#option-cargo-clean--p)\
[--package spec…](#option-cargo-clean---package)  
Clean only the specified packages. This flag may be specified multiple times. See [cargo-pkgid(1)](cargo-pkgid.md) for the SPEC format.

[--workspace](#option-cargo-clean---workspace)  
Clean artifacts of the workspace members.

### Clean Options

[--dry-run](#option-cargo-clean---dry-run)  
Displays a summary of what would be deleted without deleting anything. Use with `--verbose` to display the actual files that would be deleted.

[--doc](#option-cargo-clean---doc)  
This option will cause `cargo clean` to remove only the `doc` directory in the target directory.

[--release](#option-cargo-clean---release)  
Remove artifacts only in the `release` directory.

[--profile name](#option-cargo-clean---profile)  
Remove artifacts only in the directory with the given profile name.

[--target-dir directory](#option-cargo-clean---target-dir)  
Directory for all generated artifacts and intermediate files. May also be specified with the `CARGO_TARGET_DIR` environment variable, or the `build.target-dir` [config value](../reference/config.md). Defaults to `target` in the root of the workspace.

[--target tuple](#option-cargo-clean---target)  
Clean for the specified target architecture. Flag may be specified multiple times. The default is the host architecture. The general format of the tuple is `<arch><sub>-<vendor>-<sys>-<abi>`.

Possible values:

- Any supported target in `rustc --print target-list`.
- `"host-tuple"`, which will internally be substituted by the host’s target. This can be particularly useful if you’re cross-compiling some crates, and don’t want to specify your host’s machine as a target (for instance, an `xtask` in a shared project that may be worked on by many hosts).
- A path to a custom target specification. See [Custom Target Lookup Path](../../rustc/targets/custom.html#custom-target-lookup-path) for more information.

This may also be specified with the `build.target` [config value](../reference/config.md).

Note that specifying this flag makes Cargo run in a different mode where the target artifacts are placed in a separate directory. See the [build cache](../reference/build-cache.md) documentation for more details.

### Display Options

[-v](#option-cargo-clean--v)\
[--verbose](#option-cargo-clean---verbose)  
Use verbose output. May be specified twice for “very verbose” output which includes extra output such as dependency warnings and build script output. May also be specified with the `term.verbose` [config value](../reference/config.md).

[-q](#option-cargo-clean--q)\
[--quiet](#option-cargo-clean---quiet)  
Do not print cargo log messages. May also be specified with the `term.quiet` [config value](../reference/config.md).

[--color when](#option-cargo-clean---color)  
Control when colored output is used. Valid values:

- `auto` (default): Automatically detect if color support is available on the terminal.
- `always`: Always display colors.
- `never`: Never display colors.

May also be specified with the `term.color` [config value](../reference/config.md).

### Manifest Options

[--manifest-path path](#option-cargo-clean---manifest-path)  
Path to the `Cargo.toml` file. By default, Cargo searches for the `Cargo.toml` file in the current directory or any parent directory.

[--locked](#option-cargo-clean---locked)  
Asserts that the exact same dependencies and versions are used as when the existing `Cargo.lock` file was originally generated. Cargo will exit with an error when either of the following scenarios arises:

- The lock file is missing.
- Cargo attempted to change the lock file due to a different dependency resolution.

It may be used in environments where deterministic builds are desired, such as in CI pipelines.

[--offline](#option-cargo-clean---offline)  
Prevents Cargo from accessing the network for any reason. Without this flag, Cargo will stop with an error if it needs to access the network and the network is not available. With this flag, Cargo will attempt to proceed without the network if possible.

Beware that this may result in different dependency resolution than online mode. Cargo will restrict itself to crates that are downloaded locally, even if there might be a newer version as indicated in the local copy of the index. See the [cargo-fetch(1)](cargo-fetch.md) command to download dependencies before going offline.

May also be specified with the `net.offline` [config value](../reference/config.md).

[--frozen](#option-cargo-clean---frozen)  
Equivalent to specifying both `--locked` and `--offline`.

### Common Options

[+toolchain](#option-cargo-clean-+toolchain)  
If Cargo has been installed with rustup, and the first argument to `cargo` begins with `+`, it will be interpreted as a rustup toolchain name (such as `+stable` or `+nightly`). See the [rustup documentation](https://rust-lang.github.io/rustup/overrides.html) for more information about how toolchain overrides work.

[--config KEY=VALUE or PATH](#option-cargo-clean---config)  
Overrides a Cargo configuration value. The argument should be in TOML syntax of `KEY=VALUE`, or provided as a path to an extra configuration file. This flag may be specified multiple times. See the [command-line overrides section](../reference/config.md#command-line-overrides) for more information.

[-C PATH](#option-cargo-clean--C)  
Changes the current working directory before executing any specified operations. This affects things like where cargo looks by default for the project manifest (`Cargo.toml`), as well as the directories searched for discovering `.cargo/config.toml`, for example. This option must appear before the command name, for example `cargo -C path/to/my-project build`.

This option is only available on the [nightly channel](https://doc.rust-lang.org/book/appendix-07-nightly-rust.html) and requires the `-Z unstable-options` flag to enable (see [\#10098](https://github.com/rust-lang/cargo/issues/10098)).

[-h](#option-cargo-clean--h)\
[--help](#option-cargo-clean---help)  
Prints help information.

[-Z flag](#option-cargo-clean--Z)  
Unstable (nightly-only) flags to Cargo. Run `cargo -Z help` for details.

## ENVIRONMENT

See [the reference](../reference/environment-variables.md) for details on environment variables that Cargo reads.

## EXIT STATUS

- `0`: Cargo succeeded.
- `101`: Cargo failed to complete.

## EXAMPLES

1.  Remove the entire target directory:

        cargo clean

2.  Remove only the release artifacts:

        cargo clean --release

## SEE ALSO

[cargo(1)](cargo.md), [cargo-build(1)](cargo-build.md)
