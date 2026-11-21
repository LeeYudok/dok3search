# Go official documentation

This offline collection contains sections from three English-language resources
by the Go Authors:

- [The Go Programming Language Specification](https://go.dev/ref/spec) comes
  from the installed Go 1.26.2 SDK. It describes the Go 1.26 language.
- [Go Modules Reference](https://go.dev/ref/mod) comes from its published HTML
  and includes the module selection diagrams.
- [Effective Go](https://go.dev/doc/effective_go) comes from its published HTML.
  It is an idiom guide that predates generics and Go modules.

Each section is converted to Markdown with its examples. Links among sections
in the collection use relative paths. Links to material outside the collection
retain their online destinations. Site navigation and scripts are omitted.
`provenance.json` records the SHA-256 digests of the source snapshots and assets.

The documentation is attributed to the Go Authors. According to the [Go
copyright notice](https://go.dev/copyright), site documentation is licensed
under [CC BY 4.0](LICENSE-CC-BY) and source code under [BSD 3-Clause](LICENSE-BSD).
The BSD notice is copied from the installed SDK.

`chapters/practice/` contains five original Korean guides for offline development.
They cover project layout, testing, errors, concurrency, and diagnostics.
The guides and their examples follow the dok3search repository license.

`chapters/learn-go-with-tests/` contains three English exercises from
[Learn Go with Tests](https://github.com/quii/learn-go-with-tests): Context,
HTTP handlers, and time testing with `testing/synctest` (Go 1.25+).
See [MIT license](LICENSE-LGWT-MIT) and the [source snapshot](learn-go-with-tests-provenance.json).
