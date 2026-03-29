from __future__ import annotations

from dataclasses import dataclass

from .errors import NotReadyError


@dataclass(slots=True)
class WorkflowHandle:
    client: object
    name: str

    def start(self, workflow_id: str, payload: bytes | str) -> None:
        _ = workflow_id
        _ = payload
        raise NotReadyError("workflow.start is not wired to the Rust core yet")

    def describe(self, workflow_id: str) -> None:
        _ = workflow_id
        raise NotReadyError("workflow.describe is not wired to the Rust core yet")

    def signal(self, workflow_id: str, signal_name: str, payload: bytes | str) -> None:
        _ = workflow_id
        _ = signal_name
        _ = payload
        raise NotReadyError("workflow.signal is not wired to the Rust core yet")

    def query(self, workflow_id: str, query_name: str, payload: bytes | str = b"") -> None:
        _ = workflow_id
        _ = query_name
        _ = payload
        raise NotReadyError("workflow.query is not wired to the Rust core yet")

    def result(self, workflow_id: str) -> None:
        _ = workflow_id
        raise NotReadyError("workflow.result is not wired to the Rust core yet")

    def cancel(self, workflow_id: str) -> None:
        _ = workflow_id
        raise NotReadyError("workflow.cancel is not wired to the Rust core yet")

    def terminate(self, workflow_id: str, *, reason: str) -> None:
        _ = workflow_id
        _ = reason
        raise NotReadyError("workflow.terminate is not wired to the Rust core yet")
