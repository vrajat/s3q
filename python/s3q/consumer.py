"""Consumer handle for the Python s3q SDK."""

from __future__ import annotations

from dataclasses import dataclass
from datetime import timedelta

from .errors import translate_native_error
from .types import Message, ReceiptHandle, message_from_native


@dataclass
class Consumer:
    """Consumer handle for leasing and completing messages from one queue."""

    queue_name: str
    namespace: str
    worker_id: str
    _native: object

    @classmethod
    def from_native(cls, native: object) -> "Consumer":
        """Build a Consumer wrapper from the native binding object."""
        return cls(
            queue_name=native.queue_name,
            namespace=native.namespace,
            worker_id=native.worker_id,
            _native=native,
        )

    def read(self, *, vt: timedelta | None = None) -> Message | None:
        """Lease at most one visible message."""
        try:
            message = self._native.read(
                vt_seconds=int(vt.total_seconds()) if vt is not None else None
            )
        except Exception as error:
            raise translate_native_error(error) from error
        if message is None:
            return None
        return message_from_native(message)

    def read_batch(self, *, vt: timedelta | None = None, qty: int = 1) -> list[Message]:
        """Lease up to ``qty`` visible messages."""
        try:
            messages = self._native.read_batch(
                vt_seconds=int(vt.total_seconds()) if vt is not None else None,
                qty=qty,
            )
        except Exception as error:
            raise translate_native_error(error) from error
        return [message_from_native(item) for item in messages]

    def read_with_poll(
        self,
        *,
        vt: timedelta | None = None,
        qty: int = 1,
        poll_timeout: timedelta | None = None,
        poll_interval: timedelta | None = None,
    ) -> list[Message]:
        """Long-poll for visible messages and return an empty list on timeout."""
        try:
            messages = self._native.read_with_poll(
                vt_seconds=int(vt.total_seconds()) if vt is not None else None,
                qty=qty,
                poll_timeout_seconds=(
                    poll_timeout.total_seconds() if poll_timeout is not None else None
                ),
                poll_interval_seconds=(
                    poll_interval.total_seconds() if poll_interval is not None else None
                ),
            )
        except Exception as error:
            raise translate_native_error(error) from error
        return [message_from_native(item) for item in messages]

    def delete_message(self, receipt_handle: ReceiptHandle) -> bool:
        """Permanently delete a leased message."""
        try:
            return self._native.delete_message(str(receipt_handle))
        except Exception as error:
            raise translate_native_error(error) from error

    def archive_message(self, receipt_handle: ReceiptHandle) -> Message | None:
        """Archive and retain one leased message."""
        try:
            message = self._native.archive_message(str(receipt_handle))
        except Exception as error:
            raise translate_native_error(error) from error
        if message is None:
            return None
        return message_from_native(message)

    def archive_messages(self, receipt_handles: list[ReceiptHandle]) -> list[bool]:
        """Archive and retain multiple leased messages."""
        try:
            return self._native.archive_messages([str(handle) for handle in receipt_handles])
        except Exception as error:
            raise translate_native_error(error) from error

    def set_vt(self, receipt_handle: ReceiptHandle, vt: timedelta) -> bool:
        """Change the visibility timeout for a leased message."""
        try:
            return self._native.set_vt(str(receipt_handle), int(vt.total_seconds()))
        except Exception as error:
            raise translate_native_error(error) from error
