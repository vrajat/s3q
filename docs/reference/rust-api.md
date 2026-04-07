# Rust API

The Rust API is the source of truth for queue behavior.

Planned v1 handles:

- `Client`
- `QueueHandle`
- `Producer`
- `Consumer`
- `Inspect`

Planned v1 queue methods follow `pgmq` naming where possible:

- `send`
- `send_batch`
- `read`
- `read_batch`
- `read_with_poll`
- `delete_message`
- `archive_message`
- `archive_messages`
- `set_vt`

The implementation must preserve `pgqrs` producer and consumer worker identity.
