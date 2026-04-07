from __future__ import annotations

import argparse
import sys

from .client import Client
from .errors import NotReadyError
from .service import ServiceConfig, run_service


def build_parser() -> argparse.ArgumentParser:
    """Build the s3q command-line parser."""
    parser = argparse.ArgumentParser(prog="s3q")
    subparsers = parser.add_subparsers(dest="command", required=True)

    queue = subparsers.add_parser("queue")
    queue_subparsers = queue.add_subparsers(dest="queue_command", required=True)
    queue_create = queue_subparsers.add_parser("create")
    queue_create.add_argument("--dsn", required=True)
    queue_create.add_argument("--name", required=True)

    workflow = subparsers.add_parser("workflow")
    workflow_subparsers = workflow.add_subparsers(
        dest="workflow_command", required=True
    )
    workflow_start = workflow_subparsers.add_parser("start")
    workflow_start.add_argument("--dsn", required=True)
    workflow_start.add_argument("--name", required=True)
    workflow_start.add_argument("--workflow-id", required=True)
    workflow_start.add_argument("--input", default="{}")

    service = subparsers.add_parser("service")
    service_subparsers = service.add_subparsers(dest="service_command", required=True)
    service_run = service_subparsers.add_parser("run")
    service_run.add_argument("--config", required=True)

    return parser


def main(argv: list[str] | None = None) -> int:
    """Run the s3q CLI."""
    parser = build_parser()
    args = parser.parse_args(argv)

    try:
        if args.command == "queue" and args.queue_command == "create":
            client = Client.connect(args.dsn)
            client.create_queue(args.name)
            return 0

        if args.command == "workflow" and args.workflow_command == "start":
            client = Client.connect(args.dsn)
            client.workflow(args.name).start(args.workflow_id, args.input)
            return 0

        if args.command == "service" and args.service_command == "run":
            config = ServiceConfig.from_file(args.config)
            run_service(config)
            return 0
    except (NotReadyError, RuntimeError) as exc:
        print(exc, file=sys.stderr)
        return 2

    parser.error("unknown command")
    return 1


if __name__ == "__main__":
    raise SystemExit(main())
