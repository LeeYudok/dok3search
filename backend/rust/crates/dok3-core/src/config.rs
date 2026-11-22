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
    pub source_dir: Option<PathBuf>,
    pub data_dir: Option<PathBuf>,
    pub refresh_interval_secs: Option<u64>,
    pub admin_enabled: bool,
    /// Explicit operator opt-in that allows a non-loopback `DOK3_BIND` while
    /// `DOK3_SOURCE_DIR` is set (for container deployments behind an
    /// authenticating reverse proxy). Defaults to false so the loopback guard
    /// stays in force unless it is deliberately disabled.
    pub allow_non_loopback_bind: bool,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            bind: "127.0.0.1".to_string(),
            port: 8080,
            static_dir: None,
            design_docs_dir: None,
            source_dir: None,
            data_dir: None,
            refresh_interval_secs: None,
            admin_enabled: false,
            allow_non_loopback_bind: false,
        }
    }
}

impl ServerConfig {
    /// Loads server configuration from `DOK3_BIND`, `DOK3_PORT`, `DOK3_STATIC_DIR`, `DOK3_DESIGN_DIR`,
    /// `DOK3_SOURCE_DIR`, `DOK3_DATA_DIR`, `DOK3_REFRESH_INTERVAL_SECS`, `DOK3_ENABLE_ADMIN`,
    /// and `DOK3_ALLOW_NON_LOOPBACK_BIND`.
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

        config.source_dir = non_empty_path(lookup("DOK3_SOURCE_DIR"));
        config.data_dir = non_empty_path(lookup("DOK3_DATA_DIR"));

        if let Some(interval_raw) = lookup("DOK3_REFRESH_INTERVAL_SECS") {
            let trimmed = interval_raw.trim();
            let interval = trimmed.parse::<u64>().map_err(|_| {
                Dok3Error::InvalidInput(format!(
                    "invalid DOK3_REFRESH_INTERVAL_SECS value '{trimmed}'"
                ))
            })?;
            if interval == 0 {
                return Err(Dok3Error::InvalidInput(
                    "DOK3_REFRESH_INTERVAL_SECS must be at least 1".to_string(),
                ));
            }
            config.refresh_interval_secs = Some(interval);
        }

        if let Some(admin_raw) = lookup("DOK3_ENABLE_ADMIN") {
            config.admin_enabled = parse_bool_flag("DOK3_ENABLE_ADMIN", &admin_raw)?;
        }

        if let Some(allow_raw) = lookup("DOK3_ALLOW_NON_LOOPBACK_BIND") {
            config.allow_non_loopback_bind =
                parse_bool_flag("DOK3_ALLOW_NON_LOOPBACK_BIND", &allow_raw)?;
        }

        Ok(config)
    }
}

fn parse_bool_flag(name: &str, raw: &str) -> Result<bool, Dok3Error> {
    let trimmed = raw.trim();
    match trimmed.to_ascii_lowercase().as_str() {
        "1" | "true" => Ok(true),
        "0" | "false" => Ok(false),
        _ => Err(Dok3Error::InvalidInput(format!(
            "invalid {name} value '{trimmed}' (expected true/false/1/0)"
        ))),
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
        assert_eq!(cfg.source_dir, None);
        assert_eq!(cfg.data_dir, None);
        assert_eq!(cfg.refresh_interval_secs, None);
        assert!(!cfg.admin_enabled);
        assert!(!cfg.allow_non_loopback_bind);
    }

    #[test]
    fn parses_valid_env_overrides() {
        let map = HashMap::from([
            ("DOK3_BIND", "0.0.0.0"),
            ("DOK3_PORT", "18080"),
            ("DOK3_STATIC_DIR", " frontend/dist "),
            ("DOK3_DESIGN_DIR", ""),
            ("DOK3_DATA_DIR", " /var/lib/dok3 "),
            ("DOK3_REFRESH_INTERVAL_SECS", " 30 "),
            ("DOK3_ENABLE_ADMIN", "true"),
            ("DOK3_ALLOW_NON_LOOPBACK_BIND", "true"),
        ]);
        let cfg = ServerConfig::from_lookup(|k| map.get(k).map(|v| (*v).to_string()))
            .expect("valid overrides should parse");
        assert_eq!(cfg.bind, "0.0.0.0");
        assert_eq!(cfg.port, 18080);
        assert_eq!(cfg.static_dir, Some(PathBuf::from("frontend/dist")));
        assert_eq!(cfg.design_docs_dir, None);
        assert_eq!(cfg.data_dir, Some(PathBuf::from("/var/lib/dok3")));
        assert_eq!(cfg.refresh_interval_secs, Some(30));
        assert!(cfg.admin_enabled);
        assert!(cfg.allow_non_loopback_bind);

        let flag_one = ServerConfig::from_lookup(|k| match k {
            "DOK3_ENABLE_ADMIN" => Some("1".to_string()),
            _ => None,
        })
        .expect("1 should parse as true");
        assert!(flag_one.admin_enabled);

        let flag_false = ServerConfig::from_lookup(|k| match k {
            "DOK3_ENABLE_ADMIN" => Some("FALSE".to_string()),
            _ => None,
        })
        .expect("FALSE should parse as false");
        assert!(!flag_false.admin_enabled);

        let allow_false = ServerConfig::from_lookup(|k| match k {
            "DOK3_ALLOW_NON_LOOPBACK_BIND" => Some("0".to_string()),
            _ => None,
        })
        .expect("0 should parse as false");
        assert!(!allow_false.allow_non_loopback_bind);
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

        for bad_interval in ["0", "-5", "abc", ""] {
            let err = ServerConfig::from_lookup(|k| match k {
                "DOK3_REFRESH_INTERVAL_SECS" => Some(bad_interval.to_string()),
                _ => None,
            });
            assert!(
                matches!(err, Err(Dok3Error::InvalidInput(_))),
                "expected invalid interval error for '{bad_interval}'"
            );
        }

        for bad_admin in ["yes", "2", ""] {
            let err = ServerConfig::from_lookup(|k| match k {
                "DOK3_ENABLE_ADMIN" => Some(bad_admin.to_string()),
                _ => None,
            });
            assert!(
                matches!(err, Err(Dok3Error::InvalidInput(_))),
                "expected invalid admin flag error for '{bad_admin}'"
            );
        }

        for bad_allow in ["yes", "2", ""] {
            let err = ServerConfig::from_lookup(|k| match k {
                "DOK3_ALLOW_NON_LOOPBACK_BIND" => Some(bad_allow.to_string()),
                _ => None,
            });
            assert!(
                matches!(err, Err(Dok3Error::InvalidInput(_))),
                "expected invalid allow-non-loopback flag error for '{bad_allow}'"
            );
        }
    }
}
