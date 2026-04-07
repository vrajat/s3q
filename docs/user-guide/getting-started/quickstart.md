# Quickstart

This is the target shape for the public API. The repository scaffold is not fully wired to `pgqrs::store::s3::S3Store` yet.

## Rust

```rust
use s3q::{Client, ClientConfig};

let client = Client::connect(ClientConfig::new("s3://my-bucket/s3q/prod.db")).await?;
let queue = client.queue("emails");

let producer = queue.producer("api-worker").await?;
producer.send(br#"{"to":"user@example.com"}"#.to_vec()).await?;

let consumer = queue.consumer("email-worker").await?;
let messages = consumer.read_batch(30, 10).await?;

for message in messages {
    consumer.archive_message(&message.receipt_handle).await?;
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

The next implementation phase will replace request-building scaffolds with:

- real `pgqrs` S3Store wiring
- queue producer and consumer handles
- inspection and exact metrics
- thin Python bindings into the Rust core
