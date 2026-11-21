# cargo-search(1)

## NAME

cargo-search — Search packages in the registry. Default registry is crates.io

## SYNOPSIS

`cargo search` \[*options*\] \[*query*…\]

## DESCRIPTION

This performs a textual search for crates on . The matching crates will be displayed along with their description in TOML format suitable for copying into a `Cargo.toml` manifest.

## OPTIONS

### Search Options

[--limit limit](#option-cargo-search---limit)  
Limit the number of results (default: 10, max: 100).

[--index index](#option-cargo-search---index)  
The URL of the registry index to use.

[--registry registry](#option-cargo-search---registry)  
Name of the registry to use. Registry names are defined in [Cargo config files](../reference/config.md). If not specified, the default registry is used, which is defined by the `registry.default` config key which defaults to `crates-io`.

### Display Options

[-v](#option-cargo-search--v)\
[--verbose](#option-cargo-search---verbose)  
Use verbose output. May be specified twice for “very verbose” output which includes extra output such as dependency warnings and build script output. May also be specified with the `term.verbose` [config value](../reference/config.md).

[-q](#option-cargo-search--q)\
[--quiet](#option-cargo-search---quiet)  
Do not print cargo log messages. May also be specified with the `term.quiet` [config value](../reference/config.md).

[--color when](#option-cargo-search---color)  
Control when colored output is used. Valid values:

- `auto` (default): Automatically detect if color support is available on the terminal.
- `always`: Always display colors.
- `never`: Never display colors.

May also be specified with the `term.color` [config value](../reference/config.md).

### Common Options

[+toolchain](#option-cargo-search-+toolchain)  
If Cargo has been installed with rustup, and the first argument to `cargo` begins with `+`, it will be interpreted as a rustup toolchain name (such as `+stable` or `+nightly`). See the [rustup documentation](https://rust-lang.github.io/rustup/overrides.html) for more information about how toolchain overrides work.

[--config KEY=VALUE or PATH](#option-cargo-search---config)  
Overrides a Cargo configuration value. The argument should be in TOML syntax of `KEY=VALUE`, or provided as a path to an extra configuration file. This flag may be specified multiple times. See the [command-line overrides section](../reference/config.md#command-line-overrides) for more information.

[-C PATH](#option-cargo-search--C)  
Changes the current working directory before executing any specified operations. This affects things like where cargo looks by default for the project manifest (`Cargo.toml`), as well as the directories searched for discovering `.cargo/config.toml`, for example. This option must appear before the command name, for example `cargo -C path/to/my-project build`.

This option is only available on the [nightly channel](https://doc.rust-lang.org/book/appendix-07-nightly-rust.html) and requires the `-Z unstable-options` flag to enable (see [\#10098](https://github.com/rust-lang/cargo/issues/10098)).

[-h](#option-cargo-search--h)\
[--help](#option-cargo-search---help)  
Prints help information.

[-Z flag](#option-cargo-search--Z)  
Unstable (nightly-only) flags to Cargo. Run `cargo -Z help` for details.

## ENVIRONMENT

See [the reference](../reference/environment-variables.md) for details on environment variables that Cargo reads.

## EXIT STATUS

- `0`: Cargo succeeded.
- `101`: Cargo failed to complete.

## EXAMPLES

1.  Search for a package from crates.io:

        cargo search serde

## SEE ALSO

[cargo(1)](cargo.md), [cargo-install(1)](cargo-install.md), [cargo-publish(1)](cargo-publish.md)
