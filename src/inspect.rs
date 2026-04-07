use std::time::SystemTime;

use pgqrs::store::Store as _;

use crate::{
    types::{message_from_pgqrs, message_state},
    ArchivedMessage, Client, Error, Message, MessagePage, MessageState, QueueMetrics, QueueSummary,
    Result,
};

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

    /// List queues in the current namespace.
    pub async fn list_queues(&self) -> Result<Vec<QueueSummary>> {
        let queues = self.client.store().s3.queues().list().await?;
        Ok(queues
            .into_iter()
            .map(|queue| QueueSummary {
                queue_name: queue.queue_name,
                created_at: SystemTime::from(queue.created_at),
            })
            .collect())
    }

    /// Return exact metrics for one queue.
    pub async fn metrics(&self, queue_name: impl AsRef<str>) -> Result<QueueMetrics> {
        let queue_name = queue_name.as_ref();
        let queue = self
            .client
            .store()
            .s3
            .queues()
            .get_by_name(queue_name)
            .await?;
        Ok(self.metrics_for_queue(queue_name, queue.id).await?)
    }

    /// Return exact metrics for all queues in the namespace.
    pub async fn metrics_all(&self) -> Result<Vec<QueueMetrics>> {
        let queues = self.client.store().s3.queues().list().await?;
        let mut metrics = Vec::with_capacity(queues.len());
        for queue in queues {
            metrics.push(self.metrics_for_queue(&queue.queue_name, queue.id).await?);
        }
        Ok(metrics)
    }

    async fn metrics_for_queue(&self, queue_name: &str, queue_id: i64) -> Result<QueueMetrics> {
        let active = self
            .client
            .store()
            .s3
            .messages()
            .filter_by_fk(queue_id)
            .await?;
        let archived = self
            .client
            .store()
            .s3
            .messages()
            .list_archived_by_queue(queue_id)
            .await?;
        Ok(metrics_from_messages(queue_name, active, archived))
    }

    /// Build a request to list messages in a queue.
    pub fn list_messages(&self, queue_name: impl Into<String>) -> ListMessagesRequest<'_> {
        ListMessagesRequest {
            inspect: self,
            queue_name: queue_name.into(),
            state: None,
            limit: None,
            cursor: None,
        }
    }

    /// Inspect one message by id.
    pub async fn get_message(
        &self,
        queue_name: impl AsRef<str>,
        message_id: i64,
    ) -> Result<Message> {
        let queue = self
            .client
            .store()
            .s3
            .queues()
            .get_by_name(queue_name.as_ref())
            .await?;
        let message = self.client.store().s3.messages().get(message_id).await?;
        if message.queue_id != queue.id {
            return Err(Error::MessageNotFound(message_id));
        }
        Ok(message_from_pgqrs(message, None))
    }

    /// Build a request to list archived messages in a queue.
    pub fn list_archived_messages(
        &self,
        queue_name: impl Into<String>,
    ) -> ListArchivedMessagesRequest<'_> {
        ListArchivedMessagesRequest {
            inspect: self,
            queue_name: queue_name.into(),
            limit: None,
            cursor: None,
        }
    }
}

#[derive(Debug, Clone)]
/// Request to list messages in a queue.
pub struct ListMessagesRequest<'a> {
    inspect: &'a Inspect<'a>,
    /// Queue name to inspect.
    pub queue_name: String,
    /// Optional message state filter.
    pub state: Option<MessageState>,
    /// Optional maximum number of messages to return.
    pub limit: Option<usize>,
    /// Optional pagination cursor.
    pub cursor: Option<String>,
}

impl<'a> ListMessagesRequest<'a> {
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

    /// Continue listing after a message id returned by an earlier page.
    pub fn with_cursor(mut self, cursor: impl Into<String>) -> Self {
        self.cursor = Some(cursor.into());
        self
    }

    /// Execute this read-only inspection request.
    pub async fn execute(self) -> Result<MessagePage> {
        let queue = self
            .inspect
            .client
            .store()
            .s3
            .queues()
            .get_by_name(&self.queue_name)
            .await?;
        let messages = if self.state == Some(MessageState::Archived) {
            self.inspect
                .client
                .store()
                .s3
                .messages()
                .list_archived_by_queue(queue.id)
                .await?
        } else {
            self.inspect
                .client
                .store()
                .s3
                .messages()
                .filter_by_fk(queue.id)
                .await?
        };
        let messages = messages
            .into_iter()
            .map(|message| message_from_pgqrs(message, None))
            .collect::<Vec<_>>();
        Ok(page_messages(
            messages,
            self.state,
            self.limit,
            self.cursor,
        )?)
    }
}

