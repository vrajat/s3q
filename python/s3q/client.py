from __future__ import annotations

from dataclasses import dataclass

from .queue import QueueHandle
from .workflow import WorkflowHandle


@dataclass(slots=True)
class ClientConfig:
    dsn: str
    namespace: str = "default"
    service_name: str = "s3q"
    local_cache_dir: str | None = None


class Client:
    def __init__(self, config: ClientConfig) -> None:
        self._config = config

    @classmethod
    def connect(cls, dsn: str, *, namespace: str = "default") -> "Client":
        return cls(ClientConfig(dsn=dsn, namespace=namespace))

    @property
    def config(self) -> ClientConfig:
        return self._config

    def queue(self, name: str) -> QueueHandle:
        return QueueHandle(client=self, name=name)

    def workflow(self, name: str) -> WorkflowHandle:
        return WorkflowHandle(client=self, name=name)


def connect(dsn: str, *, namespace: str = "default") -> Client:
    return Client.connect(dsn, namespace=namespace)
