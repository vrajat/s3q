# Queue Model

The queue side of s3q should be intentionally boring.

## Core Semantics

- Delivery is **at least once**
- `receive_messages` returns a **lease**
- The message remains pending until `delete_message`
- `change_message_visibility` extends or shrinks the retry window
- Duplicate delivery is expected unless a later FIFO or dedup mode is added

## Why This Shape

SQS already gives users a well-understood contract for storage-backed queues. That contract maps cleanly to the `pgqrs` message lifecycle:

- enqueue
- lease
- ack or retry
- dead-letter or archival policy later

## What s3q Should Not Do

- It should not expose a pop-and-remove API as the primary model.
- It should not make exactly-once delivery promises at the transport layer.
- It should not overload queue APIs with orchestration concerns.
