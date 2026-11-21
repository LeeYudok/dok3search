# Environment variables

Module behavior in the `go` command may be configured using the environment variables listed below. This list only includes module-related environment variables. See [`go help environment`](https://go.dev/cmd/go/#hdr-Environment_variables) for a list of all environment variables recognized by the `go` command.

**`GO111MODULE`**

Controls whether the `go` command runs in module-aware mode or `GOPATH` mode. Three values are recognized:

- `off`: the `go` command ignores `go.mod` files and runs in `GOPATH` mode.
- `on` (or unset): the `go` command runs in module-aware mode, even when no `go.mod` file is present.
- `auto`: the `go` command runs in module-aware mode if a `go.mod` file is present in the current directory or any parent directory. In Go 1.15 and lower, this was the default.

See [Module-aware commands](mod-commands.md) for more information.

**`GOMODCACHE`**

The directory where the `go` command will store downloaded modules and related files. See [Module cache](module-cache.md) for details on the structure of this directory.

If `GOMODCACHE` is not set, it defaults to `$GOPATH/pkg/mod`.

**`GOINSECURE`**

Comma-separated list of glob patterns (in the syntax of Go's [`path.Match`](https://go.dev/pkg/path/#Match)) of module path prefixes that may always be fetched in an insecure manner. Only applies to dependencies that are being fetched directly.

Unlike the `-insecure` flag on `go get`, `GOINSECURE` does not disable module checksum database validation. `GOPRIVATE` or `GONOSUMDB` may be used to achieve that.

**`GONOPROXY`**

Comma-separated list of glob patterns (in the syntax of Go's [`path.Match`](https://go.dev/pkg/path/#Match)) of module path prefixes that should always be fetched directly from version control repositories, not from module proxies.

If `GONOPROXY` is not set, it defaults to `GOPRIVATE`. See [Privacy](private-modules.md#privacy).

**`GONOSUMDB`**

Comma-separated list of glob patterns (in the syntax of Go's [`path.Match`](https://go.dev/pkg/path/#Match)) of module path prefixes for which the `go` should not verify checksums using the checksum database.

If `GONOSUMDB` is not set, it defaults to `GOPRIVATE`. See [Privacy](private-modules.md#privacy).

**`GOPATH`**

In `GOPATH` mode, the `GOPATH` variable is a list of directories that may contain Go code.

In module-aware mode, the [module cache](glossary.md) is stored in the `pkg/mod` subdirectory of the first `GOPATH` directory. Module source code outside the cache may be stored in any directory.

If `GOPATH` is not set, it defaults to the `go` subdirectory of the user's home directory.

**`GOPRIVATE`**

Comma-separated list of glob patterns (in the syntax of Go's [`path.Match`](https://go.dev/pkg/path/#Match)) of module path prefixes that should be considered private. `GOPRIVATE` is a default value for `GONOPROXY` and `GONOSUMDB`. See [Privacy](private-modules.md#privacy). `GOPRIVATE` also determines whether a module is considered private for `GOVCS`.

**`GOPROXY`**

List of module proxy URLs, separated by commas (`,`) or pipes (`|`). When the `go` command looks up information about a module, it contacts each proxy in the list in sequence until it receives a successful response or a terminal error. A proxy may respond with a 404 (Not Found) or 410 (Gone) status to indicate the module is not available on that server.

The `go` command's error fallback behavior is determined by the separator characters between URLs. If a proxy URL is followed by a comma, the `go` command falls back to the next URL after a 404 or 410 error; all other errors are considered terminal. If the proxy URL is followed by a pipe, the `go` command falls back to the next source after any error, including non-HTTP errors like timeouts.

`GOPROXY` URLs may have the schemes `https`, `http`, or `file`. If a URL has no scheme, `https` is assumed. A module cache may be used directly as a file proxy:

``` text
GOPROXY=file://$(go env GOMODCACHE)/cache/download
```

Two keywords may be used in place of proxy URLs:

- `off`: disallows downloading modules from any source.
- `direct`: download directly from version control repositories instead of using a module proxy.

`GOPROXY` defaults to `https://proxy.golang.org,direct`. Under that configuration, the `go` command first contacts the Go module mirror run by Google, then falls back to a direct connection if the mirror does not have the module. See <https://proxy.golang.org/privacy> for the mirror's privacy policy. The `GOPRIVATE` and `GONOPROXY` environment variables may be set to prevent specific modules from being downloaded using proxies. See [Privacy](private-modules.md#privacy) for information on private proxy configuration.

See [Module proxies](module-proxy.md) and [Resolving a package to a module](modules-overview.md#resolving-a-package-to-a-module) for more information on how proxies are used.

**`GOSUMDB`**

Identifies the name of the checksum database to use and optionally its public key and URL. For example:

``` text
GOSUMDB="sum.golang.org"
GOSUMDB="sum.golang.org+<publickey>"
GOSUMDB="sum.golang.org+<publickey> https://sum.golang.org"
```

The `go` command knows the public key of `sum.golang.org` and also that the name `sum.golang.google.cn` (available inside mainland China) connects to the `sum.golang.org` database; use of any other database requires giving the public key explicitly. The URL defaults to `https://` followed by the database name.

`GOSUMDB` defaults to `sum.golang.org`, the Go checksum database run by Google. See <https://sum.golang.org/privacy> for the service's privacy policy.

If `GOSUMDB` is set to `off` or if `go get` is invoked with the `-insecure` flag, the checksum database is not consulted, and all unrecognized modules are accepted, at the cost of giving up the security guarantee of verified repeatable downloads for all modules. A better way to bypass the checksum database for specific modules is to use the `GOPRIVATE` or `GONOSUMDB` environment variables.

See [Authenticating modules](authenticating.md) and [Privacy](private-modules.md#privacy) for more information.

**`GOVCS`**

Controls the set of version control tools the `go` command may use to download public and private modules (defined by whether their paths match a pattern in `GOPRIVATE`) or other modules matching a glob pattern.

If `GOVCS` is not set, or if a module does not match any pattern in `GOVCS`, the `go` command may use `git` and `hg` for a public module, or any known version control tool for a private module. Concretely, the `go` command acts as if `GOVCS` were set to:

``` text
public:git|hg,private:all
```

See [Controlling version control tools with `GOVCS`](vcs.md#controlling-version-control-tools-with-govcs) for a complete explanation.

**`GOWORK`**

The `GOWORK` environment variable instructs the `go` command to enter workspace mode using the provided [`go.work` file](workspaces.md#gowork-files) to define the workspace. If `GOWORK` is set to `off` workspace mode is disabled. This can be used to run the `go` command in single module mode: for example, `GOWORK=off go build .` builds the `.` package in single-module mode. If `GOWORK` is empty, the `go` command will search for a `go.work` file as described in the [Workspaces](workspaces.md) section.
