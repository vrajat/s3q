# Workflow Model

Workflows are not queues with extra fields.

## Core Semantics

- A workflow has a stable identity plus one or more runs.
- A workflow can receive **signals** while it is running.
- A workflow can expose **queries** for durable state inspection.
- A workflow can spawn **child workflows**.
- A workflow can block on **timers** without losing progress.

## Why Temporal Is the Right Model

SQS is a transport primitive. Temporal is an execution primitive.

s3q should copy Temporal's ideas about:

- handles
- execution state
- signals
- result retrieval
- cancellation
- timers

It should not try to mimic Temporal internals or every Temporal feature.

## Initial Constraints

The first implementation should stay small:

- linear or modestly branching workflows
- durable state persisted through `pgqrs`
- explicit workflow handles in Rust and Python
- no attempt to match Temporal's full event history model on day one
