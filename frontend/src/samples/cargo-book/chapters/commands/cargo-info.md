# cargo-info(1)

## NAME

cargo-info — Display information about a package.

## SYNOPSIS

`cargo info` \[*options*\] *spec*

## DESCRIPTION

This command displays information about a package. It fetches data from the package’s Cargo.toml file and presents it in a human-readable format.

## OPTIONS

### Info Options

[spec](#option-cargo-info-spec)  
Fetch information about the specified package. The *spec* can be a package ID, see [cargo-pkgid(1)](cargo-pkgid.md) for the SPEC format. If the specified package is part of the current workspace, information from the local Cargo.toml file will be displayed. If the `Cargo.lock` file does not exist, it will be created. If no version is specified, the appropriate version will be selected based on the Minimum Supported Rust Version (MSRV).

[--index index](#option-cargo-info---index)  
The URL of the registry index to use.

[--registry registry](#option-cargo-info---registry)  
Name of the registry to use. Registry names are defined in [Cargo config files](../reference/config.md). If not specified, the default registry is used, which is defined by the `registry.default` config key which defaults to `crates-io`.

### Display Options

[-v](#option-cargo-info--v)\
[--verbose](#option-cargo-info---verbose)  
Use verbose output. May be specified twice for “very verbose” output which includes extra output such as dependency warnings and build script output. May also be specified with the `term.verbose` [config value](../reference/config.md).

[-q](#option-cargo-info--q)\
[--quiet](#option-cargo-info---quiet)  
Do not print cargo log messages. May also be specified with the `term.quiet` [config value](../reference/config.md).

[--color when](#option-cargo-info---color)  
Control when colored output is used. Valid values:

- `auto` (default): Automatically detect if color support is available on the terminal.
- `always`: Always display colors.
- `never`: Never display colors.

May also be specified with the `term.color` [config value](../reference/config.md).

### Manifest Options

[--locked](#option-cargo-info---locked)  
Asserts that the exact same dependencies and versions are used as when the existing `Cargo.lock` file was originally generated. Cargo will exit with an error when either of the following scenarios arises:

- The lock file is missing.
- Cargo attempted to change the lock file due to a different dependency resolution.

It may be used in environments where deterministic builds are desired, such as in CI pipelines.

[--offline](#option-cargo-info---offline)  
Prevents Cargo from accessing the network for any reason. Without this flag, Cargo will stop with an error if it needs to access the network and the network is not available. With this flag, Cargo will attempt to proceed without the network if possible.

Beware that this may result in different dependency resolution than online mode. Cargo will restrict itself to crates that are downloaded locally, even if there might be a newer version as indicated in the local copy of the index. See the [cargo-fetch(1)](cargo-fetch.md) command to download dependencies before going offline.

May also be specified with the `net.offline` [config value](../reference/config.md).

[--frozen](#option-cargo-info---frozen)  
Equivalent to specifying both `--locked` and `--offline`.

### Common Options

[+toolchain](#option-cargo-info-+toolchain)  
If Cargo has been installed with rustup, and the first argument to `cargo` begins with `+`, it will be interpreted as a rustup toolchain name (such as `+stable` or `+nightly`). See the [rustup documentation](https://rust-lang.github.io/rustup/overrides.html) for more information about how toolchain overrides work.

[--config KEY=VALUE or PATH](#option-cargo-info---config)  
Overrides a Cargo configuration value. The argument should be in TOML syntax of `KEY=VALUE`, or provided as a path to an extra configuration file. This flag may be specified multiple times. See the [command-line overrides section](../reference/config.md#command-line-overrides) for more information.

[-C PATH](#option-cargo-info--C)  
Changes the current working directory before executing any specified operations. This affects things like where cargo looks by default for the project manifest (`Cargo.toml`), as well as the directories searched for discovering `.cargo/config.toml`, for example. This option must appear before the command name, for example `cargo -C path/to/my-project build`.

This option is only available on the [nightly channel](https://doc.rust-lang.org/book/appendix-07-nightly-rust.html) and requires the `-Z unstable-options` flag to enable (see [\#10098](https://github.com/rust-lang/cargo/issues/10098)).

[-h](#option-cargo-info--h)\
[--help](#option-cargo-info---help)  
Prints help information.

[-Z flag](#option-cargo-info--Z)  
Unstable (nightly-only) flags to Cargo. Run `cargo -Z help` for details.

## ENVIRONMENT

See [the reference](../reference/environment-variables.md) for details on environment variables that Cargo reads.

## EXIT STATUS

- `0`: Cargo succeeded.
- `101`: Cargo failed to complete.

## EXAMPLES

1.  Inspect the `serde` package from crates.io:

         cargo info serde

2.  Inspect the `serde` package with version `1.0.0`:

         cargo info serde@1.0.0

3.  Inspect the `serde` package from the local registry:

         cargo info serde --registry my-registry

## SEE ALSO

[cargo(1)](cargo.md), [cargo-search(1)](cargo-search.md)
