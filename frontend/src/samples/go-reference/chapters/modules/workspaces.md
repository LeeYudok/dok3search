# Workspaces

A workspace is a collection of modules on disk that are used as the main modules when running [minimal version selection (MVS)](minimal-version-selection.md).

A workspace can be declared in a [`go.work` file](#gowork-files) that specifies relative paths to the module directories of each of the modules in the workspace. When no `go.work` file exists, the workspace consists of the single module containing the current directory.

Most `go` subcommands that work with modules operate on the set of modules determined by the current workspace. `go mod init`, `go mod why`, `go mod edit`, `go mod tidy`, `go mod vendor`, and `go get` always operate on a single main module.

A command determines whether it is in a workspace context by first examining the `GOWORK` environment variable. If `GOWORK` is set to `off`, the command will be in a single-module context. If it is empty or not provided, the command will search the current working directory, and then successive parent directories, for a file `go.work`. If a file is found, the command will operate in the workspace it defines; otherwise, the workspace will include only the module containing the working directory. If `GOWORK` names a path to an existing file that ends in .work, workspace mode will be enabled. Any other value is an error. You can use the `go env GOWORK` command to determine which `go.work` file the `go` command is using. `go env GOWORK` will be empty if the `go` command is not in workspace mode.

### `go.work` files

A workspace is defined by a UTF-8 encoded text file named `go.work`. The `go.work` file is line oriented. Each line holds a single directive, made up of a keyword followed by arguments. For example:

``` text
go 1.23.0

use ./my/first/thing
use ./my/second/thing

replace example.com/bad/thing v1.4.5 => example.com/good/thing v1.4.5
```

As in `go.mod` files, a leading keyword can be factored out of adjacent lines to create a block.

``` text
use (
    ./my/first/thing
    ./my/second/thing
)
```

The `go` command provides several subcommands for manipulating `go.work` files. [`go work init`](mod-commands.md#go-work-init) creates new `go.work` files. [`go work use`](mod-commands.md#go-work-use) adds module directories to the `go.work` file. [`go work edit`](mod-commands.md#go-work-edit) performs low-level edits. The [`golang.org/x/mod/modfile`](https://pkg.go.dev/golang.org/x/mod/modfile?tab=doc) package can be used by Go programs to make the same changes programmatically.

The go command will maintain a `go.work.sum` file that keeps track of hashes used by the workspace that are not in collective workspace modules’ go.sum files.

It is generally inadvisable to commit go.work files into version control systems, for two reasons:

- A checked-in `go.work` file might override a developer’s own `go.work` file from a parent directory, causing confusion when their `use` directives don’t apply.
- A checked-in `go.work` file may cause a continuous integration (CI) system to select and thus test the wrong versions of a module’s dependencies. CI systems should generally not be allowed to use the `go.work` file so that they can test the behavior of the module as it would be used when required by other modules, where a `go.work` file within the module has no effect.

That said, there are some cases where committing a `go.work` file makes sense. For example, when the modules in a repository are developed exclusively with each other but not together with external modules, there may not be a reason the developer would want to use a different combination of modules in a workspace. In that case, the module author should ensure the individual modules are tested and released properly.

### Lexical elements

Lexical elements in `go.work` files are defined in exactly the same way [as for `go.mod files`](go-mod-file.md#lexical-elements).

### Grammar

`go.work` syntax is specified below using Extended Backus-Naur Form (EBNF). See the [Notation section in the Go Language Specification](../spec/notation.md) for details on EBNF syntax.

``` text
GoWork = { Directive } .
Directive = GoDirective |
            ToolchainDirective |
            UseDirective |
            ReplaceDirective .
```

Newlines, identifiers, and strings are denoted with `newline`, `ident`, and `string`, respectively.

Module paths and versions are denoted with `ModulePath` and `Version`. Module paths and versions are specified in exactly the same way [as for `go.mod files`](go-mod-file.md#lexical-elements).

``` text
ModulePath = ident | string . /* see restrictions above */
Version = ident | string .    /* see restrictions above */
```

### `go` directive

A `go` directive is required in a valid `go.work` file. The version must be a valid Go release version: a positive integer followed by a dot and a non-negative integer (for example, `1.18`, `1.19`).

The `go` directive indicates the go toolchain version with which the `go.work` file is intended to work. If changes are made to the `go.work` file format, future versions of the toolchain will interpret the file according to its indicated version.

A `go.work` file may contain at most one `go` directive.

``` text
GoDirective = "go" GoVersion newline .
GoVersion = string | ident .  /* valid release version; see above */
```

Example:

``` text
go 1.23.0
```

### `toolchain` directive

A `toolchain` directive declares a suggested Go toolchain to use in a workspace. It only has an effect when the default toolchain is older than the suggested toolchain.

For details, see “[Go toolchains](https://go.dev/doc/toolchain)”.

``` text
ToolchainDirective = "toolchain" ToolchainName newline .
ToolchainName = string | ident .  /* valid toolchain name; see “Go toolchains” */
```

Example:

``` text
toolchain go1.21.0
```

### `godebug` directive

A `godebug` directive declares a single [GODEBUG setting](https://go.dev/doc/godebug) to apply when working in this workspace. The syntax and effect is the same as the [`go.mod` file’s `godebug` directive](go-mod-file.md#godebug-directive). When a workspace is in use, `godebug` directives in `go.mod` files are ignored.

### `use` directive

A `use` adds a module on disk to the set of main modules in a workspace. Its argument is a relative path to the directory containing the module’s `go.mod` file. A `use` directive does not add modules contained in subdirectories of its argument directory. Those modules may be added by the directory containing their `go.mod` file in separate `use` directives.

``` text
UseDirective = "use" ( UseSpec | "(" newline { UseSpec } ")" newline ) .
UseSpec = FilePath newline .
FilePath = /* platform-specific relative or absolute file path */
```

Example:

``` text
use ./mymod  // example.com/mymod

use (
    ../othermod
    ./subdir/thirdmod
)
```

### `replace` directive

Similar to a `replace` directive in a `go.mod` file, a `replace` directive in a `go.work` file replaces the contents of a specific version of a module, or all versions of a module, with contents found elsewhere. A wildcard replace in `go.work` overrides a version-specific `replace` in a `go.mod` file.

`replace` directives in `go.work` files override any replaces of the same module or module version in workspace modules.

``` text
ReplaceDirective = "replace" ( ReplaceSpec | "(" newline { ReplaceSpec } ")" newline ) .
ReplaceSpec = ModulePath [ Version ] "=>" FilePath newline
            | ModulePath [ Version ] "=>" ModulePath Version newline .
FilePath = /* platform-specific relative or absolute file path */
```

Example:

``` text
replace golang.org/x/net v1.2.3 => example.com/fork/net v1.4.5

replace (
    golang.org/x/net v1.2.3 => example.com/fork/net v1.4.5
    golang.org/x/net => example.com/fork/net v1.4.5
    golang.org/x/net v1.2.3 => ./fork/net
    golang.org/x/net => ./fork/net
)
```
