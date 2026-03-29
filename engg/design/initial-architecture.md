# Initial Architecture

## Layering

The repository is seeded around four layers:

1. **Storage substrate**
   `pgqrs`, specifically `S3Store`, owns durable persistence and object-store synchronization.
2. **Rust core**
   `s3q` owns queue semantics, workflow semantics, and the public core API.
3. **Python SDK**
   Python remains a thin wrapper over Rust. It should not contain a separate queue or workflow engine.
4. **Python service**
   A later service process can expose HTTP or other APIs, but it should call the same SDK surface.

## Why Separate Queue and Workflow

Queues and workflows solve different problems:

- queues move messages through a lease-and-ack lifecycle
- workflows manage execution state over time

Trying to force one abstraction to behave like the other will make both surfaces worse.

## Suggested First Implementation Order

1. Add a `pgqrs` adapter module inside the Rust crate.
2. Implement queue primitives first because their semantics map more directly to existing `pgqrs` behavior.
3. Add workflow execution records and handles next.
4. Bind that Rust surface into Python only after the Rust semantics are stable enough to avoid churn.

## Service Direction

The service should be treated as optional packaging, not as the core product. That keeps:

- correctness in one place
- CLI and service behavior aligned
- local embedded use possible from day one
