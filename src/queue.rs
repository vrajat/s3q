use std::time::Duration;

use crate::{types::ReceiptHandle, Client};

#[derive(Debug, Clone)]
pub struct QueueHandle<'a> {
    client: &'a Client,
    name: String,
}

impl<'a> QueueHandle<'a> {
    pub(crate) fn new(client: &'a Client, name: impl Into<String>) -> Self {
        Self {
            client,
            name: name.into(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn namespace(&self) -> &str {
        &self.client.config().namespace
    }

    pub fn create_queue(&self) -> CreateQueueRequest {
        CreateQueueRequest {
            queue_name: self.name.clone(),
        }
    }

    pub fn delete_queue(&self) -> DeleteQueueRequest {
        DeleteQueueRequest {
            queue_name: self.name.clone(),
        }
    }

    pub fn purge_queue(&self) -> PurgeQueueRequest {
        PurgeQueueRequest {
            queue_name: self.name.clone(),
        }
    }

    pub fn producer(&self, worker_id: impl Into<String>) -> Producer<'a> {
        Producer {
            client: self.client,
            queue_name: self.name.clone(),
            worker_id: worker_id.into(),
        }
    }

    pub fn consumer(&self, worker_id: impl Into<String>) -> Consumer<'a> {
        Consumer {
            client: self.client,
            queue_name: self.name.clone(),
            worker_id: worker_id.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Producer<'a> {
    client: &'a Client,
    queue_name: String,
    worker_id: String,
}

impl<'a> Producer<'a> {
    pub fn queue_name(&self) -> &str {
        &self.queue_name
    }

    pub fn worker_id(&self) -> &str {
        &self.worker_id
    }

    pub fn namespace(&self) -> &str {
        &self.client.config().namespace
    }

    pub fn send(&self, payload: impl Into<Vec<u8>>) -> SendRequest {
        SendRequest {
            queue_name: self.queue_name.clone(),
            producer_worker_id: self.worker_id.clone(),
            payload: payload.into(),
            delay: None,
        }
    }

    pub fn send_batch(&self, payloads: impl Into<Vec<Vec<u8>>>) -> SendBatchRequest {
        SendBatchRequest {
            queue_name: self.queue_name.clone(),
            producer_worker_id: self.worker_id.clone(),
            payloads: payloads.into(),
            delay: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Consumer<'a> {
    client: &'a Client,
    queue_name: String,
    worker_id: String,
}

impl<'a> Consumer<'a> {
    pub fn queue_name(&self) -> &str {
        &self.queue_name
    }

    pub fn worker_id(&self) -> &str {
        &self.worker_id
    }

    pub fn namespace(&self) -> &str {
        &self.client.config().namespace
    }

    pub fn read(&self, vt: Duration) -> ReadRequest {
        ReadRequest {
            queue_name: self.queue_name.clone(),
            consumer_worker_id: self.worker_id.clone(),
            vt,
        }
    }

    pub fn read_batch(&self, vt: Duration, qty: usize) -> ReadBatchRequest {
        ReadBatchRequest {
            queue_name: self.queue_name.clone(),
            consumer_worker_id: self.worker_id.clone(),
            vt,
            qty,
        }
    }

    pub fn read_with_poll(
        &self,
        vt: Duration,
        qty: usize,
        poll_timeout: Duration,
        poll_interval: Duration,
    ) -> ReadWithPollRequest {
        ReadWithPollRequest {
            queue_name: self.queue_name.clone(),
            consumer_worker_id: self.worker_id.clone(),
            vt,
            qty,
            poll_timeout,
            poll_interval,
        }
    }

    pub fn delete_message(&self, receipt_handle: impl Into<ReceiptHandle>) -> DeleteMessageRequest {
        DeleteMessageRequest {
            queue_name: self.queue_name.clone(),
            consumer_worker_id: self.worker_id.clone(),
            receipt_handle: receipt_handle.into(),
        }
    }

    pub fn archive_message(
        &self,
        receipt_handle: impl Into<ReceiptHandle>,
    ) -> ArchiveMessageRequest {
        ArchiveMessageRequest {
            queue_name: self.queue_name.clone(),
            consumer_worker_id: self.worker_id.clone(),
            receipt_handle: receipt_handle.into(),
        }
    }

    pub fn archive_messages(
        &self,
        receipt_handles: impl Into<Vec<ReceiptHandle>>,
    ) -> ArchiveMessagesRequest {
        ArchiveMessagesRequest {
            queue_name: self.queue_name.clone(),
            consumer_worker_id: self.worker_id.clone(),
            receipt_handles: receipt_handles.into(),
        }
    }

    pub fn set_vt(
        &self,
        receipt_handle: impl Into<ReceiptHandle>,
        vt: Duration,
    ) -> SetVisibilityTimeoutRequest {
        SetVisibilityTimeoutRequest {
            queue_name: self.queue_name.clone(),
            consumer_worker_id: self.worker_id.clone(),
            receipt_handle: receipt_handle.into(),
            vt,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateQueueRequest {
    pub queue_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeleteQueueRequest {
    pub queue_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PurgeQueueRequest {
    pub queue_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SendRequest {
    pub queue_name: String,
    pub producer_worker_id: String,
    pub payload: Vec<u8>,
    pub delay: Option<Duration>,
}

impl SendRequest {
    pub fn with_delay(mut self, delay: Duration) -> Self {
        self.delay = Some(delay);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SendBatchRequest {
    pub queue_name: String,
    pub producer_worker_id: String,
    pub payloads: Vec<Vec<u8>>,
    pub delay: Option<Duration>,
}

impl SendBatchRequest {
    pub fn with_delay(mut self, delay: Duration) -> Self {
        self.delay = Some(delay);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReadRequest {
    pub queue_name: String,
    pub consumer_worker_id: String,
    pub vt: Duration,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReadBatchRequest {
    pub queue_name: String,
    pub consumer_worker_id: String,
    pub vt: Duration,
    pub qty: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReadWithPollRequest {
    pub queue_name: String,
    pub consumer_worker_id: String,
    pub vt: Duration,
    pub qty: usize,
    pub poll_timeout: Duration,
    pub poll_interval: Duration,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeleteMessageRequest {
    pub queue_name: String,
    pub consumer_worker_id: String,
    pub receipt_handle: ReceiptHandle,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchiveMessageRequest {
    pub queue_name: String,
    pub consumer_worker_id: String,
    pub receipt_handle: ReceiptHandle,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchiveMessagesRequest {
    pub queue_name: String,
    pub consumer_worker_id: String,
    pub receipt_handles: Vec<ReceiptHandle>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetVisibilityTimeoutRequest {
    pub queue_name: String,
    pub consumer_worker_id: String,
    pub receipt_handle: ReceiptHandle,
    pub vt: Duration,
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::{connect, ReceiptHandle};

    #[test]
    fn producer_requests_capture_queue_and_worker() {
        let client = connect("s3://bucket/queue.db");
        let producer = client.queue("emails").producer("api-worker");

        let request = producer.send(b"hello".to_vec());

        assert_eq!(request.queue_name, "emails");
        assert_eq!(request.producer_worker_id, "api-worker");
        assert_eq!(request.payload, b"hello".to_vec());
    }

    #[test]
    fn consumer_requests_capture_queue_worker_and_receipt() {
        let client = connect("s3://bucket/queue.db");
        let consumer = client.queue("emails").consumer("worker-a");

        let receipt = ReceiptHandle::new("opaque-lease-token");
        let request = consumer.set_vt(receipt.clone(), Duration::from_secs(60));

        assert_eq!(request.queue_name, "emails");
        assert_eq!(request.consumer_worker_id, "worker-a");
        assert_eq!(request.receipt_handle, receipt);
        assert_eq!(request.vt, Duration::from_secs(60));
    }

    #[test]
    fn read_batch_uses_pgmq_vocabulary() {
        let client = connect("s3://bucket/queue.db");
        let consumer = client.queue("emails").consumer("worker-a");

        let request = consumer.read_batch(Duration::from_secs(30), 8);

        assert_eq!(request.queue_name, "emails");
        assert_eq!(request.qty, 8);
        assert_eq!(request.vt, Duration::from_secs(30));
    }
}
