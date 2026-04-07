# Development Setup

Install the pinned toolchain and run checks:

```bash
mise install
mise exec -- make check
```

Useful tasks:

- `make check`: format, Rust checks, and Python compile check
- `make test`: Rust tests plus Python compile check
- `make test-rust`: Rust tests only
- `make test-localstack`: S3 integration tests against LocalStack
- `make test-s3`: alias for `make test-localstack`
- `make test-py`: Python compile check only
- `make docs`: local Zensical dev server
- `make docs-build`: static docs build for CI

Docs are served and built with Zensical.

The LocalStack target follows the same convention as `pgqrs`: it starts a
local S3-only LocalStack container when `CI_LOCALSTACK_RUNNING` is not set,
creates the test bucket, runs the ignored Rust integration tests, and stops the
container afterward.
