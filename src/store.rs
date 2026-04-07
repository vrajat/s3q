use std::hash::{DefaultHasher, Hash, Hasher};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use pgqrs::store::{s3::S3Store, Store};

use crate::{ClientConfig, Error, Result};

static NEXT_CACHE_ID: AtomicU64 = AtomicU64::new(1);

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

        let mut pgqrs_config = pgqrs::Config::from_dsn_with_schema(&config.dsn, &config.namespace)?;
        pgqrs_config.s3.cache_id = s3_cache_id(config);
        let store = S3Store::new(&pgqrs_config).await?;
        store.bootstrap().await?;

        let admin_worker_name = format!("{}-admin", config.service_name);
        let admin = store.admin(&admin_worker_name, &pgqrs_config).await?;

        Ok(Arc::new(Self { s3: store, admin }))
    }
}

fn s3_cache_id(config: &ClientConfig) -> String {
    let mut hasher = DefaultHasher::new();
    config.dsn.hash(&mut hasher);
    config.namespace.hash(&mut hasher);
    config.service_name.hash(&mut hasher);

    let fingerprint = hasher.finish();
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis())
        .unwrap_or(0);
    let sequence = NEXT_CACHE_ID.fetch_add(1, Ordering::Relaxed);

    format!(
        "s3q-{}-{timestamp}-{sequence}-{fingerprint:x}",
        std::process::id()
    )
}

#[cfg(test)]
mod tests {
    use super::s3_cache_id;
    use crate::ClientConfig;

    #[test]
    fn s3_cache_ids_are_unique_per_connection() {
        let config = ClientConfig::new("s3://bucket/queue.sqlite");

        assert_ne!(s3_cache_id(&config), s3_cache_id(&config));
    }
}
