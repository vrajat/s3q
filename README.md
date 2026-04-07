# s3q

**s3q is a small S3-backed queue library for Rust and Python applications.**

It is designed for teams that want durable queue state in S3 without operating a queue service or database server. The queue model is intentionally small: producers send JSON messages, consumers lease messages with a visibility timeout, and completed messages can be archived for history or deleted permanently.

## Rust Example

```rust
use serde_json::json;
use std::time::Duration;

async fn run() -> s3q::Result<()> {
    let client = s3q::connect("s3://my-bucket/queues/app.db").await?;
    let queue = client.create_queue("emails").await?;
    let producer = queue.producer("api").await?;
    producer
        .send(json!({
            "kind": "welcome_email",
            "to": "user@example.com"
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

## Queue Semantics

- Delivery is at least once.
- `read` and `read_batch` lease messages for a visibility timeout.
- A leased message is completed only after `archive_message` or `delete_message`.
- `archive_message` retains completed messages for stats and debugging.
- `delete_message` permanently removes messages.
- Producers and consumers use stable worker ids for operational traceability.

## Current Status

The Rust queue mutation surface is available on the active development branch. Python bindings, CLI commands, and inspection execution are next.

## Documentation

The public docs live in `docs/` and are built with Zensical.

```bash
mise install
mise exec -- make docs
```

`make docs` starts the local docs server. Use `make docs-build` for static-site validation.

## Repository Layout

- `src/`: Rust library crate
- `python/`: Python SDK and CLI package
- `docs/`: public user documentation
- `engg/`: internal product and engineering documents
- `.buildkite/`: Buildkite CI pipeline

## Local Development

```bash
mise install
mise exec -- make check
mise exec -- make test
mise exec -- make docs-build
```
