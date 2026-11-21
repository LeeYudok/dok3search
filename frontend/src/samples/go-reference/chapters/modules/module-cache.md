# Module cache

The module cache is the directory where the `go` command stores downloaded module files. The module cache is distinct from the build cache, which contains compiled packages and other build artifacts.

The default location of the module cache is `$GOPATH/pkg/mod`. To use a different location, set the `GOMODCACHE` [environment variable](environment-variables.md).

The module cache has no maximum size, and the `go` command does not remove its contents automatically.

The cache may be shared by multiple Go projects developed on the same machine. The `go` command will use the same cache regardless of the location of the main module. Multiple instances of the `go` command may safely access the same module cache at the same time.

The `go` command creates module source files and directories in the cache with read-only permissions to prevent accidental changes to modules after they’re downloaded. This has the unfortunate side-effect of making the cache difficult to delete with commands like `rm -rf`. The cache may instead be deleted with [`go clean -modcache`](mod-commands.md#go-clean--modcache). Alternatively, when the `-modcacherw` flag is used, the `go` command will create new directories with read-write permissions. This increases the risk of editors, tests, and other programs modifying files in the module cache. The [`go mod verify`](mod-commands.md#go-mod-verify) command may be used to detect modifications to dependencies of the main module. It scans the extracted contents of each module dependency and confirms they match the expected hash in `go.sum`.

The table below explains the purpose of most files in the module cache. Some transient files (lock files, temporary directories) are omitted. For each path, `$module` is a module path, and `$version` is a version. Paths ending with slashes (`/`) are directories. Capital letters in module paths and versions are escaped using exclamation points (`Azure` is escaped as `!azure`) to avoid conflicts on case-insensitive file systems.

| Path | Description |
|----|----|
| `$module@$version/` | Directory containing extracted contents of a module `.zip` file. This serves as a module root directory for a downloaded module. It won't contain a `go.mod` file if the original module didn't have one. |
| `cache/download/` | Directory containing files downloaded from module proxies and files derived from [version control systems](vcs.md). The layout of this directory follows the [`GOPROXY` protocol](module-proxy.md#goproxy-protocol), so this directory may be used as a proxy when served by an HTTP file server or when referenced with a `file://` URL. |
| `cache/download/$module/@v/list` | List of known versions (see [`GOPROXY` protocol](module-proxy.md#goproxy-protocol)). This may change over time, so the `go` command usually fetches a new copy instead of re-using this file. |
| `cache/download/$module/@v/$version.info` | JSON metadata about the version. (see [`GOPROXY` protocol](module-proxy.md#goproxy-protocol)). This may change over time, so the `go` command usually fetches a new copy instead of re-using this file. |
| `cache/download/$module/@v/$version.mod` | The `go.mod` file for this version (see [`GOPROXY` protocol](module-proxy.md#goproxy-protocol)). If the original module did not have a `go.mod` file, this is a synthesized file with no requirements. |
| `cache/download/$module/@v/$version.zip` | The zipped contents of the module (see [`GOPROXY` protocol](module-proxy.md#goproxy-protocol) and [Module zip files](zip-files.md)). |
| `cache/download/$module/@v/$version.ziphash` | A cryptographic hash of the files in the `.zip` file. Note that the `.zip` file itself is not hashed, so file order, compression, alignment, and metadata don't affect the hash. When using a module, the `go` command verifies this hash matches the corresponding line in [`go.sum`](authenticating.md#gosum-files). The [`go mod verify`](mod-commands.md#go-mod-verify) command checks that the hashes of module `.zip` files and extracted directories match these files. |
| `cache/download/sumdb/` | Directory containing files downloaded from a [checksum database](authenticating.md#checksum-database) (typically `sum.golang.org`). |
| `cache/vcs/` | Contains cloned version control repositories for modules fetched directly from their sources. Directory names are hex-encoded hashes derived from the repository type and URL. Repositories are optimized for size on disk. For example, cloned Git repositories are bare and shallow when possible. |
