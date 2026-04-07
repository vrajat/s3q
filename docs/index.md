# s3q

s3q is a proposal and implementation effort for an **S3-backed queue product layer** built on top of `pgqrs::store::s3::S3Store`.

The goal is not to clone an existing queue product feature-for-feature. The goal is to expose a small, practical queue API with:

- `pgmq`-style API naming
- SQS-like lease-and-ack semantics
- exact metrics and read-only inspection
- `pgqrs` producer and consumer worker identity
- `S3Store` as the only v1 backend

## Product Shape

- A Rust core library that maps product terminology to `pgqrs`
- A thin Python SDK over that core
- A Python CLI and optional service layer
- A docs site hosted at `s3q.dev`

## Guiding Principle

Keep `s3q` thin. If a queue capability is missing, implement it in `pgqrs` and expose it here as product terminology.

## Read Next

- [User guide overview](user-guide/index.md)
- [Queue model](user-guide/concepts/queue-model.md)
- [Inspection and metrics](user-guide/concepts/inspection-and-metrics.md)
- [Quickstart](user-guide/getting-started/quickstart.md)

For internal planning and architecture, see `engg/product-requirements.md` in the repository.
