# Rust API

This page documents the Rust queue surface.

## Client

Connect with an S3 DSN:

```rust
let client = s3q::connect("s3://my-bucket/queues/app.db").await?;
```

Use `ClientConfig` when you need a non-default namespace or service name:

```rust
let config = s3q::ClientConfig::new("s3://my-bucket/queues/app.db")
    .with_namespace("prod")
    .with_service_name("email-service");

let client = s3q::Client::connect_with_config(config).await?;
```

## Queue

```rust
let queue = client.create_queue("emails").await?;
```

Create producers and consumers from the queue:

```rust
let producer = queue.producer("api").await?;
let consumer = queue.consumer("email-worker-1").await?;
```

## Producer

```rust
use serde_json::json;

producer.send(json!({"to": "user@example.com"})).await?;
```

Batch send:

```rust
producer
    .send_batch(vec![
        json!({"to": "a@example.com"}),
        json!({"to": "b@example.com"}),
    ])
    .await?;
```

Delayed send:

```rust
producer
    .send_delayed(
        json!({"to": "user@example.com"}),
        std::time::Duration::from_secs(60),
    )
    .await?;
```

## Consumer

Read one message:

```rust
let maybe_message = consumer.read(std::time::Duration::from_secs(30)).await?;
```

Read a batch:

```rust
let messages = consumer
    .read_batch(std::time::Duration::from_secs(30), 10)
    .await?;
```

Archive on successful processing:

```rust
for message in messages {
    process(&message.payload).await?;

    if let Some(receipt) = message.receipt_handle {
        consumer.archive_message(receipt).await?;
    }
}
```

Delete when you do not want retained history:

```rust
consumer.delete_message(receipt).await?;
```

Extend visibility for long-running work:

```rust
consumer
    .set_vt(receipt, std::time::Duration::from_secs(120))
    .await?;
```

Use `read_with_poll` when you want the consumer to wait for visible work before returning:

```rust
let messages = consumer
    .read_with_poll(
        std::time::Duration::from_secs(30),
        10,
        std::time::Duration::from_secs(20),
        std::time::Duration::from_millis(500),
    )
    .await?;
```

If no messages become visible before `poll_timeout`, `read_with_poll` returns an empty vector.

## Message

```rust
pub struct Message {
    pub message_id: i64,
    pub read_count: u32,
    pub payload: serde_json::Value,
    pub receipt_handle: Option<s3q::ReceiptHandle>,
    pub state: s3q::MessageState,
}
```

Use `message_id` for logs and debugging. Use `receipt_handle` for completion and visibility changes.

## Inspection

Use inspection APIs for read-only views:

```rust
let page = client
    .inspect()
    .list_messages("emails")
    .with_state(s3q::MessageState::Leased)
    .with_limit(100)
    .execute()
    .await?;
```

List queues and metrics:

```rust
let queues = client.inspect().list_queues().await?;
let metrics = client.inspect().metrics("emails").await?;
let all_metrics = client.inspect().metrics_all().await?;
```

Inspect retained archived messages:

```rust
let archived = client
    .inspect()
    .list_archived_messages("emails")
    .with_limit(100)
    .execute()
    .await?;
```
