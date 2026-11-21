# cargo-add(1)

## NAME

cargo-add — Add dependencies to a Cargo.toml manifest file

## SYNOPSIS

`cargo add` \[*options*\] *crate*…\
`cargo add` \[*options*\] `--path` *path*\
`cargo add` \[*options*\] `--git` *url* \[*crate*…\]

## DESCRIPTION

This command can add or modify dependencies.

The source for the dependency can be specified with:

- *crate*`@`*version*: Fetch from a registry with a version constraint of “*version*”
- `--path` *path*: Fetch from the specified *path*
- `--git` *url*: Pull from a git repo at *url*

If no source is specified, then a best effort will be made to select one, including:

- Existing dependencies in other tables (like `dev-dependencies`)
- Workspace members
- Latest release in the registry

When you add a package that is already present, the existing entry will be updated with the flags specified.

Upon successful invocation, the enabled (`+`) and disabled (`-`) [features](../reference/features.md) of the specified dependency will be listed in the command’s output.

## OPTIONS

### Source options

[--git url](#option-cargo-add---git)  
[Git URL to add the specified crate from](../reference/specifying-dependencies.md#specifying-dependencies-from-git-repositories).

[--branch branch](#option-cargo-add---branch)  
Branch to use when adding from git.

[--tag tag](#option-cargo-add---tag)  
Tag to use when adding from git.

[--rev sha](#option-cargo-add---rev)  
Specific commit to use when adding from git.

[--path path](#option-cargo-add---path)  
[Filesystem path](../reference/specifying-dependencies.md#specifying-path-dependencies) to local crate to add.

[--base base](#option-cargo-add---base)  
The [path base](../reference/unstable.md#path-bases) to use when adding a local crate.

[Unstable (nightly-only)](../reference/unstable.md#path-bases)

[--registry registry](#option-cargo-add---registry)  
Name of the registry to use. Registry names are defined in [Cargo config files](../reference/config.md). If not specified, the default registry is used, which is defined by the `registry.default` config key which defaults to `crates-io`.

### Section options

[--dev](#option-cargo-add---dev)  
Add as a [development dependency](../reference/specifying-dependencies.md#development-dependencies).

[--build](#option-cargo-add---build)  
Add as a [build dependency](../reference/specifying-dependencies.md#build-dependencies).

[--target target](#option-cargo-add---target)  
Add as a dependency to the [given target platform](../reference/specifying-dependencies.md#platform-specific-dependencies).

To avoid unexpected shell expansions, you may use quotes around each target, e.g., `--target 'cfg(unix)'`.

### Dependency options

[--dry-run](#option-cargo-add---dry-run)  
Don’t actually write the manifest

[--rename name](#option-cargo-add---rename)  
[Rename](../reference/specifying-dependencies.md#renaming-dependencies-in-cargotoml) the dependency.

[--optional](#option-cargo-add---optional)  
Mark the dependency as [optional](../reference/features.md#optional-dependencies).

[--no-optional](#option-cargo-add---no-optional)  
Mark the dependency as [required](../reference/features.md#optional-dependencies).

[--public](#option-cargo-add---public)  
Mark the dependency as public.

The dependency can be referenced in your library’s public API.

[Unstable (nightly-only)](../reference/unstable.md#public-dependency)

[--no-public](#option-cargo-add---no-public)  
Mark the dependency as private.

While you can use the crate in your implementation, it cannot be referenced in your public API.

[Unstable (nightly-only)](../reference/unstable.md#public-dependency)

[--no-default-features](#option-cargo-add---no-default-features)  
Disable the [default features](../reference/features.md#dependency-features).

[--default-features](#option-cargo-add---default-features)  
Re-enable the [default features](../reference/features.md#dependency-features).

[-F features](#option-cargo-add--F)\
[--features features](#option-cargo-add---features)  
Space or comma separated list of [features to activate](../reference/features.md#dependency-features). When adding multiple crates, the features for a specific crate may be enabled with `package-name/feature-name` syntax. This flag may be specified multiple times, which enables all specified features.

### Display Options

[-v](#option-cargo-add--v)\
[--verbose](#option-cargo-add---verbose)  
Use verbose output. May be specified twice for “very verbose” output which includes extra output such as dependency warnings and build script output. May also be specified with the `term.verbose` [config value](../reference/config.md).

[-q](#option-cargo-add--q)\
[--quiet](#option-cargo-add---quiet)  
Do not print cargo log messages. May also be specified with the `term.quiet` [config value](../reference/config.md).

[--color when](#option-cargo-add---color)  
Control when colored output is used. Valid values:

- `auto` (default): Automatically detect if color support is available on the terminal.
- `always`: Always display colors.
- `never`: Never display colors.

May also be specified with the `term.color` [config value](../reference/config.md).

### Manifest Options

[--manifest-path path](#option-cargo-add---manifest-path)  
Path to the `Cargo.toml` file. By default, Cargo searches for the `Cargo.toml` file in the current directory or any parent directory.

[-p spec](#option-cargo-add--p)\
[--package spec](#option-cargo-add---package)  
Add dependencies to only the specified package.

[--ignore-rust-version](#option-cargo-add---ignore-rust-version)  
Ignore `rust-version` specification in packages.

[--locked](#option-cargo-add---locked)  
Asserts that the exact same dependencies and versions are used as when the existing `Cargo.lock` file was originally generated. Cargo will exit with an error when either of the following scenarios arises:

- The lock file is missing.
- Cargo attempted to change the lock file due to a different dependency resolution.

It may be used in environments where deterministic builds are desired, such as in CI pipelines.

[--offline](#option-cargo-add---offline)  
Prevents Cargo from accessing the network for any reason. Without this flag, Cargo will stop with an error if it needs to access the network and the network is not available. With this flag, Cargo will attempt to proceed without the network if possible.

Beware that this may result in different dependency resolution than online mode. Cargo will restrict itself to crates that are downloaded locally, even if there might be a newer version as indicated in the local copy of the index. See the [cargo-fetch(1)](cargo-fetch.md) command to download dependencies before going offline.

May also be specified with the `net.offline` [config value](../reference/config.md).

[--frozen](#option-cargo-add---frozen)  
Equivalent to specifying both `--locked` and `--offline`.

### Common Options

[+toolchain](#option-cargo-add-+toolchain)  
If Cargo has been installed with rustup, and the first argument to `cargo` begins with `+`, it will be interpreted as a rustup toolchain name (such as `+stable` or `+nightly`). See the [rustup documentation](https://rust-lang.github.io/rustup/overrides.html) for more information about how toolchain overrides work.

[--config KEY=VALUE or PATH](#option-cargo-add---config)  
Overrides a Cargo configuration value. The argument should be in TOML syntax of `KEY=VALUE`, or provided as a path to an extra configuration file. This flag may be specified multiple times. See the [command-line overrides section](../reference/config.md#command-line-overrides) for more information.

[-C PATH](#option-cargo-add--C)  
Changes the current working directory before executing any specified operations. This affects things like where cargo looks by default for the project manifest (`Cargo.toml`), as well as the directories searched for discovering `.cargo/config.toml`, for example. This option must appear before the command name, for example `cargo -C path/to/my-project build`.

This option is only available on the [nightly channel](https://doc.rust-lang.org/book/appendix-07-nightly-rust.html) and requires the `-Z unstable-options` flag to enable (see [\#10098](https://github.com/rust-lang/cargo/issues/10098)).

[-h](#option-cargo-add--h)\
[--help](#option-cargo-add---help)  
Prints help information.

[-Z flag](#option-cargo-add--Z)  
Unstable (nightly-only) flags to Cargo. Run `cargo -Z help` for details.

## ENVIRONMENT

See [the reference](../reference/environment-variables.md) for details on environment variables that Cargo reads.

## EXIT STATUS

- `0`: Cargo succeeded.
- `101`: Cargo failed to complete.

## EXAMPLES

1.  Add `regex` as a dependency

        cargo add regex

2.  Add `trybuild` as a dev-dependency

        cargo add --dev trybuild

3.  Add an older version of `nom` as a dependency

        cargo add nom@5

4.  Add support for serializing data structures to json with `derive`s

        cargo add serde serde_json -F serde/derive

5.  Add `windows` as a platform specific dependency on `cfg(windows)`

        cargo add windows --target 'cfg(windows)'

## SEE ALSO

[cargo(1)](cargo.md), [cargo-remove(1)](cargo-remove.md)
