use std::sync::Arc;
use std::time::{Duration, SystemTime};

use crate::{
    ClientConfig, ConsumerInfo, Error, Message, MessageState, ProducerInfo, QueueInfo, Result,
};
use pgqrs::store::s3::S3Store;

#[derive(Debug)]
pub(crate) struct PgqrsAdapter {
    store: S3Store,
}

impl PgqrsAdapter {
    pub(crate) async fn connect(config: &ClientConfig) -> Result<Arc<Self>> {
        if !config.dsn.starts_with("s3://") {
            return Err(Error::InvalidArgument(
                "s3q v1 only supports s3:// DSNs backed by pgqrs S3Store".to_string(),
            ));
        }

        let pgqrs_config = pgqrs::Config::from_dsn_with_schema(&config.dsn, &config.namespace)?;
        let store = S3Store::new(&pgqrs_config).await?;
        pgqrs::admin(&store).install().await?;

        Ok(Arc::new(Self { store }))
    }

    pub(crate) async fn create_queue(&self, queue_name: &str) -> Result<QueueInfo> {
        let record = pgqrs::admin(&self.store).create_queue(queue_name).await?;
        Ok(QueueInfo {
            name: record.queue_name,
        })
    }

    pub(crate) async fn delete_queue(&self, queue_name: &str) -> Result<()> {
        pgqrs::admin(&self.store)
            .delete_queue_by_name(queue_name)
            .await?;
        Ok(())
    }

    pub(crate) async fn purge_queue(&self, queue_name: &str) -> Result<()> {
        pgqrs::admin(&self.store).purge_queue(queue_name).await?;
        Ok(())
    }

    pub(crate) async fn producer(
        &self,
        queue_name: &str,
        worker_id: &str,
    ) -> Result<BoundProducer> {
        let producer = pgqrs::producer(worker_id, queue_name)
            .create(&self.store)
            .await?;
        let info = ProducerInfo {
            queue_name: queue_name.to_string(),
            worker_id: worker_id.to_string(),
        };

        Ok(BoundProducer { producer, info })
    }

    pub(crate) async fn consumer(
        &self,
        queue_name: &str,
        worker_id: &str,
    ) -> Result<BoundConsumer> {
        let consumer = pgqrs::consumer(worker_id, queue_name)
            .create(&self.store)
            .await?;
        let info = ConsumerInfo {
            queue_name: queue_name.to_string(),
            worker_id: worker_id.to_string(),
        };

        Ok(BoundConsumer { consumer, info })
    }
}

#[derive(Debug, Clone)]
pub(crate) struct BoundProducer {
    producer: pgqrs::Producer,
    info: ProducerInfo,
}

impl BoundProducer {
    pub(crate) fn info(&self) -> &ProducerInfo {
        &self.info
    }

    pub(crate) async fn send(
        &self,
        payload: serde_json::Value,
        delay: Option<Duration>,
    ) -> Result<Message> {
        let delay = duration_secs_u32(delay.unwrap_or_default())?;
        let message = self.producer.enqueue_delayed(&payload, delay).await?;
        Ok(message_from_pgqrs(message, None))
    }

    pub(crate) async fn send_batch(
        &self,
        payloads: &[serde_json::Value],
        delay: Option<Duration>,
    ) -> Result<Vec<Message>> {
        let delay = duration_secs_u32(delay.unwrap_or_default())?;
        let messages = self.producer.batch_enqueue_delayed(payloads, delay).await?;
        Ok(messages
            .into_iter()
            .map(|message| message_from_pgqrs(message, None))
            .collect())
    }
}

#[derive(Debug, Clone)]
pub(crate) struct BoundConsumer {
    consumer: pgqrs::Consumer,
    info: ConsumerInfo,
}

impl BoundConsumer {
    pub(crate) fn info(&self) -> &ConsumerInfo {
        &self.info
    }

    pub(crate) async fn read(&self, vt: Duration) -> Result<Option<Message>> {
        let mut messages = self.read_batch(vt, 1).await?;
        Ok(messages.pop())
    }

    pub(crate) async fn read_batch(&self, vt: Duration, qty: usize) -> Result<Vec<Message>> {
        let vt = duration_secs_u32(vt)?;
        let worker_id = self.consumer.worker_id();
        let messages = self.consumer.dequeue_many_with_delay(qty, vt).await?;

        Ok(messages
            .into_iter()
            .map(|message| {
                let receipt_handle = Some(crate::ReceiptHandle::from_parts(message.id, worker_id));
                message_from_pgqrs(message, receipt_handle)
            })
            .collect())
    }

    pub(crate) async fn delete_message(
        &self,
        receipt_handle: &crate::ReceiptHandle,
    ) -> Result<bool> {
        let message_id = self.message_id_from_receipt(receipt_handle)?;
        Ok(self.consumer.delete(message_id).await?)
    }

    pub(crate) async fn archive_message(
        &self,
        receipt_handle: &crate::ReceiptHandle,
    ) -> Result<Option<Message>> {
        let message_id = self.message_id_from_receipt(receipt_handle)?;
        Ok(self
            .consumer
            .archive(message_id)
            .await?
            .map(|message| message_from_pgqrs(message, None)))
    }

    pub(crate) async fn archive_messages(
        &self,
        receipt_handles: &[crate::ReceiptHandle],
    ) -> Result<Vec<bool>> {
        let message_ids = receipt_handles
            .iter()
            .map(|handle| self.message_id_from_receipt(handle))
            .collect::<Result<Vec<_>>>()?;
        Ok(self.consumer.archive_many(message_ids).await?)
    }

    pub(crate) async fn set_vt(
        &self,
        receipt_handle: &crate::ReceiptHandle,
        vt: Duration,
    ) -> Result<bool> {
        let message_id = self.message_id_from_receipt(receipt_handle)?;
        Ok(self
            .consumer
            .extend_vt(message_id, duration_secs_u32(vt)?)
            .await?)
    }

    fn message_id_from_receipt(&self, receipt_handle: &crate::ReceiptHandle) -> Result<i64> {
        let decoded = receipt_handle.decode()?;
        if decoded.worker_id != self.consumer.worker_id() {
            return Err(Error::OwnershipMismatch);
        }
        Ok(decoded.message_id)
    }
}

fn message_from_pgqrs(
    message: pgqrs::QueueMessage,
    receipt_handle: Option<crate::ReceiptHandle>,
) -> Message {
    let state = message_state(&message);
    Message {
        message_id: message.id,
        read_count: message.read_ct.max(0) as u32,
        enqueued_at: SystemTime::from(message.enqueued_at),
        visible_at: SystemTime::from(message.vt),
        payload: message.payload,
        receipt_handle,
        state,
    }
}

fn message_state(message: &pgqrs::QueueMessage) -> MessageState {
    let now = chrono_now_system_time();
    if message.archived_at.is_some() {
        MessageState::Archived
    } else if message.consumer_worker_id.is_some() {
        MessageState::Leased
    } else if SystemTime::from(message.vt) > now {
        MessageState::Delayed
    } else {
        MessageState::Visible
    }
}

fn duration_secs_u32(duration: Duration) -> Result<u32> {
    u32::try_from(duration.as_secs()).map_err(|_| {
        Error::InvalidArgument("duration is too large for pgqrs seconds field".to_string())
    })
}

fn chrono_now_system_time() -> SystemTime {
    SystemTime::now()
}
