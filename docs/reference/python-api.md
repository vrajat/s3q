# Python API

The Python SDK mirrors the Rust queue vocabulary and uses the same queue terms:

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

Common entrypoints:

```python
client = s3q.Client.connect("s3://my-bucket/queues/app.db")
queue = client.queue("emails")
producer = queue.producer("api")
consumer = queue.consumer("email-worker-1")
inspect = client.inspect()
```

Key methods:

- `Client.connect(...)`
- `Client.connect_with_config(...)`
- `Client.create_queue(...)`
- `Client.delete_queue(...)`
- `Client.purge_queue(...)`
- `Queue.producer(...)`
- `Queue.consumer(...)`
- `Producer.send(...)`
- `Producer.send_batch(...)`
- `Consumer.read(...)`
- `Consumer.read_batch(...)`
- `Consumer.read_with_poll(...)`
- `Consumer.archive_message(...)`
- `Consumer.archive_messages(...)`
- `Consumer.delete_message(...)`
- `Consumer.set_vt(...)`
- `Inspect.list_queues(...)`
- `Inspect.metrics(...)`
- `Inspect.metrics_all(...)`
- `Inspect.get_message(...)`
- `ListMessagesRequest.execute(...)`
- `ListArchivedMessagesRequest.execute(...)`

Build requirement:

- the queue-only Python SDK wraps the Rust core through a native extension
- build it with `make build-py` before importing it into an application

The API shape is intended to match Rust closely while keeping Python-friendly arguments and exceptions.
