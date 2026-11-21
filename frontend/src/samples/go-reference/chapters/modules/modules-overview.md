# Modules, packages, and versions

A module is a collection of packages that are released, versioned, and distributed together. Modules may be downloaded directly from version control repositories or from module proxy servers.

A module is identified by a [module path](glossary.md), which is declared in a [`go.mod` file](go-mod-file.md), together with information about the module’s dependencies. The module root directory is the directory that contains the `go.mod` file. The main module is the module containing the directory where the `go` command is invoked.

Each package within a module is a collection of source files in the same directory that are compiled together. A package path is the module path joined with the subdirectory containing the package (relative to the module root). For example, the module `"golang.org/x/net"` contains a package in the directory `"html"`. That package’s path is `"golang.org/x/net/html"`.

### Module paths

A module path is the canonical name for a module, declared with the [`module` directive](go-mod-file.md#module-directive) in the module’s [`go.mod` file](glossary.md). A module’s path is the prefix for package paths within the module.

A module path should describe both what the module does and where to find it. Typically, a module path consists of a repository root path, a directory within the repository (usually empty), and a major version suffix (only for major version 2 or higher).

- The repository root path is the portion of the module path that corresponds to the root directory of the version control repository where the module is developed. Most modules are defined in their repository’s root directory, so this is usually the entire path. For example, `golang.org/x/net` is the repository root path for the module of the same name. See [Finding a repository for a module path](vcs.md#finding-a-repository-for-a-module-path) for information on how the `go` command locates a repository using HTTP requests derived from a module path.
- If the module is not defined in the repository’s root directory, the module subdirectory is the part of the module path that names the directory, not including the major version suffix. This also serves as a prefix for semantic version tags. For example, the module `golang.org/x/tools/gopls` is in the `gopls` subdirectory of the repository with root path `golang.org/x/tools`, so it has the module subdirectory `gopls`. See [Mapping versions to commits](vcs.md#mapping-versions-to-commits) and [Module directories within a repository](vcs.md#module-directories-within-a-repository).
- If the module is released at major version 2 or higher, the module path must end with a [major version suffix](#major-version-suffixes) like `/v2`. This may or may not be part of the subdirectory name. For example, the module with path `golang.org/x/repo/sub/v2` could be in the `/sub` or `/sub/v2` subdirectory of the repository `golang.org/x/repo`.

If a module might be depended on by other modules, these rules must be followed so that the `go` command can find and download the module. There are also several [lexical restrictions](go-mod-file.md#module-paths-and-versions) on characters allowed in module paths.

A module that will never be fetched as a dependency of any other module may use any valid package path for its module path, but must take care not to collide with paths that may be used by the module’s dependencies or the Go standard library. The Go standard library uses package paths that do not contain a dot in the first path element, and the `go` command does not attempt to resolve such paths from network servers. The paths `example` and `test` are reserved for users: they will not be used in the standard library and are suitable for use in self-contained modules, such as those defined in tutorials or example code or created and manipulated as part of a test.

### Versions

A version identifies an immutable snapshot of a module, which may be either a [release](glossary.md) or a [pre-release](glossary.md). Each version starts with the letter `v`, followed by a semantic version. See [Semantic Versioning 2.0.0](https://semver.org/spec/v2.0.0.html) for details on how versions are formatted, interpreted, and compared.

To summarize, a semantic version consists of three non-negative integers (the major, minor, and patch versions, from left to right) separated by dots. The patch version may be followed by an optional pre-release string starting with a hyphen. The pre-release string or patch version may be followed by a build metadata string starting with a plus. For example, `v0.0.0`, `v1.12.134`, `v8.0.5-pre`, and `v2.0.9+meta` are valid versions.

Each part of a version indicates whether the version is stable and whether it is compatible with previous versions.

- The [major version](glossary.md) must be incremented and the minor and patch versions must be set to zero after a backwards incompatible change is made to the module’s public interface or documented functionality, for example, after a package is removed.
- The [minor version](glossary.md) must be incremented and the patch version set to zero after a backwards compatible change, for example, after a new function is added.
- The [patch version](glossary.md) must be incremented after a change that does not affect the module’s public interface, such as a bug fix or optimization.
- The pre-release suffix indicates a version is a [pre-release](glossary.md). Pre-release versions sort before the corresponding release versions. For example, `v1.2.3-pre` comes before `v1.2.3`.
- The build metadata suffix is ignored for the purpose of comparing versions. The go command accepts versions with build metadata and converts them to pseudo-versions to maintain the total ordering between versions.
  - The special suffix `+incompatible` denotes a version released before migrating to modules version major version 2 or later (see [Compatibility with non-module repositories](non-module-compat.md)).
  - The special suffix `+dirty` is appended to the version information of a binary when it’s built with a Go toolchain 1.24 or later within a valid local Version Control System (VCS) repository that contains uncommitted changes in the working directory.

A version is considered unstable if its major version is 0 or it has a pre-release suffix. Unstable versions are not subject to compatibility requirements. For example, `v0.2.0` may not be compatible with `v0.1.0`, and `v1.5.0-beta` may not be compatible with `v1.5.0`.

Go may access modules in version control systems using tags, branches, or revisions that don’t follow these conventions. However, within the main module, the `go` command will automatically convert revision names that don’t follow this standard into canonical versions. The `go` command will also remove build metadata suffixes (except for `+incompatible`) as part of this process. This may result in a [pseudo-version](glossary.md), a pre-release version that encodes a revision identifier (such as a Git commit hash) and a timestamp from a version control system. For example, the command `go get golang.org/x/net@daa7c041` will convert the commit hash `daa7c041` into the pseudo-version `v0.0.0-20191109021931-daa7c04131f5`. Canonical versions are required outside the main module, and the `go` command will report an error if a non-canonical version like `master` appears in a `go.mod` file.

### Pseudo-versions

A pseudo-version is a specially formatted [pre-release](glossary.md) [version](glossary.md) that encodes information about a specific revision in a version control repository. For example, `v0.0.0-20191109021931-daa7c04131f5` is a pseudo-version.

Pseudo-versions may refer to revisions for which no [semantic version tags](glossary.md) are available. They may be used to test commits before creating version tags, for example, on a development branch.

Each pseudo-version has three parts:

- A base version prefix (`vX.0.0` or `vX.Y.Z-0`), which is either derived from a semantic version tag that precedes the revision or `vX.0.0` if there is no such tag.
- A timestamp (`yyyymmddhhmmss`), which is the UTC time the revision was created. In Git, this is the commit time, not the author time.
- A revision identifier (`abcdefabcdef`), which is a 12-character prefix of the commit hash, or in Subversion, a zero-padded revision number.

Each pseudo-version may be in one of three forms, depending on the base version. These forms ensure that a pseudo-version compares higher than its base version, but lower than the next tagged version.

- `vX.0.0-yyyymmddhhmmss-abcdefabcdef` is used when there is no known base version. As with all versions, the major version `X` must match the module’s [major version suffix](glossary.md).
- `vX.Y.Z-pre.0.yyyymmddhhmmss-abcdefabcdef` is used when the base version is a pre-release version like `vX.Y.Z-pre`.
- `vX.Y.(Z+1)-0.yyyymmddhhmmss-abcdefabcdef` is used when the base version is a release version like `vX.Y.Z`. For example, if the base version is `v1.2.3`, a pseudo-version might be `v1.2.4-0.20191109021931-daa7c04131f5`.

More than one pseudo-version may refer to the same commit by using different base versions. This happens naturally when a lower version is tagged after a pseudo-version is written.

These forms give pseudo-versions two useful properties:

- Pseudo-versions with known base versions sort higher than those versions but lower than other pre-release for later versions.
- Pseudo-versions with the same base version prefix sort chronologically.

The `go` command performs several checks to ensure that module authors have control over how pseudo-versions are compared with other versions and that pseudo-versions refer to revisions that are actually part of a module’s commit history.

- If a base version is specified, there must be a corresponding semantic version tag that is an ancestor of the revision described by the pseudo-version. This prevents developers from bypassing [minimal version selection](glossary.md) using a pseudo-version that compares higher than all tagged versions like `v1.999.999-99999999999999-daa7c04131f5`.
- The timestamp must match the revision’s timestamp. This prevents attackers from flooding [module proxies](glossary.md) with an unbounded number of otherwise identical pseudo-versions. This also prevents module consumers from changing the relative ordering of versions.
- The revision must be an ancestor of one of the module repository’s branches or tags. This prevents attackers from referring to unapproved changes or pull requests.

Pseudo-versions never need to be typed by hand. Many commands accept a commit hash or a branch name and will translate it into a pseudo-version (or tagged version if available) automatically. For example:

``` text
go get example.com/mod@master
go list -m -json example.com/mod@abcd1234
```

### Major version suffixes

Starting with major version 2, module paths must have a major version suffix like `/v2` that matches the major version. For example, if a module has the path `example.com/mod` at `v1.0.0`, it must have the path `example.com/mod/v2` at version `v2.0.0`.

Major version suffixes implement the [import compatibility rule](https://research.swtch.com/vgo-import):

> If an old package and a new package have the same import path, the new package must be backwards compatible with the old package.

By definition, packages in a new major version of a module are not backwards compatible with the corresponding packages in the previous major version. Consequently, starting with `v2`, packages need new import paths. This is accomplished by adding a major version suffix to the module path. Since the module path is a prefix of the import path for each package within the module, adding the major version suffix to the module path provides a distinct import path for each incompatible version.

Major version suffixes are not allowed at major versions `v0` or `v1`. There is no need to change the module path between `v0` and `v1` because `v0` versions are unstable and have no compatibility guarantee. Additionally, for most modules, `v1` is backwards compatible with the last `v0` version; a `v1` version acts as a commitment to compatibility, rather than an indication of incompatible changes compared with `v0`.

As a special case, modules paths starting with `gopkg.in/` must always have a major version suffix, even at `v0` and `v1`. The suffix must start with a dot rather than a slash (for example, `gopkg.in/yaml.v2`).

Major version suffixes let multiple major versions of a module coexist in the same build. This may be necessary due to a [diamond dependency problem](https://research.swtch.com/vgo-import#dependency_story). Ordinarily, if a module is required at two different versions by transitive dependencies, the higher version will be used. However, if the two versions are incompatible, neither version will satisfy all clients. Since incompatible versions must have different major version numbers, they must also have different module paths due to major version suffixes. This resolves the conflict: modules with distinct suffixes are treated as separate modules, and their packages—even packages in same subdirectory relative to their module roots—are distinct.

Many Go projects released versions at `v2` or higher without using a major version suffix before migrating to modules (perhaps before modules were even introduced). These versions are annotated with a `+incompatible` build tag (for example, `v2.0.0+incompatible`). See [Compatibility with non-module repositories](non-module-compat.md) for more information.

### Resolving a package to a module

When the `go` command loads a package using a [package path](glossary.md), it needs to determine which module provides the package.

The `go` command starts by searching the [build list](glossary.md) for modules with paths that are prefixes of the package path. For example, if the package `example.com/a/b` is imported, and the module `example.com/a` is in the build list, the `go` command will check whether `example.com/a` contains the package, in the directory `b`. At least one file with the `.go` extension must be present in a directory for it to be considered a package. [Build constraints](https://go.dev/pkg/go/build/#hdr-Build_Constraints) are not applied for this purpose. If exactly one module in the build list provides the package, that module is used. If no modules provide the package or if two or more modules provide the package, the `go` command reports an error. The `-mod=mod` flag instructs the `go` command to attempt to find new modules providing missing packages and to update `go.mod` and `go.sum`. The [`go get`](mod-commands.md#go-get) and [`go mod tidy`](mod-commands.md#go-mod-tidy) commands do this automatically.

When the `go` command looks up a new module for a package path, it checks the `GOPROXY` environment variable, which is a comma-separated list of proxy URLs or the keywords `direct` or `off`. A proxy URL indicates the `go` command should contact a [module proxy](glossary.md) using the [`GOPROXY` protocol](module-proxy.md#goproxy-protocol). `direct` indicates that the `go` command should [communicate with a version control system](vcs.md). `off` indicates that no communication should be attempted. The `GOPRIVATE` and `GONOPROXY` [environment variables](environment-variables.md) can also be used to control this behavior.

For each entry in the `GOPROXY` list, the `go` command requests the latest version of each module path that might provide the package (that is, each prefix of the package path). For each successfully requested module path, the `go` command will download the module at the latest version and check whether the module contains the requested package. If one or more modules contain the requested package, the module with the longest path is used. If one or more modules are found but none contain the requested package, an error is reported. If no modules are found, the `go` command tries the next entry in the `GOPROXY` list. If no entries are left, an error is reported.

For example, suppose the `go` command is looking for a module that provides the package `golang.org/x/net/html`, and `GOPROXY` is set to `https://corp.example.com,https://proxy.golang.org`. The `go` command may make the following requests:

- To `https://corp.example.com/` (in parallel):
  - Request for latest version of `golang.org/x/net/html`
  - Request for latest version of `golang.org/x/net`
  - Request for latest version of `golang.org/x`
  - Request for latest version of `golang.org`
- To `https://proxy.golang.org/`, if all requests to `https://corp.example.com/` have failed with 404 or 410:
  - Request for latest version of `golang.org/x/net/html`
  - Request for latest version of `golang.org/x/net`
  - Request for latest version of `golang.org/x`
  - Request for latest version of `golang.org`

After a suitable module has been found, the `go` command will add a new [requirement](go-mod-file.md#require-directive) with the new module’s path and version to the main module’s `go.mod` file. This ensures that when the same package is loaded in the future, the same module will be used at the same version. If the resolved package is not imported by a package in the main module, the new requirement will have an `// indirect` comment.
