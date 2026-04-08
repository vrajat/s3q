# Inspection and Metrics

Inspection APIs are for operational visibility. They do not mutate queue state.

Use inspection when you need to answer questions such as:

- Which queues exist?
- How many messages are visible, leased, delayed, or archived?
- Which messages are currently leased?
- Which archived messages are available for debugging?

## Metrics

`metrics(queue)` returns a snapshot for one queue. `metrics_all()` returns snapshots for all queues.

The counts are exact at the time the snapshot is produced. A producer or consumer can change the queue immediately after the snapshot returns, so do not treat metrics as a synchronization primitive.

Expected fields:

- `visible_messages`
- `leased_messages`
- `delayed_messages`
- `archived_messages`
- `total_messages`

## Message Inspection

`list_messages` and `get_message` are read-only views of message state. They are useful for debugging stuck workers, checking retries, and understanding queue backlog.

```rust
let leased = client
    .inspect()
    .list_messages("emails")
    .with_state(s3q::MessageState::Leased)
    .with_limit(100)
    .execute()
    .await?;
```

Archived messages are retained separately from active queue state:

```rust
let archived = client
    .inspect()
    .list_archived_messages("emails")
    .with_limit(100)
    .execute()
    .await?;
```

List queues and metrics directly:

```rust
let queues = client.inspect().list_queues().await?;
let metrics = client.inspect().metrics("emails").await?;
let all_metrics = client.inspect().metrics_all().await?;
```

Use the returned `next_cursor` for the next page:

```rust
if let Some(cursor) = archived.next_cursor {
    let next = client
        .inspect()
        .list_archived_messages("emails")
        .with_cursor(cursor)
        .execute()
        .await?;
}
```
