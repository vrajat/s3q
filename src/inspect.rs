use crate::{types::MessageState, Client};

#[derive(Debug, Clone, Copy)]
pub struct Inspect<'a> {
    client: &'a Client,
}

impl<'a> Inspect<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub fn namespace(&self) -> &str {
        &self.client.config().namespace
    }

    pub fn list_queues(&self) -> ListQueuesRequest {
        ListQueuesRequest {
            namespace: self.namespace().to_string(),
        }
    }

    pub fn metrics(&self, queue_name: impl Into<String>) -> MetricsRequest {
        MetricsRequest {
            queue_name: queue_name.into(),
        }
    }

    pub fn metrics_all(&self) -> MetricsAllRequest {
        MetricsAllRequest {
            namespace: self.namespace().to_string(),
        }
    }

    pub fn list_messages(&self, queue_name: impl Into<String>) -> ListMessagesRequest {
        ListMessagesRequest {
            queue_name: queue_name.into(),
            state: None,
            limit: None,
            cursor: None,
        }
    }

    pub fn get_message(&self, queue_name: impl Into<String>, message_id: i64) -> GetMessageRequest {
        GetMessageRequest {
            queue_name: queue_name.into(),
            message_id,
        }
    }

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
pub struct ListQueuesRequest {
    pub namespace: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MetricsRequest {
    pub queue_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MetricsAllRequest {
    pub namespace: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListMessagesRequest {
    pub queue_name: String,
    pub state: Option<MessageState>,
    pub limit: Option<usize>,
    pub cursor: Option<String>,
}

impl ListMessagesRequest {
    pub fn with_state(mut self, state: MessageState) -> Self {
        self.state = Some(state);
        self
    }

    pub fn with_limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn with_cursor(mut self, cursor: impl Into<String>) -> Self {
        self.cursor = Some(cursor.into());
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetMessageRequest {
    pub queue_name: String,
    pub message_id: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListArchivedMessagesRequest {
    pub queue_name: String,
    pub limit: Option<usize>,
    pub cursor: Option<String>,
}

impl ListArchivedMessagesRequest {
    pub fn with_limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

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
