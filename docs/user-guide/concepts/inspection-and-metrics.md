# Inspection and Metrics

s3q exposes read-only inspection APIs for operational visibility.

## V1 Surface

- `list_queues`
- `metrics`
- `metrics_all`
- `list_messages`
- `get_message`
- `list_archived_messages`

Inspection should never lease, acknowledge, archive, or delete a message.

## Metrics Semantics

Metrics follow `pgqrs` semantics and should be treated as exact snapshots at query time. A concurrent producer or consumer can change the queue immediately after the snapshot is returned.

## Archived Messages

Archived messages are retained for historical stats and debugging. V1 exposes archived-message inspection but does not expose replay or DLQ-specific APIs.
