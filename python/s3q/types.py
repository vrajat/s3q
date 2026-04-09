"""Python value types for the thin s3q SDK."""

from __future__ import annotations

from dataclasses import dataclass
from datetime import datetime
from enum import StrEnum
from typing import Any


class MessageState(StrEnum):
    """Public message lifecycle states exposed by s3q."""

    VISIBLE = "visible"
    LEASED = "leased"
    DELAYED = "delayed"
    ARCHIVED = "archived"


@dataclass(frozen=True, slots=True)
class ReceiptHandle:
    """Opaque lease token used to complete a leased message."""

    value: str

    def __str__(self) -> str:
        """Return the opaque wire value."""
        return self.value


@dataclass(frozen=True, slots=True)
class Message:
    """Public queue message record."""

    message_id: int
    payload: Any
    read_count: int
    enqueued_at: datetime | None = None
    visible_at: datetime | None = None
    receipt_handle: ReceiptHandle | None = None
    state: MessageState = MessageState.VISIBLE


@dataclass(frozen=True, slots=True)
class QueueSummary:
    """Read-only queue summary."""

    queue_name: str
    created_at: datetime | None = None


@dataclass(frozen=True, slots=True)
class QueueMetrics:
    """Exact queue metrics returned by inspection APIs."""

    queue_name: str
    visible_messages: int
    leased_messages: int
    delayed_messages: int
    archived_messages: int
    total_messages: int


@dataclass(frozen=True, slots=True)
class MessagePage:
    """One page of inspection results."""

    messages: list[Message]
    next_cursor: str | None = None
