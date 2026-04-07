use std::sync::Arc;

use crate::{
    config::ClientConfig, inspect::Inspect, pgqrs::PgqrsAdapter, queue::QueueHandle, Result,
};

#[derive(Debug, Clone)]
pub struct Client {
    config: ClientConfig,
    adapter: Arc<PgqrsAdapter>,
}

impl Client {
    pub async fn connect(dsn: impl Into<String>) -> Result<Self> {
        Self::connect_with_config(ClientConfig::new(dsn)).await
    }

    pub async fn connect_with_config(config: ClientConfig) -> Result<Self> {
        let adapter = PgqrsAdapter::connect(&config).await?;
        Ok(Self { config, adapter })
    }

    pub fn config(&self) -> &ClientConfig {
        &self.config
    }

    pub(crate) fn adapter(&self) -> &PgqrsAdapter {
        &self.adapter
    }

    pub fn queue(&self, name: impl Into<String>) -> QueueHandle<'_> {
        QueueHandle::new(self, name)
    }

    pub fn inspect(&self) -> Inspect<'_> {
        Inspect::new(self)
    }
}

pub async fn connect(dsn: impl Into<String>) -> Result<Client> {
    Client::connect(dsn).await
}
