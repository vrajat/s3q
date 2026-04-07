use std::sync::Arc;

use crate::{
    config::ClientConfig, inspect::Inspect, pgqrs_adapter::PgqrsAdapter, queue::QueueHandle, Result,
};

#[derive(Debug, Clone)]
/// Top-level s3q client.
///
/// A client owns the S3-backed queue connection and creates queue and
/// inspection handles.
pub struct Client {
    config: ClientConfig,
    adapter: Arc<PgqrsAdapter>,
}

impl Client {
    /// Connect to s3q using an S3 DSN.
    ///
    /// The DSN should point at the queue database object, for example
    /// `s3://my-bucket/queues/app.db`.
    pub async fn connect(dsn: impl Into<String>) -> Result<Self> {
        Self::connect_with_config(ClientConfig::new(dsn)).await
    }

    /// Connect to s3q using an explicit client configuration.
    pub async fn connect_with_config(config: ClientConfig) -> Result<Self> {
        let adapter = PgqrsAdapter::connect(&config).await?;
        Ok(Self { config, adapter })
    }

    /// Return the configuration used by this client.
    pub fn config(&self) -> &ClientConfig {
        &self.config
    }

    /// Return a handle for a queue name.
    ///
    /// This does not create the queue. Call [`QueueHandle::create_queue`] when
    /// provisioning a new queue.
    pub fn queue(&self, name: impl Into<String>) -> QueueHandle {
        QueueHandle::new(self.adapter.clone(), name, self.config.namespace.clone())
    }

    /// Return a read-only inspection handle.
    pub fn inspect(&self) -> Inspect<'_> {
        Inspect::new(self)
    }
}

/// Connect to s3q using an S3 DSN.
pub async fn connect(dsn: impl Into<String>) -> Result<Client> {
    Client::connect(dsn).await
}
