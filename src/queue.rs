use std::sync::Arc;
use std::time::Duration;

use serde_json::Value;

use crate::{
    store::StoreState, types::message_from_pgqrs, types::ReceiptHandle, Error, Message, Result,
};

#[derive(Debug, Clone)]
/// Queue-scoped handle.
///
/// A queue creates producer and consumer handles for a specific queue name.
pub struct Queue {
    store: Arc<StoreState>,
    name: String,
    namespace: String,
}

impl Queue {
    pub(crate) fn new(
        store: Arc<StoreState>,
        name: impl Into<String>,
        namespace: impl Into<String>,
    ) -> Self {
        Self {
            store,
            name: name.into(),
            namespace: namespace.into(),
        }
    }

    /// Return the queue name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return the namespace for this queue.
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Create a managed producer for this queue.
    ///
    /// Use a stable worker id so sent messages can be traced back to the
    /// producer during incident review.
    pub async fn producer(&self, worker_id: impl Into<String>) -> Result<Producer> {
        let worker_id = worker_id.into();
        let producer = pgqrs::producer(&worker_id, &self.name)
            .create(&self.store.s3)
            .await?;

        Ok(Producer {
            queue_name: self.name.clone(),
            namespace: self.namespace.clone(),
            worker_id,
            producer,
        })
    }

    /// Create a managed consumer for this queue.
    ///
    /// Use a stable worker id so leased messages can be traced back to the
    /// consumer that owned them.
    pub async fn consumer(&self, worker_id: impl Into<String>) -> Result<Consumer> {
        let worker_id = worker_id.into();
        let consumer = pgqrs::consumer(&worker_id, &self.name)
            .create(&self.store.s3)
            .await?;

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
    producer: pgqrs::Producer,
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
        self.send_with_delay(payload.into(), Duration::ZERO).await
    }

    /// Send one JSON payload after a delay.
    pub async fn send_delayed(
        &self,
        payload: impl Into<Value>,
        delay: Duration,
    ) -> Result<Message> {
        self.send_with_delay(payload.into(), delay).await
    }

    async fn send_with_delay(&self, payload: Value, delay: Duration) -> Result<Message> {
        let message = self
            .producer
            .enqueue_delayed(&payload, duration_secs_u32(delay)?)
            .await?;
        Ok(message_from_pgqrs(message, None))
    }

    /// Send multiple JSON payloads immediately.
    pub async fn send_batch(&self, payloads: impl Into<Vec<Value>>) -> Result<Vec<Message>> {
        self.send_batch_with_delay(payloads.into(), Duration::ZERO)
            .await
    }

    /// Send multiple JSON payloads after a delay.
    pub async fn send_batch_delayed(
        &self,
        payloads: impl Into<Vec<Value>>,
        delay: Duration,
    ) -> Result<Vec<Message>> {
        self.send_batch_with_delay(payloads.into(), delay).await
    }

    async fn send_batch_with_delay(
        &self,
        payloads: Vec<Value>,
        delay: Duration,
    ) -> Result<Vec<Message>> {
        let messages = self
            .producer
            .batch_enqueue_delayed(&payloads, duration_secs_u32(delay)?)
            .await?;
        Ok(messages
            .into_iter()
            .map(|message| message_from_pgqrs(message, None))
            .collect())
    }
}

#[derive(Debug, Clone)]
/// Consumer handle for leasing and completing messages from one queue.
pub struct Consumer {
    queue_name: String,
    namespace: String,
    worker_id: String,
    consumer: pgqrs::Consumer,
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
        let mut messages = self.read_batch(vt, 1).await?;
        Ok(messages.pop())
    }

    /// Lease up to `qty` visible messages for the supplied visibility timeout.
    pub async fn read_batch(&self, vt: Duration, qty: usize) -> Result<Vec<Message>> {
        let vt = duration_secs_u32(vt)?;
        let worker_id = self.consumer.worker_id();
        let messages = self.consumer.dequeue_many_with_delay(qty, vt).await?;

        Ok(messages
            .into_iter()
            .map(|message| {
                let receipt_handle = Some(ReceiptHandle::from_parts(message.id, worker_id));
                message_from_pgqrs(message, receipt_handle)
            })
            .collect())
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
    pub async fn delete_message(&self, receipt_handle: ReceiptHandle) -> Result<bool> {
        let message_id = self.message_id_from_receipt(&receipt_handle)?;
        Ok(self.consumer.delete(message_id).await?)
    }

    /// Archive a leased message and retain it for history.
    ///
    /// The receipt handle must come from a message leased by this consumer.
    pub async fn archive_message(&self, receipt_handle: ReceiptHandle) -> Result<Option<Message>> {
        let message_id = self.message_id_from_receipt(&receipt_handle)?;
        Ok(self
            .consumer
            .archive(message_id)
            .await?
            .map(|message| message_from_pgqrs(message, None)))
    }

    /// Archive multiple leased messages.
    ///
    /// Each receipt handle must come from a message leased by this consumer.
    pub async fn archive_messages(&self, receipt_handles: Vec<ReceiptHandle>) -> Result<Vec<bool>> {
        let message_ids = receipt_handles
            .iter()
            .map(|handle| self.message_id_from_receipt(handle))
            .collect::<Result<Vec<_>>>()?;
        Ok(self.consumer.archive_many(message_ids).await?)
    }

    /// Set the visibility timeout for a leased message.
    ///
    /// The receipt handle must come from a message leased by this consumer.
    pub async fn set_vt(&self, receipt_handle: ReceiptHandle, vt: Duration) -> Result<bool> {
        let message_id = self.message_id_from_receipt(&receipt_handle)?;
        Ok(self
            .consumer
            .extend_vt(message_id, duration_secs_u32(vt)?)
            .await?)
    }

    fn message_id_from_receipt(&self, receipt_handle: &ReceiptHandle) -> Result<i64> {
        if receipt_handle.worker_id() != self.consumer.worker_id() {
            return Err(Error::OwnershipMismatch);
        }
        Ok(receipt_handle.message_id())
    }
}

fn duration_secs_u32(duration: Duration) -> Result<u32> {
    u32::try_from(duration.as_secs()).map_err(|_| {
        Error::InvalidArgument("duration is too large for pgqrs seconds field".to_string())
    })
}

#[cfg(test)]
mod tests {
    use crate::ReceiptHandle;

    #[test]
    fn receipt_handles_round_trip_message_and_worker_ids() {
        let receipt = ReceiptHandle::from_parts(42, 7);
        let encoded = receipt.encode();
        let parsed = ReceiptHandle::parse(&encoded).expect("receipt should parse");

        assert_eq!(parsed.message_id(), 42);
        assert_eq!(parsed.worker_id(), 7);
    }
}
