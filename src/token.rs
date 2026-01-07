//! Token manager for DCE API authentication.
//!
//! Handles automatic token acquisition and refresh with thread-safe caching.

use std::sync::Arc;
use std::time::{Duration, Instant};

use reqwest::Client as HttpClient;
use serde::Serialize;
use tokio::sync::RwLock;

use crate::error::{Error, ErrorCode, Result};
use crate::models::{ApiResponse, TokenResponse};

/// Token expiry time in seconds (default 1 hour).
pub const TOKEN_EXPIRY_SECONDS: u64 = 3600;

/// Token expiry buffer in seconds (refresh 60s before expiry).
pub const TOKEN_EXPIRY_BUFFER: u64 = 60;

/// Authentication endpoint path.
pub const AUTH_ENDPOINT: &str = "/dceapi/cms/auth/accessToken";

/// Internal token state.
#[derive(Debug, Default)]
struct TokenState {
    /// The access token.
    token: String,
    /// When the token expires.
    expires_at: Option<Instant>,
}

/// Request body for authentication.
#[derive(Debug, Serialize)]
struct AuthRequest {
    secret: String,
}

/// Token manager for handling authentication.
///
/// This struct manages the access token lifecycle:
/// - Acquires new tokens when needed
/// - Caches tokens to avoid unnecessary requests
/// - Automatically refreshes tokens before expiry
///
/// Thread-safe: Uses `RwLock` for concurrent access.
#[derive(Debug)]
pub struct TokenManager {
    api_key: String,
    secret: String,
    base_url: String,
    http_client: HttpClient,
    state: Arc<RwLock<TokenState>>,
}

impl TokenManager {
    /// Create a new token manager.
    pub fn new(
        api_key: impl Into<String>,
        secret: impl Into<String>,
        base_url: impl Into<String>,
        http_client: HttpClient,
    ) -> Self {
        TokenManager {
            api_key: api_key.into(),
            secret: secret.into(),
            base_url: base_url.into(),
            http_client,
            state: Arc::new(RwLock::new(TokenState::default())),
        }
    }

    /// Get a valid access token.
    ///
    /// Returns a cached token if still valid, otherwise acquires a new one.
    pub async fn token(&self) -> Result<String> {
        // Try to get cached token with read lock
        {
            let state = self.state.read().await;
            if !state.token.is_empty() && !self.is_expired_locked(&state) {
                return Ok(state.token.clone());
            }
        }

        // Need to refresh - acquire write lock
        self.refresh_and_get_token().await
    }

    /// Force refresh the token.
    pub async fn refresh(&self) -> Result<()> {
        let mut state = self.state.write().await;
        self.refresh_locked(&mut state).await
    }

    /// Refresh and return the new token.
    async fn refresh_and_get_token(&self) -> Result<String> {
        let mut state = self.state.write().await;

        // Double-check after acquiring write lock
        if !state.token.is_empty() && !self.is_expired_locked(&state) {
            return Ok(state.token.clone());
        }

        self.refresh_locked(&mut state).await?;
        Ok(state.token.clone())
    }

    /// Internal refresh method (must hold write lock).
    async fn refresh_locked(&self, state: &mut TokenState) -> Result<()> {
        let auth_url = format!("{}{}", self.base_url, AUTH_ENDPOINT);

        let req_body = AuthRequest {
            secret: self.secret.clone(),
        };

        let response = self
            .http_client
            .post(&auth_url)
            .header("Content-Type", "application/json")
            .header("apikey", &self.api_key)
            .json(&req_body)
            .send()
            .await
            .map_err(|e| Error::auth(format!("failed to send auth request: {}", e)))?;

        let resp_text = response
            .text()
            .await
            .map_err(|e| Error::auth(format!("failed to read auth response: {}", e)))?;

        let api_resp: ApiResponse = serde_json::from_str(&resp_text)
            .map_err(|e| Error::auth(format!("failed to parse auth response: {}, body: {}", e, resp_text)))?;

        if api_resp.code != ErrorCode::Success as i32 {
            return Err(self.handle_auth_error(api_resp.code, &api_resp.msg));
        }

        let token_resp: TokenResponse = serde_json::from_value(api_resp.data)
            .map_err(|e| Error::auth(format!("failed to parse token data: {}", e)))?;

        if token_resp.access_token.is_empty() {
            return Err(Error::auth("received empty access token"));
        }

        // Update state
        state.token = token_resp.access_token;
        let expires_in = if token_resp.expires_in > 0 {
            token_resp.expires_in as u64
        } else {
            TOKEN_EXPIRY_SECONDS
        };
        // Subtract buffer to refresh before actual expiry
        let effective_expiry = expires_in.saturating_sub(TOKEN_EXPIRY_BUFFER);
        state.expires_at = Some(Instant::now() + Duration::from_secs(effective_expiry));

        Ok(())
    }

    /// Handle authentication error and return appropriate error type.
    fn handle_auth_error(&self, code: i32, message: &str) -> Error {
        match ErrorCode::from_code(code) {
            Some(ErrorCode::ParamError) => Error::auth(format!("invalid parameters: {}", message)),
            Some(ErrorCode::NoPermission) => Error::auth(format!("permission denied: {}", message)),
            Some(ErrorCode::ServerError) => Error::auth(format!("server error: {}", message)),
            Some(ErrorCode::RateLimit) => Error::auth(format!("rate limited: {}", message)),
            _ => Error::auth(format!("authentication failed (code {}): {}", code, message)),
        }
    }

    /// Check if token is expired (must hold lock).
    fn is_expired_locked(&self, state: &TokenState) -> bool {
        if state.token.is_empty() {
            return true;
        }
        match state.expires_at {
            Some(expires_at) => Instant::now() >= expires_at,
            None => true,
        }
    }

    /// Check if the cached token is expired.
    pub async fn is_expired(&self) -> bool {
        let state = self.state.read().await;
        self.is_expired_locked(&state)
    }

    /// Clear the cached token.
    pub async fn clear_token(&self) {
        let mut state = self.state.write().await;
        state.token.clear();
        state.expires_at = None;
    }

    /// Get the cached token without triggering refresh.
    pub async fn get_cached_token(&self) -> String {
        let state = self.state.read().await;
        state.token.clone()
    }
}
