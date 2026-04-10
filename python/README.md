# Python Package

This package is the thin Python SDK, CLI, and future service layer for `s3q`.

The Python SDK is intentionally queue-only and mirrors the Rust concepts:

- `Client`
- `Queue`
- `Producer`
- `Consumer`
- `Inspect`
- `Message`, `ReceiptHandle`, `QueueMetrics`, and related types

The Python methods are implemented as a thin wrapper over a native Rust extension module.

Current status:

- the queue-only Python API is implemented
- the package expects the native extension to be built before use
- the CLI and service layers still follow after the SDK
