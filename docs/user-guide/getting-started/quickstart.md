# Quickstart

This is the target shape for the public API. The Rust queue mutation surface is wired to `pgqrs::store::s3::S3Store`; inspection, Python, and CLI implementation are staged separately.

## Rust

```rust
use serde_json::json;
use std::time::Duration;

let client = s3q::connect("s3://my-bucket/s3q/prod.db").await?;
let queue = client.queue("emails");

queue.create_queue().await?;

let producer = queue.producer("api-worker").await?;
let sent = producer.send(json!({"to": "user@example.com"})).await?;

let consumer = queue.consumer("email-worker").await?;
let messages = consumer.read_batch(Duration::from_secs(30), 10).await?;

for message in messages {
    if let Some(receipt_handle) = message.receipt_handle {
        consumer.archive_message(receipt_handle).await?;
    }
}
```

## Python

```python
from s3q import Client

client = Client.connect("s3://my-bucket/s3q/prod.db")

queue = client.queue("emails")
producer = queue.producer("api-worker")
producer.send(b'{"to":"user@example.com"}')

consumer = queue.consumer("email-worker")
for message in consumer.read_batch(vt=30, qty=10):
    consumer.archive_message(message.receipt_handle)
```

## What Happens Next

The next implementation phases will add:

- inspection and exact metrics
- `read_with_poll`
- thin Python bindings into the Rust core
