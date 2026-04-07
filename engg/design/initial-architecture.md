# Initial Architecture

**Status:** Superseded by `queue-architecture.md` and ADR `0002-queue-only-v1.md`.

## Layering

The repository was originally seeded around four layers:

1. **Storage substrate**
   `pgqrs`, specifically `S3Store`, owns durable persistence and object-store synchronization.
2. **Rust core**
   `s3q` owns queue semantics and the public core API.
3. **Python SDK**
   Python remains a thin wrapper over Rust. It should not contain a separate queue or workflow engine.
4. **Python service**
   A later service process can expose HTTP or other APIs, but it should call the same SDK surface.

## Historical Note

This document is kept as initial project history. The active v1 direction is queue-only:

- `s3q` is a thin product layer over `pgqrs`
- `S3Store` is the only backend
- workflow APIs are out of scope for v1

## Suggested First Implementation Order

1. Add a `pgqrs` adapter module inside the Rust crate.
2. Implement queue primitives first because their semantics map more directly to existing `pgqrs` behavior.
3. Bind that Rust surface into Python only after the Rust semantics are stable enough to avoid churn.

## Service Direction

The service should be treated as optional packaging, not as the core product. That keeps:

- correctness in one place
- CLI and service behavior aligned
- local embedded use possible from day one
