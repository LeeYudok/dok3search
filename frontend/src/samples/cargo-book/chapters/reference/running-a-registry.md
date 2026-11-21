# Running a Registry

A minimal registry can be implemented by having a git repository that contains an index, and a server that contains the compressed `.crate` files created by [`cargo package`](../commands/cargo-package.md). Users won’t be able to use Cargo to publish to it, but this may be sufficient for closed environments. The index format is described in [Registry Index](registry-index.md).

A full-featured registry that supports publishing will additionally need to have a web API service that conforms to the API used by Cargo. The web API is described in [Registry Web API](registry-web-api.md).

Commercial and community projects are available for building and running a registry. See  for a list of what is available.
