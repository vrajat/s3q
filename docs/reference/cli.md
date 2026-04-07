# CLI Reference

The CLI is an operational wrapper around the Python SDK.

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

Availability: the CLI is not ready for application use yet.
