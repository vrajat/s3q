use std::sync::Arc;

use pgqrs::store::Store as _;

use crate::{config::ClientConfig, inspect::Inspect, queue::Queue, store::StoreState, Result};

#[derive(Debug, Clone)]
/// Top-level s3q client.
///
/// A client owns the S3-backed queue connection and creates queue and
/// inspection handles.
pub struct Client {
    config: ClientConfig,
    store: Arc<StoreState>,
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
        let store = StoreState::connect(&config).await?;
        Ok(Self { config, store })
    }

    /// Return the configuration used by this client.
    pub fn config(&self) -> &ClientConfig {
        &self.config
    }

    /// Create a queue and return a queue-scoped handle for it.
    pub async fn create_queue(&self, name: impl Into<String>) -> Result<Queue> {
        let name = name.into();
        self.store.s3.queue(&name).await?;
        Ok(self.queue(name))
    }

    /// Delete a queue.
    ///
    /// Deletion fails if the backing queue still has messages or associated
    /// workers that prevent safe deletion.
    pub async fn delete_queue(&self, name: impl AsRef<str>) -> Result<()> {
        let queue = self.store.s3.queues().get_by_name(name.as_ref()).await?;
        Ok(self.store.admin.delete_queue(&queue).await?)
    }

    /// Purge messages from a queue while keeping the queue itself.
    pub async fn purge_queue(&self, name: impl AsRef<str>) -> Result<()> {
        Ok(self.store.admin.purge_queue(name.as_ref()).await?)
    }

    /// Return a queue handle for a queue name.
    ///
    /// This does not create the queue. Call [`Client::create_queue`] when
    /// provisioning queues.
    pub fn queue(&self, name: impl Into<String>) -> Queue {
        Queue::new(self.store.clone(), name, self.config.namespace.clone())
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
