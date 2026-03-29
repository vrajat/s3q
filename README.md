# s3q

**s3q is an S3-backed queue and durable workflow library built around `pgqrs` S3 storage.**

The intended product shape is:

- **Queue APIs inspired by SQS**
- **Workflow APIs inspired by Temporal**
- **Rust core**
- **Thin Python SDK**
- **Python CLI and optional Python service**

This repository is currently in the **seed stage**. The code in `src/` and `python/` defines the initial package shape and target API vocabulary; it is not yet wired to `pgqrs::store::s3::S3Store`.

## Design Direction

- Keep queue operations transport-oriented: send, receive, lease, delete, visibility.
- Keep workflow operations execution-oriented: start, signal, query, result, cancel, terminate.
- Treat queue and workflow as separate but composable subsystems.
- Let `pgqrs` own the durable storage substrate; let `s3q` own the product surface.

## Repository Layout

- `src/`: Rust library crate and public API scaffolding
- `python/`: thin Python SDK, CLI, and service scaffolding
- `docs/`: user-facing docs intended for `s3q.dev`
- `engg/`: internal product and engineering documents
- `.buildkite/`: CI pipeline

## Initial Local Commands

```bash
make check
make test
```

## First Documents

- Product requirements: `engg/product-requirements.md`
- Initial architecture sketch: `engg/design/initial-architecture.md`
- Docs site home: `docs/index.md`

## Near-Term Implementation Plan

1. Add a real Rust adapter over `pgqrs` S3Store.
2. Implement SQS-shaped queue operations on top of the lease-and-ack model.
3. Implement Temporal-inspired workflow handles and execution records.
4. Replace Python stubs with thin bindings to the Rust core.
5. Add a Python service for HTTP/API exposure and worker orchestration.
