# ADR 0002: Queue-Only V1

## Status

Accepted

## Context

s3q was initially seeded as an S3-backed queue plus durable workflow project. Subsequent product review narrowed v1 to a queue-only product layer over `pgqrs::store::s3::S3Store`.

The queue surface should follow `pgmq` behavior when in doubt, preserve SQS-like lease-and-ack semantics, and expose explicit archive operations for retained history.

## Decision

s3q v1 will focus only on queues and inspection:

- no public workflow APIs
- no public DLQ or replay APIs
- `S3Store` is the only v1 backend
- missing queue capabilities belong in `pgqrs`, not in `s3q`
- producer and consumer worker identity remains first-class
- archive is a user-facing retained-completion operation

## Consequences

- The product surface is smaller and easier to validate.
- The repository docs and scaffold must remove workflow language.
- Future workflow work requires a new PRD and design process.
- `s3q` remains a thin product layer instead of a parallel queue engine.

## Alternatives Considered

### Queue Plus Workflow

Keep the original queue plus durable workflow scope.

Rejected because it made the v1 surface too broad and pulled the project away from the practical `pgqrs` constraint.

### Queue Only Without Archive

Expose delete-only completion in v1.

Rejected because `pgqrs` already supports archive, and archived messages are useful for stats and debugging.

## References

- `engg/product-requirements.md`
- `engg/design/queue-architecture.md`
- `engg/design/api-design.md`
