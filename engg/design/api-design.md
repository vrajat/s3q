# API Design

**Status:** Proposed  
**Scope:** Public Rust, Python, and CLI APIs for queue and inspection operations in `s3q` v1

## 1. Goal

Define the public API surface for `s3q` before implementation starts.

This document should answer:

- what the Rust API looks like
- what the Python API looks like
- what the CLI looks like
- which names and behaviors are in v1
- which current scaffolded APIs should be replaced

## 2. Design Rules

### 2.1 Follow `pgmq` When in Doubt

The default API precedent is `pgmq`.

That means:

- prefer `send`, not `send_message`
- prefer `send_batch`, not `send_message_batch`
- prefer `read`, `read_batch`, and `read_with_poll`
- prefer `metrics` and `metrics_all`
- prefer `set_vt`

### 2.2 Keep SQS-like Semantics

Even when names follow `pgmq`, semantics should stay:

- lease-and-ack
- visibility timeout based
- at-least-once delivery
- duplicate delivery possible

### 2.3 Thin-Layer Constraint

`s3q` is only a thin product layer.

That means:

- `s3q` should only support `pgqrs::store::s3::S3Store` in v1
- if a needed capability is missing, it should be added to `pgqrs`
- `s3q` should translate `pgqrs` capabilities into product terminology, not reimplement them

### 2.4 Keep Python Thin

Python should closely mirror the Rust surface.

Avoid:

- Python-only semantics
- a second queue engine in Python
- CLI behavior that diverges from SDK behavior

## 3. Public Surface Areas

`s3q` exposes two product surfaces:

1. queue operations
2. inspection operations

These should be visible in all three interfaces:

- Rust SDK
- Python SDK
- CLI

## 4. Rust API

## 4.1 Top-Level Client

The Rust entrypoint should stay client-oriented.

```rust
let client = s3q::Client::connect("s3://bucket/queues.db")?;
```

Preferred shape:

```rust
let client = s3q::connect("s3://bucket/queues.db");
let queue = client.queue("emails");
let inspect = client.inspect();
```

Recommended client methods:

- `Client::connect(...)` or free `connect(...)`
- `client.queue(name)`
- `client.inspect()`
- optional later: `client.with_namespace(...)`

## 4.2 Queue Handle

The queue handle should own queue-scoped operations.

Recommended methods:

- `create_queue()`
- `delete_queue()`
- `purge_queue()`
- `producer(worker_name)`
- `consumer(worker_name)`

### Naming Decision

For v1, use:

- `send`
- `send_batch`
- `read`
- `read_batch`
- `read_with_poll`
- `delete_message`
- `archive_message`
- `archive_messages`
- `set_vt`

This keeps the queue lifecycle readable while still following `pgmq`.

### Ownership Decision

For finalization and visibility operations, the API should be ownership-aware and consumer-bound.

V1 decision:

- preserve producer and consumer identities from `pgqrs`
- expose a `ReceiptHandle` concept in the public API
- use receipt-handle based completion APIs
- do not rely on bare message id alone for consumer completion paths

`ReceiptHandle` should be backed by `pgqrs` capability. If the current `pgqrs` surface does not expose an appropriate lease token, that support should be added to `pgqrs`.

### Producer Handle

Producer operations should live on a first-class `Producer` handle.

Recommended methods:

- `send(payload)`
- `send_batch(payloads)`

Suggested usage:

```rust
let producer = client.queue("emails").producer("api-worker").await?;
producer.send(json!({"to": "user@example.com"})).await?;
```

### Consumer Handle

Consumer operations should live on a first-class `Consumer` handle.

Recommended methods:

- `read(vt)`
- `read_batch(vt, qty)`
- `read_with_poll(vt, qty, poll_timeout, poll_interval)`
- `delete_message(receipt_handle)`
- `archive_message(receipt_handle)`
- `archive_messages(receipt_handles)`
- `set_vt(receipt_handle, vt_offset)`
- optional later:
  - `heartbeat()`
  - `status()`
  - `interrupt()`

Suggested usage:

```rust
let consumer = client.queue("emails").consumer("worker-a").await?;
let messages = consumer.read_batch(30, 10).await?;

for msg in messages {
    consumer.archive_message(msg.receipt_handle.as_ref().unwrap()).await?;
}
```

## 4.3 Inspection Handle

The inspection API should be separate from the queue handle.

Recommended shape:

```rust
let inspect = client.inspect();
```

Methods:

- `list_queues()`
- `metrics(queue_name)`
- `metrics_all()`
- `list_messages(queue_name, filter)`
- `get_message(queue_name, message_id)`
- `list_archived_messages(queue_name, filter)`

This separation keeps read-only operations distinct from queue mutations.

## 4.4 Rust Data Types

The Rust API should expose stable product-facing types.

Recommended v1 types:

- `QueueInfo`
- `QueueMetrics`
- `ProducerInfo`
- `ConsumerInfo`
- `Message`
- `ArchivedMessage`
- `ReadRequest`
- `ReadBatchRequest`
- `ListMessagesRequest`
- `ListArchivedMessagesRequest`
- `MessageState`
- `ReceiptHandle`

### Message Shape

Suggested public message shape:

```rust
pub struct Message<T = serde_json::Value> {
    pub message_id: i64,
    pub read_count: i32,
    pub enqueued_at: SystemTime,
    pub visible_at: SystemTime,
    pub payload: T,
    pub receipt_handle: Option<ReceiptHandle>,
    pub state: MessageState,
}
```

Notes:

- inspection paths may return `receipt_handle = None`
- read/read_batch return leased messages with receipt handles
- archived listing can reuse the same shape with `state = Archived`

Suggested type:

```rust
pub struct ReceiptHandle(pub String);
```

