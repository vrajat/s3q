use std::time::SystemTime;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ReceiptHandle(String);

impl ReceiptHandle {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
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
    pub payload: Vec<u8>,
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
