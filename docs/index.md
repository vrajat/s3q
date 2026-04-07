# s3q

**s3q is a small queue library for applications that want durable queue state in S3.**

Use it when you want a simple job queue without running a queue service or a database server. Messages are stored in an S3-backed queue, consumers lease work for a visibility timeout, and completed work can either be deleted or archived for later debugging and stats.

## What You Can Build

- Background job workers
- Durable task queues for batch jobs
- Small control-plane queues for services and CLIs
- Operational queues where archived messages help explain incidents

## Queue Semantics

- `send` writes a JSON message to a queue.
- `read` and `read_batch` lease messages for a visibility timeout.
- `archive_message` marks a message as completed and keeps it for history.
- `delete_message` permanently removes a completed message.
- `set_vt` changes the visibility timeout for a leased message.
- `metrics` and `metrics_all` return exact queue snapshots at query time.

Messages are delivered at least once. A consumer should make its handler idempotent and finish each message by archiving or deleting it.

## Quick Example

```rust
use serde_json::json;
use std::time::Duration;

async fn run() -> s3q::Result<()> {
    let client = s3q::connect("s3://my-bucket/queues/app.db").await?;
    client.create_queue("emails").await?;

    let queue = client.queue("emails");
    let producer = queue.producer("api").await?;
    producer
        .send(json!({
            "to": "user@example.com",
            "template": "welcome"
        }))
        .await?;

    let consumer = queue.consumer("email-worker-1").await?;
    let messages = consumer.read_batch(Duration::from_secs(30), 10).await?;

    for message in messages {
        send_email(&message.payload).await?;

        if let Some(receipt) = message.receipt_handle {
            consumer.archive_message(receipt).await?;
        }
    }

    Ok(())
}

async fn send_email(_payload: &serde_json::Value) -> s3q::Result<()> {
    Ok(())
}
```

## Read Next

- [Quickstart](user-guide/getting-started/quickstart.md)
- [Queue model](user-guide/concepts/queue-model.md)
- [Rust API](user-guide/api/rust.md)
- [Basic queue guide](user-guide/guides/basic-queue.md)
- [Inspection and metrics](user-guide/concepts/inspection-and-metrics.md)
