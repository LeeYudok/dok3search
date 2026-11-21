# cargo-locate-project(1)

## NAME

cargo-locate-project — Print a JSON representation of a Cargo.toml file’s location

## SYNOPSIS

`cargo locate-project` \[*options*\]

## DESCRIPTION

This command will print a JSON object to stdout with the full path to the manifest. The manifest is found by searching upward for a file named `Cargo.toml` starting from the current working directory.

If the project happens to be a part of a workspace, the manifest of the project, rather than the workspace root, is output. This can be overridden by the `--workspace` flag. The root workspace is found by traversing further upward or by using the field `package.workspace` after locating the manifest of a workspace member.

## OPTIONS

[--workspace](#option-cargo-locate-project---workspace)  
Locate the `Cargo.toml` at the root of the workspace, as opposed to the current workspace member.

### Display Options

[--message-format fmt](#option-cargo-locate-project---message-format)  
The representation in which to print the project location. Valid values:

- `json` (default): JSON object with the path under the key “root”.
- `plain`: Just the path.

[-v](#option-cargo-locate-project--v)\
[--verbose](#option-cargo-locate-project---verbose)  
Use verbose output. May be specified twice for “very verbose” output which includes extra output such as dependency warnings and build script output. May also be specified with the `term.verbose` [config value](../reference/config.md).

[-q](#option-cargo-locate-project--q)\
[--quiet](#option-cargo-locate-project---quiet)  
Do not print cargo log messages. May also be specified with the `term.quiet` [config value](../reference/config.md).

[--color when](#option-cargo-locate-project---color)  
Control when colored output is used. Valid values:

- `auto` (default): Automatically detect if color support is available on the terminal.
- `always`: Always display colors.
- `never`: Never display colors.

May also be specified with the `term.color` [config value](../reference/config.md).

### Manifest Options

[--manifest-path path](#option-cargo-locate-project---manifest-path)  
Path to the `Cargo.toml` file. By default, Cargo searches for the `Cargo.toml` file in the current directory or any parent directory.

### Common Options

[+toolchain](#option-cargo-locate-project-+toolchain)  
If Cargo has been installed with rustup, and the first argument to `cargo` begins with `+`, it will be interpreted as a rustup toolchain name (such as `+stable` or `+nightly`). See the [rustup documentation](https://rust-lang.github.io/rustup/overrides.html) for more information about how toolchain overrides work.

[--config KEY=VALUE or PATH](#option-cargo-locate-project---config)  
Overrides a Cargo configuration value. The argument should be in TOML syntax of `KEY=VALUE`, or provided as a path to an extra configuration file. This flag may be specified multiple times. See the [command-line overrides section](../reference/config.md#command-line-overrides) for more information.

[-C PATH](#option-cargo-locate-project--C)  
Changes the current working directory before executing any specified operations. This affects things like where cargo looks by default for the project manifest (`Cargo.toml`), as well as the directories searched for discovering `.cargo/config.toml`, for example. This option must appear before the command name, for example `cargo -C path/to/my-project build`.

This option is only available on the [nightly channel](https://doc.rust-lang.org/book/appendix-07-nightly-rust.html) and requires the `-Z unstable-options` flag to enable (see [\#10098](https://github.com/rust-lang/cargo/issues/10098)).

[-h](#option-cargo-locate-project--h)\
[--help](#option-cargo-locate-project---help)  
Prints help information.

[-Z flag](#option-cargo-locate-project--Z)  
Unstable (nightly-only) flags to Cargo. Run `cargo -Z help` for details.

## ENVIRONMENT

See [the reference](../reference/environment-variables.md) for details on environment variables that Cargo reads.

## EXIT STATUS

- `0`: Cargo succeeded.
- `101`: Cargo failed to complete.

## EXAMPLES

1.  Display the path to the manifest based on the current directory:

        cargo locate-project

## SEE ALSO

[cargo(1)](cargo.md), [cargo-metadata(1)](cargo-metadata.md)
