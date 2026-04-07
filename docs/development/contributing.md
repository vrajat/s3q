# Contributing

## Current Priorities

- Keep the public queue API small and application-oriented.
- Preserve producer and consumer worker identity.
- Keep inspection APIs read-only.
- Keep Python and CLI layers thin over the core queue API.
- Keep public docs useful for application developers, not just implementers.

## Repository Rules

- Put user-facing documentation in `docs/`.
- Put engineering decisions in `engg/`.
- Keep Rust queue logic in `src/`.
- Keep Python CLI and service concerns in `python/`.
- Do not add workflow APIs to v1.

## First Good Contributions

- Implement inspection metrics.
- Wire `read_with_poll`.
- Add queue-only examples for Rust and Python.
- Add end-to-end tests against local object storage.
- Improve docs with runnable application examples.
