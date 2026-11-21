# cargo-logout(1)

## NAME

cargo-logout — Remove an API token from the registry locally

## SYNOPSIS

`cargo logout` \[*options*\]

## DESCRIPTION

This command will run a credential provider to remove a saved token.

For the default `cargo:token` credential provider, credentials are stored in `$CARGO_HOME/credentials.toml` where `$CARGO_HOME` defaults to `.cargo` in your home directory.

If a registry has a credential-provider specified, it will be used. Otherwise, the providers from the config value `registry.global-credential-providers` will be attempted, starting from the end of the list.

If `--registry` is not specified, then the credentials for the default registry will be removed (configured by [`registry.default`](../reference/config.md#registrydefault), which defaults to ).

This will not revoke the token on the server. If you need to revoke the token, visit the registry website and follow its instructions (see  to revoke the token for ).

## OPTIONS

### Logout Options

[--registry registry](#option-cargo-logout---registry)  
Name of the registry to use. Registry names are defined in [Cargo config files](../reference/config.md). If not specified, the default registry is used, which is defined by the `registry.default` config key which defaults to `crates-io`.

### Display Options

[-v](#option-cargo-logout--v)\
[--verbose](#option-cargo-logout---verbose)  
Use verbose output. May be specified twice for “very verbose” output which includes extra output such as dependency warnings and build script output. May also be specified with the `term.verbose` [config value](../reference/config.md).

[-q](#option-cargo-logout--q)\
[--quiet](#option-cargo-logout---quiet)  
Do not print cargo log messages. May also be specified with the `term.quiet` [config value](../reference/config.md).

[--color when](#option-cargo-logout---color)  
Control when colored output is used. Valid values:

- `auto` (default): Automatically detect if color support is available on the terminal.
- `always`: Always display colors.
- `never`: Never display colors.

May also be specified with the `term.color` [config value](../reference/config.md).

### Common Options

[+toolchain](#option-cargo-logout-+toolchain)  
If Cargo has been installed with rustup, and the first argument to `cargo` begins with `+`, it will be interpreted as a rustup toolchain name (such as `+stable` or `+nightly`). See the [rustup documentation](https://rust-lang.github.io/rustup/overrides.html) for more information about how toolchain overrides work.

[--config KEY=VALUE or PATH](#option-cargo-logout---config)  
Overrides a Cargo configuration value. The argument should be in TOML syntax of `KEY=VALUE`, or provided as a path to an extra configuration file. This flag may be specified multiple times. See the [command-line overrides section](../reference/config.md#command-line-overrides) for more information.

[-C PATH](#option-cargo-logout--C)  
Changes the current working directory before executing any specified operations. This affects things like where cargo looks by default for the project manifest (`Cargo.toml`), as well as the directories searched for discovering `.cargo/config.toml`, for example. This option must appear before the command name, for example `cargo -C path/to/my-project build`.

This option is only available on the [nightly channel](https://doc.rust-lang.org/book/appendix-07-nightly-rust.html) and requires the `-Z unstable-options` flag to enable (see [\#10098](https://github.com/rust-lang/cargo/issues/10098)).

[-h](#option-cargo-logout--h)\
[--help](#option-cargo-logout---help)  
Prints help information.

[-Z flag](#option-cargo-logout--Z)  
Unstable (nightly-only) flags to Cargo. Run `cargo -Z help` for details.

## ENVIRONMENT

See [the reference](../reference/environment-variables.md) for details on environment variables that Cargo reads.

## EXIT STATUS

- `0`: Cargo succeeded.
- `101`: Cargo failed to complete.

## EXAMPLES

1.  Remove the default registry token:

        cargo logout

2.  Remove the token for a specific registry:

        cargo logout --registry my-registry

## SEE ALSO

[cargo(1)](cargo.md), [cargo-login(1)](cargo-login.md)
