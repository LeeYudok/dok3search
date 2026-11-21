# cargo-owner(1)

## NAME

cargo-owner — Manage the owners of a crate on the registry

## SYNOPSIS

`cargo owner` \[*options*\] `--add` *login* \[*crate*\]\
`cargo owner` \[*options*\] `--remove` *login* \[*crate*\]\
`cargo owner` \[*options*\] `--list` \[*crate*\]

## DESCRIPTION

This command will modify the owners for a crate on the registry. Owners of a crate can upload new versions and yank old versions. Non-team owners can also modify the set of owners, so take care!

This command requires you to be authenticated with either the `--token` option or using [cargo-login(1)](cargo-login.md).

If the crate name is not specified, it will use the package name from the current directory.

See [the reference](../reference/publishing.md#cargo-owner) for more information about owners and publishing.

## OPTIONS

### Owner Options

[-a](#option-cargo-owner--a)\
[--add login…](#option-cargo-owner---add)  
Invite the given user or team as an owner.

[-r](#option-cargo-owner--r)\
[--remove login…](#option-cargo-owner---remove)  
Remove the given user or team as an owner.

[-l](#option-cargo-owner--l)\
[--list](#option-cargo-owner---list)  
List owners of a crate.

[--token token](#option-cargo-owner---token)  
API token to use when authenticating. This overrides the token stored in the credentials file (which is created by [cargo-login(1)](cargo-login.md)).

[Cargo config](../reference/config.md) environment variables can be used to override the tokens stored in the credentials file. The token for crates.io may be specified with the `CARGO_REGISTRY_TOKEN` environment variable. Tokens for other registries may be specified with environment variables of the form `CARGO_REGISTRIES_NAME_TOKEN` where `NAME` is the name of the registry in all capital letters.

[--index index](#option-cargo-owner---index)  
The URL of the registry index to use.

[--registry registry](#option-cargo-owner---registry)  
Name of the registry to use. Registry names are defined in [Cargo config files](../reference/config.md). If not specified, the default registry is used, which is defined by the `registry.default` config key which defaults to `crates-io`.

### Display Options

[-v](#option-cargo-owner--v)\
[--verbose](#option-cargo-owner---verbose)  
Use verbose output. May be specified twice for “very verbose” output which includes extra output such as dependency warnings and build script output. May also be specified with the `term.verbose` [config value](../reference/config.md).

[-q](#option-cargo-owner--q)\
[--quiet](#option-cargo-owner---quiet)  
Do not print cargo log messages. May also be specified with the `term.quiet` [config value](../reference/config.md).

[--color when](#option-cargo-owner---color)  
Control when colored output is used. Valid values:

- `auto` (default): Automatically detect if color support is available on the terminal.
- `always`: Always display colors.
- `never`: Never display colors.

May also be specified with the `term.color` [config value](../reference/config.md).

### Common Options

[+toolchain](#option-cargo-owner-+toolchain)  
If Cargo has been installed with rustup, and the first argument to `cargo` begins with `+`, it will be interpreted as a rustup toolchain name (such as `+stable` or `+nightly`). See the [rustup documentation](https://rust-lang.github.io/rustup/overrides.html) for more information about how toolchain overrides work.

[--config KEY=VALUE or PATH](#option-cargo-owner---config)  
Overrides a Cargo configuration value. The argument should be in TOML syntax of `KEY=VALUE`, or provided as a path to an extra configuration file. This flag may be specified multiple times. See the [command-line overrides section](../reference/config.md#command-line-overrides) for more information.

[-C PATH](#option-cargo-owner--C)  
Changes the current working directory before executing any specified operations. This affects things like where cargo looks by default for the project manifest (`Cargo.toml`), as well as the directories searched for discovering `.cargo/config.toml`, for example. This option must appear before the command name, for example `cargo -C path/to/my-project build`.

This option is only available on the [nightly channel](https://doc.rust-lang.org/book/appendix-07-nightly-rust.html) and requires the `-Z unstable-options` flag to enable (see [\#10098](https://github.com/rust-lang/cargo/issues/10098)).

[-h](#option-cargo-owner--h)\
[--help](#option-cargo-owner---help)  
Prints help information.

[-Z flag](#option-cargo-owner--Z)  
Unstable (nightly-only) flags to Cargo. Run `cargo -Z help` for details.

## ENVIRONMENT

See [the reference](../reference/environment-variables.md) for details on environment variables that Cargo reads.

## EXIT STATUS

- `0`: Cargo succeeded.
- `101`: Cargo failed to complete.

## EXAMPLES

1.  List owners of a package:

        cargo owner --list foo

2.  Invite an owner to a package:

        cargo owner --add username foo

3.  Remove an owner from a package:

        cargo owner --remove username foo

## SEE ALSO

[cargo(1)](cargo.md), [cargo-login(1)](cargo-login.md), [cargo-publish(1)](cargo-publish.md)
