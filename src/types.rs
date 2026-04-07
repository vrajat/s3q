use std::time::SystemTime;

use crate::{Error, Result};

const RECEIPT_PREFIX: &str = "s3q1";

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ReceiptHandle(String);

impl ReceiptHandle {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub(crate) fn from_parts(message_id: i64, worker_id: i64) -> Self {
        Self::new(format!("{RECEIPT_PREFIX}:{message_id}:{worker_id}"))
    }

    pub(crate) fn decode(&self) -> Result<DecodedReceiptHandle> {
        let mut parts = self.0.split(':');
        let Some(prefix) = parts.next() else {
            return Err(Error::InvalidArgument("empty receipt handle".to_string()));
        };
        let Some(message_id) = parts.next() else {
            return Err(Error::InvalidArgument(
                "missing message id in receipt handle".to_string(),
            ));
        };
        let Some(worker_id) = parts.next() else {
            return Err(Error::InvalidArgument(
                "missing worker id in receipt handle".to_string(),
            ));
        };

        if prefix != RECEIPT_PREFIX || parts.next().is_some() {
            return Err(Error::InvalidArgument("invalid receipt handle".to_string()));
        }

        Ok(DecodedReceiptHandle {
            message_id: message_id.parse().map_err(|_| {
                Error::InvalidArgument("invalid message id in receipt handle".to_string())
            })?,
            worker_id: worker_id.parse().map_err(|_| {
                Error::InvalidArgument("invalid worker id in receipt handle".to_string())
            })?,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct DecodedReceiptHandle {
    pub message_id: i64,
    pub worker_id: i64,
}

impl AsRef<str> for ReceiptHandle {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl From<String> for ReceiptHandle {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<&str> for ReceiptHandle {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageState {
    Visible,
    Leased,
    Delayed,
    Archived,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Message {
    pub message_id: i64,
    pub read_count: u32,
    pub enqueued_at: SystemTime,
    pub visible_at: SystemTime,
    pub payload: serde_json::Value,
    pub receipt_handle: Option<ReceiptHandle>,
    pub state: MessageState,
}

pub type ArchivedMessage = Message;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueueInfo {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueueMetrics {
    pub queue_name: String,
    pub visible_messages: u64,
    pub leased_messages: u64,
    pub delayed_messages: u64,
    pub archived_messages: u64,
    pub total_messages: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProducerInfo {
    pub queue_name: String,
    pub worker_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConsumerInfo {
    pub queue_name: String,
    pub worker_id: String,
}
