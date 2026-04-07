# User Guide

s3q has two surfaces:

- **Queue APIs** for creating queues, sending messages, leasing work, and completing messages.
- **Inspection APIs** for checking queue health, message state, and archived history.

Most applications only need a client, a queue, one or more producers, and one or more consumers.

## The Basic Flow

1. Connect with an S3 DSN such as `s3://my-bucket/queues/app.db`.
2. Create a queue with `client.create_queue("emails")`.
3. Send messages through a named producer.
4. Read messages through a named consumer.
5. Archive or delete each leased message by using its receipt handle.
6. Inspect metrics when you need operational visibility.

## Important Concepts

- A **producer** has a stable worker id so sent messages can be traced back to the source.
- A **consumer** has a stable worker id so leased messages can be tracked to the worker that owns them.
- A **receipt handle** represents the lease returned by `read` or `read_batch`.
- `archive_message` is the normal completion path when you want historical stats and debugging data.
- `delete_message` is destructive and should be reserved for data you do not need to retain.

## Recommended Reading Order

- [Quickstart](getting-started/quickstart.md)
- [Queue model](concepts/queue-model.md)
- [Message lifecycle](guides/message-lifecycle.md)
- [Rust API](api/rust.md)
- [Inspection and metrics](concepts/inspection-and-metrics.md)
