"""Python client surface for s3q."""

from __future__ import annotations

from dataclasses import dataclass

try:
    from . import _native
except ImportError:  # pragma: no cover - exercised in environments without the extension
    _native = None

from .errors import NotReadyError, translate_native_error
from .inspect import Inspect
from .queue import Queue


@dataclass
class ClientConfig:
    """Configuration for a Python s3q client."""

    dsn: str
    namespace: str = "default"
    service_name: str = "s3q"
    local_cache_dir: str | None = None


class Client:
    """Top-level Python s3q client."""

    def __init__(self, config: ClientConfig, native_client: object) -> None:
        """Create a client from an explicit configuration."""
        self._config = config
        self._native = native_client

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
        config = ClientConfig(
            dsn=dsn,
            namespace=namespace,
            service_name=service_name,
            local_cache_dir=local_cache_dir,
        )
        if _native is None:
            raise NotReadyError(
                "Python native module is not built yet; build the package before using the Python SDK"
            )
        try:
            native_client = _native.ClientCore(
                dsn,
                namespace=namespace,
                service_name=service_name,
                local_cache_dir=local_cache_dir,
            )
        except Exception as error:
            raise translate_native_error(error) from error
        return cls(config, native_client)

    @property
    def config(self) -> ClientConfig:
        """Return the client configuration."""
        return self._config

    def create_queue(self, name: str) -> Queue:
        """Create a queue and return a queue handle."""
        try:
            self._native.create_queue(name)
        except Exception as error:
            raise translate_native_error(error) from error
        return self.queue(name)

    def delete_queue(self, name: str) -> None:
        """Delete a queue."""
        try:
            self._native.delete_queue(name)
        except Exception as error:
            raise translate_native_error(error) from error

    def purge_queue(self, name: str) -> None:
        """Purge active messages from a queue."""
        try:
            self._native.purge_queue(name)
        except Exception as error:
            raise translate_native_error(error) from error

    def queue(self, name: str) -> Queue:
        """Return a queue-scoped handle."""
        return Queue(
            client=self,
            name=name,
            namespace=self._config.namespace,
            native_client=self._native,
        )

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
