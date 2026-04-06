# Implementation Plan

**Status:** Proposed  
**Scope:** Implement the queue-only `s3q` product defined by the PRD, queue architecture, and API design docs

## 1. Goal

Turn the current queue-only design into working code with a clear execution order.

This plan should answer:

- what gets built first
- which files and modules are responsible for each layer
- what must be finished before the next phase begins
- how Rust, Python, CLI, and inspection features are staged
- how docs stay aligned through implementation rather than being deferred to the end

## 2. Inputs

This plan depends on:

- `engg/product-requirements.md`
- `engg/design/queue-architecture.md`
- `engg/design/api-design.md`
- `engg/design/repository-setup-plan.md`

The repository setup plan is a parallel prerequisite. Product implementation should not assume the current scaffold is the final developer/CI environment.

## 3. Build Principles

### 3.1 Rust First

The Rust layer owns:

- `pgqrs` integration
- queue semantics
- inspection semantics
- state mapping
- error normalization
- producer and consumer handle lifecycle

Python and CLI should come after the Rust surface is stable enough to avoid thrash.

### 3.2 Thin-Layer Rule

`s3q` is only a thin product layer over `pgqrs`.

That means:

- `s3q` supports only `S3Store` in v1
- any missing queue capability should be implemented in `pgqrs`
- `s3q` should not grow its own queue semantics to compensate

### 3.3 Follow `pgmq` Naming

When there is ambiguity in names or surface area, follow `pgmq`.

That means the product should converge toward:

- `send`
- `send_batch`
- `read`
- `read_batch`
- `read_with_poll`
- `delete_message`
- `archive_message`
- `archive_messages`
- `set_vt`
- `metrics`
- `metrics_all`

### 3.4 Keep V1 Small

V1 should implement:

- queue operations
- archive
- inspection

V1 should defer:

- replay
- DLQ APIs
- FIFO
- dedup
- queue attributes
- worker/admin productization

### 3.5 Keep Docs Live

Docs should not be treated as an end-of-project cleanup phase.

The intended model is:

- docs infrastructure and skeleton are created during repository setup
- each implementation phase updates the affected docs
- a final docs phase, if any, is for polish and consistency only

## 4. Desired End State

At the end of this implementation plan, `s3q` should provide:

- a working Rust client over `pgqrs::store::s3::S3Store`
- a thin Python wrapper over the Rust core
- a queue CLI and inspection CLI
- queue metrics and archived-message inspection
- docs and tests aligned with the queue-only product

## 5. Module Plan

## 5.1 Rust

The current `src/` scaffold should be reshaped around queue and inspection concerns.

Suggested module layout:

- `src/lib.rs`
  Public exports only
- `src/client.rs`
  `Client`, connection/bootstrap helpers
- `src/config.rs`
  runtime config and DSN handling
- `src/error.rs`
  product-level error surface
- `src/queue.rs`
  queue handle plus producer and consumer handles
- `src/inspect.rs`
  inspection handle and read-only operations
- `src/types.rs`
  public message, metrics, queue, and state types
- `src/pgqrs.rs`
  internal adapter layer over `pgqrs`

The current workflow module should be removed from the public surface in the implementation phase.

## 5.2 Python

Suggested Python module layout:

- `python/s3q/__init__.py`
- `python/s3q/client.py`
- `python/s3q/queue.py`
- `python/s3q/inspect.py`
- `python/s3q/types.py`
- `python/s3q/errors.py`
- `python/s3q/cli.py`
- `python/s3q/service.py`

The current Python workflow scaffold should be removed.

## 5.3 CLI

The CLI should remain in Python, but it should use the Python SDK rather than direct store logic.

That keeps:

- CLI behavior aligned with SDK behavior
- one product semantics path
- future service integration simpler

## 6. Phased Execution Plan

## Phase 0: Repository Baseline

This phase is defined in `engg/design/repository-setup-plan.md`.

Minimum expectation before significant product work:

- `mise.toml`
- improved Buildkite pipeline
- docs migration direction settled
- docs skeleton established
- README/docs cleanup underway

This phase can overlap with design work, but product implementation should not get deep before the baseline is in place.

## Phase 1: Rust Surface Refactor

### Objective

Replace the current scaffolded public API with the approved queue-only API design.

### Tasks

- remove workflow exports from `src/lib.rs`
- replace SQS-style scaffold names with `pgmq`-style names
- add `inspect` surface to the public client
- remove queue attributes from the v1 public surface
- make producer and consumer handles first-class
- introduce public message and metrics types aligned to the API design
- define product-level errors and result types

### Files

- `src/lib.rs`
- `src/client.rs`
- `src/error.rs`
- `src/queue.rs`
- new `src/inspect.rs`
- new `src/types.rs`
- remove or retire `src/workflow.rs`

### Exit Criteria

- Rust crate compiles
- public surface matches the API design doc
- no workflow APIs remain in the exported product surface
- docs reflect the new Rust surface at a high level

## Phase 2: `pgqrs` Capability and Adapter Layer

### Objective

Ensure `pgqrs` exposes the capabilities required by `s3q`, then wire the Rust surface to `pgqrs::store::s3::S3Store`.

### Tasks

- add or finalize any missing `pgqrs` capabilities required by `s3q`
- especially add receipt-handle support if it does not already exist in the needed form
- ensure `read_with_poll` can be backed by `pgqrs` polling behavior rather than `s3q` reimplementation
- ensure `list_messages` can use filtering provided by `pgqrs`
- add internal connection and store bootstrap logic
- map `create_queue`, `delete_queue`, and `purge_queue`
- map producer `send` and `send_batch`
- map consumer `read` and `read_batch`
- implement `set_vt`
- map consumer `delete_message`
- map consumer `archive_message` and `archive_messages`
- normalize `pgqrs` records into `s3q` message types

