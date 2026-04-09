# Rust API Reference

This page is a compact reference for the Rust queue API. For guided examples, start with the [quickstart](../user-guide/getting-started/quickstart.md).

## Connection

```rust
let client = s3q::connect("s3://my-bucket/queues/app.db").await?;
```

```rust
let config = s3q::ClientConfig::new("s3://my-bucket/queues/app.db")
    .with_namespace("prod")
    .with_service_name("email-service");

let client = s3q::Client::connect_with_config(config).await?;
```

## Queue

| Method | Description |
| --- | --- |
| `client.create_queue(name).await` | Create the queue and return `Queue` |
| `client.purge_queue(name).await` | Remove active queue contents |
| `client.delete_queue(name).await` | Delete the queue |
| `client.queue(name)` | Create a queue-scoped handle |
| `queue.producer(worker_id).await` | Create a producer handle |
| `queue.consumer(worker_id).await` | Create a consumer handle |

## Producer

| Method | Description |
| --- | --- |
| `producer.send(payload).await` | Send one JSON payload |
| `producer.send_batch(payloads).await` | Send multiple JSON payloads |
| `producer.send_delayed(payload, delay).await` | Send one delayed payload |
| `producer.send_batch_delayed(payloads, delay).await` | Send multiple delayed payloads |

## Consumer

| Method | Description |
| --- | --- |
| `consumer.read(vt).await` | Lease one visible message |
| `consumer.read_batch(vt, qty).await` | Lease up to `qty` visible messages |
| `consumer.read_with_poll(vt, qty, timeout, interval).await` | Long-poll for visible messages and return an empty batch on timeout |
| `consumer.archive_message(receipt).await` | Complete and retain a message |
| `consumer.archive_messages(receipts).await` | Complete and retain multiple messages |
| `consumer.delete_message(receipt).await` | Permanently remove a message |
| `consumer.set_vt(receipt, vt).await` | Change the lease visibility timeout |

## Message

| Field | Description |
| --- | --- |
| `message_id` | Stable message id for logs and debugging |
| `read_count` | Number of reads recorded for the message |
| `payload` | JSON payload |
| `receipt_handle` | Lease token used for completion |
| `state` | `Visible`, `Leased`, `Delayed`, or `Archived` |

Treat `receipt_handle` as opaque.

## Inspection

| Method | Description |
| --- | --- |
| `client.inspect().list_queues()` | List queues |
| `client.inspect().metrics(queue)` | Metrics for one queue |
| `client.inspect().metrics_all()` | Metrics for all queues |
| `client.inspect().list_messages(queue)` | List active messages |
| `client.inspect().get_message(queue, message_id)` | Inspect one message |
| `client.inspect().list_archived_messages(queue)` | List archived messages |
