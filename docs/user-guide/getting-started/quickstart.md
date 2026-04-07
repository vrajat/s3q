# Quickstart

This guide creates a queue, sends a message, leases it with a consumer, and archives it after successful processing.

## Install

Add `s3q` and `serde_json` to your Rust application:

```toml
[dependencies]
s3q = "0.1"
serde_json = "1"
```

## Connect

Use an S3 DSN that points to the queue database object:

```rust
async fn connect() -> s3q::Result<s3q::Client> {
    s3q::connect("s3://my-bucket/queues/app.db").await
}
```

## Create a Queue

```rust
async fn setup(client: &s3q::Client) -> s3q::Result<()> {
    let queue = client.queue("emails");
    queue.create_queue().await?;
    Ok(())
}
```

`create_queue` is safe to keep in setup code. If the queue already exists, handle that error the same way you handle any duplicate provisioning operation in your app.

## Send Messages

Create a producer with a stable worker id. Use a value that helps you identify the sender later, such as a service name, host name, or deployment unit.

```rust
use serde_json::json;

async fn enqueue_welcome_email(client: &s3q::Client) -> s3q::Result<()> {
    let queue = client.queue("emails");
    let producer = queue.producer("api").await?;

    producer
        .send(json!({
            "kind": "welcome_email",
            "to": "user@example.com"
        }))
        .await?;

    Ok(())
}
```

Use `send_batch` when you already have multiple payloads:

```rust
use serde_json::json;

async fn enqueue_batch(client: &s3q::Client) -> s3q::Result<()> {
    let queue = client.queue("emails");
    let producer = queue.producer("bulk-import").await?;

    producer
        .send_batch(vec![
            json!({"kind": "welcome_email", "to": "a@example.com"}),
            json!({"kind": "welcome_email", "to": "b@example.com"}),
        ])
        .await?;

    Ok(())
}
```

## Read and Complete Messages

Create a consumer with a stable worker id. The visibility timeout controls how long this consumer owns the message before it can be retried.

```rust
use std::time::Duration;

async fn drain_once(client: &s3q::Client) -> s3q::Result<()> {
    let queue = client.queue("emails");
    let consumer = queue.consumer("email-worker-1").await?;

    let messages = consumer.read_batch(Duration::from_secs(30), 10).await?;

    for message in messages {
        process_email(&message.payload).await?;

        if let Some(receipt) = message.receipt_handle {
            consumer.archive_message(receipt).await?;
        }
    }

    Ok(())
}

async fn process_email(_payload: &serde_json::Value) -> s3q::Result<()> {
    Ok(())
}
```

If processing takes longer than expected, extend the lease before it expires:

```rust
use std::time::Duration;

async fn extend_processing(
    consumer: &s3q::Consumer<'_>,
    receipt: s3q::ReceiptHandle,
) -> s3q::Result<()> {
    consumer.set_vt(receipt, Duration::from_secs(120)).await?;
    Ok(())
}
```

## Check Metrics

Inspection APIs are read-only. They never lease, archive, or delete messages.

```rust
async fn show_metrics(client: &s3q::Client) -> s3q::Result<()> {
    let metrics = client.inspect().metrics("emails");
    println!("{metrics:?}");
    Ok(())
}
```

## Next Steps

- [Learn the queue model](../concepts/queue-model.md)
- [Read the Rust API reference](../api/rust.md)
- [Build a worker loop](../guides/basic-queue.md)
- [Understand archive vs delete](../guides/message-lifecycle.md)
