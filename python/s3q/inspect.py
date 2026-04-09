"""Read-only inspection handles for the Python s3q SDK."""

from __future__ import annotations

from dataclasses import dataclass

from .errors import NotReadyError
from .types import Message, MessagePage, MessageState, QueueMetrics, QueueSummary


@dataclass(slots=True)
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
        _ = (self.queue_name, self.state, self.limit, self.cursor)
        raise NotReadyError("inspect.list_messages is not wired to the Rust core yet")


@dataclass(slots=True)
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
        _ = (self.queue_name, self.limit, self.cursor)
        raise NotReadyError(
            "inspect.list_archived_messages is not wired to the Rust core yet"
        )


@dataclass(slots=True)
class Inspect:
    """Read-only inspection surface."""

    client: object

    def list_queues(self) -> list[QueueSummary]:
        """List queues known to the store."""
        raise NotReadyError("inspect.list_queues is not wired to the Rust core yet")

    def metrics(self, queue_name: str) -> QueueMetrics:
        """Return exact metrics for one queue."""
        _ = queue_name
        raise NotReadyError("inspect.metrics is not wired to the Rust core yet")

    def metrics_all(self) -> list[QueueMetrics]:
        """Return exact metrics for all queues."""
        raise NotReadyError("inspect.metrics_all is not wired to the Rust core yet")

    def get_message(self, queue_name: str, message_id: int) -> Message:
        """Return one message by id."""
        _ = (queue_name, message_id)
        raise NotReadyError("inspect.get_message is not wired to the Rust core yet")

    def list_messages(self, queue_name: str) -> ListMessagesRequest:
        """Start a filtered active-message listing request."""
        return ListMessagesRequest(inspect=self, queue_name=queue_name)

    def list_archived_messages(self, queue_name: str) -> ListArchivedMessagesRequest:
        """Start an archived-message listing request."""
        return ListArchivedMessagesRequest(inspect=self, queue_name=queue_name)
