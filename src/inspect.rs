use crate::{types::MessageState, Client};

#[derive(Debug, Clone, Copy)]
/// Read-only inspection handle.
///
/// Inspection APIs are intended for metrics and debugging. They should not
/// lease, archive, delete, or otherwise mutate messages.
pub struct Inspect<'a> {
    client: &'a Client,
}

impl<'a> Inspect<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Return the namespace inspected by this handle.
    pub fn namespace(&self) -> &str {
        &self.client.config().namespace
    }

    /// Build a request to list queues in the current namespace.
    pub fn list_queues(&self) -> ListQueuesRequest {
        ListQueuesRequest {
            namespace: self.namespace().to_string(),
        }
    }

    /// Build a request for metrics for one queue.
    pub fn metrics(&self, queue_name: impl Into<String>) -> MetricsRequest {
        MetricsRequest {
            queue_name: queue_name.into(),
        }
    }

    /// Build a request for metrics for all queues in the namespace.
    pub fn metrics_all(&self) -> MetricsAllRequest {
        MetricsAllRequest {
            namespace: self.namespace().to_string(),
        }
    }

    /// Build a request to list active messages in a queue.
    pub fn list_messages(&self, queue_name: impl Into<String>) -> ListMessagesRequest {
        ListMessagesRequest {
            queue_name: queue_name.into(),
            state: None,
            limit: None,
            cursor: None,
        }
    }

    /// Build a request to inspect one message by id.
    pub fn get_message(&self, queue_name: impl Into<String>, message_id: i64) -> GetMessageRequest {
        GetMessageRequest {
            queue_name: queue_name.into(),
            message_id,
        }
    }

    /// Build a request to list archived messages in a queue.
    pub fn list_archived_messages(
        &self,
        queue_name: impl Into<String>,
    ) -> ListArchivedMessagesRequest {
        ListArchivedMessagesRequest {
            queue_name: queue_name.into(),
            limit: None,
            cursor: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Request to list queues in a namespace.
pub struct ListQueuesRequest {
    /// Namespace to list queues from.
    pub namespace: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Request for metrics for one queue.
pub struct MetricsRequest {
    /// Queue name to inspect.
    pub queue_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Request for metrics for all queues in a namespace.
pub struct MetricsAllRequest {
    /// Namespace to inspect.
    pub namespace: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Request to list messages in a queue.
pub struct ListMessagesRequest {
    /// Queue name to inspect.
    pub queue_name: String,
    /// Optional message state filter.
    pub state: Option<MessageState>,
    /// Optional maximum number of messages to return.
    pub limit: Option<usize>,
    /// Optional pagination cursor.
    pub cursor: Option<String>,
}

impl ListMessagesRequest {
    /// Filter messages by state.
    pub fn with_state(mut self, state: MessageState) -> Self {
        self.state = Some(state);
        self
    }

    /// Limit the number of messages returned.
    pub fn with_limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Continue listing from a cursor returned by an earlier request.
    pub fn with_cursor(mut self, cursor: impl Into<String>) -> Self {
        self.cursor = Some(cursor.into());
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Request to inspect one message by id.
pub struct GetMessageRequest {
    /// Queue name to inspect.
    pub queue_name: String,
    /// Message id to inspect.
    pub message_id: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Request to list archived messages in a queue.
pub struct ListArchivedMessagesRequest {
    /// Queue name to inspect.
    pub queue_name: String,
    /// Optional maximum number of archived messages to return.
    pub limit: Option<usize>,
    /// Optional pagination cursor.
    pub cursor: Option<String>,
}

impl ListArchivedMessagesRequest {
    /// Limit the number of archived messages returned.
    pub fn with_limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Continue listing from a cursor returned by an earlier request.
    pub fn with_cursor(mut self, cursor: impl Into<String>) -> Self {
        self.cursor = Some(cursor.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::{ListMessagesRequest, MessageState};

    #[test]
    fn inspection_requests_are_queue_scoped() {
        let request = ListMessagesRequest {
            queue_name: "emails".to_string(),
            state: None,
            limit: None,
            cursor: None,
        }
        .with_state(MessageState::Archived)
        .with_limit(10);

        assert_eq!(request.queue_name, "emails");
        assert_eq!(request.state, Some(MessageState::Archived));
        assert_eq!(request.limit, Some(10));
    }
}
