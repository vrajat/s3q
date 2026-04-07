use std::time::SystemTime;

use crate::{Error, Result};

const RECEIPT_PREFIX: &str = "s3q1";

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Opaque handle for a leased message.
///
/// Receipt handles are returned by `read` and `read_batch` and are required to
/// archive, delete, or update visibility for a leased message.
pub struct ReceiptHandle {
    message_id: i64,
    worker_id: i64,
}

impl ReceiptHandle {
    /// Parse a receipt handle from its encoded string representation.
    pub fn parse(value: impl AsRef<str>) -> Result<Self> {
        let value = value.as_ref();
        let mut parts = value.split(':');
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

        Ok(Self::from_parts(
            message_id.parse().map_err(|_| {
                Error::InvalidArgument("invalid message id in receipt handle".to_string())
            })?,
            worker_id.parse().map_err(|_| {
                Error::InvalidArgument("invalid worker id in receipt handle".to_string())
            })?,
        ))
    }

    pub(crate) fn from_parts(message_id: i64, worker_id: i64) -> Self {
        Self {
            message_id,
            worker_id,
        }
    }

    pub(crate) fn message_id(&self) -> i64 {
        self.message_id
    }

    pub(crate) fn worker_id(&self) -> i64 {
        self.worker_id
    }

    /// Encode the receipt handle for storage or transport.
    pub fn encode(&self) -> String {
        format!("{RECEIPT_PREFIX}:{}:{}", self.message_id, self.worker_id)
    }
}

impl std::fmt::Display for ReceiptHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.encode())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Public message state.
pub enum MessageState {
    /// Message is ready to be leased by a consumer.
    Visible,
    /// Message is currently leased by a consumer.
    Leased,
    /// Message is scheduled for later delivery.
    Delayed,
    /// Message has been archived and retained for history.
    Archived,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Queue message returned by s3q.
pub struct Message {
    /// Stable message id.
    pub message_id: i64,
    /// Number of times the message has been read.
    pub read_count: u32,
    /// Time when the message was enqueued.
    pub enqueued_at: SystemTime,
    /// Time when the message becomes visible.
    pub visible_at: SystemTime,
    /// JSON message payload.
    pub payload: serde_json::Value,
    /// Receipt handle for leased messages.
    pub receipt_handle: Option<ReceiptHandle>,
    /// Projected message state.
    pub state: MessageState,
}

/// Archived message record.
pub type ArchivedMessage = Message;

#[derive(Debug, Clone, PartialEq, Eq)]
/// Queue metadata.
pub struct QueueInfo {
    /// Queue name.
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Metrics snapshot for a queue.
pub struct QueueMetrics {
    /// Queue name.
    pub queue_name: String,
    /// Number of visible messages.
    pub visible_messages: u64,
    /// Number of leased messages.
    pub leased_messages: u64,
    /// Number of delayed messages.
    pub delayed_messages: u64,
    /// Number of archived messages.
    pub archived_messages: u64,
    /// Total number of messages included in the snapshot.
    pub total_messages: u64,
}
