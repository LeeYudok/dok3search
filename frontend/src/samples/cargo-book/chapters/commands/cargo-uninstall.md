# cargo-uninstall(1)

## NAME

cargo-uninstall — Remove a Rust binary

## SYNOPSIS

`cargo uninstall` \[*options*\] \[*spec*…\]

## DESCRIPTION

This command removes a package installed with [cargo-install(1)](cargo-install.md). The *spec* argument is a package ID specification of the package to remove (see [cargo-pkgid(1)](cargo-pkgid.md)).

By default all binaries are removed for a crate but the `--bin` and `--example` flags can be used to only remove particular binaries.

The installation root is determined, in order of precedence:

- `--root` option
- `CARGO_INSTALL_ROOT` environment variable
- `install.root` Cargo [config value](../reference/config.md)
- `CARGO_HOME` environment variable
- `$HOME/.cargo`

## OPTIONS

### Uninstall Options

[-p](#option-cargo-uninstall--p)\
[--package spec…](#option-cargo-uninstall---package)  
Package to uninstall.

[--bin name…](#option-cargo-uninstall---bin)  
Only uninstall the binary *name*.

[--root dir](#option-cargo-uninstall---root)  
Directory to uninstall packages from.

### Display Options

[-v](#option-cargo-uninstall--v)\
[--verbose](#option-cargo-uninstall---verbose)  
Use verbose output. May be specified twice for “very verbose” output which includes extra output such as dependency warnings and build script output. May also be specified with the `term.verbose` [config value](../reference/config.md).

[-q](#option-cargo-uninstall--q)\
[--quiet](#option-cargo-uninstall---quiet)  
Do not print cargo log messages. May also be specified with the `term.quiet` [config value](../reference/config.md).

[--color when](#option-cargo-uninstall---color)  
Control when colored output is used. Valid values:

- `auto` (default): Automatically detect if color support is available on the terminal.
- `always`: Always display colors.
- `never`: Never display colors.

May also be specified with the `term.color` [config value](../reference/config.md).

### Common Options

[+toolchain](#option-cargo-uninstall-+toolchain)  
If Cargo has been installed with rustup, and the first argument to `cargo` begins with `+`, it will be interpreted as a rustup toolchain name (such as `+stable` or `+nightly`). See the [rustup documentation](https://rust-lang.github.io/rustup/overrides.html) for more information about how toolchain overrides work.

[--config KEY=VALUE or PATH](#option-cargo-uninstall---config)  
Overrides a Cargo configuration value. The argument should be in TOML syntax of `KEY=VALUE`, or provided as a path to an extra configuration file. This flag may be specified multiple times. See the [command-line overrides section](../reference/config.md#command-line-overrides) for more information.

[-C PATH](#option-cargo-uninstall--C)  
Changes the current working directory before executing any specified operations. This affects things like where cargo looks by default for the project manifest (`Cargo.toml`), as well as the directories searched for discovering `.cargo/config.toml`, for example. This option must appear before the command name, for example `cargo -C path/to/my-project build`.

This option is only available on the [nightly channel](https://doc.rust-lang.org/book/appendix-07-nightly-rust.html) and requires the `-Z unstable-options` flag to enable (see [\#10098](https://github.com/rust-lang/cargo/issues/10098)).

[-h](#option-cargo-uninstall--h)\
[--help](#option-cargo-uninstall---help)  
Prints help information.

[-Z flag](#option-cargo-uninstall--Z)  
Unstable (nightly-only) flags to Cargo. Run `cargo -Z help` for details.

## ENVIRONMENT

See [the reference](../reference/environment-variables.md) for details on environment variables that Cargo reads.

## EXIT STATUS

- `0`: Cargo succeeded.
- `101`: Cargo failed to complete.

## EXAMPLES

1.  Uninstall a previously installed package.

        cargo uninstall ripgrep

## SEE ALSO

[cargo(1)](cargo.md), [cargo-install(1)](cargo-install.md)
