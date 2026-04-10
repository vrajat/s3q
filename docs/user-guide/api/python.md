# Python API

This page shows the basic Python queue workflow.

## Connect

```python
import s3q

client = s3q.Client.connect("s3://my-bucket/queues/app.db")
```

Use `ClientConfig` when you need a non-default namespace or service name:

```python
config = (
    s3q.ClientConfig("s3://my-bucket/queues/app.db")
    .with_namespace("prod")
    .with_service_name("email-service")
)

client = s3q.Client.connect_with_config(config)
```

## Queue

```python
queue = client.create_queue("emails")
```

Create producers and consumers from the queue:

```python
producer = queue.producer("api")
consumer = queue.consumer("email-worker-1")
```

## Producer

```python
message = producer.send({"to": "user@example.com"})
```

Batch send:

```python
messages = producer.send_batch(
    [
        {"to": "a@example.com"},
        {"to": "b@example.com"},
    ]
)
```

Delayed send:

```python
from datetime import timedelta

producer.send(
    {"to": "user@example.com"},
    delay=timedelta(seconds=60),
)
```

## Consumer

Read one message:

```python
message = consumer.read(vt=timedelta(seconds=30))
```

Read a batch:

```python
messages = consumer.read_batch(vt=timedelta(seconds=30), qty=10)
```

Archive after successful processing:

```python
for message in messages:
    process(message.payload)
    consumer.archive_message(message.receipt_handle)
```

Delete when you do not want retained history:

```python
consumer.delete_message(message.receipt_handle)
```

Extend visibility for long-running work:

```python
consumer.set_vt(message.receipt_handle, timedelta(seconds=120))
```

Wait briefly for visible work:

```python
messages = consumer.read_with_poll(
    vt=timedelta(seconds=30),
    qty=10,
    poll_timeout=timedelta(seconds=20),
    poll_interval=timedelta(milliseconds=500),
)
```

## Inspection

List queues and metrics:

```python
queues = client.inspect().list_queues()
metrics = client.inspect().metrics("emails")
all_metrics = client.inspect().metrics_all()
```

Inspect current leased messages:

```python
page = (
    client.inspect()
    .list_messages("emails")
    .with_state(s3q.MessageState.LEASED)
    .with_limit(100)
    .execute()
)
```

Inspect retained archived messages:

```python
archived = (
    client.inspect()
    .list_archived_messages("emails")
    .with_limit(100)
    .execute()
)
```

## Build

The Python package uses a native Rust extension. Build it before importing the SDK into an application:

```bash
make build-py
```
