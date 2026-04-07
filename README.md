# s3q

**s3q is a thin S3-backed queue product layer over `pgqrs::store::s3::S3Store`.**

The intended product shape is:

- **Queue APIs shaped by `pgmq` naming**
- **Lease-and-ack semantics familiar from SQS**
- **Producer and consumer identities preserved from `pgqrs`**
- **Read-only inspection APIs for metrics and message visibility**
- **Rust core**
- **Thin Python SDK**
- **Python CLI and optional service**

This repository is currently in the **seed stage**. The code in `src/` and `python/` defines the initial package shape and target API vocabulary; it is not yet wired to `pgqrs::store::s3::S3Store`.

## Design Direction

- Keep `s3q` thin. Missing queue capabilities belong in `pgqrs`, not in a parallel implementation inside this repository.
- Support only `S3Store` as the v1 backend.
- Use `pgmq` API vocabulary when in doubt: `send`, `read`, `set_vt`, `archive_message`, `metrics`.
- Preserve `pgqrs` producer and consumer worker identity so ownership, heartbeats, and incident history remain useful.
- Keep inspection APIs read-only.

## Repository Layout

- `src/`: Rust library crate and public API scaffolding
- `python/`: thin Python SDK, CLI, and service scaffolding
- `docs/`: user-facing docs intended for `s3q.dev`
- `engg/`: internal product and engineering documents
- `.buildkite/`: CI pipeline

## Local Commands

```bash
mise install
mise exec -- make check
mise exec -- make test
mise exec -- make docs
```

The `Makefile` remains the task entrypoint. `mise` pins tool versions and is used by CI.

## First Documents

- Product requirements: `engg/product-requirements.md`
- Queue architecture: `engg/design/queue-architecture.md`
- API design: `engg/design/api-design.md`
- Implementation plan: `engg/design/implementation-plan.md`
- Repository setup plan: `engg/design/repository-setup-plan.md`

## Near-Term Implementation Plan

1. Finish the repository baseline: `mise`, CI, Zensical docs, and engineering-doc structure.
2. Refactor the Rust scaffold to the approved queue-only API.
3. Add any missing capabilities to `pgqrs`, then wrap `S3Store` from `s3q`.
4. Implement Rust queue and inspection surfaces.
5. Mirror the Rust surface in the thin Python SDK.
6. Build the Python CLI on top of the Python SDK.
