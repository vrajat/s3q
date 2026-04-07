# Rust API

The Rust API is the source of truth for queue behavior. The current phase wires the queue mutation surface to `pgqrs::store::s3::S3Store`.

V1 handles:

- `Client`
- `QueueHandle`
- `Producer`
- `Consumer`
- `Inspect`

V1 queue methods follow `pgmq` naming where possible:

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

Queue ownership shape:

```rust
use serde_json::json;
use std::time::Duration;

let client = s3q::connect("s3://bucket/queues.db").await?;
let queue = client.queue("emails");

queue.create_queue().await?;

let producer = queue.producer("api-worker").await?;
let sent = producer.send(json!({"to": "user@example.com"})).await?;

let consumer = queue.consumer("email-worker").await?;
let messages = consumer.read_batch(Duration::from_secs(30), 10).await?;
```

`read_with_poll` remains planned for the dedicated polling phase. The implementation must use `pgqrs` polling support rather than a separate polling engine in `s3q`.

Inspection is separate:

```rust
let metrics = client.inspect().metrics("emails");
let messages = client.inspect().list_messages("emails");
```
