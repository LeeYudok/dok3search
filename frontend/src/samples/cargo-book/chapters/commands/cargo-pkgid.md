# cargo-pkgid(1)

## NAME

cargo-pkgid — Print a fully qualified package specification

## SYNOPSIS

`cargo pkgid` \[*options*\] \[*spec*\]

## DESCRIPTION

Given a *spec* argument, print out the fully qualified package ID specifier for a package or dependency in the current workspace. This command will generate an error if *spec* is ambiguous as to which package it refers to in the dependency graph. If no *spec* is given, then the specifier for the local package is printed.

This command requires that a lockfile is available and dependencies have been fetched.

A package specifier consists of a name, version, and source URL. You are allowed to use partial specifiers to succinctly match a specific package as long as it matches only one package. This specifier is also used by other parts in Cargo, such as [cargo-metadata(1)](cargo-metadata.md) and [JSON messages](../reference/external-tools.md#json-messages) emitted by Cargo.

The format of a *spec* can be one of the following:

| SPEC Structure | Example SPEC |
|----|----|
| *name* | `bitflags` |
| *name*`@`*version* | `bitflags@1.0.4` |
| *url* | `https://github.com/rust-lang/cargo` |
| *url*`#`*version* | `https://github.com/rust-lang/cargo#0.33.0` |
| *url*`#`*name* | `https://github.com/rust-lang/crates.io-index#bitflags` |
| *url*`#`*name*`@`*version* | `https://github.com/rust-lang/cargo#crates-io@0.21.0` |

The specification grammar can be found in chapter [Package ID Specifications](../reference/pkgid-spec.md).

## OPTIONS

### Package Selection

[-p spec](#option-cargo-pkgid--p)\
[--package spec](#option-cargo-pkgid---package)  
Get the package ID for the given package instead of the current package.

### Display Options

[-v](#option-cargo-pkgid--v)\
[--verbose](#option-cargo-pkgid---verbose)  
Use verbose output. May be specified twice for “very verbose” output which includes extra output such as dependency warnings and build script output. May also be specified with the `term.verbose` [config value](../reference/config.md).

[-q](#option-cargo-pkgid--q)\
[--quiet](#option-cargo-pkgid---quiet)  
Do not print cargo log messages. May also be specified with the `term.quiet` [config value](../reference/config.md).

[--color when](#option-cargo-pkgid---color)  
Control when colored output is used. Valid values:

- `auto` (default): Automatically detect if color support is available on the terminal.
- `always`: Always display colors.
- `never`: Never display colors.

May also be specified with the `term.color` [config value](../reference/config.md).

### Manifest Options

[--manifest-path path](#option-cargo-pkgid---manifest-path)  
Path to the `Cargo.toml` file. By default, Cargo searches for the `Cargo.toml` file in the current directory or any parent directory.

[--locked](#option-cargo-pkgid---locked)  
Asserts that the exact same dependencies and versions are used as when the existing `Cargo.lock` file was originally generated. Cargo will exit with an error when either of the following scenarios arises:

- The lock file is missing.
- Cargo attempted to change the lock file due to a different dependency resolution.

It may be used in environments where deterministic builds are desired, such as in CI pipelines.

[--offline](#option-cargo-pkgid---offline)  
Prevents Cargo from accessing the network for any reason. Without this flag, Cargo will stop with an error if it needs to access the network and the network is not available. With this flag, Cargo will attempt to proceed without the network if possible.

Beware that this may result in different dependency resolution than online mode. Cargo will restrict itself to crates that are downloaded locally, even if there might be a newer version as indicated in the local copy of the index. See the [cargo-fetch(1)](cargo-fetch.md) command to download dependencies before going offline.

May also be specified with the `net.offline` [config value](../reference/config.md).

[--frozen](#option-cargo-pkgid---frozen)  
Equivalent to specifying both `--locked` and `--offline`.

### Common Options

[+toolchain](#option-cargo-pkgid-+toolchain)  
If Cargo has been installed with rustup, and the first argument to `cargo` begins with `+`, it will be interpreted as a rustup toolchain name (such as `+stable` or `+nightly`). See the [rustup documentation](https://rust-lang.github.io/rustup/overrides.html) for more information about how toolchain overrides work.

[--config KEY=VALUE or PATH](#option-cargo-pkgid---config)  
Overrides a Cargo configuration value. The argument should be in TOML syntax of `KEY=VALUE`, or provided as a path to an extra configuration file. This flag may be specified multiple times. See the [command-line overrides section](../reference/config.md#command-line-overrides) for more information.

[-C PATH](#option-cargo-pkgid--C)  
Changes the current working directory before executing any specified operations. This affects things like where cargo looks by default for the project manifest (`Cargo.toml`), as well as the directories searched for discovering `.cargo/config.toml`, for example. This option must appear before the command name, for example `cargo -C path/to/my-project build`.

This option is only available on the [nightly channel](https://doc.rust-lang.org/book/appendix-07-nightly-rust.html) and requires the `-Z unstable-options` flag to enable (see [\#10098](https://github.com/rust-lang/cargo/issues/10098)).

[-h](#option-cargo-pkgid--h)\
[--help](#option-cargo-pkgid---help)  
Prints help information.

[-Z flag](#option-cargo-pkgid--Z)  
Unstable (nightly-only) flags to Cargo. Run `cargo -Z help` for details.

## ENVIRONMENT

See [the reference](../reference/environment-variables.md) for details on environment variables that Cargo reads.

## EXIT STATUS

- `0`: Cargo succeeded.
- `101`: Cargo failed to complete.

## EXAMPLES

1.  Retrieve package specification for `foo` package:

        cargo pkgid foo

2.  Retrieve package specification for version 1.0.0 of `foo`:

        cargo pkgid foo@1.0.0

3.  Retrieve package specification for `foo` from crates.io:

        cargo pkgid https://github.com/rust-lang/crates.io-index#foo

4.  Retrieve package specification for `foo` from a local package:

        cargo pkgid file:///path/to/local/package#foo

## SEE ALSO

[cargo(1)](cargo.md), [cargo-generate-lockfile(1)](cargo-generate-lockfile.md), [cargo-metadata(1)](cargo-metadata.md), [Package ID Specifications](../reference/pkgid-spec.md), [JSON messages](../reference/external-tools.md#json-messages)
