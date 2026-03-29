from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path
import tomllib


@dataclass(slots=True)
class ServiceConfig:
    dsn: str
    namespace: str = "default"
    queue_poll_interval_ms: int = 500
    workflow_poll_interval_ms: int = 500

    @classmethod
    def from_file(cls, path: str | Path) -> "ServiceConfig":
        with Path(path).open("rb") as handle:
            data = tomllib.load(handle)
        return cls(**data)


def run_service(config: ServiceConfig) -> None:
    raise RuntimeError(
        "service runner is scaffolded but not implemented; wire it to the Rust core first"
    )
