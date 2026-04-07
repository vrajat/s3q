# Rust API

The Rust API is the source of truth for queue behavior. The current phase exposes the v1 handle and request vocabulary; the next phase wires those requests to `pgqrs::store::s3::S3Store`.

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
let client = s3q::connect("s3://bucket/queues.db");
let queue = client.queue("emails");

let producer = queue.producer("api-worker");
let send = producer.send(br#"{"to":"user@example.com"}"#.to_vec());

let consumer = queue.consumer("email-worker");
let read = consumer.read_batch(std::time::Duration::from_secs(30), 10);
```

Inspection is separate:

```rust
let metrics = client.inspect().metrics("emails");
let messages = client.inspect().list_messages("emails");
```
