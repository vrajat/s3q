# Queue Model

s3q exposes a queue-only product layer over `pgqrs::store::s3::S3Store`.

## Core Semantics

- Delivery is **at least once**
- `read` and `read_batch` return leased messages
- The message remains active until `delete_message` or `archive_message`
- `set_vt` extends or shrinks the visibility timeout for the owned lease
- Duplicate delivery is expected unless a later FIFO or dedup mode is added
- Producers and consumers have stable worker identities

## Why This Shape

`pgmq` provides a compact and proven queue API vocabulary. SQS provides the familiar lease-and-ack contract for storage-backed queues. Both map cleanly to the `pgqrs` message lifecycle:

- enqueue
- lease
- delete, archive, or retry
- inspect retained history

## What s3q Should Not Do

- It should not expose a pop-and-remove API as the primary model.
- It should not make exactly-once delivery promises at the transport layer.
- It should not overload queue APIs with orchestration concerns.
- It should not implement queue capabilities outside `pgqrs`.
