# Module proxies

### `GOPROXY` protocol

A module proxy is an HTTP server that can respond to `GET` requests for paths specified below. The requests have no query parameters, and no specific headers are required, so even a site serving from a fixed file system (including a `file://` URL) can be a module proxy.

Successful HTTP responses must have the status code 200 (OK). Redirects (3xx) are followed. Responses with status codes 4xx and 5xx are treated as errors. The error codes 404 (Not Found) and 410 (Gone) indicate that the requested module or version is not available on the proxy, but it may be found elsewhere. Error responses should have content type `text/plain` with `charset` either `utf-8` or `us-ascii`.

The `go` command may be configured to contact proxies or source control servers using the `GOPROXY` environment variable, which accepts a list of proxy URLs. The list may include the keywords `direct` or `off` (see [Environment variables](environment-variables.md) for details). List elements may be separated by commas (`,`) or pipes (`|`), which determine error fallback behavior. When a URL is followed by a comma, the `go` command falls back to later sources only after a 404 (Not Found) or 410 (Gone) response. When a URL is followed by a pipe, the `go` command falls back to later sources after any error, including non-HTTP errors such as timeouts. This error handling behavior lets a proxy act as a gatekeeper for unknown modules. For example, a proxy could respond with error 403 (Forbidden) for modules not on an approved list (see [Private proxy serving private modules](private-modules.md#private-proxy-serving-private-modules)).

The table below specifies queries that a module proxy must respond to. For each path, `$base` is the path portion of a proxy URL,`$module` is a module path, and `$version` is a version. For example, if the proxy URL is `https://example.com/mod`, and the client is requesting the `go.mod` file for the module `golang.org/x/text` at version `v0.3.2`, the client would send a `GET` request for `https://example.com/mod/golang.org/x/text/@v/v0.3.2.mod`.

To avoid ambiguity when serving from case-insensitive file systems, the `$module` and `$version` elements are case-encoded by replacing every uppercase letter with an exclamation mark followed by the corresponding lower-case letter. This allows modules `example.com/M` and `example.com/m` to both be stored on disk, since the former is encoded as `example.com/!m`.

**`$base/$module/@v/list`**

Returns a list of known versions of the given module in plain text, one per line. This list should not include pseudo-versions.

**`$base/$module/@v/$version.info`**

Returns JSON-formatted metadata about a specific version of a module. The response must be a JSON object that corresponds to the Go data structure below:

``` text
type Info struct {
    Version string    // version string
    Time    time.Time // commit time
}
```

The `Version` field is required and must contain a valid, [canonical version](glossary.md) (see [Versions](modules-overview.md#versions)). The `$version` in the request path does not need to be the same version or even a valid version; this endpoint may be used to find versions for branch names or revision identifiers. However, if `$version` is a canonical version with a major version compatible with `$module`, the `Version` field in a successful response must be the same.

The `Time` field is optional. If present, it must be a string in RFC 3339 format. It indicates the time when the version was created.

More fields may be added in the future, so other names are reserved.

**`$base/$module/@v/$version.mod`**

Returns the `go.mod` file for a specific version of a module. If the module does not have a `go.mod` file at the requested version, a file containing only a `module` statement with the requested module path must be returned. Otherwise, the original, unmodified `go.mod` file must be returned.

**`$base/$module/@v/$version.zip`**

Returns a zip file containing the contents of a specific version of a module. See [Module zip files](zip-files.md) for details on how this zip file must be formatted.

**`$base/$module/@latest`**

Returns JSON-formatted metadata about the latest known version of a module in the same format as `$base/$module/@v/$version.info`. The latest version should be the version of the module that the `go` command should use if `$base/$module/@v/list` is empty or no listed version is suitable. This endpoint is optional, and module proxies are not required to implement it.

When resolving the latest version of a module, the `go` command will request `$base/$module/@v/list`, then, if no suitable versions are found, `$base/$module/@latest`. The `go` command prefers, in order: the semantically highest release version, the semantically highest pre-release version, and the chronologically most recent pseudo-version. In Go 1.12 and earlier, the `go` command considered pseudo-versions in `$base/$module/@v/list` to be pre-release versions, but this is no longer true since Go 1.13.

A module proxy must always serve the same content for successful responses for `$base/$module/$version.mod` and `$base/$module/$version.zip` queries. This content is [cryptographically authenticated](authenticating.md) using [`go.sum` files](authenticating.md#gosum-files) and, by default, the [checksum database](authenticating.md#checksum-database).

The `go` command caches most content it downloads from module proxies in its module cache in `$GOPATH/pkg/mod/cache/download`. Even when downloading directly from version control systems, the `go` command synthesizes explicit `info`, `mod`, and `zip` files and stores them in this directory, the same as if it had downloaded them directly from a proxy. The cache layout is the same as the proxy URL space, so serving `$GOPATH/pkg/mod/cache/download` at (or copying it to) `https://example.com/proxy` would let users access cached module versions by setting `GOPROXY` to `https://example.com/proxy`.

### Communicating with proxies

The `go` command may download module source code and metadata from a [module proxy](glossary.md). The `GOPROXY` [environment variable](environment-variables.md) may be used to configure which proxies the `go` command may connect to and whether it may communicate directly with [version control systems](vcs.md). Downloaded module data is saved in the [module cache](glossary.md). The `go` command will only contact a proxy when it needs information not already in the cache.

The [`GOPROXY` protocol](#goproxy-protocol) section describes requests that may be sent to a `GOPROXY` server. However, it’s also helpful to understand when the `go` command makes these requests. For example, `go build` follows the procedure below:

- Compute the [build list](glossary.md) by reading [`go.mod` files](glossary.md) and performing [minimal version selection (MVS)](glossary.md).
- Read the packages named on the command line and the packages they import.
- If a package is not provided by any module in the build list, find a module that provides it. Add a module requirement on its latest version to `go.mod`, and start over.
- Build packages after everything is loaded.

When the `go` command computes the build list, it loads the `go.mod` file for each module in the [module graph](glossary.md). If a `go.mod` file is not in the cache, the `go` command will download it from the proxy using a `$module/@v/$version.mod` request (where `$module` is the module path and `$version` is the version). These requests can be tested with a tool like `curl`. For example, the command below downloads the `go.mod` file for `golang.org/x/mod` at version `v0.2.0`:

``` text
$ curl https://proxy.golang.org/golang.org/x/mod/@v/v0.2.0.mod
module golang.org/x/mod

go 1.12

require (
    golang.org/x/crypto v0.0.0-20191011191535-87dc89f01550
    golang.org/x/tools v0.0.0-20191119224855-298f0cb1881e
    golang.org/x/xerrors v0.0.0-20191011141410-1b5146add898
)
```

In order to load a package, the `go` command needs the source code for the module that provides it. Module source code is distributed in `.zip` files which are extracted into the module cache. If a module `.zip` is not in the cache, the `go` command will download it using a `$module/@v/$version.zip` request.

``` text
$ curl -O https://proxy.golang.org/golang.org/x/mod/@v/v0.2.0.zip
$ unzip -l v0.2.0.zip | head
Archive:  v0.2.0.zip
  Length      Date    Time    Name
---------  ---------- -----   ----
     1479  00-00-1980 00:00   golang.org/x/mod@v0.2.0/LICENSE
     1303  00-00-1980 00:00   golang.org/x/mod@v0.2.0/PATENTS
      559  00-00-1980 00:00   golang.org/x/mod@v0.2.0/README
       21  00-00-1980 00:00   golang.org/x/mod@v0.2.0/codereview.cfg
      214  00-00-1980 00:00   golang.org/x/mod@v0.2.0/go.mod
     1476  00-00-1980 00:00   golang.org/x/mod@v0.2.0/go.sum
     5224  00-00-1980 00:00   golang.org/x/mod@v0.2.0/gosumcheck/main.go
```

Note that `.mod` and `.zip` requests are separate, even though `go.mod` files are usually contained within `.zip` files. The `go` command may need to download `go.mod` files for many different modules, and `.mod` files are much smaller than `.zip` files. Additionally, if a Go project does not have a `go.mod` file, the proxy will serve a synthetic `go.mod` file that only contains a [`module` directive](go-mod-file.md#module-directive). Synthetic `go.mod` files are generated by the `go` command when downloading from a [version control system](vcs.md).

If the `go` command needs to load a package not provided by any module in the build list, it will attempt to find a new module that provides it. The section [Resolving a package to a module](modules-overview.md#resolving-a-package-to-a-module) describes this process. In summary, the `go` command requests information about the latest version of each module path that could possibly contain the package. For example, for the package `golang.org/x/net/html`, the `go` command would try to find the latest versions of the modules `golang.org/x/net/html`, `golang.org/x/net`, `golang.org/x/`, and `golang.org`. Only `golang.org/x/net` actually exists and provides that package, so the `go` command uses the latest version of that module. If more than one module provides the package, the `go` command will use the module with the longest path.

When the `go` command requests the latest version of a module, it first sends a request for `$module/@v/list`. If the list is empty or none of the returned versions can be used, it sends a request for `$module/@latest`. Once a version is chosen, the `go` command sends a `$module/@v/$version.info` request for metadata. It may then send `$module/@v/$version.mod` and `$module/@v/$version.zip` requests to load the `go.mod` file and source code.

``` text
$ curl https://proxy.golang.org/golang.org/x/mod/@v/list
v0.1.0
v0.2.0

$ curl https://proxy.golang.org/golang.org/x/mod/@v/v0.2.0.info
{"Version":"v0.2.0","Time":"2020-01-02T17:33:45Z"}
```

After downloading a `.mod` or `.zip` file, the `go` command computes a cryptographic hash and checks that it matches a hash in the main module’s `go.sum` file. If the hash is not present in `go.sum`, by default, the `go` command retrieves it from the [checksum database](authenticating.md#checksum-database). If the computed hash does not match, the `go` command reports a security error and does not install the file in the module cache. The `GOPRIVATE` and `GONOSUMDB` [environment variables](environment-variables.md) may be used to disable requests to the checksum database for specific modules. The `GOSUMDB` environment variable may also be set to `off` to disable requests to the checksum database entirely. See [Authenticating modules](authenticating.md) for more information. Note that version lists and version metadata returned for `.info` requests are not authenticated and may change over time.

### Serving modules directly from a proxy

Most modules are developed and served from a version control repository. In [direct mode](glossary.md), the `go` command downloads such a module with a version control tool (see [Version control systems](vcs.md)). It’s also possible to serve a module directly from a module proxy. This is useful for organizations that want to serve modules without exposing their version control servers and for organizations that use version control tools the `go` command does not support.

When the `go` command downloads a module in direct mode, it first looks up the module server’s URL with an HTTP GET request based on the module path. It looks for a `<meta>` tag with the name `go-import` in the HTML response. The tag’s content must contain the [repository root path](glossary.md), the version control system, and the URL, separated by spaces. See [Finding a repository for a module path](vcs.md#finding-a-repository-for-a-module-path) for details.

If the version control system is `mod`, the `go` command downloads the module from the given URL using the [`GOPROXY` protocol](#goproxy-protocol).

For example, suppose the `go` command is attempting to download the module `example.com/gopher` at version `v1.0.0`. It sends a request to `https://example.com/gopher?go-get=1`. The server responds with an HTML document containing the tag:

``` text
<meta name="go-import" content="example.com/gopher mod https://modproxy.example.com">
```

Based on this response, the `go` command downloads the module by sending requests for `https://modproxy.example.com/example.com/gopher/@v/v1.0.0.info`, `v1.0.0.mod`, and `v1.0.0.zip`.

Note that modules served directly from a proxy cannot be downloaded with `go get` in GOPATH mode.
