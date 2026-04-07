# Development

This repository is organized to keep product docs, engineering docs, Rust code, and Python code separate.

## Top-Level Layout

- `src/`: Rust library crate
- `python/`: Python SDK, CLI, and service layer
- `docs/`: public docs
- `engg/`: internal docs

## Local Tasks

Use `mise` for tool versions and `make` for tasks:

```bash
mise install
mise exec -- make check
mise exec -- make test
mise exec -- make docs
```

## Expected Evolution

1. Keep the repository baseline stable.
2. Refactor the Rust scaffold to the approved queue-only API.
3. Add required capabilities to `pgqrs`.
4. Wire `s3q` to `pgqrs::store::s3::S3Store`.
5. Mirror the Rust surface in Python and CLI.
