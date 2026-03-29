from __future__ import annotations

from dataclasses import dataclass

from .errors import NotReadyError


@dataclass(slots=True)
class QueueHandle:
    client: object
    name: str

    def create_queue(self) -> None:
        raise NotReadyError("queue.create_queue is not wired to the Rust core yet")

    def delete_queue(self) -> None:
        raise NotReadyError("queue.delete_queue is not wired to the Rust core yet")

    def send_message(self, payload: bytes | str) -> None:
        _ = payload
        raise NotReadyError("queue.send_message is not wired to the Rust core yet")

    def receive_messages(self, *, max_messages: int = 1) -> None:
        _ = max_messages
        raise NotReadyError("queue.receive_messages is not wired to the Rust core yet")

    def delete_message(self, receipt_handle: str) -> None:
        _ = receipt_handle
        raise NotReadyError("queue.delete_message is not wired to the Rust core yet")

    def change_message_visibility(
        self,
        receipt_handle: str,
        *,
        visibility_timeout_seconds: int,
    ) -> None:
        _ = receipt_handle
        _ = visibility_timeout_seconds
        raise NotReadyError(
            "queue.change_message_visibility is not wired to the Rust core yet"
        )
