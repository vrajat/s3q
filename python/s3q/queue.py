from __future__ import annotations

from dataclasses import dataclass

from .errors import NotReadyError


@dataclass(slots=True)
class Queue:
    """Queue-scoped Python API scaffold."""

    client: object
    """Client that created this queue handle."""
    name: str
    """Queue name."""

    def send_message(self, payload: bytes | str) -> None:
        """Send one message payload."""
        _ = payload
        raise NotReadyError("queue.send_message is not wired to the Rust core yet")

    def receive_messages(self, *, max_messages: int = 1) -> None:
        """Receive leased messages."""
        _ = max_messages
        raise NotReadyError("queue.receive_messages is not wired to the Rust core yet")

    def delete_message(self, receipt_handle: str) -> None:
        """Delete a leased message by receipt handle."""
        _ = receipt_handle
        raise NotReadyError("queue.delete_message is not wired to the Rust core yet")

    def change_message_visibility(
        self,
        receipt_handle: str,
        *,
        visibility_timeout_seconds: int,
    ) -> None:
        """Change the visibility timeout for a leased message."""
        _ = receipt_handle
        _ = visibility_timeout_seconds
        raise NotReadyError(
            "queue.change_message_visibility is not wired to the Rust core yet"
        )
