# ADR 0001: Separate Queue and Workflow Surfaces

## Status

Superseded by `0002-queue-only-v1.md`

## Context

s3q is inspired by both SQS and Temporal.

There is a risk of collapsing the product into a single abstraction where workflows are treated as fancy messages or queues are treated as small workflows. That would weaken both APIs.

## Decision

s3q will expose:

- a queue API shaped around SQS-like lease-and-ack operations
- a workflow API shaped around Temporal-like execution handles

These surfaces may share storage and infrastructure, but they will remain separate in the public API.

## Consequences

- documentation can explain queue and workflow independently
- the Rust core can use different internal models where needed
- Python can stay thin without inventing a merged abstraction
- some shared helpers may exist, but they should not blur the conceptual boundary
