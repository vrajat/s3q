"""Producer handle for the Python s3q SDK."""

from __future__ import annotations

from dataclasses import dataclass
from datetime import timedelta
import json
from typing import Any

from .errors import translate_native_error
from .types import Message, message_from_native


@dataclass
class Producer:
    """Producer handle for sending messages to one queue."""

    queue_name: str
    namespace: str
    worker_id: str
    _native: object

    @classmethod
    def from_native(cls, native: object) -> "Producer":
        """Build a Producer wrapper from the native binding object."""
        return cls(
            queue_name=native.queue_name,
            namespace=native.namespace,
            worker_id=native.worker_id,
            _native=native,
        )

    def send(self, payload: Any, *, delay: timedelta | None = None) -> Message:
        """Send one payload immediately or after an optional delay."""
        try:
            return message_from_native(
                self._native.send(
                    json.dumps(payload),
                    delay_seconds=delay.total_seconds() if delay is not None else None,
                )
            )
        except Exception as error:
            raise translate_native_error(error) from error

    def send_batch(
        self, payloads: list[Any], *, delay: timedelta | None = None
    ) -> list[Message]:
        """Send multiple payloads immediately or after an optional delay."""
        try:
            result = self._native.send_batch(
                [json.dumps(payload) for payload in payloads],
                delay_seconds=delay.total_seconds() if delay is not None else None,
            )
        except Exception as error:
            raise translate_native_error(error) from error
        return [message_from_native(item) for item in result]
