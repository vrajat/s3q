"""Queue handle for the Python s3q SDK."""

from __future__ import annotations

from dataclasses import dataclass

from .consumer import Consumer
from .errors import translate_native_error
from .producer import Producer


@dataclass
class Queue:
    """Queue-scoped handle used to create producer and consumer handles."""

    client: object
    name: str
    namespace: str
    native_client: object

    def producer(self, worker_id: str) -> Producer:
        """Create a managed producer handle for this queue."""
        try:
            native = self.native_client.producer(self.name, worker_id)
        except Exception as error:
            raise translate_native_error(error) from error
        return Producer.from_native(native)

    def consumer(self, worker_id: str) -> Consumer:
        """Create a managed consumer handle for this queue."""
        try:
            native = self.native_client.consumer(self.name, worker_id)
        except Exception as error:
            raise translate_native_error(error) from error
        return Consumer.from_native(native)
