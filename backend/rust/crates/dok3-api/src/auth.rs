//! Request-scoped authentication and group-to-ACL principal resolution.

use axum::Json;
use axum::http::{HeaderMap, StatusCode, header};
use axum::response::{IntoResponse, Response};
use dok3_core::Dok3Error;
use dok3_domain::{AclScope, Principal};
use serde::Serialize;

/// Supported authentication modes (`DOK3_AUTH_MODE`).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum AuthMode {
    #[default]
    Public,
    TrustedHeader,
    Bearer,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BearerTokenEntry {
    pub token: String,
    pub subject: String,
    pub allowed_scopes: Vec<AclScope>,
    pub is_admin: bool,
}

/// Runtime authentication configuration attached to [`crate::ApiState`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthConfig {
    pub mode: AuthMode,
    pub user_header: String,
    pub groups_header: String,
    pub tokens: Vec<BearerTokenEntry>,
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            mode: AuthMode::Public,
            user_header: "X-Auth-User".to_string(),
            groups_header: "X-Auth-Groups".to_string(),
            tokens: Vec::new(),
        }
    }
}

impl AuthConfig {
    /// Builds [`AuthConfig`] from `ServerConfig` strings.
    pub fn from_parts(
        mode_raw: &str,
        user_header: &str,
        groups_header: &str,
        tokens_raw: Option<&str>,
    ) -> Result<Self, Dok3Error> {
        let mode = match mode_raw.trim().to_ascii_lowercase().as_str() {
            "" | "public" => AuthMode::Public,
            "trusted_header" => AuthMode::TrustedHeader,
            "bearer" => AuthMode::Bearer,
            other => {
                return Err(Dok3Error::InvalidInput(format!(
                    "unsupported auth mode '{other}'"
                )));
            }
        };
        let mut tokens = Vec::new();
        if let Some(raw) = tokens_raw {
            for entry in raw.split(';').map(str::trim).filter(|s| !s.is_empty()) {
                let parts: Vec<&str> = entry.split(':').collect();
                if parts.len() < 3 || parts.len() > 4 {
                    return Err(Dok3Error::InvalidInput(
                        "invalid DOK3_AUTH_TOKENS entry (expected token:subject:scope1,scope2[:admin])"
                            .to_string(),
                    ));
                }
                let token = parts[0].trim();
                let subject = parts[1].trim();
                if token.is_empty() || subject.is_empty() {
                    return Err(Dok3Error::InvalidInput(
                        "DOK3_AUTH_TOKENS token and subject must be non-empty".to_string(),
                    ));
                }
                let mut scopes = vec![AclScope::Public];
                for raw_scope in parts[2].split(',').map(str::trim).filter(|s| !s.is_empty()) {
                    if raw_scope == "public" {
                        continue;
                    }
                    let scope = if raw_scope.starts_with("internal:") {
                        raw_scope.parse::<AclScope>()?
                    } else {
                        AclScope::try_internal(raw_scope)?
                    };
                    if !scopes.contains(&scope) {
                        scopes.push(scope);
                    }
                }
                let is_admin = parts
                    .get(3)
                    .is_some_and(|flag| flag.trim().eq_ignore_ascii_case("admin"));
                tokens.push(BearerTokenEntry {
                    token: token.to_string(),
                    subject: subject.to_string(),
                    allowed_scopes: scopes,
                    is_admin,
                });
            }
        }
        Ok(Self {
            mode,
            user_header: if user_header.trim().is_empty() {
                "X-Auth-User".to_string()
            } else {
                user_header.trim().to_string()
            },
            groups_header: if groups_header.trim().is_empty() {
                "X-Auth-Groups".to_string()
            } else {
                groups_header.trim().to_string()
            },
            tokens,
        })
    }

