# Private modules

Go modules are frequently developed and distributed on version control servers and module proxies that aren’t available on the public internet. The `go` command can download and build modules from private sources, though it usually requires some configuration.

The environment variables below may be used to configure access to private modules. See [Environment variables](environment-variables.md) for details. See also [Privacy](#privacy) for information on controlling information sent to public servers.

- `GOPROXY` — list of module proxy URLs. The `go` command will attempt to download modules from each server in sequence. The keyword `direct` instructs the `go` command to download modules from version control repositories where they’re developed instead of using a proxy.
- `GOPRIVATE` — list of glob patterns of module path prefixes that should be considered private. Acts as a default value for `GONOPROXY` and `GONOSUMDB`.
- `GONOPROXY` — list of glob patterns of module path prefixes that should not be downloaded from a proxy. The `go` command will download matching modules from version control repositories where they’re developed, regardless of `GOPROXY`.
- `GONOSUMDB` — list of glob patterns of module path prefixes that should not be checked using the public checksum database, [sum.golang.org](https://sum.golang.org).
- `GOINSECURE` — list of glob patterns of module path prefixes that may be retrieved over HTTP and other insecure protocols.

These variables may be set in the development environment (for example, in a `.profile` file), or they may be set permanently with [`go env -w`](https://go.dev/cmd/go/#hdr-Print_Go_environment_information).

The rest of this section describes common patterns for providing access to private module proxies and version control repositories.

### Private proxy serving all modules

A central private proxy server that serves all modules (public and private) provides the most control for administrators and requires the least configuration for individual developers.

To configure the `go` command to use such a server, set the following environment variables, replacing `https://proxy.corp.example.com` with your proxy URL and `corp.example.com` with your module prefix:

``` text
GOPROXY=https://proxy.corp.example.com
GONOSUMDB=corp.example.com
```

The `GOPROXY` setting instructs the `go` command to only download modules from `https://proxy.corp.example.com`; the `go` command will not connect to other proxies or version control repositories.

The `GONOSUMDB` setting instructs the `go` command not to use the public checksum database to authenticate modules with paths starting with `corp.example.com`.

A proxy running in this configuration will likely need read access to private version control servers. It will also need access to the public internet to download new versions of public modules.

There are several existing implementations of `GOPROXY` servers that may be used this way. A minimal implementation would serve files from a [module cache](glossary.md) directory and would use [`go mod download`](mod-commands.md#go-mod-download) (with suitable configuration) to retrieve missing modules.

### Private proxy serving private modules

A private proxy server may serve private modules without also serving publicly available modules. The `go` command can be configured to fall back to public sources for modules that aren’t available on the private server.

To configure the `go` command to work this way, set the following environment variables, replacing `https://proxy.corp.example.com` with the proxy URL and `corp.example.com` with the module prefix:

``` text
GOPROXY=https://proxy.corp.example.com,https://proxy.golang.org,direct
GONOSUMDB=corp.example.com
```

The `GOPROXY` setting instructs the `go` command to try to download modules from `https://proxy.corp.example.com` first. If that server responds with 404 (Not Found) or 410 (Gone), the `go` command will fall back to `https://proxy.golang.org`, then to direct connections to repositories.

The `GONOSUMDB` setting instructs the `go` command not to use the public checksum database to authenticate modules whose paths start with `corp.example.com`.

Note that a proxy used in this configuration may still control access to public modules, even though it doesn’t serve them. If the proxy responds to a request with an error status other than 404 or 410, the `go` command will not fall back to later entries in the `GOPROXY` list. For example, the proxy could respond with 403 (Forbidden) for a module with an unsuitable license or with known security vulnerabilities.

### Direct access to private modules

The `go` command may be configured to bypass public proxies and download private modules directly from version control servers. This is useful when running a private proxy server is not feasible.

To configure the `go` command to work this way, set `GOPRIVATE`, replacing `corp.example.com` the private module prefix:

``` text
GOPRIVATE=corp.example.com
```

The `GOPROXY` variable does not need to be changed in this situation. It defaults to `https://proxy.golang.org,direct`, which instructs the `go` command to attempt to download modules from `https://proxy.golang.org` first, then fall back to a direct connection if that proxy responds with 404 (Not Found) or 410 (Gone).

The `GOPRIVATE` setting instructs the `go` command not to connect to a proxy or to the checksum database for modules starting with `corp.example.com`.

An internal HTTP server may still be needed to [resolve module paths to repository URLs](vcs.md#finding-a-repository-for-a-module-path). For example, when the `go` command downloads the module `corp.example.com/mod`, it will send a GET request to `https://corp.example.com/mod?go-get=1`, and it will look for the repository URL in the response. To avoid this requirement, ensure that each private module path has a VCS suffix (like `.git`) marking the repository root prefix. For example, when the `go` command downloads the module `corp.example.com/repo.git/mod`, it will clone the Git repository at `https://corp.example.com/repo.git` or `ssh://corp.example.com/repo.git` without needing to make additional requests.

Developers will need read access to repositories containing private modules. This may be configured in global VCS configuration files like `.gitconfig`. It’s best if VCS tools are configured not to need interactive authentication prompts. By default, when invoking Git, the `go` command disables interactive prompts by setting `GIT_TERMINAL_PROMPT=0`, but it respects explicit settings.

### Passing credentials to private proxies

The `go` command can authenticate to proxy servers using HTTP [basic authentication](https://en.wikipedia.org/wiki/Basic_access_authentication) or request headers supplied by `GOAUTH`.

Credentials may be specified in a [`.netrc` file](https://www.gnu.org/software/inetutils/manual/html_node/The-_002enetrc-file.html). For example, a `.netrc` file containing the lines below would configure the `go` command to connect to the machine `proxy.corp.example.com` with the given username and password.

``` text
machine proxy.corp.example.com
login jrgopher
password hunter2
```

The location of the file may be set with the `NETRC` environment variable. If `NETRC` is not set, the `go` command will read `$HOME/.netrc` on UNIX-like platforms or `%USERPROFILE%\_netrc` on Windows.

Fields in `.netrc` are separated with spaces, tabs, and newlines. Unfortunately, these characters cannot be used in usernames or passwords. Note also that the machine name cannot be a full URL, so it’s not possible to specify different usernames and passwords for different paths on the same machine.

In Go 1.24 and later, `GOAUTH` may use `.netrc`, run `git credential fill` in a specified working directory, or invoke a custom command. A custom command can provide arbitrary HTTP request headers, such as a Bearer token, for one or more HTTPS URL prefixes. This allows credentials to be scoped to a path on a proxy host, which `.netrc` cannot do. See `go help goauth` for the command protocol and configuration options.

The `go` command only applies credentials obtained through `GOAUTH`, including its default `netrc` method, to HTTPS requests.

Alternatively, credentials may be specified directly in `GOPROXY` URLs. For example:

``` text
GOPROXY=https://jrgopher:hunter2@proxy.corp.example.com
```

Use caution when taking this approach: environment variables may appear in shell history and in logs.

### Passing credentials to private repositories

The `go` command may download a module directly from a version control repository. This is necessary for private modules if a private proxy is not used. See [Direct access to private modules](#direct-access-to-private-modules) for configuration.

The `go` command runs version control tools like `git` when downloading modules directly. These tools perform their own authentication, so you may need to configure credentials in a tool-specific configuration file like `.gitconfig`.

To ensure this works smoothly, make sure the `go` command uses the correct repository URL and that the version control tool doesn’t require a password to be entered interactively. The `go` command prefers `https://` URLs over other schemes like `ssh://` unless the scheme was specified when [looking up the repository URL](vcs.md#finding-a-repository-for-a-module-path). For GitHub repositories specifically, the `go` command assumes `https://`.

For most servers, you can configure your client to authenticate over HTTP. For example, GitHub supports using [OAuth personal access tokens as HTTP passwords](https://docs.github.com/en/free-pro-team@latest/github/extending-github/git-automation-with-oauth-tokens). You can store HTTP passwords in a `.netrc` file, as when [passing credentials to private proxies](#passing-credentials-to-private-proxies).

Alternatively, you can rewrite `https://` URLs to another scheme. For example, in `.gitconfig`:

``` text
[url "git@github.com:"]
    insteadOf = https://github.com/
```

For more information, see [Why does “go get” use HTTPS when cloning a repository?](https://go.dev/doc/faq#git_https)

### Privacy

The `go` command may download modules and metadata from module proxy servers and version control systems. The environment variable `GOPROXY` controls which servers are used. The environment variables `GOPRIVATE` and `GONOPROXY` control which modules are fetched from proxies.

The default value of `GOPROXY` is:

``` text
https://proxy.golang.org,direct
```

With this setting, when the `go` command downloads a module or module metadata, it will first send a request to `proxy.golang.org`, a public module proxy operated by Google ([privacy policy](https://proxy.golang.org/privacy)). See [`GOPROXY` protocol](module-proxy.md#goproxy-protocol) for details on what information is sent in each request. The `go` command does not transmit personally identifiable information, but it does transmit the full module path being requested. If the proxy responds with a 404 (Not Found) or 410 (Gone) status, the `go` command will attempt to connect directly to the version control system providing the module. See [Version control systems](vcs.md) for details.

The `GOPRIVATE` or `GONOPROXY` environment variables may be set to lists of glob patterns matching module prefixes that are private and should not be requested from any proxy. For example:

``` text
GOPRIVATE=*.corp.example.com,*.research.example.com
```

`GOPRIVATE` simply acts as a default for `GONOPROXY` and `GONOSUMDB`, so it’s not necessary to set `GONOPROXY` unless `GONOSUMDB` should have a different value. When a module path is matched by `GONOPROXY`, the `go` command ignores `GOPROXY` for that module and fetches it directly from its version control repository. This is useful when no proxy serves private modules. See [Direct access to private modules](#direct-access-to-private-modules).

If there is a [trusted proxy serving all modules](#private-proxy-serving-all-modules), then `GONOPROXY` should not be set. For example, if `GOPROXY` is set to one source, the `go` command will not download modules from other sources. `GONOSUMDB` should still be set in this situation.

``` text
GOPROXY=https://proxy.corp.example.com
GONOSUMDB=*.corp.example.com,*.research.example.com
```

If there is a [trusted proxy serving only private modules](#private-proxy-serving-private-modules), `GONOPROXY` should not be set, but care must be taken to ensure the proxy responds with the correct status codes. For example, consider the following configuration:

``` text
GOPROXY=https://proxy.corp.example.com,https://proxy.golang.org
GONOSUMDB=*.corp.example.com,*.research.example.com
```

Suppose that due to a typo, a developer attempts to download a module that doesn’t exist.

``` text
go mod download corp.example.com/secret-product/typo@latest
```

The `go` command first requests this module from `proxy.corp.example.com`. If that proxy responds with 404 (Not Found) or 410 (Gone), the `go` command will fall back to `proxy.golang.org`, transmitting the `secret-product` path in the request URL. If the private proxy responds with any other error code, the `go` command prints the error and will not fall back to other sources.

In addition to proxies, the `go` command may connect to the checksum database to verify cryptographic hashes of modules not listed in `go.sum`. The `GOSUMDB` environment variable sets the name, URL, and public key of the checksum database. The default value of `GOSUMDB` is `sum.golang.org`, the public checksum database operated by Google ([privacy policy](https://sum.golang.org/privacy)). See [Checksum database](authenticating.md#checksum-database) for details on what is transmitted with each request. As with proxies, the `go` command does not transmit personally identifiable information, but it does transmit the full module path being requested, and the checksum database cannot compute checksums for non-public modules.

The `GONOSUMDB` environment variable may be set to patterns indicating which modules are private and should not be requested from the checksum database. `GOPRIVATE` acts as a default for `GONOSUMDB` and `GONOPROXY`, so it’s not necessary to set `GONOSUMDB` unless `GONOPROXY` should have a different value.

A proxy may [mirror the checksum database](https://go.googlesource.com/proposal/+/master/design/25530-sumdb.md#proxying-a-checksum-database). If a proxy in `GOPROXY` does this, the `go` command will not connect to the checksum database directly.

`GOSUMDB` may be set to `off` to disable use of the checksum database entirely. With this setting, the `go` command will not authenticate downloaded modules unless they’re already in `go.sum`. See [Authenticating modules](authenticating.md).
