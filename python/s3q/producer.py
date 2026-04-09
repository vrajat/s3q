"""Producer handle for the Python s3q SDK."""

from __future__ import annotations

from dataclasses import dataclass
from datetime import timedelta
from typing import Any

from .errors import NotReadyError
from .types import Message


@dataclass(slots=True)
class Producer:
    """Producer handle for sending messages to one queue."""

    queue_name: str
    namespace: str
    worker_id: str

    def send(self, payload: Any, *, delay: timedelta | None = None) -> Message:
        """Send one payload immediately or after an optional delay."""
        _ = (payload, delay)
        raise NotReadyError("producer.send is not wired to the Rust core yet")

    def send_batch(
        self, payloads: list[Any], *, delay: timedelta | None = None
    ) -> list[Message]:
        """Send multiple payloads immediately or after an optional delay."""
        _ = (payloads, delay)
        raise NotReadyError("producer.send_batch is not wired to the Rust core yet")
