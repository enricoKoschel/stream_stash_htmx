use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode, decode_header};
use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use thiserror::Error;
use tokio::sync::RwLock;

const GOOGLE_CERTS_URL: &str = "https://www.googleapis.com/oauth2/v3/certs";
const GOOGLE_ISSUERS: &[&str] = &["https://accounts.google.com", "accounts.google.com"];

#[derive(Debug, Error)]
pub enum GoogleAuthError {
    #[error("Invalid ID token: {0}")]
    InvalidToken(String),
    #[error("CSRF token mismatch")]
    CsrfMismatch,
    #[error("Missing CSRF cookie")]
    MissingCsrfCookie,
    #[error("Failed to fetch Google keys: {0}")]
    KeyFetchError(String),
    #[error("Key not found for kid: {0}")]
    KeyNotFound(String),
    #[error("Google is not authoritative for this user")]
    GoogleNotAuthoritative,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct VerifiedUser {
    pub sub: String,
    pub email: Option<String>,
    pub email_verified: Option<bool>,
    pub name: Option<String>,
    pub picture: Option<String>,
    pub hosted_domain: Option<String>,
}

impl VerifiedUser {
    /// Returns true if Google is authoritative for this user's email.
    /// This means you can trust the email without additional verification.
    ///
    /// Google is authoritative when:
    /// - The email ends with @gmail.com (Gmail account), or
    /// - email_verified is true AND hd (hosted domain) is set (Google Workspace account)
    pub fn is_google_authoritative(&self) -> bool {
        let Some(email) = &self.email else {
            return false;
        };

        // Gmail accounts
        if email.ends_with("@gmail.com") {
            return true;
        }

        // Google Workspace accounts
        if self.email_verified == Some(true) && self.hosted_domain.is_some() {
            return true;
        }

        false
    }
}

#[derive(Debug, Deserialize)]
struct GoogleCerts {
    keys: Vec<GoogleKey>,
}

#[derive(Debug, Deserialize)]
struct GoogleKey {
    kid: String,
    n: String,
    e: String,
    alg: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct GoogleIdTokenClaims {
    iss: String,
    sub: String,
    aud: String,
    exp: u64,
    iat: u64,
    email: Option<String>,
    email_verified: Option<bool>,
    name: Option<String>,
    picture: Option<String>,
    hd: Option<String>,
}

struct CachedKeys {
    keys: HashMap<String, DecodingKey>,
    expires_at: Option<Instant>,
}

#[derive(Clone)]
pub struct GoogleAuthService {
    client_id: String,
    http_client: Client,
    cached_keys: Arc<RwLock<CachedKeys>>,
}

impl GoogleAuthService {
    pub fn new(client_id: String) -> Self {
        Self {
            client_id,
            http_client: Client::new(),
            cached_keys: Arc::new(RwLock::new(CachedKeys {
                keys: HashMap::new(),
                expires_at: None,
            })),
        }
    }

    pub async fn verify_token(&self, id_token: &str) -> Result<VerifiedUser, GoogleAuthError> {
        let header = decode_header(id_token)
            .map_err(|e| GoogleAuthError::InvalidToken(format!("Invalid header: {}", e)))?;

        let kid = header
            .kid
            .ok_or_else(|| GoogleAuthError::InvalidToken("Missing kid in header".to_string()))?;

        let decoding_key = self.get_decoding_key(&kid).await?;

        let mut validation = Validation::new(Algorithm::RS256);
        validation.set_audience(&[&self.client_id]);
        validation.set_issuer(GOOGLE_ISSUERS);
        validation.validate_exp = true;
        validation.validate_nbf = true;

        let token_data = decode::<GoogleIdTokenClaims>(id_token, &decoding_key, &validation)
            .map_err(|e| {
                GoogleAuthError::InvalidToken(format!("Token validation failed: {}", e))
            })?;

        let claims = token_data.claims;

        let user = VerifiedUser {
            sub: claims.sub,
            email: claims.email,
            email_verified: claims.email_verified,
            name: claims.name,
            picture: claims.picture,
            hosted_domain: claims.hd,
        };

        if user.is_google_authoritative() {
            Ok(user)
        } else {
            Err(GoogleAuthError::GoogleNotAuthoritative)
        }
    }

    async fn get_decoding_key(&self, kid: &str) -> Result<DecodingKey, GoogleAuthError> {
        // Check if cache is valid and contains the key
        {
            let cache = self.cached_keys.read().await;
            let is_expired = cache
                .expires_at
                .map(|exp| Instant::now() >= exp)
                .unwrap_or(true);

            if !is_expired && let Some(key) = cache.keys.get(kid) {
                return Ok(key.clone());
            }
        }

        self.refresh_keys().await?;

        let cache = self.cached_keys.read().await;
        cache
            .keys
            .get(kid)
            .cloned()
            .ok_or_else(|| GoogleAuthError::KeyNotFound(kid.to_string()))
    }

    async fn refresh_keys(&self) -> Result<(), GoogleAuthError> {
        let response = self
            .http_client
            .get(GOOGLE_CERTS_URL)
            .send()
            .await
            .map_err(|e| GoogleAuthError::KeyFetchError(e.to_string()))?;

        let max_age = response
            .headers()
            .get("cache-control")
            .and_then(|v| v.to_str().ok())
            .and_then(parse_max_age);

        let expires_at = max_age.map(|secs| Instant::now() + Duration::from_secs(secs));

        let certs = response
            .json::<GoogleCerts>()
            .await
            .map_err(|e| GoogleAuthError::KeyFetchError(e.to_string()))?;

        let mut cache = self.cached_keys.write().await;
        cache.keys.clear();
        cache.expires_at = expires_at;

        for key in certs.keys {
            if key.alg != "RS256" {
                continue;
            }

            let decoding_key = DecodingKey::from_rsa_components(&key.n, &key.e).map_err(|e| {
                GoogleAuthError::KeyFetchError(format!("Invalid key format: {}", e))
            })?;

            cache.keys.insert(key.kid, decoding_key);
        }

        Ok(())
    }

    pub fn verify_csrf(
        &self,
        cookie_token: Option<&str>,
        body_token: &str,
    ) -> Result<(), GoogleAuthError> {
        let cookie_token = cookie_token.ok_or(GoogleAuthError::MissingCsrfCookie)?;

        if cookie_token != body_token {
            return Err(GoogleAuthError::CsrfMismatch);
        }

        Ok(())
    }
}

/// Parses max-age value from Cache-Control header
/// e.g., "public, max-age=19591, must-revalidate, no-transform" -> Some(19591)
fn parse_max_age(cache_control: &str) -> Option<u64> {
    cache_control
        .split(',')
        .map(|s| s.trim())
        .find(|s| s.starts_with("max-age="))
        .and_then(|s| s.strip_prefix("max-age="))
        .and_then(|s| s.parse().ok())
}
