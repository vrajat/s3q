"""Python client surface for s3q."""

from __future__ import annotations

from dataclasses import dataclass

from .errors import NotReadyError
from .inspect import Inspect
from .queue import Queue


@dataclass(slots=True)
class ClientConfig:
    """Configuration for a Python s3q client."""

    dsn: str
    namespace: str = "default"
    service_name: str = "s3q"
    local_cache_dir: str | None = None


class Client:
    """Top-level Python s3q client."""

    def __init__(self, config: ClientConfig) -> None:
        """Create a client from an explicit configuration."""
        self._config = config

    @classmethod
    def connect(
        cls,
        dsn: str,
        *,
        namespace: str = "default",
        service_name: str = "s3q",
        local_cache_dir: str | None = None,
    ) -> "Client":
        """Create a client from an S3 DSN."""
        return cls(
            ClientConfig(
                dsn=dsn,
                namespace=namespace,
                service_name=service_name,
                local_cache_dir=local_cache_dir,
            )
        )

    @property
    def config(self) -> ClientConfig:
        """Return the client configuration."""
        return self._config

    def create_queue(self, name: str) -> Queue:
        """Create a queue and return a queue handle."""
        _ = name
        raise NotReadyError("client.create_queue is not wired to the Rust core yet")

    def delete_queue(self, name: str) -> None:
        """Delete a queue."""
        _ = name
        raise NotReadyError("client.delete_queue is not wired to the Rust core yet")

    def purge_queue(self, name: str) -> None:
        """Purge active messages from a queue."""
        _ = name
        raise NotReadyError("client.purge_queue is not wired to the Rust core yet")

    def queue(self, name: str) -> Queue:
        """Return a queue-scoped handle."""
        return Queue(client=self, name=name, namespace=self._config.namespace)

    def inspect(self) -> Inspect:
        """Return the read-only inspection handle."""
        return Inspect(client=self)


def connect(
    dsn: str,
    *,
    namespace: str = "default",
    service_name: str = "s3q",
    local_cache_dir: str | None = None,
) -> Client:
    """Create a client from an S3 DSN."""
    return Client.connect(
        dsn,
        namespace=namespace,
        service_name=service_name,
        local_cache_dir=local_cache_dir,
    )
