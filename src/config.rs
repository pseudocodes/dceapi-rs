//! Configuration for the DCE API client.

use std::env;
use std::time::Duration;

use crate::error::{Error, Result};

/// Default API base URL.
pub const DEFAULT_BASE_URL: &str = "http://www.dce.com.cn";

/// Default HTTP timeout in seconds.
pub const DEFAULT_TIMEOUT_SECS: u64 = 30;

/// Default language.
pub const DEFAULT_LANG: &str = "zh";

/// Default trade type (1 = futures).
pub const DEFAULT_TRADE_TYPE: i32 = 1;

/// Environment variable name for API key.
pub const ENV_API_KEY: &str = "DCE_API_KEY";

/// Environment variable name for API secret.
pub const ENV_SECRET: &str = "DCE_SECRET";

/// Client configuration.
#[derive(Debug, Clone)]
pub struct Config {
    /// API base URL. Defaults to "http://www.dce.com.cn".
    pub base_url: String,

    /// API key (required).
    pub api_key: String,

    /// API secret (required).
    pub secret: String,

    /// HTTP request timeout. Defaults to 30 seconds.
    pub timeout: Duration,

    /// Language for API responses. "zh" or "en". Defaults to "zh".
    pub lang: String,

    /// Trade type. 1 = futures, 2 = options. Defaults to 1.
    pub trade_type: i32,
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

impl Config {
    /// Create a new configuration with default values.
    ///
    /// Note: `api_key` and `secret` must be set before using the client.
    pub fn new() -> Self {
        Config {
            base_url: DEFAULT_BASE_URL.to_string(),
            api_key: String::new(),
            secret: String::new(),
            timeout: Duration::from_secs(DEFAULT_TIMEOUT_SECS),
            lang: DEFAULT_LANG.to_string(),
            trade_type: DEFAULT_TRADE_TYPE,
        }
    }

    /// Create a configuration from environment variables.
    ///
    /// Reads `DCE_API_KEY` and `DCE_SECRET` from environment.
    pub fn from_env() -> Self {
        let mut config = Self::new();
        config.api_key = env::var(ENV_API_KEY).unwrap_or_default();
        config.secret = env::var(ENV_SECRET).unwrap_or_default();
        config
    }

    /// Set the base URL.
    pub fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into();
        self
    }

    /// Set the API key.
    pub fn with_api_key(mut self, api_key: impl Into<String>) -> Self {
        self.api_key = api_key.into();
        self
    }

    /// Set the API secret.
    pub fn with_secret(mut self, secret: impl Into<String>) -> Self {
        self.secret = secret.into();
        self
    }

    /// Set the HTTP timeout.
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Set the language.
    pub fn with_lang(mut self, lang: impl Into<String>) -> Self {
        self.lang = lang.into();
        self
    }

    /// Set the trade type.
    pub fn with_trade_type(mut self, trade_type: i32) -> Self {
        self.trade_type = trade_type;
        self
    }

    /// Validate the configuration.
    ///
    /// Returns an error if required fields are missing.
    pub fn validate(&self) -> Result<()> {
        if self.api_key.is_empty() {
            return Err(Error::validation("api_key", "API key is required"));
        }
        if self.secret.is_empty() {
            return Err(Error::validation("secret", "secret is required"));
        }
        Ok(())
    }

    /// Apply default values to empty fields.
    pub fn apply_defaults(&mut self) {
        if self.base_url.is_empty() {
            self.base_url = DEFAULT_BASE_URL.to_string();
        }
        if self.timeout.is_zero() {
            self.timeout = Duration::from_secs(DEFAULT_TIMEOUT_SECS);
        }
        if self.lang.is_empty() {
            self.lang = DEFAULT_LANG.to_string();
        }
        if self.trade_type == 0 {
            self.trade_type = DEFAULT_TRADE_TYPE;
        }
    }
}
