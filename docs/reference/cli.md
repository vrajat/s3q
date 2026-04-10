# CLI Reference

The CLI is an operational wrapper around the Python SDK.

## Implemented Commands

These commands are currently implemented:

```bash
s3q queue create --dsn <dsn> --name <queue>
s3q service run --config <config-path>
```

## Target Queue Surface

As the CLI grows, it should mirror the same queue vocabulary as the SDKs.

Command groups:

- `s3q queue`
- `s3q inspect`

Queue commands mutate queue state:

```bash
s3q queue create emails
s3q queue send emails --json '{"to":"user@example.com"}'
s3q queue read emails --vt 30 --limit 10
s3q queue archive emails <receipt-handle>
s3q queue delete emails <receipt-handle>
```

Inspect commands are read-only:

```bash
s3q inspect queues
s3q inspect metrics emails
s3q inspect messages emails --state leased --limit 100
s3q inspect archived emails --limit 100
```

Availability: the CLI surface is still incomplete and is not ready for general application use yet.
