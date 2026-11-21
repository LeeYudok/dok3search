# Authenticating modules

When the `go` command downloads a module [zip file](zip-files.md) or [`go.mod` file](go-mod-file.md) into the [module cache](module-cache.md), it computes a cryptographic hash and compares it with a known value to verify the file hasn’t changed since it was first downloaded. The `go` command reports a security error if a downloaded file does not have the correct hash.

For `go.mod` files, the `go` command computes the hash from the file content. For module zip files, the `go` command computes the hash from the names and contents of files within the archive in a deterministic order. The hash is not affected by file order, compression, alignment, and other metadata. See [`golang.org/x/mod/sumdb/dirhash`](https://pkg.go.dev/golang.org/x/mod/sumdb/dirhash?tab=doc) for hash implementation details.

The `go` command compares each hash with the corresponding line in the main module’s [`go.sum` file](#gosum-files). If the hash is different from the hash in `go.sum`, the `go` command reports a security error and deletes the downloaded file without adding it into the module cache.

If the `go.sum` file is not present, or if it doesn’t contain a hash for the downloaded file, the `go` command may verify the hash using the [checksum database](#checksum-database), a global source of hashes for publicly available modules. Once the hash is verified, the `go` command adds it to `go.sum` and adds the downloaded file in the module cache. If a module is private (matched by the `GOPRIVATE` or `GONOSUMDB` environment variables) or if the checksum database is disabled (by setting `GOSUMDB=off`), the `go` command accepts the hash and adds the file to the module cache without verifying it.

The module cache is usually shared by all Go projects on a system, and each module may have its own `go.sum` file with potentially different hashes. To avoid the need to trust other modules, the `go` command verifies hashes using the main module’s `go.sum` whenever it accesses a file in the module cache. Zip file hashes are expensive to compute, so the `go` command checks pre-computed hashes stored alongside zip files instead of re-hashing the files. The [`go mod verify`](mod-commands.md#go-mod-verify) command may be used to check that zip files and extracted directories have not been modified since they were added to the module cache.

### go.sum files

A module may have a text file named `go.sum` in its root directory, alongside its `go.mod` file. The `go.sum` file contains cryptographic hashes of the module’s direct and indirect dependencies. When the `go` command downloads a module `.mod` or `.zip` file into the [module cache](module-cache.md), it computes a hash and checks that the hash matches the corresponding hash in the main module’s `go.sum` file. `go.sum` may be empty or absent if the module has no dependencies or if all dependencies are replaced with local directories using [`replace` directives](go-mod-file.md#replace-directive).

Each line in `go.sum` has three fields separated by spaces: a module path, a version (possibly ending with `/go.mod`), and a hash.

- The module path is the name of the module the hash belongs to.
- The version is the version of the module the hash belongs to. If the version ends with `/go.mod`, the hash is for the module’s `go.mod` file only; otherwise, the hash is for the files within the module’s `.zip` file.
- The hash column consists of an algorithm name (like `h1`) and a base64-encoded cryptographic hash, separated by a colon (`:`). Currently, SHA-256 (`h1`) is the only supported hash algorithm. If a vulnerability in SHA-256 is discovered in the future, support will be added for another algorithm (named `h2` and so on).

The `go.sum` file may contain hashes for multiple versions of a module. The `go` command may need to load `go.mod` files from multiple versions of a dependency in order to perform [minimal version selection](minimal-version-selection.md). `go.sum` may also contain hashes for module versions that aren’t needed anymore (for example, after an upgrade). [`go mod tidy`](mod-commands.md#go-mod-tidy) will add missing hashes and will remove unnecessary hashes from `go.sum`.

### Checksum database

The checksum database is a global source of `go.sum` lines. The `go` command can use this in many situations to detect misbehavior by proxies or origin servers.

The checksum database allows for global consistency and reliability for all publicly available module versions. It makes untrusted proxies possible since they can’t serve the wrong code without it going unnoticed. It also ensures that the bits associated with a specific version do not change from one day to the next, even if the module’s author subsequently alters the tags in their repository.

The checksum database is served by [sum.golang.org](https://sum.golang.org), which is run by Google. It is a [Transparent Log](https://research.swtch.com/tlog) (or “Merkle Tree”) of `go.sum` line hashes, which is backed by [Trillian](https://github.com/google/trillian). The main advantage of a Merkle tree is that independent auditors can verify that it hasn’t been tampered with, so it is more trustworthy than a simple database.

The `go` command interacts with the checksum database using the protocol originally outlined in [Proposal: Secure the Public Go Module Ecosystem](https://go.googlesource.com/proposal/+/master/design/25530-sumdb.md#checksum-database).

The table below specifies queries that the checksum database must respond to. For each path, `$base` is the path portion of the checksum database URL, `$module` is a module path, and `$version` is a version. For example, if the checksum database URL is `https://sum.golang.org`, and the client is requesting the record for the module `golang.org/x/text` at version `v0.3.2`, the client would send a `GET` request for `https://sum.golang.org/lookup/golang.org/x/text@v0.3.2`.

To avoid ambiguity when serving from case-insensitive file systems, the `$module` and `$version` elements are [case-encoded](https://pkg.go.dev/golang.org/x/mod/module#EscapePath) by replacing every uppercase letter with an exclamation mark followed by the corresponding lower-case letter. This allows modules `example.com/M` and `example.com/m` to both be stored on disk, since the former is encoded as `example.com/!m`.

Parts of the path surrounded by square brackets, like `[.p/$W]` denote optional values.

| Path | Description |
|----|----|
| `$base/latest` | Returns a signed, encoded tree description for the latest log. This signed description is in the form of a [note](https://pkg.go.dev/golang.org/x/mod/sumdb/note), which is text that has been signed by one or more server keys and can be verified using the server's public key. The tree description provides the size of the tree and the hash of the tree head at that size. This encoding is described in ` `[`golang.org/x/mod/sumdb/tlog#FormatTree`](https://pkg.go.dev/golang.org/x/mod/sumdb/tlog#FormatTree). |
|  |  |
| `$base/lookup/$module@$version` | Returns the log record number for the entry about `$module` at `$version`, followed by the data for the record (that is, the `go.sum` lines for `$module` at `$version`) and a signed, encoded tree description that contains the record. |
|  |  |
| `$base/tile/$H/$L/$K[.p/$W]` | Returns a [log tile](https://research.swtch.com/tlog#serving_tiles), which is a set of hashes that make up a section of the log. Each tile is defined in a two-dimensional coordinate at tile level `$L`, `$K`th from the left, with a tile height of `$H`. The optional `.p/$W` suffix indicates a partial log tile with only `$W` hashes. Clients must fall back to fetching the full tile if a partial tile is not found. |
|  |  |
| `$base/tile/$H/data/$K[.p/$W]` | Returns the record data for the leaf hashes in `/tile/$H/0/$K[.p/$W]` (with a literal `data` path element). |
|  |  |

If the `go` command consults the checksum database, then the first step is to retrieve the record data through the `/lookup` endpoint. If the module version is not yet recorded in the log, the checksum database will try to fetch it from the origin server before replying. This `/lookup` data provides the sum for this module version as well as its position in the log, which informs the client of which tiles should be fetched to perform proofs. The `go` command performs “inclusion” proofs (that a specific record exists in the log) and “consistency” proofs (that the tree hasn’t been tampered with) before adding new `go.sum` lines to the main module’s `go.sum` file. It’s important that the data from `/lookup` should never be used without first authenticating it against the signed tree hash and authenticating the signed tree hash against the client’s timeline of signed tree hashes.

Signed tree hashes and new tiles served by the checksum database are stored in the module cache, so the `go` command only needs to fetch tiles that are missing.

The `go` command doesn’t need to directly connect to the checksum database. It can request module sums via a module proxy that [mirrors the checksum database](https://go.googlesource.com/proposal/+/master/design/25530-sumdb.md#proxying-a-checksum-database) and supports the protocol above. This can be particularly helpful for private, corporate proxies which block requests outside the organization.

The `GOSUMDB` environment variable identifies the name of checksum database to use and optionally its public key and URL, as in:

``` text
GOSUMDB="sum.golang.org"
GOSUMDB="sum.golang.org+<publickey>"
GOSUMDB="sum.golang.org+<publickey> https://sum.golang.org"
```

The `go` command knows the public key of `sum.golang.org`, and also that the name `sum.golang.google.cn` (available inside mainland China) connects to the `sum.golang.org` checksum database; use of any other database requires giving the public key explicitly. The URL defaults to `https://` followed by the database name.

`GOSUMDB` defaults to `sum.golang.org`, the Go checksum database run by Google. See [https://sum.golang.org/privacy](https://sum.golang.org/privacy) for the service’s privacy policy.

If `GOSUMDB` is set to `off`, or if `go get` is invoked with the `-insecure` flag, the checksum database is not consulted, and all unrecognized modules are accepted, at the cost of giving up the security guarantee of verified repeatable downloads for all modules. A better way to bypass the checksum database for specific modules is to use the `GOPRIVATE` or `GONOSUMDB` environment variables. See [Private Modules](private-modules.md) for details.

The `go env -w` command can be used to [set these variables](https://go.dev/pkg/cmd/go/#hdr-Print_Go_environment_information) for future `go` command invocations.
