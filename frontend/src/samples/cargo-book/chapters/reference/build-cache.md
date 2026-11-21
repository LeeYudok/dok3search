# Build cache

Cargo stores the output of a build into the “target” and “build” directories. By default, both directories point to a directory named `target` in the root of your [*workspace*](../appendix/glossary.md#workspace ""workspace" (glossary entry)"). To change the location of the target-dir, you can set the `CARGO_TARGET_DIR` [environment variable](environment-variables.md), the [`build.target-dir`](config.md#buildtarget-dir) config value, or the `--target-dir` command-line flag. To change the location of the build-dir, you can set the `CARGO_BUILD_BUILD_DIR` [environment variable](environment-variables.md) or the [`build.build-dir`](config.md#buildbuild-dir) config value.

Artifacts are split in two categories:

- Final build artifacts
  - Final build artifacts are output meant for end users of Cargo
  - e.g. binaries for bin crates, output of `cargo doc`, Cargo `--timings` reports
  - Stored in the target-dir
- Intermediate build artifacts
  - Intermediate build artifacts are internal to Cargo and the Rust compiler
  - End users will generally not need to interact with intermediate build artifacts
  - Stored in the Cargo build-dir

The directory layout depends on whether or not you are using the `--target` flag to build for a specific platform. If `--target` is not specified, Cargo runs in a mode where it builds for the host architecture. The output goes into the root of the target directory, with each [profile](profiles.md) stored in a separate subdirectory:

| Directory | Description |
|----|----|
| `target/debug/` | Contains output for the `dev` profile. |
| `target/release/` | Contains output for the `release` profile (with the `--release` option). |
| `target/foo/` | Contains build output for the `foo` profile (with the `--profile=foo` option). |

For historical reasons, the `dev` and `test` profiles are stored in the `debug` directory, and the `release` and `bench` profiles are stored in the `release` directory. User-defined profiles are stored in a directory with the same name as the profile.

When building for another target with `--target`, the output is placed in a directory with the name of the [target](../appendix/glossary.md#target ""target" (glossary entry)"):

| Directory                 | Example                                 |
|---------------------------|-----------------------------------------|
| `target/<tuple>/debug/`   | `target/thumbv7em-none-eabihf/debug/`   |
| `target/<tuple>/release/` | `target/thumbv7em-none-eabihf/release/` |

> ![](../../assets/inline/build-cache-1.svg)Note
>
> When not using `--target`, this has a consequence that Cargo will share your dependencies with build scripts and proc macros. [`RUSTFLAGS`](config.md#buildrustflags) will be shared with every `rustc` invocation. With the `--target` flag, build scripts and proc macros are built separately (for the host architecture), and do not share `RUSTFLAGS`.

Within the profile directory (such as `debug` or `release`), artifacts are placed into the following directories:

| Directory | Description |
|----|----|
| `target/debug/` | Contains the output of the package being built (the [binary executables](cargo-targets.md#binaries) and [library targets](cargo-targets.md#library)). |
| `target/debug/examples/` | Contains [example targets](cargo-targets.md#examples). |

Some commands place their output in dedicated directories in the top level of the `target` directory:

| Directory | Description |
|----|----|
| `target/doc/` | Contains rustdoc documentation ([`cargo doc`](../commands/cargo-doc.md)). |
| `target/package/` | Contains the output of the [`cargo package`](../commands/cargo-package.md). |

Cargo also creates several other directories and files in the build-dir needed for the build process. The build-dir layout is considered internal to Cargo, and is subject to change. Some of these directories are:

| Directory | Description |
|----|----|
| `<build-dir>/debug/incremental/` | `rustc` [incremental output](profiles.md#incremental), a cache used to speed up subsequent builds. |
| `<build-dir>/debug/build/` | Build step results including compilation artifacts and [build script](build-scripts.md) output. |

> ![](../../assets/inline/build-cache-2.svg)Note
>
> The build-dir layout was changed in Cargo 1.100.0, thus prior versions use a different layout. (see [\#15010](https://github.com/rust-lang/cargo/issues/15010))

## Dep-info files

Next to each compiled artifact is a file called a “dep info” file with a `.d` suffix. This file is a Makefile-like syntax that indicates all of the file dependencies required to rebuild the artifact. These are intended to be used with external build systems so that they can detect if Cargo needs to be re-executed. The paths in the file are absolute by default. See the [`build.dep-info-basedir`](config.md#builddep-info-basedir) config option to use relative paths.

``` Makefile
# Example dep-info file found in target/debug/foo.d
/path/to/myproj/target/debug/foo: /path/to/myproj/src/lib.rs /path/to/myproj/src/main.rs
```

## Shared cache

A third party tool, [sccache](https://github.com/mozilla/sccache), can be used to share built dependencies across different workspaces.

To setup `sccache`, install it with `cargo install --locked sccache` and set `RUSTC_WRAPPER` environment variable to `sccache` before invoking Cargo. If you use bash, it makes sense to add `export RUSTC_WRAPPER=sccache` to `.bashrc`. Alternatively, you can set [`build.rustc-wrapper`](config.md#buildrustc-wrapper) in the [Cargo configuration](config.md). Refer to sccache documentation for more details.
