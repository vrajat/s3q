# Worker Identity

s3q asks producers and consumers to use stable worker ids.

```rust
let producer = queue.producer("api").await?;
let consumer = queue.consumer("email-worker-1").await?;
```

Stable ids make queue activity easier to inspect later. They answer operational questions such as:

- Which service sent this message?
- Which worker leased it?
- Which worker completed it?
- Which worker was active during an incident?

## Naming Guidance

Use ids that are stable and meaningful:

- `api`
- `billing-worker-1`
- `email-worker-us-east-1a`
- `backfill-2026-04-07`

Avoid ids that make incident review harder:

- random UUIDs without a service prefix
- process ids alone
- host names alone when multiple services share a host

## Completion Ownership

Only the consumer that owns a lease should complete that message. The public API reflects this by putting completion methods on `Consumer`:

```rust
consumer.archive_message(receipt).await?;
consumer.delete_message(receipt).await?;
consumer.set_vt(receipt, std::time::Duration::from_secs(120)).await?;
```

Do not pass receipt handles to unrelated consumers. If your app hands work across internal tasks, keep completion routed through the consumer that leased the message.
