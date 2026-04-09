"""Queue handle for the Python s3q SDK."""

from __future__ import annotations

from dataclasses import dataclass

from .consumer import Consumer
from .producer import Producer


@dataclass(slots=True)
class Queue:
    """Queue-scoped handle used to create producer and consumer handles."""

    client: object
    name: str
    namespace: str

    def producer(self, worker_id: str) -> Producer:
        """Create a managed producer handle for this queue."""
        return Producer(
            queue_name=self.name,
            namespace=self.namespace,
            worker_id=worker_id,
        )

    def consumer(self, worker_id: str) -> Consumer:
        """Create a managed consumer handle for this queue."""
        return Consumer(
            queue_name=self.name,
            namespace=self.namespace,
            worker_id=worker_id,
        )
