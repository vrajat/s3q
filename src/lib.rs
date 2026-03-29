//! s3q is an S3-backed queue and durable workflow library.
//!
//! The intended implementation builds on top of `pgqrs` and, specifically,
//! `pgqrs::store::s3::S3Store`. This crate currently exposes the initial API
//! vocabulary and request/response types for the first implementation phase.

mod client;
mod config;
mod error;
mod queue;
mod workflow;

pub use client::{connect, Client};
pub use config::ClientConfig;
pub use error::{Error, Result};
pub use queue::{
    ChangeMessageVisibilityRequest, CreateQueueRequest, DeleteMessageRequest, DeleteQueueRequest,
    GetQueueAttributesRequest, PurgeQueueRequest, QueueApi, QueueAttributes, QueueHandle,
    ReceiveMessagesRequest, ReceivedMessage, SendMessageBatchRequest, SendMessageRequest,
    SetQueueAttributesRequest,
};
pub use workflow::{
    CancelWorkflowRequest, ChildWorkflowRequest, DescribeWorkflowRequest, ListWorkflowsRequest,
    QueryWorkflowRequest, ResultWorkflowRequest, SignalWorkflowRequest, StartWorkflowRequest,
    TerminateWorkflowRequest, TimerSpec, WorkflowApi, WorkflowExecution, WorkflowHandle,
    WorkflowIdReusePolicy, WorkflowStatus,
};