    /// Resolves the effective [`Principal`] from request headers.
    pub fn resolve_principal(
        &self,
        headers: &HeaderMap,
        default_principal: &Principal,
    ) -> Result<Principal, AuthRejection> {
        match self.mode {
            AuthMode::Public => Ok(default_principal.clone()),
            AuthMode::TrustedHeader => {
                let user = headers
                    .get(self.user_header.as_str())
                    .and_then(|v| v.to_str().ok())
                    .map(str::trim)
                    .filter(|s| !s.is_empty());
                let Some(subject) = user else {
                    return Ok(Principal::anonymous_public());
                };
                let mut scopes = vec![AclScope::Public];
                let mut is_admin = false;
                if let Some(groups_raw) = headers
                    .get(self.groups_header.as_str())
                    .and_then(|v| v.to_str().ok())
                {
                    for grp in groups_raw
                        .split(',')
                        .map(str::trim)
                        .filter(|s| !s.is_empty())
                    {
                        if grp.eq_ignore_ascii_case("admin") {
                            is_admin = true;
                        }
                        if grp.eq_ignore_ascii_case("public") {
                            continue;
                        }
                        if let Ok(scope) = AclScope::try_internal(grp)
                            && !scopes.contains(&scope)
                        {
                            scopes.push(scope);
                        }
                    }
                }
                Ok(Principal {
                    subject: subject.to_string(),
                    allowed_scopes: scopes,
                    is_admin,
                })
            }
            AuthMode::Bearer => {
                let Some(auth_hdr) = headers.get(header::AUTHORIZATION) else {
                    return Ok(Principal::anonymous_public());
                };
                let Ok(auth_str) = auth_hdr.to_str() else {
                    return Err(AuthRejection("invalid Authorization header"));
                };
                let Some(token) = auth_str.strip_prefix("Bearer ").map(str::trim) else {
                    return Err(AuthRejection("expected Bearer token"));
                };
                for entry in &self.tokens {
                    if constant_time_eq(entry.token.as_bytes(), token.as_bytes()) {
                        return Ok(Principal {
                            subject: entry.subject.clone(),
                            allowed_scopes: entry.allowed_scopes.clone(),
                            is_admin: entry.is_admin,
                        });
                    }
                }
                Err(AuthRejection("invalid bearer token"))
            }
        }
    }
}

fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut diff = 0u8;
    for (&x, &y) in a.iter().zip(b.iter()) {
        diff |= x ^ y;
    }
    diff == 0
}

/// HTTP 401 Unauthorized rejection for invalid bearer credentials.
#[derive(Debug)]
pub struct AuthRejection(pub &'static str);

#[derive(Serialize)]
struct AuthErrorEnvelope<'a> {
    error: AuthErrorBody<'a>,
}

#[derive(Serialize)]
struct AuthErrorBody<'a> {
    code: &'static str,
    message: &'a str,
}

impl IntoResponse for AuthRejection {
    fn into_response(self) -> Response {
        (
            StatusCode::UNAUTHORIZED,
            Json(AuthErrorEnvelope {
                error: AuthErrorBody {
                    code: "unauthorized",
                    message: self.0,
                },
            }),
        )
            .into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trusted_header_maps_user_and_groups_to_internal_scopes() {
        let cfg =
            AuthConfig::from_parts("trusted_header", "X-Auth-User", "X-Auth-Groups", None).unwrap();
        let mut headers = HeaderMap::new();
        headers.insert("X-Auth-User", "alice".parse().unwrap());
        headers.insert("X-Auth-Groups", "eng, ops, admin".parse().unwrap());

        let principal = cfg
            .resolve_principal(&headers, &Principal::anonymous_public())
            .unwrap();
        assert_eq!(principal.subject, "alice");
        assert!(principal.is_admin);
        assert!(principal.allows(&AclScope::Public));
        assert!(principal.allows(&AclScope::internal("eng")));
        assert!(principal.allows(&AclScope::internal("ops")));
    }

    #[test]
    fn bearer_mode_validates_tokens_and_rejects_invalid() {
        let cfg = AuthConfig::from_parts(
            "bearer",
            "X-Auth-User",
            "X-Auth-Groups",
            Some("secret-1:bob:team-a,team-b;admin-tok:root:ops:admin"),
        )
        .unwrap();

        let mut ok_headers = HeaderMap::new();
        ok_headers.insert(header::AUTHORIZATION, "Bearer secret-1".parse().unwrap());
        let bob = cfg
            .resolve_principal(&ok_headers, &Principal::anonymous_public())
            .unwrap();
        assert_eq!(bob.subject, "bob");
        assert!(!bob.is_admin);
        assert!(bob.allows(&AclScope::internal("team-a")));

        let mut bad_headers = HeaderMap::new();
        bad_headers.insert(header::AUTHORIZATION, "Bearer wrong".parse().unwrap());
        assert!(
            cfg.resolve_principal(&bad_headers, &Principal::anonymous_public())
                .is_err()
        );
    }
}
