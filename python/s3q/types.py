"""Python value types for the thin s3q SDK."""

from __future__ import annotations

from dataclasses import dataclass
from datetime import datetime
from enum import Enum
import json
from typing import Any


class MessageState(str, Enum):
    """Public message lifecycle states exposed by s3q."""

    VISIBLE = "visible"
    LEASED = "leased"
    DELAYED = "delayed"
    ARCHIVED = "archived"


@dataclass(frozen=True)
class ReceiptHandle:
    """Opaque lease token used to complete a leased message."""

    value: str

    def __str__(self) -> str:
        """Return the opaque wire value."""
        return self.value


@dataclass(frozen=True)
class Message:
    """Public queue message record."""

    message_id: int
    payload: Any
    read_count: int
    enqueued_at: datetime | None = None
    visible_at: datetime | None = None
    receipt_handle: ReceiptHandle | None = None
    state: MessageState = MessageState.VISIBLE


@dataclass(frozen=True)
class QueueSummary:
    """Read-only queue summary."""

    queue_name: str
    created_at: datetime | None = None


@dataclass(frozen=True)
class QueueMetrics:
    """Exact queue metrics returned by inspection APIs."""

    queue_name: str
    visible_messages: int
    leased_messages: int
    delayed_messages: int
    archived_messages: int
    total_messages: int


@dataclass(frozen=True)
class MessagePage:
    """One page of inspection results."""

    messages: list[Message]
    next_cursor: str | None = None


def _dt_from_seconds(value: float | int | None) -> datetime | None:
    if value is None:
        return None
    return datetime.fromtimestamp(value)


def message_from_native(data: dict[str, Any]) -> Message:
    """Build a public Message from a native mapping."""

    receipt = data.get("receipt_handle")
    return Message(
        message_id=int(data["message_id"]),
        payload=json.loads(data["payload_json"]),
        read_count=int(data["read_count"]),
        enqueued_at=_dt_from_seconds(data.get("enqueued_at")),
        visible_at=_dt_from_seconds(data.get("visible_at")),
        receipt_handle=ReceiptHandle(receipt) if receipt is not None else None,
        state=MessageState(data["state"]),
    )


def queue_summary_from_native(data: dict[str, Any]) -> QueueSummary:
    """Build a QueueSummary from a native mapping."""

    return QueueSummary(
        queue_name=data["queue_name"],
        created_at=_dt_from_seconds(data.get("created_at")),
    )


def queue_metrics_from_native(data: dict[str, Any]) -> QueueMetrics:
    """Build QueueMetrics from a native mapping."""

    return QueueMetrics(
        queue_name=data["queue_name"],
        visible_messages=int(data["visible_messages"]),
        leased_messages=int(data["leased_messages"]),
        delayed_messages=int(data["delayed_messages"]),
        archived_messages=int(data["archived_messages"]),
        total_messages=int(data["total_messages"]),
    )


def message_page_from_native(data: dict[str, Any]) -> MessagePage:
    """Build a MessagePage from a native mapping."""

    return MessagePage(
        messages=[message_from_native(item) for item in data["messages"]],
        next_cursor=data.get("next_cursor"),
    )
