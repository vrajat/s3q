# Queue Architecture

**Status:** Proposed  
**Scope:** Queue-only `s3q` architecture on top of `pgqrs::store::s3::S3Store`

## 1. Goal

Define how `s3q` should implement its queue and inspection APIs using `pgqrs` as the durable substrate.

This document is the bridge between:

- the product requirements in `engg/product-requirements.md`
- the later API design doc
- the later implementation plan

## 2. Architecture Summary

`s3q` should be a **thin product layer over `pgqrs`**, not a second queue engine.

This is a hard constraint:

- if `s3q` needs a capability that `pgqrs` does not expose cleanly, that capability should be added to `pgqrs`
- `s3q` should not implement queue semantics outside `pgqrs`

The intended layering is:

1. `pgqrs::store::s3::S3Store` provides durable storage backed by S3 plus a local SQLite cache.
2. `pgqrs` queue/message tables and worker/admin builders provide the low-level queue mechanics.
3. `s3q` Rust code defines the public queue and inspection APIs, request validation, and product-facing types.
4. Python SDK, CLI, and optional service call into the Rust core.

The main architectural decision is:

- **reuse `pgqrs` queue semantics directly**
- **wrap them in a `pgmq`-shaped public API**
- **push missing queue capabilities into `pgqrs`, not `s3q`**

## 3. What `pgqrs` Already Provides

`pgqrs` already provides most of the queue substrate `s3q` needs:

- durable queue and message storage
- enqueue and batch enqueue
- dequeue with visibility timeout and read count tracking
- producer and consumer worker identities
- worker lifecycle status and heartbeats
- delete by owned consumer
- archive by owned consumer
- metrics for individual queues and all queues
- archived message listing
- S3-backed storage through `S3Store`

Important existing primitives:

- message archive is represented in the main message store via `archived_at`
- queue metrics already include archived message counts
- `move_to_dlq` exists internally, but public DLQ APIs are out of scope for `s3q` v1

This means `s3q` should avoid inventing new persistence structures or queue logic.
If `pgqrs` cannot express a required public behavior cleanly, the correct fix is to extend `pgqrs`.

## 4. Core Data Model

At the `s3q` layer, the queue model should be expressed in product terms even though it is implemented using `pgqrs` records.

### Core Entities

- **Queue**
  Named logical queue.
- **Producer**
  A worker identity that produces messages into a queue.
- **Consumer**
  A worker identity that reads and owns leased messages from a queue.
- **Message**
  Durable payload plus queue lifecycle metadata.
- **Lease**
  A temporary claim on a message acquired by `read` or `read_batch`.
- **Archive**
  Retained terminal message state used for historical stats and debugging.

### State Model

The public message state model should be:

- **visible**
- **leased**
- **delayed**
- **archived**

These states are inferred from `pgqrs` fields rather than stored as a new enum.

Suggested mapping:

- `archived`: `archived_at IS NOT NULL`
- `leased`: `archived_at IS NULL` and currently owned by a consumer worker
- `visible`: `archived_at IS NULL`, unowned, and `vt <= now`
- `delayed`: `archived_at IS NULL`, unowned, and `vt > now`

This mapping should be centralized inside `s3q` so CLI and SDK inspection APIs render the same state labels.

## 5. Queue Operation Mapping

`s3q` should stay close to `pgmq` API naming while using `pgqrs` internals.

### Queue Management

- `create_queue`
  Maps to `pgqrs::admin(...).create_queue(...)` or equivalent store queue creation.
- `delete_queue`
  Maps to queue deletion in `pgqrs` and remains destructive.
- `purge_queue`
  Maps to `pgqrs` queue purge behavior for active queue contents.

### Worker-Bound Handles

`s3q` should preserve the producer/consumer identity model from `pgqrs`.

That means:

- queue-scoped mutation should not collapse into anonymous stateless methods
- producer operations should live on a `Producer` handle
- ownership-sensitive consumer operations should live on a `Consumer` handle

This keeps:

- durable worker ids
- heartbeats
- activity tracking
- ownership validation
- incident/debuggability semantics

### Message Production

- `send`
  Maps to producer enqueue.
- `send_batch`
  Maps to producer batch enqueue.

### Message Consumption

- `read`
  Maps to dequeue of one message with lease semantics.
- `read_batch`
  Maps to dequeue of multiple messages with lease semantics.
- `read_with_poll`
  Should use polling support exposed by `pgqrs`. If the current surface is not sufficient for the desired public API, the polling capability should be added or refined in `pgqrs`.

