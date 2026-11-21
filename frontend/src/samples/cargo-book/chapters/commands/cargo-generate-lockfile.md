# cargo-generate-lockfile(1)

## NAME

cargo-generate-lockfile — Generate the lockfile for a package

## SYNOPSIS

`cargo generate-lockfile` \[*options*\]

## DESCRIPTION

This command will create the `Cargo.lock` lockfile for the current package or workspace. If the lockfile already exists, it will be rebuilt with the latest available version of every package.

See also [cargo-update(1)](cargo-update.md) which is also capable of creating a `Cargo.lock` lockfile and has more options for controlling update behavior.

## OPTIONS

### Display Options

[-v](#option-cargo-generate-lockfile--v)\
[--verbose](#option-cargo-generate-lockfile---verbose)  
Use verbose output. May be specified twice for “very verbose” output which includes extra output such as dependency warnings and build script output. May also be specified with the `term.verbose` [config value](../reference/config.md).

[-q](#option-cargo-generate-lockfile--q)\
[--quiet](#option-cargo-generate-lockfile---quiet)  
Do not print cargo log messages. May also be specified with the `term.quiet` [config value](../reference/config.md).

[--color when](#option-cargo-generate-lockfile---color)  
Control when colored output is used. Valid values:

- `auto` (default): Automatically detect if color support is available on the terminal.
- `always`: Always display colors.
- `never`: Never display colors.

May also be specified with the `term.color` [config value](../reference/config.md).

### Manifest Options

[--manifest-path path](#option-cargo-generate-lockfile---manifest-path)  
Path to the `Cargo.toml` file. By default, Cargo searches for the `Cargo.toml` file in the current directory or any parent directory.

[--ignore-rust-version](#option-cargo-generate-lockfile---ignore-rust-version)  
Ignore `rust-version` specification in packages.

[--publish-time yyyy-mm-ddThh:mm:ssZ](#option-cargo-generate-lockfile---publish-time)  
Latest publish time allowed for registry packages (Unstable)

This is a best-effort filter on allowed packages, including:

- packages from unsupported registries are always accepted
- only the current yank state is respected, not the state as of `--publish-time`
- precision of the publish time

[--locked](#option-cargo-generate-lockfile---locked)  
Asserts that the exact same dependencies and versions are used as when the existing `Cargo.lock` file was originally generated. Cargo will exit with an error when either of the following scenarios arises:

- The lock file is missing.
- Cargo attempted to change the lock file due to a different dependency resolution.

It may be used in environments where deterministic builds are desired, such as in CI pipelines.

[--offline](#option-cargo-generate-lockfile---offline)  
Prevents Cargo from accessing the network for any reason. Without this flag, Cargo will stop with an error if it needs to access the network and the network is not available. With this flag, Cargo will attempt to proceed without the network if possible.

Beware that this may result in different dependency resolution than online mode. Cargo will restrict itself to crates that are downloaded locally, even if there might be a newer version as indicated in the local copy of the index. See the [cargo-fetch(1)](cargo-fetch.md) command to download dependencies before going offline.

May also be specified with the `net.offline` [config value](../reference/config.md).

[--frozen](#option-cargo-generate-lockfile---frozen)  
Equivalent to specifying both `--locked` and `--offline`.

### Common Options

[+toolchain](#option-cargo-generate-lockfile-+toolchain)  
If Cargo has been installed with rustup, and the first argument to `cargo` begins with `+`, it will be interpreted as a rustup toolchain name (such as `+stable` or `+nightly`). See the [rustup documentation](https://rust-lang.github.io/rustup/overrides.html) for more information about how toolchain overrides work.

[--config KEY=VALUE or PATH](#option-cargo-generate-lockfile---config)  
Overrides a Cargo configuration value. The argument should be in TOML syntax of `KEY=VALUE`, or provided as a path to an extra configuration file. This flag may be specified multiple times. See the [command-line overrides section](../reference/config.md#command-line-overrides) for more information.

[-C PATH](#option-cargo-generate-lockfile--C)  
Changes the current working directory before executing any specified operations. This affects things like where cargo looks by default for the project manifest (`Cargo.toml`), as well as the directories searched for discovering `.cargo/config.toml`, for example. This option must appear before the command name, for example `cargo -C path/to/my-project build`.

This option is only available on the [nightly channel](https://doc.rust-lang.org/book/appendix-07-nightly-rust.html) and requires the `-Z unstable-options` flag to enable (see [\#10098](https://github.com/rust-lang/cargo/issues/10098)).

[-h](#option-cargo-generate-lockfile--h)\
[--help](#option-cargo-generate-lockfile---help)  
Prints help information.

[-Z flag](#option-cargo-generate-lockfile--Z)  
Unstable (nightly-only) flags to Cargo. Run `cargo -Z help` for details.

## ENVIRONMENT

See [the reference](../reference/environment-variables.md) for details on environment variables that Cargo reads.

## EXIT STATUS

- `0`: Cargo succeeded.
- `101`: Cargo failed to complete.

## EXAMPLES

1.  Create or update the lockfile for the current package or workspace:

        cargo generate-lockfile

## SEE ALSO

[cargo(1)](cargo.md), [cargo-update(1)](cargo-update.md)