The handle should be treated as opaque by `s3q`.

### Message State

```rust
pub enum MessageState {
    Visible,
    Leased,
    Delayed,
    Archived,
}
```

## 4.5 Rust Error Model

The Rust surface should define product-level errors over raw `pgqrs` errors.

Suggested categories:

- `QueueNotFound`
- `MessageNotFound`
- `LeaseExpired`
- `OwnershipMismatch`
- `InvalidArgument`
- `StoreUnavailable`
- `Internal`

The exact enum can be finalized during implementation, but the public model should be stable enough for Python bindings.

## 5. Python API

## 5.1 Python Client

The Python API should mirror the Rust client shape closely.

Preferred usage:

```python
import s3q

client = s3q.connect("s3://bucket/queues.db")
queue = client.queue("emails")
inspect = client.inspect()
```

Recommended client methods:

- `Client.connect(...)`
- `connect(...)`
- `client.queue(name)`
- `client.inspect()`

## 5.2 Python Queue Handle

Recommended queue methods:

- `create_queue()`
- `delete_queue()`
- `purge_queue()`
- `producer(worker_name)`
- `consumer(worker_name)`

Use Python-friendly arguments, but keep names aligned with Rust.

Recommended producer methods:

- `send(payload, delay=None)`
- `send_batch(payloads, delay=None)`

Recommended consumer methods:

- `read(vt: int | None = None)`
- `read_batch(vt: int | None = None, qty: int = 1)`
- `read_with_poll(vt: int | None = None, qty: int = 1, poll_timeout=None, poll_interval=None)`
- `delete_message(receipt_handle)`
- `archive_message(receipt_handle)`
- `archive_messages(receipt_handles)`
- `set_vt(receipt_handle, vt_offset)`

## 5.3 Python Inspection Handle

Recommended methods:

- `list_queues()`
- `metrics(queue_name)`
- `metrics_all()`
- `list_messages(queue_name, state=None, limit=None, cursor=None)`
- `get_message(queue_name, message_id)`
- `list_archived_messages(queue_name, limit=None, cursor=None)`

## 5.4 Python Types

Recommended Python dataclasses or binding-backed objects:

- `QueueInfo`
- `QueueMetrics`
- `Message`
- `MessageState`

The Python `Message` type should expose:

- `message_id`
- `payload`
- `read_count`
- `enqueued_at`
- `visible_at`
- `receipt_handle`
- `state`

## 6. CLI Design

## 6.1 Top-Level Shape

The CLI should reflect the two public surfaces:

- `s3q queue ...`
- `s3q inspect ...`

## 6.2 Queue Commands

Recommended v1 commands:

- `s3q queue create <queue>`
- `s3q queue delete <queue>`
- `s3q queue purge <queue>`
- `s3q queue send <queue>`
- `s3q queue send-batch <queue>`
- `s3q queue read <queue>`
- `s3q queue read-batch <queue>`
- `s3q queue read-with-poll <queue>`
- `s3q queue delete-message <queue>`
- `s3q queue archive-message <queue>`
- `s3q queue set-vt <queue>`

Notes:

- queue name should be positional in the CLI
- DSN can be a global option or env var
- body input can be file, stdin, or literal string later

## 6.3 Inspection Commands

Recommended v1 commands:

- `s3q inspect queues`
- `s3q inspect metrics <queue>`
- `s3q inspect metrics-all`
- `s3q inspect messages <queue>`
- `s3q inspect message <queue> <message-id>`
- `s3q inspect archived <queue>`

## 6.4 CLI Output

CLI output should prefer:

- human-readable tables by default
- JSON output via `--json`

This is especially important for:

- metrics
- list messages
- archived message inspection

## 7. Current Scaffold Changes Required

The current scaffold does not match the target API.

### Rust

Current mismatches in `src/queue.rs`:

- `send_message` should become `send`
- `send_message_batch` should become `send_batch`
- `receive_messages` should become `read`, `read_batch`, and `read_with_poll`
- `change_message_visibility` should become `set_vt`
- queue attributes should be removed from the v1 public surface unless reintroduced deliberately later
- receipt-handle based completion should be introduced
- producer and consumer handles should become first-class

### Python

Current mismatches in `python/s3q/queue.py`:

- `send_message` should become `send`
- `receive_messages` should become `read` and `read_batch`
- `change_message_visibility` should become `set_vt`
- archive methods are missing
- inspection surface is missing
- receipt-handle based completion should be introduced
- producer and consumer handles should become first-class

### CLI

Current mismatches in `python/s3q/cli.py`:

- workflow commands must be removed
- queue commands must expand to the queue-only PRD surface
- inspection commands must be added

## 8. Deferred APIs

These should not be part of v1:

- replay of archived messages
- public DLQ APIs
- FIFO APIs
- dedup APIs
- `pop`
- queue attribute management
- full worker/admin management APIs

## 9. Open API Questions

These should be resolved in the implementation plan if not sooner:

- Should `read(vt)` default to a store-configured visibility timeout when omitted?
- Should `read_batch` return `[]` or `None` when no messages are available?
- Should inspection filters use cursor pagination or simple limit-first pagination in v1?
- Should the Rust API expose generic typed payloads immediately, or default to JSON values first?
- What exact receipt-handle format should `pgqrs` expose to `s3q`?
- Which worker lifecycle methods should be public in `s3q` v1?

## 10. Recommendation

For v1, the cleanest API is:

- `pgmq` naming
- SQS-style lease semantics
- explicit archive
- first-class producer and consumer handles
- separate inspection handle
- JSON-first payloads
- receipt-handle based completion APIs

That is the smallest coherent product surface that matches the PRD and stays implementable on top of `pgqrs`.
