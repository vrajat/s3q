# Queue Model

s3q is a lease-and-ack queue, not a pop-and-remove queue.

When a consumer reads a message, the message is leased to that consumer for a visibility timeout. The message is not complete until the consumer archives or deletes it. If the worker crashes or the lease expires before completion, the message can be delivered again.

## Message States

- **Visible**: the message is ready to be leased.
- **Leased**: a consumer owns the message until the visibility timeout expires.
- **Delayed**: the message was scheduled for later and is not visible yet.
- **Archived**: the message was completed and retained for stats or debugging.

## Producers and Consumers

Producers and consumers have stable worker ids.

```rust
let producer = client.queue("emails").producer("api").await?;
let consumer = client.queue("emails").consumer("email-worker-1").await?;
```

Use names that help incident review. For example, `api`, `billing-worker-a`, or `email-worker-us-east-1a` are better than generated random ids when you want to understand who sent or owned a message.

## Receipt Handles

`read` and `read_batch` return a receipt handle for each leased message. Use that handle to complete or extend the lease.

```rust
let messages = consumer.read_batch(std::time::Duration::from_secs(30), 10).await?;

for message in messages {
    if let Some(receipt) = message.receipt_handle {
        consumer.archive_message(receipt).await?;
    }
}
```

Treat receipt handles as opaque strings. Do not parse them or store them as durable business identifiers. Use `message_id` when you need a stable message id for logs.

## Archive vs Delete

Use `archive_message` when a message completed successfully and you want to retain it for historical stats or debugging.

Use `delete_message` when you intentionally want to remove the message and do not need retained history.

```rust
consumer.archive_message(receipt).await?;
```

```rust
consumer.delete_message(receipt).await?;
```

## Delivery Guarantees

s3q provides at-least-once delivery. A message can be delivered more than once, so handlers should be idempotent.

Common patterns:

- Include an idempotency key in the payload.
- Make downstream writes safe to retry.
- Archive only after the side effect succeeds.
- Extend visibility for long-running work.
