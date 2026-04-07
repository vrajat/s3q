use std::sync::Arc;

use pgqrs::store::{s3::S3Store, Store};

use crate::{ClientConfig, Error, Result};

#[derive(Debug)]
pub(crate) struct StoreState {
    pub(crate) s3: S3Store,
    pub(crate) admin: pgqrs::Admin,
}

impl StoreState {
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

        Ok(Arc::new(Self { s3: store, admin }))
    }
}
