# CLI API

This page shows the queue terms the CLI uses.

The CLI is intended to mirror the same queue model as the Rust and Python SDKs:

- `queue create`
- `queue delete`
- `queue purge`
- `queue send`
- `queue read`
- `queue archive`
- `queue delete-message`
- `queue set-vt`
- `inspect queues`
- `inspect metrics`
- `inspect messages`
- `inspect archived`

## Current Status

The CLI implementation is still behind the SDKs. Today the implemented command surface is:

```bash
s3q queue create --dsn s3://my-bucket/queues/app.db --name emails
```

and:

```bash
s3q service run --config path/to/config.toml
```

## Queue Workflow

The intended queue flow looks like this:

```bash
s3q queue create --dsn s3://my-bucket/queues/app.db --name emails
s3q queue send --dsn s3://my-bucket/queues/app.db --name emails --json '{"to":"user@example.com"}'
s3q queue read --dsn s3://my-bucket/queues/app.db --name emails --worker email-worker-1 --vt 30 --qty 10
s3q queue archive --dsn s3://my-bucket/queues/app.db --name emails --worker email-worker-1 --receipt <receipt-handle>
```

The final command set should use the same nouns as the SDKs:

- `Client` maps to `--dsn` and shared connection options
- `Queue` maps to `--name`
- `Producer` maps to `queue send`
- `Consumer` maps to `queue read`, `queue archive`, `queue delete-message`, and `queue set-vt`
- `Inspect` maps to `inspect ...`

For the current exact command list, use the CLI reference.
