#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClientConfig {
    pub dsn: String,
    pub namespace: String,
    pub local_cache_dir: Option<String>,
    pub service_name: String,
}

impl ClientConfig {
    pub fn new(dsn: impl Into<String>) -> Self {
        Self {
            dsn: dsn.into(),
            namespace: "default".to_string(),
            local_cache_dir: None,
            service_name: "s3q".to_string(),
        }
    }

    pub fn with_namespace(mut self, namespace: impl Into<String>) -> Self {
        self.namespace = namespace.into();
        self
    }

    pub fn with_local_cache_dir(mut self, path: impl Into<String>) -> Self {
        self.local_cache_dir = Some(path.into());
        self
    }

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
