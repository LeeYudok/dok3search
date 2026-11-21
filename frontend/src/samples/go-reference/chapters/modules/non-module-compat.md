# Compatibility with non-module repositories

To ensure a smooth transition from `GOPATH` to modules, the `go` command can download and build packages in module-aware mode from repositories that have not migrated to modules by adding a [`go.mod` file](glossary.md).

When the `go` command downloads a module at a given version [directly](vcs.md) from a repository, it looks up a repository URL for the module path, maps the version to a revision within the repository, then extracts an archive of the repository at that revision. If the [module’s path](glossary.md) is equal to the [repository root path](glossary.md), and the repository root directory does not contain a `go.mod` file, the `go` command synthesizes a `go.mod` file in the module cache that contains a [`module` directive](go-mod-file.md#module-directive) and nothing else. Since synthetic `go.mod` files do not contain [`require` directives](go-mod-file.md#require-directive) for their dependencies, other modules that depend on them may need additional `require` directives (with `// indirect` comments) to ensure each dependency is fetched at the same version on every build.

When the `go` command downloads a module from a [proxy](module-proxy.md#communicating-with-proxies), it downloads the `go.mod` file separately from the rest of the module content. The proxy is expected to serve a synthetic `go.mod` file if the original module didn’t have one.

### `+incompatible` versions

A module released at major version 2 or higher must have a matching [major version suffix](modules-overview.md#major-version-suffixes) on its module path. For example, if a module is released at `v2.0.0`, its path must have a `/v2` suffix. This allows the `go` command to treat multiple major versions of a project as distinct modules, even if they’re developed in the same repository.

The major version suffix requirement was introduced when module support was added to the `go` command, and many repositories had already tagged releases with major version `2` or higher before that. To maintain compatibility with these repositories, the `go` command adds an `+incompatible` suffix to versions with major version 2 or higher without a `go.mod` file. `+incompatible` indicates that a version is part of the same module as versions with lower major version numbers; consequently, the `go` command may automatically upgrade to higher `+incompatible` versions even though it may break the build.

Consider the example requirement below:

``` text
require example.com/m v4.1.2+incompatible
```

The version `v4.1.2+incompatible` refers to the [semantic version tag](glossary.md) `v4.1.2` in the repository that provides the module `example.com/m`. The module must be in the repository root directory (that is, the [repository root path](glossary.md) must also be `example.com/m`), and a `go.mod` file must not be present. The module may have versions with lower major version numbers like `v1.5.2`, and the `go` command may upgrade automatically to `v4.1.2+incompatible` from those versions (see [minimal version selection (MVS)](minimal-version-selection.md) for information on how upgrades work).

A repository that migrates to modules after version `v2.0.0` is tagged should usually release a new major version. In the example above, the author should create a module with the path `example.com/m/v5` and should release version `v5.0.0`. The author should also update imports of packages in the module to use the prefix `example.com/m/v5` instead of `example.com/m`. See [Go Modules: v2 and Beyond](https://go.dev/blog/v2-go-modules) for a more detailed example.

Note that the `+incompatible` suffix should not appear on a tag in a repository; a tag like `v4.1.2+incompatible` will be ignored. The suffix only appears in versions used by the `go` command. See [Mapping versions to commits](vcs.md#mapping-versions-to-commits) for details on the distinction between versions and tags.

Note also that the `+incompatible` suffix may appear on [pseudo-versions](glossary.md). For example, `v2.0.1-20200722182040-012345abcdef+incompatible` may be a valid pseudo-version.

### Minimal module compatibility

A module released at major version 2 or higher is required to have a [major version suffix](glossary.md) on its [module path](glossary.md). The module may or may not be developed in a [major version subdirectory](glossary.md) within its repository. This has implications for packages that import packages within the module when building `GOPATH` mode.

Normally in `GOPATH` mode, a package is stored in a directory matching its [repository’s root path](glossary.md) joined with its directory within the repository. For example, a package in the repository with root path `example.com/repo` in the subdirectory `sub` would be stored in `$GOPATH/src/example.com/repo/sub` and would be imported as `example.com/repo/sub`.

For a module with a major version suffix, one might expect to find the package `example.com/repo/v2/sub` in the directory `$GOPATH/src/example.com/repo/v2/sub`. This would require the module to be developed in the `v2` subdirectory of its repository. The `go` command supports this but does not require it (see [Mapping versions to commits](vcs.md#mapping-versions-to-commits)).

If a module is *not* developed in a major version subdirectory, then its directory in `GOPATH` will not contain the major version suffix, and its packages may be imported without the major version suffix. In the example above, the package would be found in the directory `$GOPATH/src/example.com/repo/sub` and would be imported as `example.com/repo/sub`.

This creates a problem for packages intended to be built in both module mode and `GOPATH` mode: module mode requires a suffix, while `GOPATH` mode does not.

To fix this, minimal module compatibility was added in Go 1.11 and was backported to Go 1.9.7 and 1.10.3. When an import path is resolved to a directory in `GOPATH` mode:

- When resolving an import of the form `$modpath/$vn/$dir` where:
  - `$modpath` is a valid module path,
  - `$vn` is a major version suffix,
  - `$dir` is a possibly empty subdirectory,
- If all of the following are true:
  - The package `$modpath/$vn/$dir` is not present in any relevant [`vendor` directory](glossary.md).
  - A `go.mod` file is present in the same directory as the importing file or in any parent directory up to the `$GOPATH/src` root,
  - No `$GOPATH[i]/src/$modpath/$vn/$suffix` directory exists (for any root `$GOPATH[i]`),
  - The file `$GOPATH[d]/src/$modpath/go.mod` exists (for some root `$GOPATH[d]`) and declares the module path as `$modpath/$vn`,
- Then the import of `$modpath/$vn/$dir` is resolved to the directory `$GOPATH[d]/src/$modpath/$dir`.

This rules allow packages that have been migrated to modules to import other packages that have been migrated to modules when built in `GOPATH` mode even when a major version subdirectory was not used.
