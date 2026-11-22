# dok3search container image (frontend SPA + Rust server + knowledge corpus).
#
# Build for the doksam node (single-node Kubernetes, linux/amd64) and import it
# into containerd. The image is self-contained: no external network is needed at
# runtime, so it is safe for the air-gapped deployment target.
#
#   podman build --platform linux/amd64 -t dok3search:<git-sha> .
#   docker build --platform linux/amd64 -t dok3search:<git-sha> .
#
# Reproducibility: every stage pins a specific toolchain tag, `npm ci` uses the
# committed package-lock.json, and Cargo builds with `--locked`. Pin the base
# image digests (`image@sha256:...`) in release automation for stricter
# bit-for-bit reproducibility.

# syntax=docker/dockerfile:1

# --- Stage 1: build the React/Vite SPA --------------------------------------
FROM node:22-bookworm-slim AS frontend
WORKDIR /build/frontend
# Copy manifests first so dependency install is cached independently of sources.
COPY frontend/package.json frontend/package-lock.json ./
RUN npm ci
COPY frontend/ ./
# frontend/src/documents.ts imports ../../docs/manual/... as raw source.
COPY docs/manual /build/docs/manual
RUN npm run build

# --- Stage 2: build the Rust server -----------------------------------------
FROM rust:1.100-bookworm AS backend
WORKDIR /build
# rusqlite is built with the `bundled` feature, so a C toolchain is required
# (the rust image already ships gcc/make). The full workspace is copied so the
# path dependencies resolve; only the dok3-server binary is published.
COPY backend/rust/ backend/rust/
# `dok3-search` embeds the operating manual at compile time via include_str!.
COPY docs/manual /build/docs/manual
RUN cargo build --manifest-path backend/rust/Cargo.toml -p dok3-server --release --locked

# --- Stage 3: runtime image -------------------------------------------------
FROM debian:bookworm-slim AS runtime

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates curl \
    && rm -rf /var/lib/apt/lists/* \
    && groupadd --gid 10001 dok3 \
    && useradd --uid 10001 --gid 10001 --home-dir /app --no-create-home --shell /usr/sbin/nologin dok3

WORKDIR /app

# Server binary.
COPY --from=backend /build/backend/rust/target/release/dok3search /usr/local/bin/dok3search

# Static SPA served by the server (DOK3_STATIC_DIR). The server runs as dok3,
# so the dist tree must be readable by it; without --chown the files stay
# root-owned and ServeDir answers 404 for files it cannot open (favicon.svg).
COPY --chown=dok3:dok3 --from=frontend /build/frontend/dist /app/frontend/dist

# Searchable knowledge corpus (DOK3_SOURCE_DIR). The connector walks this tree
# recursively and indexes Markdown; the design docs also back DOK3_DESIGN_DIR.
COPY --chown=dok3:dok3 frontend/src/samples /app/knowledge/samples
COPY --chown=dok3:dok3 docs/manual /app/knowledge/manual

# Persistent store mount point (DOK3_DATA_DIR); the deployment mounts a PVC here.
RUN chmod -R a+rX /app/knowledge /app/frontend/dist \
    && install -d -o dok3 -g dok3 /data

# Container-friendly defaults. The deployment overrides DOK3_DATA_DIR and may
# tune DOK3_REFRESH_INTERVAL_SECS. DOK3_ALLOW_NON_LOOPBACK_BIND is the explicit
# opt-in that lets DOK3_SOURCE_DIR run on a 0.0.0.0 bind behind the protected
# ingress; the loopback guard stays the default outside the container.
ENV DOK3_BIND=0.0.0.0 \
    DOK3_PORT=8080 \
    DOK3_STATIC_DIR=/app/frontend/dist \
    DOK3_DESIGN_DIR=/app/knowledge/samples/design \
    DOK3_SOURCE_DIR=/app/knowledge \
    DOK3_DATA_DIR=/data \
    DOK3_ALLOW_NON_LOOPBACK_BIND=true

USER dok3:dok3
EXPOSE 8080

ENTRYPOINT ["/usr/local/bin/dok3search"]