### Files

- new `src/pgqrs.rs`
- `src/client.rs`
- `src/queue.rs`
- `src/types.rs`
- `src/error.rs`

### Key Design Choice

The adapter layer should be internal. `pgqrs` types should not leak into the stable `s3q` public API.

### Exit Criteria

- queue mutation APIs work end-to-end through `S3Store`
- producer and consumer handles preserve `pgqrs` ownership semantics
- Rust integration tests can create a queue, send, read, delete, archive, and set visibility
- Rust usage docs are updated for queue handles, producers, and consumers

## Phase 3: Inspection and Metrics

### Objective

Implement the read-only inspection surface.

### Tasks

- implement `list_queues`
- implement `metrics`
- implement `metrics_all`
- implement `get_message`
- implement `list_archived_messages`
- implement `list_messages` using `pgqrs` filtering plus `s3q` terminology mapping
- centralize message state projection:
  - visible
  - leased
  - delayed
  - archived

### Files

- `src/inspect.rs`
- `src/types.rs`
- `src/pgqrs.rs`
- `src/error.rs`

### Hard Part

`list_messages` is likely the first place where `s3q` adds product terminology over raw `pgqrs` access. It needs:

- state projection
- filtering
- stable public output

### Exit Criteria

- all inspection APIs are read-only
- metrics terminology is consistent across SDK and CLI
- archived message inspection works
- inspection docs are updated for metrics and message listing

## Phase 4: `read_with_poll`

### Objective

Add the long-polling convenience API using `pgqrs` polling support on the consumer path.

### Tasks

- finalize which `pgqrs` polling surface `s3q` wraps
- implement timeout and polling interval handling
- define no-message behavior
- add tests for empty queue and wake-up-on-new-message behavior

### Files

- `src/queue.rs`
- `src/pgqrs.rs`
- tests

### Exit Criteria

- `read_with_poll` behavior is documented and deterministic
- no lease semantics regressions versus `read`
- polling docs are updated

## Phase 5: Python SDK

### Objective

Make Python mirror the Rust product surface.

### Tasks

- remove workflow-related Python code
- add Python queue APIs matching Rust
- add Python producer and consumer handles
- add Python inspection APIs
- expose Python message and metrics types
- normalize exceptions from Rust into Python-friendly errors

### Files

- `python/s3q/__init__.py`
- `python/s3q/client.py`
- `python/s3q/queue.py`
- new `python/s3q/producer.py`
- new `python/s3q/consumer.py`
- new `python/s3q/inspect.py`
- new `python/s3q/types.py`
- `python/s3q/errors.py`
- remove `python/s3q/workflow.py`

### Exit Criteria

- Python mirrors Rust method names and behavior closely
- no Python-only queue semantics are introduced
- Python docs are updated to reflect the implemented surface

## Phase 6: CLI

### Objective

Implement the queue and inspection CLI using the Python SDK.

### Tasks

- remove workflow commands
- add `queue` commands:
  - `create`
  - `delete`
  - `purge`
  - `send`
  - `send-batch`
  - `read`
  - `read-batch`
  - `read-with-poll`
  - `delete-message`
  - `archive-message`
  - `set-vt`
- add `inspect` commands:
  - `queues`
  - `metrics`
  - `metrics-all`
  - `messages`
  - `message`
  - `archived`
- add table and JSON output modes

### Files

- `python/s3q/cli.py`
- `python/s3q/service.py`

### Exit Criteria

- CLI reflects the API design doc
- inspection commands are safe and read-only
- CLI reference docs are updated

## 7. Testing Plan

## 7.1 Rust Tests

Rust should carry the semantic test burden.

Required coverage:

- create/delete/purge queue
- producer send/send_batch
- consumer read/read_batch
- visibility timeout behavior
- delete_message ownership behavior
- archive_message ownership behavior
- list_queues
- metrics/metrics_all
- get_message
- list_messages state mapping
- list_archived_messages
- read_with_poll

## 7.2 Python Tests

Python tests should focus on:

- surface parity
- binding correctness
- CLI/API smoke coverage

Python should not become the only place where queue semantics are validated.

## 7.3 Integration Environment

Primary integration target:

- `pgqrs::store::s3::S3Store`

Recommended environment:

- local or CI S3-compatible object storage
- queue lifecycle tests through the real Rust product surface

## 8. Suggested Milestones

### Milestone A: Rust Queue Core

- Phase 1 complete
- Phase 2 complete

Outcome:

- working Rust queue operations over `S3Store`

### Milestone B: Inspection

- Phase 3 complete
- Phase 4 complete

Outcome:

- working Rust inspection and polling APIs

### Milestone C: Python and CLI

- Phase 5 complete
- Phase 6 complete

Outcome:

- working Python SDK and CLI

### Milestone D: Docs and Release Readiness

- repository setup baseline complete

Outcome:

- project is understandable and usable by others, with docs kept current throughout the implementation phases

## 9. Open Implementation Questions

- What exact receipt-handle format should `pgqrs` expose to `s3q`?
- Does `set_vt` accept any leased message or only messages currently owned by the caller?
- Should `list_messages` pagination be cursor-based in v1, or is limit-only enough?
- Should `send` and `send_batch` be generic over typed payloads immediately, or JSON-first in v1?
- How much of `pgqrs` admin/worker metadata should remain hidden in v1?
- Which worker lifecycle methods should be public in the consumer surface in v1?

## 10. Recommended Immediate Next Actions

1. Complete the repository baseline work in `repository-setup-plan.md`
2. Refactor the Rust scaffold to match the approved queue-only API names
3. Implement the internal `pgqrs` adapter layer
4. Add end-to-end Rust tests before expanding Python

That sequence keeps semantics and product shape stable before surface-area expansion.
