# User Guide

s3q has two product surfaces:

1. **Queues**
2. **Workflows**

They share infrastructure and storage, but they should remain conceptually separate.

## Queue Surface

Queues should feel familiar to teams that already understand SQS:

- `send_message`
- `send_message_batch`
- `receive_messages`
- `delete_message`
- `change_message_visibility`

The core semantic is **lease then ack**, not destructive pop.

## Workflow Surface

Workflows should feel familiar to teams that already understand Temporal:

- `start_workflow`
- `describe_workflow`
- `signal`
- `query`
- `result`
- `cancel`
- `terminate`

The core semantic is **durable execution over persisted state**.
