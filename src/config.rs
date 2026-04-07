#[derive(Debug, Clone, PartialEq, Eq)]
/// Configuration for an s3q client.
pub struct ClientConfig {
    /// S3 DSN for the queue database object.
    pub dsn: String,
    /// Logical namespace used for the queue schema.
    pub namespace: String,
    /// Optional local cache directory for backends that support local cache.
    pub local_cache_dir: Option<String>,
    /// Service name used for managed internal worker identity.
    pub service_name: String,
}

impl ClientConfig {
    /// Create a client configuration with default namespace and service name.
    pub fn new(dsn: impl Into<String>) -> Self {
        Self {
            dsn: dsn.into(),
            namespace: "default".to_string(),
            local_cache_dir: None,
            service_name: "s3q".to_string(),
        }
    }

    /// Set the logical namespace.
    pub fn with_namespace(mut self, namespace: impl Into<String>) -> Self {
        self.namespace = namespace.into();
        self
    }

    /// Set the local cache directory.
    pub fn with_local_cache_dir(mut self, path: impl Into<String>) -> Self {
        self.local_cache_dir = Some(path.into());
        self
    }

    /// Set the service name used for managed worker records.
    pub fn with_service_name(mut self, service_name: impl Into<String>) -> Self {
        self.service_name = service_name.into();
        self
    }
}

#[cfg(test)]
mod tests {
    use super::ClientConfig;

    #[test]
    fn client_config_defaults_are_stable() {
        let config = ClientConfig::new("s3://bucket/queue.db");

        assert_eq!(config.namespace, "default");
        assert_eq!(config.service_name, "s3q");
        assert!(config.local_cache_dir.is_none());
    }
}
