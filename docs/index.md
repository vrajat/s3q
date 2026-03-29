# s3q

s3q is a proposal and implementation effort for an **S3-backed queue plus durable workflow system** built on top of `pgqrs`.

The intent is not to clone existing products feature-for-feature. The goal is to borrow the right mental models:

- **SQS** for queue semantics
- **Temporal** for workflow semantics
- **pgqrs S3Store** for durable storage and recovery

## Product Shape

- A Rust core library that owns queue and workflow semantics
- A thin Python SDK over that core
- A Python CLI and optional service layer
- A docs site hosted at `s3q.dev`

## Guiding Principle

Use familiar APIs where that reduces product risk, but let `pgqrs` storage constraints drive the actual implementation.

## Read Next

- [User guide overview](user-guide/index.md)
- [Queue model](user-guide/concepts/queue-model.md)
- [Workflow model](user-guide/concepts/workflow-model.md)

For internal planning and architecture, see `engg/product-requirements.md` in the repository.
