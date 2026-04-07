from __future__ import annotations

from dataclasses import dataclass

from .queue import QueueHandle
from .workflow import WorkflowHandle


@dataclass(slots=True)
class ClientConfig:
    """Configuration for a Python s3q client."""

    dsn: str
    """S3 DSN for the queue database object."""
    namespace: str = "default"
    """Logical namespace used for queues."""
    service_name: str = "s3q"
    """Service name used for managed worker identity."""
    local_cache_dir: str | None = None
    """Optional local cache directory."""


class Client:
    """Top-level Python s3q client scaffold."""

    def __init__(self, config: ClientConfig) -> None:
        """Create a client from an explicit configuration."""
        self._config = config

    @classmethod
    def connect(cls, dsn: str, *, namespace: str = "default") -> "Client":
        """Create a client from an S3 DSN."""
        return cls(ClientConfig(dsn=dsn, namespace=namespace))

    @property
    def config(self) -> ClientConfig:
        """Return the client configuration."""
        return self._config

    def queue(self, name: str) -> QueueHandle:
        """Return a queue handle for a queue name."""
        return QueueHandle(client=self, name=name)

    def workflow(self, name: str) -> WorkflowHandle:
        """Return a workflow handle scaffold.

        Workflows are not part of the s3q v1 product surface.
        """
        return WorkflowHandle(client=self, name=name)


def connect(dsn: str, *, namespace: str = "default") -> Client:
    """Create a client from an S3 DSN."""
    return Client.connect(dsn, namespace=namespace)
