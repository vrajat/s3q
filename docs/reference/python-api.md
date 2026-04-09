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

- the queue-only Python SDK surface is defined
- Python types mirror the Rust concepts
- the methods are not wired to the Rust core yet

Use the Rust API for real queue operations until the Python package is backed by the Rust core.
