# Development Setup

Install the pinned toolchain and run checks:

```bash
mise install
mise exec -- make check
```

Useful tasks:

- `make check`: format, Rust checks, Python compile check, and docs build
- `make test`: Rust tests plus Python compile check
- `make test-rust`: Rust tests only
- `make test-py`: Python compile check only
- `make docs`: local Zensical dev server
- `make docs-build`: static docs build for CI

Docs are served and built with Zensical.
