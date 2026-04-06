# s3q Product Requirements

**Status:** Seed / pre-implementation  
**Audience:** Maintainers and contributors  
**Scope:** Rust core, Python SDK, Python CLI/service, queue product surface plus inspection APIs

## 1. Problem Statement

We want a practical **S3-backed queue system** built on top of `pgqrs`, specifically `pgqrs::store::s3::S3Store`.

The product should do two things well:

- provide familiar **queue APIs** for producing and consuming messages
- provide strong **inspection APIs** for understanding queue state by reading durable state from the backing store

This is intentionally smaller than a general durable workflow engine.

Archive is part of the queue product in v1. Public DLQ concepts are not.

## 2. Product Positioning

s3q is:

- a **pgmq-shaped queue library with SQS-like semantics**
- with **database-backed inspection and observability APIs**
- built as a **Rust core**
- exposed through a **thin Python SDK**
- with a **Python CLI and optional Python service**

s3q is **not** a workflow engine in the first version.

## 3. Target Users

- teams that want durable queue semantics on object storage
- teams that want simple producer/consumer infrastructure without operating a full queue service
- teams that want CLI and API inspection of queue state for debugging and operations
- Python users who want operational tooling over the same queue core

## 4. Goals

### Functional Goals

- expose a queue API centered on lease and ack
- expose explicit archive operations for retained message history
- expose read-only inspection APIs for queue and message state
- support Rust and Python with aligned naming
- keep Python as a thin wrapper over the Rust core
- make CLI and service entrypoints available in Python

### Product Goals

- make the queue API familiar to users of SQS-style queue systems while staying close to `pgmq` naming and surface area
- make inspection APIs useful for day-to-day operational debugging
- preserve room for practical deviations when `pgqrs` and `S3Store` make a different tradeoff necessary

## 5. Non-Goals

- full SQS compatibility
- workflow orchestration
- Temporal-style workflow handles, signals, queries, or timers
- exactly-once delivery guarantees
- public DLQ APIs in v1
- deep metrics, tracing, and dashboards in the first phase
- a very large service surface before the core library works

## 6. Product Invariants

These should remain stable even if implementation details change.

1. **Durability**
   Queue state must survive crashes and restarts.
2. **Lease-and-Ack Semantics**
   Messages are finalized only on delete or equivalent acknowledgement.
3. **At-Least-Once Delivery**
   Duplicate delivery is allowed and expected under failure.
4. **Archive Is Explicit**
   User-initiated archive must remain distinct from delete.
5. **Read-Only Inspection Surface**
   Inspection APIs must not mutate queue state.
6. **Thin Python Layer**
   Python should not grow a second queue engine.
7. **pgqrs-Constrained Design**
   Storage behavior must stay grounded in what `pgqrs` and `S3Store` can support cleanly.

## 7. Public Surface Areas

s3q should expose two public surfaces:

1. **Queue APIs**  
   Mutating operations that create queues, send messages, lease messages, ack messages, and manage queue settings.
2. **Inspection APIs**  
   Read-only operations that help users understand queue state by reading durable records from the backing store.

## 8. Initial Queue API

The initial queue API should follow `pgmq` naming and surface area when in doubt, while preserving SQS-like lease/ack semantics:

- `create_queue`
- `delete_queue`
- `purge_queue`
- `send`
- `send_batch`
- `read`
- `read_batch`
- `read_with_poll`
- `delete_message`
- `archive_message`
- `archive_messages`
- `set_vt`

### Required Queue Semantics

- `read` and `read_batch` return leased messages
- delete finalizes the message
- archive finalizes the message while retaining it for debugging and historical stats
- visibility timeout controls the retry window
- duplicate delivery is possible
- batch operations are convenience APIs, not a new semantic model
- FIFO and dedup are explicitly deferred or optional

### Archive Semantics

- archive is an explicit user API in v1
- archived messages are retained for stats and debugging
- archive is not a synonym for delete
- queue deletion must remain destructive and distinct from archival behavior
- automatic dead-letter behavior is deferred from the public v1 API, even if internal policies later use archived storage
- replay APIs are deferred from v1

## 9. Initial Inspection API

The initial inspection API should be small, read-only, and operationally useful:

- `list_queues`
- `metrics`
- `metrics_all`
- `list_messages`
- `get_message`
- `list_archived_messages`

### Required Inspection Semantics

- inspection must not create leases
- inspection must not acknowledge messages
- inspection results may be approximate when the underlying store only supports approximate counts cheaply
- users should be able to inspect message state such as visible, leased, delayed, or archived when available

### Suggested First Filters

- queue name
- message id
- state
- limit
- cursor or offset, depending on what fits the storage model best

## 10. Queue State Model

The product should adopt a simple queue state model:

- **visible**: available for receipt
- **leased**: currently hidden by visibility timeout
- **delayed**: not yet visible due to delay
- **archived**: removed from active processing but retained for stats and replay/debugging

This state model is mainly important for inspection APIs and CLI output.

## 11. Architecture Constraints

The architecture should follow this layering:

1. `pgqrs` and `S3Store` provide the durable substrate.
2. Rust core defines queue semantics and inspection semantics.
3. Python SDK wraps the Rust core.
4. Python CLI and service call into the Python SDK.

This means:

- the Rust core owns correctness
- Python owns ergonomics and operational entrypoints
- the service should not fork semantics away from the library

## 12. CLI Direction

The CLI should mirror the two public surfaces.

Queue commands:

- `s3q queue create`
- `s3q queue delete`
- `s3q queue purge`
- `s3q queue send`
- `s3q queue send-batch`
- `s3q queue read`
- `s3q queue read-batch`
- `s3q queue read-with-poll`
- `s3q queue delete-message`
- `s3q queue archive-message`
- `s3q queue set-vt`

Inspection commands:

- `s3q inspect queues`
- `s3q inspect metrics`
- `s3q inspect metrics-all`
- `s3q inspect messages`
- `s3q inspect message`
- `s3q inspect archived`

The inspection commands should be safe to run in production because they are read-only.

## 13. First Delivery Plan

### Phase 0: Seed

- repository layout
- PRD
- architecture sketch
- queue and inspection vocabulary in Rust and Python

### Phase 1: Queue MVP

- connect to `pgqrs` S3Store
- create queue
- send message
- read messages with lease
- delete message
- archive message
- set visibility timeout

### Phase 2: Inspection MVP

- list queues
- metrics
- metrics all
- list messages
- get message by id
- list archived messages

### Phase 3: Python Product Surface

- thin Python bindings
- CLI commands for queue and inspection
- minimal service process

## 14. Open Questions

- What queue and message state is already available from `pgqrs` admin APIs versus requiring new reads?
- Which inspection counts can be exact and which should be explicitly approximate?
- What is the right pagination model for `list_messages` on top of the underlying store?
- How much queue worker metadata is available or worth exposing?
- Should `peek_messages` exist as a convenience API, or is `list_messages` enough?
- Do we eventually need archive reasons such as `user_archive` vs `system_archive` to support a future DLQ concept without changing the storage model?

## 15. References

- `pgqrs.vrajat.com`
- `https://turbopuffer.com/blog/object-storage-queue`
- SQS as the queue API inspiration
