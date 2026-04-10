from __future__ import annotations

from datetime import datetime, timedelta
import unittest
from unittest import mock

import s3q
import s3q.client as client_module


class FakeProducerCore:
    def __init__(self, queue_name: str, worker_id: str) -> None:
        self.queue_name = queue_name
        self.namespace = "default"
        self.worker_id = worker_id

    def send(self, payload_json: str, delay_seconds: float | None = None) -> dict:
        return {
            "message_id": 1,
            "read_count": 0,
            "enqueued_at": 10.0,
            "visible_at": 10.0 + (delay_seconds or 0),
            "payload_json": payload_json,
            "receipt_handle": None,
            "state": "visible",
        }

    def send_batch(
        self, payloads_json: list[str], delay_seconds: float | None = None
    ) -> list[dict]:
        return [
            {
                "message_id": index + 1,
                "read_count": 0,
                "enqueued_at": 20.0,
                "visible_at": 20.0 + (delay_seconds or 0),
                "payload_json": payload,
                "receipt_handle": None,
                "state": "visible",
            }
            for index, payload in enumerate(payloads_json)
        ]


class FakeConsumerCore:
    def __init__(self, queue_name: str, worker_id: str) -> None:
        self.queue_name = queue_name
        self.namespace = "default"
        self.worker_id = worker_id

    def read(self, vt_seconds: int | None = None) -> dict | None:
        return {
            "message_id": 10,
            "read_count": 1,
            "enqueued_at": 30.0,
            "visible_at": 60.0,
            "payload_json": '{"kind":"one"}',
            "receipt_handle": "s3q1:10:99",
            "state": "leased",
        }

    def read_batch(self, vt_seconds: int | None = None, qty: int = 1) -> list[dict]:
        return [
            {
                "message_id": 11 + index,
                "read_count": 1,
                "enqueued_at": 30.0,
                "visible_at": 60.0,
                "payload_json": f'{{"kind":"batch","n":{index}}}',
                "receipt_handle": f"s3q1:{11 + index}:99",
                "state": "leased",
            }
            for index in range(qty)
        ]

    def read_with_poll(
        self,
        vt_seconds: int | None = None,
        qty: int = 1,
        poll_timeout_seconds: float | None = None,
        poll_interval_seconds: float | None = None,
    ) -> list[dict]:
        if poll_timeout_seconds == 0:
            return []
        return self.read_batch(vt_seconds=vt_seconds, qty=qty)

    def delete_message(self, receipt_handle: str) -> bool:
        raise PermissionError("message is owned by another consumer")

    def archive_message(self, receipt_handle: str) -> dict | None:
        return {
            "message_id": 10,
            "read_count": 1,
            "enqueued_at": 30.0,
            "visible_at": 60.0,
            "payload_json": '{"kind":"one"}',
            "receipt_handle": None,
            "state": "archived",
        }

    def archive_messages(self, receipt_handles: list[str]) -> list[bool]:
        return [True for _ in receipt_handles]

    def set_vt(self, receipt_handle: str, vt_seconds: int) -> bool:
        return vt_seconds == 30


class FakeClientCore:
    def __init__(
        self,
        dsn: str,
        namespace: str = "default",
        service_name: str = "s3q",
        local_cache_dir: str | None = None,
    ) -> None:
        self.dsn = dsn
        self.namespace = namespace
        self.service_name = service_name
        self.local_cache_dir = local_cache_dir

    def create_queue(self, name: str) -> None:
        self.last_created = name

    def delete_queue(self, name: str) -> None:
        self.last_deleted = name

    def purge_queue(self, name: str) -> None:
        self.last_purged = name

    def producer(self, queue_name: str, worker_id: str) -> FakeProducerCore:
        return FakeProducerCore(queue_name, worker_id)

    def consumer(self, queue_name: str, worker_id: str) -> FakeConsumerCore:
        return FakeConsumerCore(queue_name, worker_id)

    def list_queues(self) -> list[dict]:
        return [{"queue_name": "emails", "created_at": 100.0}]

    def metrics(self, queue_name: str) -> dict:
        return {
            "queue_name": queue_name,
            "visible_messages": 1,
            "leased_messages": 2,
            "delayed_messages": 3,
            "archived_messages": 4,
            "total_messages": 10,
        }

    def metrics_all(self) -> list[dict]:
        return [self.metrics("emails")]

    def get_message(self, queue_name: str, message_id: int) -> dict:
        return {
            "message_id": message_id,
            "read_count": 0,
            "enqueued_at": 40.0,
            "visible_at": 40.0,
            "payload_json": '{"kind":"inspect"}',
            "receipt_handle": None,
            "state": "visible",
        }

    def list_messages(
        self,
        queue_name: str,
        state: str | None = None,
        limit: int | None = None,
        cursor: str | None = None,
    ) -> dict:
        _ = (queue_name, state, limit, cursor)
        return {
            "messages": [
                {
                    "message_id": 1,
                    "read_count": 0,
                    "enqueued_at": 50.0,
                    "visible_at": 50.0,
                    "payload_json": '{"kind":"page"}',
                    "receipt_handle": None,
                    "state": "visible",
                }
            ],
            "next_cursor": "1",
        }

    def list_archived_messages(
        self,
        queue_name: str,
        limit: int | None = None,
        cursor: str | None = None,
    ) -> dict:
        _ = (queue_name, limit, cursor)
        return {
            "messages": [
                {
                    "message_id": 2,
                    "read_count": 1,
                    "enqueued_at": 60.0,
                    "visible_at": 60.0,
                    "payload_json": '{"kind":"archived"}',
                    "receipt_handle": None,
                    "state": "archived",
                }
            ],
            "next_cursor": None,
        }