### Message Finalization

- `delete_message`
  Maps to owned delete and should live on `Consumer`.
- `archive_message`
  Maps to owned archive and should live on `Consumer`.
- `archive_messages`
  Maps to owned batch archive and should live on `Consumer`.
- `set_vt`
  Maps to visibility extension or visibility reset on a leased message and should live on `Consumer`.

## 6. Inspection Architecture

Inspection is a first-class product surface.

It should be implemented as **read-only queries over the same durable state** used by queue operations.

### Inspection APIs

V1 inspection APIs:

- `list_queues`
- `metrics`
- `metrics_all`
- `list_messages`
- `get_message`
- `list_archived_messages`

### Mapping to `pgqrs`

- `list_queues`
  Maps to queue table listing.
- `metrics`
  Maps to `queue_metrics`.
- `metrics_all`
  Maps to `all_queues_metrics`.
- `get_message`
  Maps to direct message lookup.
- `list_archived_messages`
  Maps to `list_archived_by_queue`.
- `list_messages`
  Should use filtering provided by `pgqrs`, with `s3q` translating the results into product terminology.

### Inspection Design Rules

- inspection must never create a lease
- inspection must never acknowledge or archive
- inspection should surface a normalized public state model
- metrics and message listing should agree on terminology

## 7. Archive Design

Archive is a public queue API in `s3q` v1.

### Intended Semantics

- archive is an explicit user action
- archive is distinct from delete
- archived messages are retained for debugging and historical stats
- archive remains terminal in v1 from the public API perspective

### Important Architectural Constraint

Even though `pgqrs` already has replay-oriented internal APIs, `s3q` should not expose replay in v1.

So `s3q` should treat archive as:

- **publicly inspectable**
- **publicly writeable through explicit archive operations**
- **not publicly replayable yet**

This preserves a clean public model now while leaving room for future replay features.

## 8. S3Store Integration Strategy

`s3q` should treat `S3Store` as the only backend for the product in v1.

The architectural stance should be:

- `s3q` depends on `pgqrs` behavior, not on direct SQLite table management
- `s3q` should not bypass `pgqrs` to talk to local SQLite files directly
- `s3q` should rely on `pgqrs` store APIs for both queue writes and inspection reads
- `s3q` should not expose backend selection as a product feature in v1

This matters because `S3Store` already owns:

- local cache management
- object store synchronization
- durability mode behavior
- backend-specific operational constraints

`s3q` should remain agnostic to those details except where product behavior needs to describe them.

## 9. Concurrency and Delivery Model

The product should inherit the `pgqrs` queue model:

- at-least-once delivery
- visibility timeout based leasing
- duplicate delivery under failure
- terminal completion through delete or archive

`s3q` should not promise:

- exactly-once delivery
- strict FIFO behavior
- cross-consumer ordering guarantees

For the public product narrative, the right framing is:

- `read` leases work
- `delete_message` or `archive_message` completes work
- failure to complete before visibility timeout allows redelivery

## 10. Error and Ownership Model

`pgqrs` already distinguishes between actions that require consumer ownership and actions that are simple reads.

`s3q` should preserve that distinction.

This means:

- deletion and archive should require a valid lease/ownership path
- ownership should be tied to a real consumer worker identity
- inspection APIs should not require consumer ownership
- failed ownership-sensitive operations should produce clear, product-level errors

The API design doc should define the exact error vocabulary, but the architecture should preserve the ownership boundary from the start.

## 11. Rust and Python Boundary

The Rust layer should own:

- queue state mapping
- queue/inspection request handling
- integration with `pgqrs`
- normalization of metrics and message state
- producer and consumer handle lifecycle

The Python layer should own:

- ergonomic wrappers
- CLI integration
- optional service packaging

The Python layer should not:

- duplicate queue semantics
- reimplement polling logic independently
- perform direct database inspection outside the Rust core

## 12. Open Technical Questions

These need resolution in the next docs or implementation work:

- What exact `pgqrs` surface should back `read_with_poll` for `s3q`?
- What exact `pgqrs` filtering surface should back `list_messages`?
- What should the public receipt-handle format be, and which parts of it should remain opaque to `s3q`?
- Which worker lifecycle controls should be public in v1 versus internal?
- Should batch delete be in the first SDK surface, given that batch archive is already supported cleanly?

## 13. Next Documents

The next design docs should be:

1. `engg/design/api-design.md`
2. `engg/design/implementation-plan.md`

The API design doc should settle naming, method signatures, and payload types.
The implementation plan should then order the code changes and test coverage.
