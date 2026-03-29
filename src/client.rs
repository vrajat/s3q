use crate::{config::ClientConfig, queue::QueueApi, workflow::WorkflowApi};

#[derive(Debug, Clone)]
pub struct Client {
    config: ClientConfig,
}

impl Client {
    pub fn new(config: ClientConfig) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &ClientConfig {
        &self.config
    }

    pub fn queues(&self) -> QueueApi<'_> {
        QueueApi::new(self)
    }

    pub fn workflows(&self) -> WorkflowApi<'_> {
        WorkflowApi::new(self)
    }
}

pub fn connect(dsn: impl Into<String>) -> Client {
    Client::new(ClientConfig::new(dsn))
}
