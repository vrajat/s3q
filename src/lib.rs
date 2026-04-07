//! s3q is a thin S3-backed queue product layer over `pgqrs::store::s3::S3Store`.
//!
//! This crate currently wires the queue mutation API to `pgqrs`. Inspection,
//! polling, Python, and CLI surfaces are staged separately.

mod client;
mod config;
mod error;
mod inspect;
mod pgqrs;
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
pub use types::{
    ArchivedMessage, ConsumerInfo, Message, MessageState, ProducerInfo, QueueInfo, QueueMetrics,
    ReceiptHandle,
};
