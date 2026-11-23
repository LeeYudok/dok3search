# Crate dok3-domain

## Overview

Core domain types and port traits (DocumentId, Revision, SectionId, GenerationId).

## Items

- `acl` — Access-control scopes and principal derivation (default-deny).
- `admin` — Administration DTOs for background indexing jobs, classification, and source management.
- `change` — Durable change journal contracts for incremental indexing and generation catch-up.
- `document` — Document, section, and section-reading contracts.
- `ids` — Strongly typed domain identifiers and sequence numbers.
- `ports` — Hexagonal port traits decoupled from concrete engines and web frameworks.
- `search` — Search query, response, snapshot, and health readiness contracts.
