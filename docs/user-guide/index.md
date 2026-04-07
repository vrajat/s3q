# User Guide

s3q has two v1 product surfaces:

1. **Queue APIs** for sending, reading, leasing, deleting, and archiving messages.
2. **Inspection APIs** for read-only metrics and message inspection.

Both surfaces are backed by `pgqrs::store::s3::S3Store`.

## Queue Surface

Queues use `pgmq`-style names:

- `send`
- `send_batch`
- `read`
- `read_batch`
- `read_with_poll`
- `delete_message`
- `archive_message`
- `archive_messages`
- `set_vt`

The core semantic is **lease then ack**, not destructive pop.

## Inspection Surface

Inspection is read-only and operational:

- `list_queues`
- `metrics`
- `metrics_all`
- `list_messages`
- `get_message`
- `list_archived_messages`

Metrics should follow `pgqrs` semantics and are exact snapshots of durable queue state.
