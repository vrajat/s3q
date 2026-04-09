# Python Package

This package is the thin Python SDK, CLI, and future service layer for `s3q`.

The Python SDK is intentionally queue-only and mirrors the Rust concepts:

- `Client`
- `Queue`
- `Producer`
- `Consumer`
- `Inspect`
- `Message`, `ReceiptHandle`, `QueueMetrics`, and related types

Today the surface exists primarily to lock the vocabulary and public shapes. The queue methods are not wired to the Rust core yet, and real queue operations should still use the Rust API.