#[derive(Debug, Clone)]
/// Request to list archived messages in a queue.
pub struct ListArchivedMessagesRequest<'a> {
    inspect: &'a Inspect<'a>,
    /// Queue name to inspect.
    pub queue_name: String,
    /// Optional maximum number of archived messages to return.
    pub limit: Option<usize>,
    /// Optional pagination cursor.
    pub cursor: Option<String>,
}

impl<'a> ListArchivedMessagesRequest<'a> {
    /// Limit the number of archived messages returned.
    pub fn with_limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Continue listing after a message id returned by an earlier page.
    pub fn with_cursor(mut self, cursor: impl Into<String>) -> Self {
        self.cursor = Some(cursor.into());
        self
    }

    /// Execute this read-only inspection request.
    pub async fn execute(self) -> Result<MessagePage> {
        let queue = self
            .inspect
            .client
            .store()
            .s3
            .queues()
            .get_by_name(&self.queue_name)
            .await?;
        let messages = self
            .inspect
            .client
            .store()
            .s3
            .messages()
            .list_archived_by_queue(queue.id)
            .await?;
        let messages = messages
            .into_iter()
            .map(|message| message_from_pgqrs(message, None))
            .collect::<Vec<ArchivedMessage>>();
        Ok(page_messages(
            messages,
            Some(MessageState::Archived),
            self.limit,
            self.cursor,
        )?)
    }
}

fn metrics_from_messages(
    queue_name: &str,
    active: Vec<pgqrs::QueueMessage>,
    archived: Vec<pgqrs::QueueMessage>,
) -> QueueMetrics {
    let mut visible_messages = 0;
    let mut leased_messages = 0;
    let mut delayed_messages = 0;

    for message in &active {
        match message_state(message) {
            MessageState::Visible => visible_messages += 1,
            MessageState::Leased => leased_messages += 1,
            MessageState::Delayed => delayed_messages += 1,
            MessageState::Archived => {}
        }
    }

    let archived_messages = archived.len() as u64;
    QueueMetrics {
        queue_name: queue_name.to_string(),
        visible_messages,
        leased_messages,
        delayed_messages,
        archived_messages,
        total_messages: active.len() as u64 + archived_messages,
    }
}

fn page_messages(
    mut messages: Vec<Message>,
    state: Option<MessageState>,
    limit: Option<usize>,
    cursor: Option<String>,
) -> Result<MessagePage> {
    let cursor = match cursor {
        Some(cursor) => Some(cursor.parse::<i64>().map_err(|_| {
            Error::InvalidArgument("message cursor must be a message id".to_string())
        })?),
        None => None,
    };

    messages.sort_by_key(|message| message.message_id);

    let start = cursor
        .and_then(|cursor| {
            messages
                .iter()
                .position(|message| message.message_id > cursor)
        })
        .unwrap_or(0);

    let filtered = messages
        .into_iter()
        .skip(start)
        .filter(|message| match state {
            Some(state) => message.state == state,
            None => true,
        })
        .collect::<Vec<_>>();

    let limit = limit.unwrap_or(filtered.len());
    let mut page = filtered.into_iter().take(limit + 1).collect::<Vec<_>>();
    let next_cursor = if page.len() > limit {
        page.pop();
        page.last().map(|message| message.message_id.to_string())
    } else {
        None
    };

    Ok(MessagePage {
        messages: page,
        next_cursor,
    })
}

#[cfg(test)]
mod tests {
    use crate::{Message, MessageState};
    use std::time::SystemTime;

    use super::page_messages;

    fn message(message_id: i64, state: MessageState) -> Message {
        Message {
            message_id,
            read_count: 0,
            enqueued_at: SystemTime::UNIX_EPOCH,
            visible_at: SystemTime::UNIX_EPOCH,
            payload: serde_json::json!({}),
            receipt_handle: None,
            state,
        }
    }

    #[test]
    fn page_messages_filters_and_returns_next_cursor() {
        let page = page_messages(
            vec![
                message(3, MessageState::Visible),
                message(1, MessageState::Visible),
                message(2, MessageState::Leased),
            ],
            Some(MessageState::Visible),
            Some(1),
            None,
        )
        .expect("page should build");

        assert_eq!(page.messages.len(), 1);
        assert_eq!(page.messages[0].message_id, 1);
        assert_eq!(page.next_cursor, Some("1".to_string()));
    }
}
