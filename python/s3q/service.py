from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path
import tomllib


@dataclass(slots=True)
class ServiceConfig:
    """Configuration for the planned s3q service runner."""

    dsn: str
    """S3 DSN for the queue database object."""
    namespace: str = "default"
    """Logical namespace used for queues."""
    queue_poll_interval_ms: int = 500
    """Queue polling interval in milliseconds."""
    workflow_poll_interval_ms: int = 500
    """Workflow polling interval in milliseconds."""

    @classmethod
    def from_file(cls, path: str | Path) -> "ServiceConfig":
        """Load service configuration from a TOML file."""
        with Path(path).open("rb") as handle:
            data = tomllib.load(handle)
        return cls(**data)


def run_service(config: ServiceConfig) -> None:
    """Run the service scaffold."""
    raise RuntimeError(
        "service runner is scaffolded but not implemented; wire it to the Rust core first"
    )
