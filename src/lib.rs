//! s3q is a small S3-backed queue library.
//!
//! The public API is built around queues, producers, consumers, leases, and
//! read-only inspection.

mod client;
mod config;
mod error;
mod inspect;
mod pgqrs_adapter;
mod queue;
mod types;

pub use client::{connect, Client};
pub use config::ClientConfig;
pub use error::{Error, Result};
pub use inspect::{
    GetMessageRequest, Inspect, ListArchivedMessagesRequest, ListMessagesRequest,
    ListQueuesRequest, MetricsAllRequest, MetricsRequest,
};
pub use queue::{Consumer, Producer, QueueHandle};
pub use types::{ArchivedMessage, Message, MessageState, QueueInfo, QueueMetrics, ReceiptHandle};
