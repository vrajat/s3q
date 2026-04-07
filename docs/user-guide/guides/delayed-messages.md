# Delayed Messages

Use delayed messages when work should not become visible immediately.

```rust
use serde_json::json;
use std::time::Duration;

async fn enqueue_reminder(producer: &s3q::Producer<'_>) -> s3q::Result<()> {
    producer
        .send_delayed(
            json!({
                "kind": "reminder_email",
                "to": "user@example.com"
            }),
            Duration::from_secs(3600),
        )
        .await?;

    Ok(())
}
```

Delayed messages are not returned by `read` until their delay expires.

Batch delayed send is also available:

```rust
use serde_json::json;
use std::time::Duration;

async fn enqueue_many(producer: &s3q::Producer<'_>) -> s3q::Result<()> {
    producer
        .send_batch_delayed(
            vec![
                json!({"kind": "reminder_email", "to": "a@example.com"}),
                json!({"kind": "reminder_email", "to": "b@example.com"}),
            ],
            Duration::from_secs(3600),
        )
        .await?;

    Ok(())
}
```
