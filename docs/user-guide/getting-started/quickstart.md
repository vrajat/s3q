# Quickstart

This is the target shape for the public API. The repository scaffold is not fully wired to `pgqrs` yet.

## Rust

```rust
use s3q::{connect, ClientConfig};

let client = s3q::Client::new(
    ClientConfig::new("s3://my-bucket/s3q/prod.db").with_namespace("prod"),
);

let send = client
    .queues()
    .queue("emails")
    .send_message(br#"{"to":"user@example.com"}"#.to_vec());

let start = client
    .workflows()
    .workflow("image_pipeline")
    .start("image-123", br#"{"asset":"a.png"}"#.to_vec());
```

## Python

```python
from s3q import Client

client = Client.connect("s3://my-bucket/s3q/prod.db", namespace="prod")

queue = client.queue("emails")
workflow = client.workflow("image_pipeline")
```

## What Happens Next

The next implementation phase will replace request-building scaffolds with:

- real `pgqrs` S3Store wiring
- queue operations
- workflow execution records
- Python bindings into the Rust core
