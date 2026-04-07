//! s3q is a thin S3-backed queue product layer over `pgqrs::store::s3::S3Store`.
//!
//! This crate currently exposes the queue-only v1 API vocabulary and
//! request/response types. The next implementation phase wires these handles to
//! `pgqrs`.

mod client;
mod config;
mod error;
mod inspect;
mod queue;
mod types;

pub use client::{connect, Client};
pub use config::ClientConfig;
pub use error::{Error, Result};
pub use inspect::{
    GetMessageRequest, Inspect, ListArchivedMessagesRequest, ListMessagesRequest,
    ListQueuesRequest, MetricsAllRequest, MetricsRequest,
};
pub use queue::{
    ArchiveMessageRequest, ArchiveMessagesRequest, Consumer, CreateQueueRequest,
    DeleteMessageRequest, DeleteQueueRequest, Producer, PurgeQueueRequest, QueueHandle,
    ReadBatchRequest, ReadRequest, ReadWithPollRequest, SendBatchRequest, SendRequest,
    SetVisibilityTimeoutRequest,
};
pub use types::{
    ArchivedMessage, ConsumerInfo, Message, MessageState, ProducerInfo, QueueInfo, QueueMetrics,
    ReceiptHandle,
};
