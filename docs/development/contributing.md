# Contributing

## Current Priorities

- Wire the Rust crate to `pgqrs::store::s3::S3Store`
- Keep `s3q` as a thin queue product layer
- Preserve `pgqrs` producer and consumer worker identity
- Keep the Python package thin
- Bias toward operationally simple features first

## Repository Rules

- Put user-facing documentation in `docs/`
- Put engineering decisions in `engg/`
- Keep Rust core logic in `src/`
- Keep Python CLI and service concerns in `python/`

## First Good Contributions

- implement queue create/send/read/delete/archive
- implement inspection metrics
- add queue-only golden examples for Rust and Python
- add end-to-end tests against local object storage
