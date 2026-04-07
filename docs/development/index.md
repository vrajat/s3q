# Development

This page is for contributors working on the s3q repository. Application developers should start with the [quickstart](../user-guide/getting-started/quickstart.md).

## Top-Level Layout

- `src/`: Rust library crate
- `python/`: Python SDK and CLI package
- `docs/`: public documentation
- `engg/`: internal product and engineering documents
- `.buildkite/`: CI pipeline

## Local Tasks

Use `mise` for tool versions and `make` for tasks:

```bash
mise install
mise exec -- make check
mise exec -- make test
mise exec -- make docs-build
```

Use `make docs` when you want the local Zensical dev server:

```bash
mise exec -- make docs
```

## Documentation Rule

Public docs should help a developer use s3q in an application. Keep internal design rationale in `engg/`, not in the user guide.

When a public API changes, update the matching docs page in the same phase.
