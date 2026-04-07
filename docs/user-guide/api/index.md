# API Reference

The s3q API is organized around four handles:

- `Client`: connection and top-level entrypoint
- `QueueHandle`: queue-scoped setup and handle creation
- `Producer`: sends messages
- `Consumer`: reads and completes messages
- `Inspect`: read-only metrics and message inspection

Start with the [Rust API](rust.md). Python and CLI surfaces mirror the same queue terms as they are implemented.
