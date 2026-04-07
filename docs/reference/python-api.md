# Python API

The Python SDK mirrors the Rust queue vocabulary:

- `client.queue("emails")`
- `queue.producer("api")`
- `producer.send(...)`
- `queue.consumer("email-worker-1")`
- `consumer.read_batch(...)`
- `consumer.archive_message(...)`
- `consumer.delete_message(...)`
- `consumer.set_vt(...)`

Availability: the Python bindings are not ready for application use yet. Use the Rust API until the Python package is published.
