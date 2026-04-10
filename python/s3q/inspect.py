"""Read-only inspection handles for the Python s3q SDK."""

from __future__ import annotations

from dataclasses import dataclass

from .errors import translate_native_error
from .types import (
    Message,
    MessagePage,
    MessageState,
    QueueMetrics,
    QueueSummary,
    message_from_native,
    message_page_from_native,
    queue_metrics_from_native,
    queue_summary_from_native,
)


@dataclass
class ListMessagesRequest:
    """Read-only builder for filtered message listing."""

    inspect: "Inspect"
    queue_name: str
    state: MessageState | None = None
    limit: int | None = None
    cursor: str | None = None

    def with_state(self, state: MessageState | None) -> "ListMessagesRequest":
        """Filter listed messages by state."""
        self.state = state
        return self

    def with_limit(self, limit: int | None) -> "ListMessagesRequest":
        """Limit the number of messages returned in one page."""
        self.limit = limit
        return self

    def with_cursor(self, cursor: str | None) -> "ListMessagesRequest":
        """Resume listing from a previous cursor."""
        self.cursor = cursor
        return self

    def execute(self) -> MessagePage:
        """Run the listing request."""
        try:
            result = self.inspect.client._native.list_messages(
                self.queue_name,
                state=self.state.value if self.state is not None else None,
                limit=self.limit,
                cursor=self.cursor,
            )
        except Exception as error:
            raise translate_native_error(error) from error
        return message_page_from_native(result)


@dataclass
class ListArchivedMessagesRequest:
    """Read-only builder for archived message listing."""

    inspect: "Inspect"
    queue_name: str
    limit: int | None = None
    cursor: str | None = None

    def with_limit(self, limit: int | None) -> "ListArchivedMessagesRequest":
        """Limit the number of archived messages returned in one page."""
        self.limit = limit
        return self

    def with_cursor(self, cursor: str | None) -> "ListArchivedMessagesRequest":
        """Resume archived listing from a previous cursor."""
        self.cursor = cursor
        return self

    def execute(self) -> MessagePage:
        """Run the archived listing request."""
        try:
            result = self.inspect.client._native.list_archived_messages(
                self.queue_name,
                limit=self.limit,
                cursor=self.cursor,
            )
        except Exception as error:
            raise translate_native_error(error) from error
        return message_page_from_native(result)


@dataclass
class Inspect:
    """Read-only inspection surface."""

    client: object

    def list_queues(self) -> list[QueueSummary]:
        """List queues known to the store."""
        try:
            result = self.client._native.list_queues()
        except Exception as error:
            raise translate_native_error(error) from error
        return [queue_summary_from_native(item) for item in result]

    def metrics(self, queue_name: str) -> QueueMetrics:
        """Return exact metrics for one queue."""
        try:
            result = self.client._native.metrics(queue_name)
        except Exception as error:
            raise translate_native_error(error) from error
        return queue_metrics_from_native(result)

    def metrics_all(self) -> list[QueueMetrics]:
        """Return exact metrics for all queues."""
        try:
            result = self.client._native.metrics_all()
        except Exception as error:
            raise translate_native_error(error) from error
        return [queue_metrics_from_native(item) for item in result]

    def get_message(self, queue_name: str, message_id: int) -> Message:
        """Return one message by id."""
        try:
            result = self.client._native.get_message(queue_name, message_id)
        except Exception as error:
            raise translate_native_error(error) from error
        return message_from_native(result)

    def list_messages(self, queue_name: str) -> ListMessagesRequest:
        """Start a filtered active-message listing request."""
        return ListMessagesRequest(inspect=self, queue_name=queue_name)

    def list_archived_messages(self, queue_name: str) -> ListArchivedMessagesRequest:
        """Start an archived-message listing request."""
        return ListArchivedMessagesRequest(inspect=self, queue_name=queue_name)
