"""Consumer handle for the Python s3q SDK."""

from __future__ import annotations

from dataclasses import dataclass
from datetime import timedelta

from .errors import NotReadyError
from .types import Message, ReceiptHandle


@dataclass(slots=True)
class Consumer:
    """Consumer handle for leasing and completing messages from one queue."""

    queue_name: str
    namespace: str
    worker_id: str

    def read(self, *, vt: timedelta | None = None) -> Message | None:
        """Lease at most one visible message."""
        _ = vt
        raise NotReadyError("consumer.read is not wired to the Rust core yet")

    def read_batch(self, *, vt: timedelta | None = None, qty: int = 1) -> list[Message]:
        """Lease up to ``qty`` visible messages."""
        _ = (vt, qty)
        raise NotReadyError("consumer.read_batch is not wired to the Rust core yet")

    def read_with_poll(
        self,
        *,
        vt: timedelta | None = None,
        qty: int = 1,
        poll_timeout: timedelta | None = None,
        poll_interval: timedelta | None = None,
    ) -> list[Message]:
        """Long-poll for visible messages and return an empty list on timeout."""
        _ = (vt, qty, poll_timeout, poll_interval)
        raise NotReadyError("consumer.read_with_poll is not wired to the Rust core yet")

    def delete_message(self, receipt_handle: ReceiptHandle) -> bool:
        """Permanently delete a leased message."""
        _ = receipt_handle
        raise NotReadyError("consumer.delete_message is not wired to the Rust core yet")

    def archive_message(self, receipt_handle: ReceiptHandle) -> Message | None:
        """Archive and retain one leased message."""
        _ = receipt_handle
        raise NotReadyError("consumer.archive_message is not wired to the Rust core yet")

    def archive_messages(self, receipt_handles: list[ReceiptHandle]) -> list[bool]:
        """Archive and retain multiple leased messages."""
        _ = receipt_handles
        raise NotReadyError("consumer.archive_messages is not wired to the Rust core yet")

    def set_vt(self, receipt_handle: ReceiptHandle, vt: timedelta) -> bool:
        """Change the visibility timeout for a leased message."""
        _ = (receipt_handle, vt)
        raise NotReadyError("consumer.set_vt is not wired to the Rust core yet")
