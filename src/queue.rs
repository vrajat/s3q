use std::time::Duration;

use serde_json::Value;

use crate::{types::ReceiptHandle, Client, ConsumerInfo, Message, ProducerInfo, QueueInfo, Result};

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

    pub async fn create_queue(&self) -> Result<QueueInfo> {
        self.client.adapter().create_queue(&self.name).await
    }

    pub async fn delete_queue(&self) -> Result<()> {
        self.client.adapter().delete_queue(&self.name).await
    }

    pub async fn purge_queue(&self) -> Result<()> {
        self.client.adapter().purge_queue(&self.name).await
    }

    pub async fn producer(&self, worker_id: impl Into<String>) -> Result<Producer<'a>> {
        let worker_id = worker_id.into();
        let bound = self
            .client
            .adapter()
            .producer(&self.name, &worker_id)
            .await?;

        Ok(Producer {
            client: self.client,
            queue_name: self.name.clone(),
            worker_id,
            bound,
        })
    }

    pub async fn consumer(&self, worker_id: impl Into<String>) -> Result<Consumer<'a>> {
        let worker_id = worker_id.into();
        let bound = self
            .client
            .adapter()
            .consumer(&self.name, &worker_id)
            .await?;

        Ok(Consumer {
            client: self.client,
            queue_name: self.name.clone(),
            worker_id,
            bound,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Producer<'a> {
    client: &'a Client,
    queue_name: String,
    worker_id: String,
    bound: crate::pgqrs::BoundProducer,
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

    pub fn info(&self) -> &ProducerInfo {
        self.bound.info()
    }

    pub async fn send(&self, payload: impl Into<Value>) -> Result<Message> {
        self.bound.send(payload.into(), None).await
    }

    pub async fn send_delayed(
        &self,
        payload: impl Into<Value>,
        delay: Duration,
    ) -> Result<Message> {
        self.bound.send(payload.into(), Some(delay)).await
    }

    pub async fn send_batch(&self, payloads: impl Into<Vec<Value>>) -> Result<Vec<Message>> {
        let payloads = payloads.into();
        self.bound.send_batch(&payloads, None).await
    }

    pub async fn send_batch_delayed(
        &self,
        payloads: impl Into<Vec<Value>>,
        delay: Duration,
    ) -> Result<Vec<Message>> {
        let payloads = payloads.into();
        self.bound.send_batch(&payloads, Some(delay)).await
    }
}

#[derive(Debug, Clone)]
pub struct Consumer<'a> {
    client: &'a Client,
    queue_name: String,
    worker_id: String,
    bound: crate::pgqrs::BoundConsumer,
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

    pub fn info(&self) -> &ConsumerInfo {
        self.bound.info()
    }

    pub async fn read(&self, vt: Duration) -> Result<Option<Message>> {
        self.bound.read(vt).await
    }

    pub async fn read_batch(&self, vt: Duration, qty: usize) -> Result<Vec<Message>> {
        self.bound.read_batch(vt, qty).await
    }

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

    pub async fn delete_message(&self, receipt_handle: impl Into<ReceiptHandle>) -> Result<bool> {
        self.bound.delete_message(&receipt_handle.into()).await
    }

    pub async fn archive_message(
        &self,
        receipt_handle: impl Into<ReceiptHandle>,
    ) -> Result<Option<Message>> {
        self.bound.archive_message(&receipt_handle.into()).await
    }

    pub async fn archive_messages(
        &self,
        receipt_handles: impl Into<Vec<ReceiptHandle>>,
    ) -> Result<Vec<bool>> {
        self.bound.archive_messages(&receipt_handles.into()).await
    }

    pub async fn set_vt(
        &self,
        receipt_handle: impl Into<ReceiptHandle>,
        vt: Duration,
    ) -> Result<bool> {
        self.bound.set_vt(&receipt_handle.into(), vt).await
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
