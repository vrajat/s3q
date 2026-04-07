# Basic Queue

This guide shows a minimal producer and consumer for a background email queue.

## Producer

```rust
use serde_json::json;

async fn enqueue_email(client: &s3q::Client, to: &str) -> s3q::Result<()> {
    let queue = client.queue("emails");
    let producer = queue.producer("api").await?;

    producer
        .send(json!({
            "kind": "welcome_email",
            "to": to
        }))
        .await?;

    Ok(())
}
```

## Consumer

```rust
use std::time::Duration;

async fn drain_emails(client: &s3q::Client) -> s3q::Result<()> {
    let queue = client.queue("emails");
    let consumer = queue.consumer("email-worker-1").await?;

    let messages = consumer.read_batch(Duration::from_secs(30), 25).await?;

    for message in messages {
        match send_email(&message.payload).await {
            Ok(()) => {
                if let Some(receipt) = message.receipt_handle {
                    consumer.archive_message(receipt).await?;
                }
            }
            Err(error) => {
                eprintln!("email message {} failed: {error}", message.message_id);
            }
        }
    }

    Ok(())
}

async fn send_email(_payload: &serde_json::Value) -> s3q::Result<()> {
    Ok(())
}
```

If the handler returns an error and the message is not archived or deleted, the message becomes available again after its visibility timeout expires.

## Worker Loop

Run the consumer repeatedly in your worker process:

```rust
use std::time::Duration;

async fn worker_loop(client: s3q::Client) -> s3q::Result<()> {
    loop {
        drain_emails(&client).await?;
        wait_for_more_work(Duration::from_secs(2)).await;
    }
}

async fn wait_for_more_work(_duration: Duration) {}
```

Use `read_with_poll` when you want the worker to wait for messages instead of sleeping between empty reads.
