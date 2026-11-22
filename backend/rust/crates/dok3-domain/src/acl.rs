//! Access-control scopes and principal derivation (default-deny).

use std::fmt;
use std::str::FromStr;

use dok3_core::Dok3Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Security scope attached to indexed documents and original blobs.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AclScope {
    Public,
    Internal(String),
}

impl AclScope {
    /// Constructs an `Internal` scope; panics if `name` is empty or whitespace.
    #[must_use]
    pub fn internal(name: impl Into<String>) -> Self {
        let name = name.into();
        let trimmed = name.trim();
        assert!(
            !trimmed.is_empty() && !trimmed.chars().any(char::is_whitespace),
            "internal ACL scope name must be non-empty without whitespace"
        );
        Self::Internal(trimmed.to_string())
    }

    /// Attempts to construct an `Internal` scope, rejecting empty or whitespace values.
    pub fn try_internal(name: impl Into<String>) -> Result<Self, Dok3Error> {
        let name = name.into();
        let trimmed = name.trim();
        if trimmed.is_empty() || trimmed.chars().any(char::is_whitespace) {
            return Err(Dok3Error::InvalidInput(
                "internal ACL scope name must be non-empty without whitespace".to_string(),
            ));
        }
        Ok(Self::Internal(trimmed.to_string()))
    }
}

impl fmt::Display for AclScope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Public => f.write_str("public"),
            Self::Internal(scope) => write!(f, "internal:{scope}"),
        }
    }
}

impl FromStr for AclScope {
    type Err = Dok3Error;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        let trimmed = raw.trim();
        if trimmed == "public" {
            return Ok(Self::Public);
        }
        if let Some(rest) = trimmed.strip_prefix("internal:") {
            if rest.is_empty() || rest.chars().any(char::is_whitespace) {
                return Err(Dok3Error::InvalidInput(format!(
                    "invalid internal ACL scope '{raw}'"
                )));
            }
            return Ok(Self::Internal(rest.to_string()));
        }
        Err(Dok3Error::InvalidInput(format!(
            "unsupported ACL scope '{raw}' (expected 'public' or 'internal:<name>')"
        )))
    }
}

impl Serialize for AclScope {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for AclScope {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let raw = String::deserialize(deserializer)?;
        Self::from_str(&raw).map_err(serde::de::Error::custom)
    }
}

/// Authenticated caller identity and server-derived allowed scopes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Principal {
    pub subject: String,
    pub allowed_scopes: Vec<AclScope>,
    pub is_admin: bool,
}

impl Principal {
    /// Returns the least-privilege anonymous public caller.
    #[must_use]
    pub fn anonymous_public() -> Self {
        Self {
            subject: "anonymous".to_string(),
            allowed_scopes: vec![AclScope::Public],
            is_admin: false,
        }
    }

    /// Returns a local loopback administrator principal (`is_admin: true`).
    #[must_use]
    pub fn local_admin() -> Self {
        Self {
            subject: "local-admin".to_string(),
            allowed_scopes: vec![AclScope::Public],
            is_admin: true,
        }
    }

    /// Default-deny authorization check: a scope is allowed only if explicitly granted.
    #[must_use]
    pub fn allows(&self, scope: &AclScope) -> bool {
        self.allowed_scopes.contains(scope)
    }

    /// Derives effective query scopes. A requested scope may narrow access but never escalate it.
    #[must_use]
    pub fn effective_scopes(&self, requested: Option<&AclScope>) -> Vec<AclScope> {
        match requested {
            Some(scope) if self.allows(scope) => vec![scope.clone()],
            Some(_) => Vec::new(),
            None => self.allowed_scopes.clone(),
        }
    }
}