class FakeNativeModule:
    ClientCore = FakeClientCore


class S3QPythonSdkTests(unittest.TestCase):
    def test_connect_requires_native_module(self) -> None:
        with mock.patch.object(client_module, "_native", None):
            with self.assertRaises(s3q.NotReadyError):
                s3q.connect("s3://bucket/queue.db")

    def test_client_queue_and_producer_send_are_wired(self) -> None:
        with mock.patch.object(client_module, "_native", FakeNativeModule):
            client = s3q.connect("s3://bucket/queue.db")
            queue = client.create_queue("emails")
            producer = queue.producer("api-worker")
            message = producer.send({"kind": "welcome"}, delay=timedelta(seconds=5))

            self.assertEqual(queue.name, "emails")
            self.assertEqual(producer.worker_id, "api-worker")
            self.assertEqual(message.payload, {"kind": "welcome"})
            self.assertEqual(message.state, s3q.MessageState.VISIBLE)
            self.assertIsNone(message.receipt_handle)
            self.assertIsInstance(message.enqueued_at, datetime)

    def test_consumer_methods_convert_results_and_errors(self) -> None:
        with mock.patch.object(client_module, "_native", FakeNativeModule):
            consumer = s3q.connect("s3://bucket/queue.db").queue("emails").consumer("worker-a")

            message = consumer.read(vt=timedelta(seconds=30))
            batch = consumer.read_batch(vt=timedelta(seconds=30), qty=2)
            polled = consumer.read_with_poll(
                vt=timedelta(seconds=30),
                qty=2,
                poll_timeout=timedelta(seconds=1),
                poll_interval=timedelta(milliseconds=250),
            )
            archived = consumer.archive_message(s3q.ReceiptHandle("s3q1:10:99"))
            self.assertTrue(
                consumer.archive_messages(
                    [s3q.ReceiptHandle("s3q1:10:99"), s3q.ReceiptHandle("s3q1:11:99")]
                )
            )
            self.assertTrue(consumer.set_vt(s3q.ReceiptHandle("s3q1:10:99"), timedelta(seconds=30)))

            self.assertEqual(message.payload, {"kind": "one"})
            self.assertEqual(len(batch), 2)
            self.assertEqual(len(polled), 2)
            self.assertEqual(archived.state, s3q.MessageState.ARCHIVED)

            with self.assertRaises(s3q.OwnershipMismatchError):
                consumer.delete_message(s3q.ReceiptHandle("s3q1:10:99"))

    def test_inspection_methods_convert_native_results(self) -> None:
        with mock.patch.object(client_module, "_native", FakeNativeModule):
            inspect = s3q.connect("s3://bucket/queue.db").inspect()

            queues = inspect.list_queues()
            metrics = inspect.metrics("emails")
            all_metrics = inspect.metrics_all()
            message = inspect.get_message("emails", 42)
            page = (
                inspect.list_messages("emails")
                .with_state(s3q.MessageState.VISIBLE)
                .with_limit(1)
                .with_cursor("0")
                .execute()
            )
            archived = inspect.list_archived_messages("emails").execute()

            self.assertEqual(queues[0].queue_name, "emails")
            self.assertEqual(metrics.total_messages, 10)
            self.assertEqual(all_metrics[0].queue_name, "emails")
            self.assertEqual(message.message_id, 42)
            self.assertEqual(page.next_cursor, "1")
            self.assertEqual(archived.messages[0].state, s3q.MessageState.ARCHIVED)


if __name__ == "__main__":
    unittest.main()
