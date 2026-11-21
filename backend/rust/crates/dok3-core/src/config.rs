//! Runtime configuration loaded from environment variables.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::error::Dok3Error;

/// Configuration for the dok3search HTTP and static asset server.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServerConfig {
    pub bind: String,
    pub port: u16,
    pub static_dir: Option<PathBuf>,
    pub design_docs_dir: Option<PathBuf>,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            bind: "127.0.0.1".to_string(),
            port: 8080,
            static_dir: None,
            design_docs_dir: None,
        }
    }
}

impl ServerConfig {
    /// Loads server configuration from `DOK3_BIND`, `DOK3_PORT`, `DOK3_STATIC_DIR`, and `DOK3_DESIGN_DIR`.
    pub fn from_env() -> Result<Self, Dok3Error> {
        Self::from_lookup(|key| std::env::var(key).ok())
    }

    /// Loads server configuration from an arbitrary key lookup function.
    pub fn from_lookup<F>(mut lookup: F) -> Result<Self, Dok3Error>
    where
        F: FnMut(&str) -> Option<String>,
    {
        let mut config = Self::default();

        if let Some(bind) = lookup("DOK3_BIND") {
            let trimmed = bind.trim();
            if trimmed.is_empty() {
                return Err(Dok3Error::InvalidInput(
                    "DOK3_BIND cannot be empty".to_string(),
                ));
            }
            config.bind = trimmed.to_string();
        }

        if let Some(port_raw) = lookup("DOK3_PORT") {
            let trimmed = port_raw.trim();
            let port = trimmed.parse::<u16>().map_err(|_| {
                Dok3Error::InvalidInput(format!("invalid DOK3_PORT value '{trimmed}'"))
            })?;
            if port == 0 {
                return Err(Dok3Error::InvalidInput(
                    "DOK3_PORT must be between 1 and 65535".to_string(),
                ));
            }
            config.port = port;
        }

        config.static_dir = non_empty_path(lookup("DOK3_STATIC_DIR"));
        config.design_docs_dir = non_empty_path(lookup("DOK3_DESIGN_DIR"));

        Ok(config)
    }
}

fn non_empty_path(value: Option<String>) -> Option<PathBuf> {
    value
        .map(|raw| raw.trim().to_string())
        .filter(|trimmed| !trimmed.is_empty())
        .map(PathBuf::from)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::path::PathBuf;

    use super::ServerConfig;
    use crate::error::Dok3Error;

    #[test]
    fn defaults_bind_to_loopback_8080() {
        let cfg = ServerConfig::from_lookup(|_| None).expect("default config should succeed");
        assert_eq!(cfg.bind, "127.0.0.1");
        assert_eq!(cfg.port, 8080);
        assert_eq!(cfg.static_dir, None);
        assert_eq!(cfg.design_docs_dir, None);
    }

    #[test]
    fn parses_valid_env_overrides() {
        let map = HashMap::from([
            ("DOK3_BIND", "0.0.0.0"),
            ("DOK3_PORT", "18080"),
            ("DOK3_STATIC_DIR", " frontend/dist "),
            ("DOK3_DESIGN_DIR", ""),
        ]);
        let cfg = ServerConfig::from_lookup(|k| map.get(k).map(|v| (*v).to_string()))
            .expect("valid overrides should parse");
        assert_eq!(cfg.bind, "0.0.0.0");
        assert_eq!(cfg.port, 18080);
        assert_eq!(cfg.static_dir, Some(PathBuf::from("frontend/dist")));
        assert_eq!(cfg.design_docs_dir, None);
    }

    #[test]
    fn rejects_invalid_port_or_empty_bind() {
        let bad_port = ServerConfig::from_lookup(|k| match k {
            "DOK3_PORT" => Some("abc".to_string()),
            _ => None,
        });
        assert!(matches!(bad_port, Err(Dok3Error::InvalidInput(_))));

        let zero_port = ServerConfig::from_lookup(|k| match k {
            "DOK3_PORT" => Some("0".to_string()),
            _ => None,
        });
        assert!(matches!(zero_port, Err(Dok3Error::InvalidInput(_))));

        let empty_bind = ServerConfig::from_lookup(|k| match k {
            "DOK3_BIND" => Some("   ".to_string()),
            _ => None,
        });
        assert!(matches!(empty_bind, Err(Dok3Error::InvalidInput(_))));
    }
}
