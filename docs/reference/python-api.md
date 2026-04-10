# Python API

The Python SDK mirrors the Rust queue vocabulary:

- `client.queue("emails")`
- `queue.producer("api")`
- `producer.send(...)`
- `queue.consumer("email-worker-1")`
- `consumer.read_batch(...)`
- `consumer.read_with_poll(...)`
- `consumer.archive_message(...)`
- `consumer.delete_message(...)`
- `consumer.set_vt(...)`
- `client.inspect().metrics(...)`
- `client.inspect().list_messages(...)`

Current status:

- the queue-only Python SDK wraps the Rust core through a native extension
- Python types mirror the Rust concepts
- the package must be built with its native extension before use

The API shape is intended to match Rust closely while keeping Python-friendly arguments and exceptions.
