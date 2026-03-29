use std::time::{Duration, SystemTime};

use crate::Client;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueueAttributes {
    pub visibility_timeout: Duration,
    pub retention_period: Duration,
    pub max_receive_count: u32,
    pub fifo: bool,
    pub content_based_deduplication: bool,
}

impl Default for QueueAttributes {
    fn default() -> Self {
        Self {
            visibility_timeout: Duration::from_secs(30),
            retention_period: Duration::from_secs(60 * 60 * 24 * 4),
            max_receive_count: 16,
            fifo: false,
            content_based_deduplication: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReceivedMessage {
    pub message_id: String,
    pub receipt_handle: String,
    pub body: Vec<u8>,
    pub receive_count: u32,
    pub leased_until: Option<SystemTime>,
}

#[derive(Debug, Clone, Copy)]
pub struct QueueApi<'a> {
    client: &'a Client,
}

impl<'a> QueueApi<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub fn namespace(&self) -> &str {
        &self.client.config().namespace
    }

    pub fn queue(&self, name: impl Into<String>) -> QueueHandle<'a> {
        QueueHandle {
            client: self.client,
            name: name.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct QueueHandle<'a> {
    client: &'a Client,
    pub name: String,
}

impl<'a> QueueHandle<'a> {
    pub fn namespace(&self) -> &str {
        &self.client.config().namespace
    }

    pub fn create_queue(&self) -> CreateQueueRequest {
        CreateQueueRequest {
            queue_name: self.name.clone(),
            attributes: QueueAttributes::default(),
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

    pub fn get_queue_attributes(&self) -> GetQueueAttributesRequest {
        GetQueueAttributesRequest {
            queue_name: self.name.clone(),
        }
    }

    pub fn set_queue_attributes(&self, attributes: QueueAttributes) -> SetQueueAttributesRequest {
        SetQueueAttributesRequest {
            queue_name: self.name.clone(),
            attributes,
        }
    }

    pub fn send_message(&self, body: impl Into<Vec<u8>>) -> SendMessageRequest {
        SendMessageRequest {
            queue_name: self.name.clone(),
            body: body.into(),
            delay: None,
        }
    }

    pub fn send_message_batch(&self, bodies: impl Into<Vec<Vec<u8>>>) -> SendMessageBatchRequest {
        SendMessageBatchRequest {
            queue_name: self.name.clone(),
            bodies: bodies.into(),
            delay: None,
        }
    }

    pub fn receive_messages(&self) -> ReceiveMessagesRequest {
        ReceiveMessagesRequest {
            queue_name: self.name.clone(),
            max_messages: 1,
            visibility_timeout: None,
            wait_time: None,
        }
    }

    pub fn delete_message(&self, receipt_handle: impl Into<String>) -> DeleteMessageRequest {
        DeleteMessageRequest {
            queue_name: self.name.clone(),
            receipt_handle: receipt_handle.into(),
        }
    }

    pub fn change_message_visibility(
        &self,
        receipt_handle: impl Into<String>,
        visibility_timeout: Duration,
    ) -> ChangeMessageVisibilityRequest {
        ChangeMessageVisibilityRequest {
            queue_name: self.name.clone(),
            receipt_handle: receipt_handle.into(),
            visibility_timeout,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateQueueRequest {
    pub queue_name: String,
    pub attributes: QueueAttributes,
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
pub struct GetQueueAttributesRequest {
    pub queue_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetQueueAttributesRequest {
    pub queue_name: String,
    pub attributes: QueueAttributes,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SendMessageRequest {
    pub queue_name: String,
    pub body: Vec<u8>,
    pub delay: Option<Duration>,
}

impl SendMessageRequest {
    pub fn with_delay(mut self, delay: Duration) -> Self {
        self.delay = Some(delay);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SendMessageBatchRequest {
    pub queue_name: String,
    pub bodies: Vec<Vec<u8>>,
    pub delay: Option<Duration>,
}

impl SendMessageBatchRequest {
    pub fn with_delay(mut self, delay: Duration) -> Self {
        self.delay = Some(delay);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReceiveMessagesRequest {
    pub queue_name: String,
    pub max_messages: usize,
    pub visibility_timeout: Option<Duration>,
    pub wait_time: Option<Duration>,
}

impl ReceiveMessagesRequest {
    pub fn with_max_messages(mut self, max_messages: usize) -> Self {
        self.max_messages = max_messages;
        self
    }

    pub fn with_visibility_timeout(mut self, visibility_timeout: Duration) -> Self {
        self.visibility_timeout = Some(visibility_timeout);
        self
    }

    pub fn with_wait_time(mut self, wait_time: Duration) -> Self {
        self.wait_time = Some(wait_time);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeleteMessageRequest {
    pub queue_name: String,
    pub receipt_handle: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChangeMessageVisibilityRequest {
    pub queue_name: String,
    pub receipt_handle: String,
    pub visibility_timeout: Duration,
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::connect;

    use super::QueueAttributes;

    #[test]
    fn queue_attributes_defaults_match_mvp_expectations() {
        let attributes = QueueAttributes::default();

        assert_eq!(attributes.visibility_timeout, Duration::from_secs(30));
        assert!(!attributes.fifo);
    }

    #[test]
    fn queue_requests_capture_queue_name() {
        let client = connect("s3://bucket/queue.db");
        let queue = client.queues().queue("emails");

        let request = queue.receive_messages().with_max_messages(8);

        assert_eq!(request.queue_name, "emails");
        assert_eq!(request.max_messages, 8);
    }
}
