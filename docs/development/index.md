# Development

This repository is organized to keep product docs, engineering docs, Rust code, and Python code separate from the start.

## Top-Level Layout

- `src/`: Rust library crate
- `python/`: Python SDK, CLI, and service layer
- `docs/`: public docs
- `engg/`: internal docs

## Expected Evolution

1. Stabilize API vocabulary.
2. Add `pgqrs` integration.
3. Replace stubs with working queue and workflow operations.
4. Add a service API and deployment model.
