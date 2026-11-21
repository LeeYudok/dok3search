# Glossary

**build constraint:** A condition that determines whether a Go source file is used when compiling a package. Build constraints may be expressed with file name suffixes (for example, `foo_linux_amd64.go`) or with build constraint comments (for example, `// +build linux,amd64`). See [Build Constraints](https://go.dev/pkg/go/build/#hdr-Build_Constraints).

**build list:** The list of module versions that will be used for a build command such as `go build`, `go list`, or `go test`. The build list is determined from the [main module’s](glossary.md) [`go.mod` file](glossary.md) and `go.mod` files in transitively required modules using [minimal version selection](glossary.md). The build list contains versions for all modules in the [module graph](glossary.md), not just those relevant to a specific command.

**canonical version:** A correctly formatted [version](glossary.md) without a build metadata suffix other than `+incompatible`. For example, `v1.2.3` is a canonical version, but `v1.2.3+meta` is not.

**current module:** Synonym for [main module](glossary.md).

**deprecated module:** A module that is no longer supported by its authors (though major versions are considered distinct modules for this purpose). A deprecated module is marked with a [deprecation comment](go-mod-file.md#deprecation) in the latest version of its [`go.mod` file](glossary.md).

**direct dependency:** A package whose path appears in an [`import` declaration](../spec/packages.md#import-declarations) in a `.go` source file for a package or test in the [main module](glossary.md), or the module containing such a package. (Compare [indirect dependency](glossary.md).)

**direct mode:** A setting of [environment variables](environment-variables.md) that causes the `go` command to download a module directly from a [version control system](vcs.md), as opposed to a [module proxy](glossary.md). `GOPROXY=direct` does this for all modules. `GOPRIVATE` and `GONOPROXY` do this for modules matching a list of patterns.

**`go.mod` file:** The file that defines a module’s path, requirements, and other metadata. Appears in the [module’s root directory](glossary.md). See the section on [`go.mod` files](go-mod-file.md).

**`go.work` file** The file that defines the set of modules to be used in a [workspace](workspaces.md). See the section on [`go.work` files](workspaces.md#gowork-files)

**import path:** A string used to import a package in a Go source file. Synonymous with [package path](glossary.md).

**indirect dependency:** A package transitively imported by a package or test in the [main module](glossary.md), but whose path does not appear in any [`import` declaration](../spec/packages.md#import-declarations) in the main module; or a module that appears in the [module graph](glossary.md) but does not provide any package directly imported by the main module. (Compare [direct dependency](glossary.md).)

**lazy module loading:** A change in Go 1.17 that avoids loading the [module graph](glossary.md) for commands that do not need it in modules that specify `go 1.17` or higher. See [Lazy module loading](graph-pruning.md#lazy-module-loading).

**main module:** The module in which the `go` command is invoked. The main module is defined by a [`go.mod` file](glossary.md) in the current directory or a parent directory. See [Modules, packages, and versions](modules-overview.md).

**major version:** The first number in a semantic version (`1` in `v1.2.3`). In a release with incompatible changes, the major version must be incremented, and the minor and patch versions must be set to 0. Semantic versions with major version 0 are considered unstable.

**major version subdirectory:** A subdirectory within a version control repository matching a module’s [major version suffix](glossary.md) where a module may be defined. For example, the module `example.com/mod/v2` in the repository with [root path](glossary.md) `example.com/mod` may be defined in the repository root directory or the major version subdirectory `v2`. See [Module directories within a repository](vcs.md#module-directories-within-a-repository).

**major version suffix:** A module path suffix that matches the major version number. For example, `/v2` in `example.com/mod/v2`. Major version suffixes are required at `v2.0.0` and later and are not allowed at earlier versions. See the section on [Major version suffixes](modules-overview.md#major-version-suffixes).

**minimal version selection (MVS):** The algorithm used to determine the versions of all modules that will be used in a build. See the section on [Minimal version selection](minimal-version-selection.md) for details.

**minor version:** The second number in a semantic version (`2` in `v1.2.3`). In a release with new, backwards compatible functionality, the minor version must be incremented, and the patch version must be set to 0.

**module:** A collection of packages that are released, versioned, and distributed together.

**module cache:** A local directory storing downloaded modules, located in `GOPATH/pkg/mod`. See [Module cache](module-cache.md).

**module graph:** The directed graph of module requirements, rooted at the [main module](glossary.md). Each vertex in the graph is a module; each edge is a version from a `require` statement in a `go.mod` file (subject to `replace` and `exclude` statements in the main module’s `go.mod` file).

**module graph pruning:** A change in Go 1.17 that reduces the size of the module graph by omitting transitive dependencies of modules that specify `go 1.17` or higher. See [Module graph pruning](graph-pruning.md).

**module path:** A path that identifies a module and acts as a prefix for package import paths within the module. For example, `"golang.org/x/net"`.

**module proxy:** A web server that implements the [`GOPROXY` protocol](module-proxy.md#goproxy-protocol). The `go` command downloads version information, `go.mod` files, and module zip files from module proxies.

**module root directory:** The directory that contains the `go.mod` file that defines a module.

**module subdirectory:** The portion of a [module path](glossary.md) after the [repository root path](glossary.md) that indicates the subdirectory where the module is defined. When non-empty, the module subdirectory is also a prefix for [semantic version tags](glossary.md). The module subdirectory does not include the [major version suffix](glossary.md), if there is one, even if the module is in a [major version subdirectory](glossary.md). See [Module paths](modules-overview.md#module-paths).

**package:** A collection of source files in the same directory that are compiled together. See the [Packages section](../spec/packages.md) in the Go Language Specification.

**package path:** The path that uniquely identifies a package. A package path is a [module path](glossary.md) joined with a subdirectory within the module. For example `"golang.org/x/net/html"` is the package path for the package in the module `"golang.org/x/net"` in the `"html"` subdirectory. Synonym of [import path](glossary.md).

**patch version:** The third number in a semantic version (`3` in `v1.2.3`). In a release with no changes to the module’s public interface, the patch version must be incremented.

**pre-release version:** A version with a dash followed by a series of dot-separated identifiers immediately following the patch version, for example, `v1.2.3-beta4`. Pre-release versions are considered unstable and are not assumed to be compatible with other versions. A pre-release version sorts before the corresponding release version: `v1.2.3-pre` comes before `v1.2.3`. See also [release version](glossary.md).

**pseudo-version:** A version that encodes a revision identifier (such as a Git commit hash) and a timestamp from a version control system. For example, `v0.0.0-20191109021931-daa7c04131f5`. Used for [compatibility with non-module repositories](non-module-compat.md) and in other situations when a tagged version is not available.

**release version:** A version without a pre-release suffix. For example, `v1.2.3`, not `v1.2.3-pre`. See also [pre-release version](glossary.md).

**repository root path:** The portion of a [module path](glossary.md) that corresponds to a version control repository’s root directory. See [Module paths](modules-overview.md#module-paths).

**retracted version:** A version that should not be depended upon, either because it was published prematurely or because a severe problem was discovered after it was published. See [`retract` directive](go-mod-file.md#retract-directive).

**semantic version tag:** A tag in a version control repository that maps a [version](glossary.md) to a specific revision. See [Mapping versions to commits](vcs.md#mapping-versions-to-commits).

**selected version:** The version of a given module chosen by [minimal version selection](minimal-version-selection.md). The selected version is the highest version for the module’s path found in the [module graph](glossary.md).

**vendor directory:** A directory named `vendor` that contains packages from other modules needed to build packages in the main module. Maintained with [`go mod vendor`](mod-commands.md#go-mod-vendor). See [Vendoring](mod-commands.md#vendoring).

**version:** An identifier for an immutable snapshot of a module, written as the letter `v` followed by a semantic version. See the section on [Versions](modules-overview.md#versions).

**workspace:** A collection of modules on disk that are used as the main modules when running [minimal version selection (MVS)](minimal-version-selection.md). See the section on [Workspaces](workspaces.md)
