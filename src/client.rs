use crate::{config::ClientConfig, inspect::Inspect, queue::QueueHandle};

#[derive(Debug, Clone)]
pub struct Client {
    config: ClientConfig,
}

impl Client {
    pub fn new(config: ClientConfig) -> Self {
        Self { config }
    }

    pub fn connect(dsn: impl Into<String>) -> Self {
        Self::new(ClientConfig::new(dsn))
    }

    pub fn config(&self) -> &ClientConfig {
        &self.config
    }

    pub fn queue(&self, name: impl Into<String>) -> QueueHandle<'_> {
        QueueHandle::new(self, name)
    }

    pub fn inspect(&self) -> Inspect<'_> {
        Inspect::new(self)
    }
}

pub fn connect(dsn: impl Into<String>) -> Client {
    Client::connect(dsn)
}
