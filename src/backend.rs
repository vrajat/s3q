use std::sync::Arc;

use pgqrs::store::{s3::S3Store, Store};

use crate::{ClientConfig, Error, Result};

#[derive(Debug)]
pub(crate) struct Backend {
    store: S3Store,
    admin: pgqrs::Admin,
}

impl Backend {
    pub(crate) async fn connect(config: &ClientConfig) -> Result<Arc<Self>> {
        if !config.dsn.starts_with("s3://") {
            return Err(Error::InvalidArgument(
                "s3q v1 only supports s3:// DSNs".to_string(),
            ));
        }

        let pgqrs_config = pgqrs::Config::from_dsn_with_schema(&config.dsn, &config.namespace)?;
        let store = S3Store::new(&pgqrs_config).await?;
        store.bootstrap().await?;

        let admin_worker_name = format!("{}-admin", config.service_name);
        let admin = store.admin(&admin_worker_name, &pgqrs_config).await?;

        Ok(Arc::new(Self { store, admin }))
    }

    pub(crate) async fn create_queue(&self, queue_name: &str) -> Result<()> {
        self.store.queue(queue_name).await?;
        Ok(())
    }

    pub(crate) async fn delete_queue(&self, queue_name: &str) -> Result<()> {
        let queue = self.store.queues().get_by_name(queue_name).await?;
        self.admin.delete_queue(&queue).await?;
        Ok(())
    }

    pub(crate) async fn purge_queue(&self, queue_name: &str) -> Result<()> {
        self.admin.purge_queue(queue_name).await?;
        Ok(())
    }

    pub(crate) async fn producer(
        &self,
        queue_name: &str,
        worker_id: &str,
    ) -> Result<pgqrs::Producer> {
        Ok(pgqrs::producer(worker_id, queue_name)
            .create(&self.store)
            .await?)
    }

    pub(crate) async fn consumer(
        &self,
        queue_name: &str,
        worker_id: &str,
    ) -> Result<pgqrs::Consumer> {
        Ok(pgqrs::consumer(worker_id, queue_name)
            .create(&self.store)
            .await?)
    }
}
