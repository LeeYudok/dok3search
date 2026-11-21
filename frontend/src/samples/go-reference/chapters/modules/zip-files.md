# Module zip files

Module versions are distributed as `.zip` files. There is rarely any need to interact directly with these files, since the `go` command creates, downloads, and extracts them automatically from [module proxies](glossary.md) and version control repositories. However, it’s still useful to know about these files to understand cross-platform compatibility constraints or when implementing a module proxy.

The [`go mod download`](mod-commands.md#go-mod-download) command downloads zip files for one or more modules, then extracts those files into the [module cache](glossary.md). Depending on `GOPROXY` and other [environment variables](environment-variables.md), the `go` command may either download zip files from a proxy or clone source control repositories and create zip files from them. The `-json` flag may be used to find the location of download zip files and their extracted contents in the module cache.

The [`golang.org/x/mod/zip`](https://pkg.go.dev/golang.org/x/mod/zip?tab=doc) package may be used to create, extract, or check contents of zip files programmatically.

### File path and size constraints

There are a number of restrictions on the content of module zip files. These constraints ensure that zip files can be extracted safely and consistently on a wide range of platforms.

- A module zip file may be at most 500 MiB in size. The total uncompressed size of its files is also limited to 500 MiB. `go.mod` files are limited to 16 MiB. `LICENSE` files are also limited to 16 MiB. These limits exist to mitigate denial of service attacks on users, proxies, and other parts of the module ecosystem. Repositories that contain more than 500 MiB of files in a module directory tree should tag module versions at commits that only include files needed to build the module’s packages; videos, models, and other large assets are usually not needed for builds.
- Each file within a module zip file must begin with the prefix `$module@$version/` where `$module` is the module path and `$version` is the version, for example, `golang.org/x/mod@v0.3.0/`. The module path must be valid, the version must be valid and canonical, and the version must match the module path’s major version suffix. See [Module paths and versions](go-mod-file.md#module-paths-and-versions) for specific definitions and restrictions.
- File modes, timestamps, and other metadata are ignored.
- Empty directories (entries with paths ending with a slash) may be included in module zip files but are not extracted. The `go` command does not include empty directories in zip files it creates.
- Symbolic links and other irregular files are ignored when creating zip files, since they aren’t portable across operating systems and file systems, and there’s no portable way to represent them in the zip file format.
- Files within directories named `vendor` are ignored when creating zip files, since `vendor` directories outside the main module are never used.
- Files within directories containing `go.mod` files, other than the module root directory, are ignored when creating zip files, since they are not part of the module. The `go` command ignores subdirectories containing `go.mod` files when extracting zip files.
- No two files within a zip file may have paths equal under Unicode case-folding (see [`strings.EqualFold`](https://pkg.go.dev/strings?tab=doc#EqualFold)). This ensures that zip files can be extracted on case-insensitive file systems without collisions.
- A `go.mod` file may or may not appear in the top-level directory (`$module@$version/go.mod`). If present, it must have the name `go.mod` (all lowercase). Files named `go.mod` are not allowed in any other directory.
- File and directory names within a module may consist of Unicode letters, ASCII digits, the ASCII space character (U+0020), and the ASCII punctuation characters `!#$%&()+,-.=@[]^_{}~`. Note that package paths may not contain all these characters. See [`module.CheckFilePath`](https://pkg.go.dev/golang.org/x/mod/module?tab=doc#CheckFilePath) and [`module.CheckImportPath`](https://pkg.go.dev/golang.org/x/mod/module?tab=doc#CheckImportPath) for the differences.
- A file or directory name up to the first dot must not be a reserved file name on Windows, regardless of case (`CON`, `com1`, `NuL`, and so on).
