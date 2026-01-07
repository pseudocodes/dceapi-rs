//! HTTP client for DCE API requests.
//!
//! Provides the base HTTP functionality with automatic token handling and retry logic.

use std::sync::Arc;

use reqwest::Client as HttpClient;
use serde::{de::DeserializeOwned, Serialize};

use crate::config::Config;
use crate::error::{Error, ErrorCode, Result};
use crate::models::ApiResponse;
use crate::token::TokenManager;

/// Request options that can be set per-request.
#[derive(Debug, Clone)]
pub struct RequestOptions {
    /// Trade type override (1 = futures, 2 = options).
    pub trade_type: Option<i32>,
    /// Language override.
    pub lang: Option<String>,
}

impl Default for RequestOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl RequestOptions {
    /// Create new request options with defaults.
    pub fn new() -> Self {
        RequestOptions {
            trade_type: None,
            lang: None,
        }
    }

    /// Set trade type.
    pub fn with_trade_type(mut self, trade_type: i32) -> Self {
        self.trade_type = Some(trade_type);
        self
    }

    /// Set language.
    pub fn with_lang(mut self, lang: impl Into<String>) -> Self {
        self.lang = Some(lang.into());
        self
    }
}

/// Base HTTP client for API requests.
#[derive(Debug, Clone)]
pub struct BaseClient {
    config: Arc<Config>,
    http_client: HttpClient,
    token_manager: Arc<TokenManager>,
}

impl BaseClient {
    /// Create a new base client.
    pub fn new(config: Config, http_client: HttpClient, token_manager: Arc<TokenManager>) -> Self {
        BaseClient {
            config: Arc::new(config),
            http_client,
            token_manager,
        }
    }

    /// Execute an HTTP request.
    ///
    /// Handles token management, serialization, and response parsing.
    pub async fn do_request<T, R>(
        &self,
        method: reqwest::Method,
        path: &str,
        body: Option<&T>,
        opts: Option<RequestOptions>,
    ) -> Result<R>
    where
        T: Serialize,
        R: DeserializeOwned,
    {
        let opts = opts.unwrap_or_default();
        
        // First attempt
        let result = self.execute_request(&method, path, body, &opts).await;
        
        // Check if we need to retry due to token expiry
        if let Err(Error::Api { code, .. }) = &result {
            if *code == ErrorCode::TokenExpired as i32 {
                // Refresh token and retry once
                self.token_manager.refresh().await?;
                return self.execute_request(&method, path, body, &opts).await;
            }
        }
        
        result
    }

    /// Execute a single HTTP request (no retry).
    async fn execute_request<T, R>(
        &self,
        method: &reqwest::Method,
        path: &str,
        body: Option<&T>,
        opts: &RequestOptions,
    ) -> Result<R>
    where
        T: Serialize,
        R: DeserializeOwned,
    {
        // Get token
        let token = self.token_manager.token().await?;

        // Build URL
        let url = format!("{}{}", self.config.base_url, path);

        // Build request
        let mut request = self.http_client.request(method.clone(), &url);

        // Set headers
        request = request
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", token))
            .header("apikey", &self.config.api_key)
            .header(
                "tradeType",
                opts.trade_type.unwrap_or(self.config.trade_type).to_string(),
            );

        if let Some(lang) = opts.lang.as_ref().or(Some(&self.config.lang)) {
            request = request.header("lang", lang);
        }

        // Set body if present
        if let Some(body) = body {
            request = request.json(body);
        }

        // Send request
        let response = request.send().await?;

        // Read response body
        let resp_text = response.text().await?;

        // Handle response
        self.parse_response(&resp_text)
    }

    /// Parse API response and handle error codes.
    fn parse_response<R>(&self, resp_text: &str) -> Result<R>
    where
        R: DeserializeOwned,
    {
        // Parse API response
        let api_resp: ApiResponse = serde_json::from_str(resp_text).map_err(|e| {
            Error::parse(resp_text, format!("failed to parse response: {}", e))
        })?;

        // Handle response based on code
        match ErrorCode::from_code(api_resp.code) {
            Some(ErrorCode::Success) => {
                // Success - deserialize data
                serde_json::from_value(api_resp.data).map_err(|e| {
                    Error::parse(
                        resp_text,
                        format!("failed to deserialize response data: {}", e),
                    )
                })
            }

            Some(ErrorCode::ParamError) => {
                // 400: Parameter error
                Err(Error::api(ErrorCode::ParamError as i32, api_resp.msg))
            }

            Some(ErrorCode::NoPermission) => {
                // 401: Permission denied
                Err(Error::api(ErrorCode::NoPermission as i32, api_resp.msg))
            }

            Some(ErrorCode::TokenExpired) => {
                // 402: Token expired
                Err(Error::api(ErrorCode::TokenExpired as i32, api_resp.msg))
            }

            Some(ErrorCode::ServerError) => {
                // 500: Server error
                Err(Error::api(ErrorCode::ServerError as i32, api_resp.msg))
            }

            Some(ErrorCode::RateLimit) => {
                // 501: Rate limit
                Err(Error::api(ErrorCode::RateLimit as i32, api_resp.msg))
            }

            None => {
                // Unknown error code
                Err(Error::api(api_resp.code, api_resp.msg))
            }
        }
    }

    /// Convenience method for GET requests.
    pub async fn do_get<R>(&self, path: &str, opts: Option<RequestOptions>) -> Result<R>
    where
        R: DeserializeOwned,
    {
        self.do_request::<(), R>(reqwest::Method::GET, path, None, opts)
            .await
    }

    /// Convenience method for POST requests.
    pub async fn do_post<T, R>(
        &self,
        path: &str,
        body: &T,
        opts: Option<RequestOptions>,
    ) -> Result<R>
    where
        T: Serialize,
        R: DeserializeOwned,
    {
        self.do_request(reqwest::Method::POST, path, Some(body), opts)
            .await
    }

    /// Get reference to the config.
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// Get reference to the token manager.
    pub fn token_manager(&self) -> &TokenManager {
        &self.token_manager
    }
}
