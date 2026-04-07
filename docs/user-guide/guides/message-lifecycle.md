# Message Lifecycle

A message moves through a small set of states:

1. `send` creates a visible message.
2. `read` leases the message to one consumer.
3. The consumer processes the payload.
4. The consumer completes the message with `archive_message` or `delete_message`.

## Visibility Timeout

The visibility timeout is the retry window. While the lease is active, other consumers should not receive the message. If the consumer does not complete the message before the timeout expires, the message can be read again.

```rust
let message = consumer
    .read(std::time::Duration::from_secs(30))
    .await?;
```

For long-running work, call `set_vt` before the lease expires:

```rust
consumer
    .set_vt(receipt, std::time::Duration::from_secs(120))
    .await?;
```

## Archive

Archive is the recommended success path when you want retained history.

```rust
consumer.archive_message(receipt).await?;
```

Archived messages are useful for:

- historical stats
- debugging a bad deployment
- verifying which worker completed a message
- inspecting old payloads when a customer reports an issue

## Delete

Delete permanently removes the message:

```rust
consumer.delete_message(receipt).await?;
```

Use delete when the payload should not be retained or when historical stats do not matter for that queue.

## Failed Processing

If processing fails, do not archive the message. Let the visibility timeout expire so the message can be retried.

```rust
match process(&message.payload).await {
    Ok(()) => {
        if let Some(receipt) = message.receipt_handle {
            consumer.archive_message(receipt).await?;
        }
    }
    Err(error) => {
        eprintln!("message {} failed: {error}", message.message_id);
    }
}
```
