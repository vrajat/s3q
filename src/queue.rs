use std::sync::Arc;
use std::time::Duration;

use serde_json::Value;

use crate::{pgqrs_adapter::PgqrsAdapter, types::ReceiptHandle, Message, QueueInfo, Result};

#[derive(Debug, Clone)]
/// Queue-scoped handle.
///
/// A queue handle creates queues and returns producer and consumer handles for
/// a specific queue name.
pub struct QueueHandle {
    adapter: Arc<PgqrsAdapter>,
    name: String,
    namespace: String,
}

impl QueueHandle {
    pub(crate) fn new(
        adapter: Arc<PgqrsAdapter>,
        name: impl Into<String>,
        namespace: impl Into<String>,
    ) -> Self {
        Self {
            adapter,
            name: name.into(),
            namespace: namespace.into(),
        }
    }

    /// Return the queue name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return the namespace for this queue handle.
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Create the queue.
    pub async fn create_queue(&self) -> Result<QueueInfo> {
        self.adapter.create_queue(&self.name).await
    }

    /// Delete the queue.
    ///
    /// Deletion fails if the backing queue still has messages or associated
    /// workers that prevent safe deletion.
    pub async fn delete_queue(&self) -> Result<()> {
        self.adapter.delete_queue(&self.name).await
    }

    /// Purge messages from the queue while keeping the queue itself.
    pub async fn purge_queue(&self) -> Result<()> {
        self.adapter.purge_queue(&self.name).await
    }

    /// Create a managed producer handle for this queue.
    ///
    /// Use a stable worker id so sent messages can be traced back to the
    /// producer during incident review.
    pub async fn producer(&self, worker_id: impl Into<String>) -> Result<Producer> {
        let worker_id = worker_id.into();
        let producer = self.adapter.producer(&self.name, &worker_id).await?;

        Ok(Producer {
            queue_name: self.name.clone(),
            namespace: self.namespace.clone(),
            worker_id,
            producer,
        })
    }

    /// Create a managed consumer handle for this queue.
    ///
    /// Use a stable worker id so leased messages can be traced back to the
    /// consumer that owned them.
    pub async fn consumer(&self, worker_id: impl Into<String>) -> Result<Consumer> {
        let worker_id = worker_id.into();
        let consumer = self.adapter.consumer(&self.name, &worker_id).await?;

        Ok(Consumer {
            queue_name: self.name.clone(),
            namespace: self.namespace.clone(),
            worker_id,
            consumer,
        })
    }
}

#[derive(Debug, Clone)]
/// Producer handle for sending messages to one queue.
pub struct Producer {
    queue_name: String,
    namespace: String,
    worker_id: String,
    producer: crate::pgqrs_adapter::Producer,
}

impl Producer {
    /// Return the queue name this producer sends to.
    pub fn queue_name(&self) -> &str {
        &self.queue_name
    }

    /// Return the stable producer worker id.
    pub fn worker_id(&self) -> &str {
        &self.worker_id
    }

    /// Return the namespace used by this producer.
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Send one JSON payload immediately.
    pub async fn send(&self, payload: impl Into<Value>) -> Result<Message> {
        self.producer.send(payload.into(), None).await
    }

    /// Send one JSON payload after a delay.
    pub async fn send_delayed(
        &self,
        payload: impl Into<Value>,
        delay: Duration,
    ) -> Result<Message> {
        self.producer.send(payload.into(), Some(delay)).await
    }

    /// Send multiple JSON payloads immediately.
    pub async fn send_batch(&self, payloads: impl Into<Vec<Value>>) -> Result<Vec<Message>> {
        let payloads = payloads.into();
        self.producer.send_batch(&payloads, None).await
    }

    /// Send multiple JSON payloads after a delay.
    pub async fn send_batch_delayed(
        &self,
        payloads: impl Into<Vec<Value>>,
        delay: Duration,
    ) -> Result<Vec<Message>> {
        let payloads = payloads.into();
        self.producer.send_batch(&payloads, Some(delay)).await
    }
}

#[derive(Debug, Clone)]
/// Consumer handle for leasing and completing messages from one queue.
pub struct Consumer {
    queue_name: String,
    namespace: String,
    worker_id: String,
    consumer: crate::pgqrs_adapter::Consumer,
}

impl Consumer {
    /// Return the queue name this consumer reads from.
    pub fn queue_name(&self) -> &str {
        &self.queue_name
    }

    /// Return the stable consumer worker id.
    pub fn worker_id(&self) -> &str {
        &self.worker_id
    }

    /// Return the namespace used by this consumer.
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Lease at most one visible message for the supplied visibility timeout.
    pub async fn read(&self, vt: Duration) -> Result<Option<Message>> {
        self.consumer.read(vt).await
    }

    /// Lease up to `qty` visible messages for the supplied visibility timeout.
    pub async fn read_batch(&self, vt: Duration, qty: usize) -> Result<Vec<Message>> {
        self.consumer.read_batch(vt, qty).await
    }

    /// Long-poll for visible messages.
    ///
    /// This API is reserved for the polling phase and currently returns
    /// [`Error::NotImplemented`](crate::Error::NotImplemented).
    pub async fn read_with_poll(
        &self,
        vt: Duration,
        qty: usize,
        poll_timeout: Duration,
        poll_interval: Duration,
    ) -> Result<Vec<Message>> {
        let _ = (vt, qty, poll_timeout, poll_interval);
        Err(crate::Error::NotImplemented(
            "read_with_poll is wired in Phase 4 using pgqrs polling support",
        ))
    }

    /// Permanently delete a leased message.
    ///
    /// The receipt handle must come from a message leased by this consumer.
    pub async fn delete_message(&self, receipt_handle: impl Into<ReceiptHandle>) -> Result<bool> {
        self.consumer.delete_message(&receipt_handle.into()).await
    }

    /// Archive a leased message and retain it for history.
    ///
    /// The receipt handle must come from a message leased by this consumer.
    pub async fn archive_message(
        &self,
        receipt_handle: impl Into<ReceiptHandle>,
    ) -> Result<Option<Message>> {
        self.consumer.archive_message(&receipt_handle.into()).await
    }

    /// Archive multiple leased messages.
    ///
    /// Each receipt handle must come from a message leased by this consumer.
    pub async fn archive_messages(
        &self,
        receipt_handles: impl Into<Vec<ReceiptHandle>>,
    ) -> Result<Vec<bool>> {
        self.consumer
            .archive_messages(&receipt_handles.into())
            .await
    }

    /// Set the visibility timeout for a leased message.
    ///
    /// The receipt handle must come from a message leased by this consumer.
    pub async fn set_vt(
        &self,
        receipt_handle: impl Into<ReceiptHandle>,
        vt: Duration,
    ) -> Result<bool> {
        self.consumer.set_vt(&receipt_handle.into(), vt).await
    }
}

#[cfg(test)]
mod tests {
    use crate::ReceiptHandle;

    #[test]
    fn receipt_handles_round_trip_message_and_worker_ids() {
        let receipt = ReceiptHandle::from_parts(42, 7);
        let decoded = receipt.decode().expect("receipt should decode");

        assert_eq!(decoded.message_id, 42);
        assert_eq!(decoded.worker_id, 7);
    }
}
