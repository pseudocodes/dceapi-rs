//! DCE API client.
//!
//! The main entry point for using the DCE API.

use std::sync::Arc;

use reqwest::Client as HttpClient;

use crate::config::Config;
use crate::error::{Error, Result};
use crate::http::BaseClient;
use crate::services::{
    CommonService, DeliveryService, MarketService, MemberService, NewsService, SettleService,
    TradeService,
};
use crate::token::TokenManager;

/// DCE API client.
///
/// This is the main entry point for using the DCE API. It provides access to all
/// API services through dedicated service instances.
///
/// # Example
///
/// ```no_run
/// use dceapi::{Client, Config};
///
/// #[tokio::main]
/// async fn main() -> dceapi::Result<()> {
///     let config = Config::new()
///         .with_api_key("your-api-key")
///         .with_secret("your-secret");
///     
///     let client = Client::new(config)?;
///     
///     // Get current trade date
///     let trade_date = client.common.get_curr_trade_date(None).await?;
///     println!("Trade date: {}", trade_date.date);
///     
///     Ok(())
/// }
/// ```
#[derive(Debug, Clone)]
pub struct Client {
    config: Arc<Config>,
    token_manager: Arc<TokenManager>,

    /// News service for articles and announcements.
    pub news: NewsService,

    /// Common service for trade dates and varieties.
    pub common: CommonService,

    /// Market service for quotes and market data.
    pub market: MarketService,

    /// Delivery service for delivery data.
    pub delivery: DeliveryService,

    /// Member service for member rankings.
    pub member: MemberService,

    /// Trade service for trading parameters.
    pub trade: TradeService,

    /// Settlement service for settlement parameters.
    pub settle: SettleService,
}

impl Client {
    /// Create a new DCE API client.
    ///
    /// # Arguments
    /// * `config` - Client configuration with API credentials
    ///
    /// # Errors
    /// Returns an error if the configuration is invalid (missing API key or secret).
    ///
    /// # Example
    ///
    /// ```no_run
    /// use dceapi::{Client, Config};
    ///
    /// let config = Config::new()
    ///     .with_api_key("your-api-key")
    ///     .with_secret("your-secret");
    ///
    /// let client = Client::new(config).expect("Failed to create client");
    /// ```
    pub fn new(mut config: Config) -> Result<Self> {
        // Apply defaults
        config.apply_defaults();

        // Validate configuration
        config.validate()?;

        // Create HTTP client
        let http_client = HttpClient::builder()
            .timeout(config.timeout)
            .gzip(true)
            .brotli(true)
            .deflate(true)
            .build()
            .map_err(|e| {
                Error::validation(
                    "http_client",
                    format!("failed to create HTTP client: {}", e),
                )
            })?;

        // Create token manager
        let token_manager = Arc::new(TokenManager::new(
            &config.api_key,
            &config.secret,
            &config.base_url,
            http_client.clone(),
        ));

        // Create base client
        let base_client = BaseClient::new(config.clone(), http_client, token_manager.clone());

        // Create client with all services
        Ok(Client {
            config: Arc::new(config),
            token_manager,
            news: NewsService::new(base_client.clone()),
            common: CommonService::new(base_client.clone()),
            market: MarketService::new(base_client.clone()),
            delivery: DeliveryService::new(base_client.clone()),
            member: MemberService::new(base_client.clone()),
            trade: TradeService::new(base_client.clone()),
            settle: SettleService::new(base_client),
        })
    }

    /// Create a new client from environment variables.
    ///
    /// Reads `DCE_API_KEY` and `DCE_SECRET` from the environment.
    ///
    /// # Errors
    /// Returns an error if the environment variables are not set.
    pub fn from_env() -> Result<Self> {
        let config = Config::from_env();
        Self::new(config)
    }

    /// Get the client configuration (read-only).
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// Get the token manager.
    ///
    /// This can be used for advanced token management, such as forcing a refresh.
    pub fn token_manager(&self) -> &TokenManager {
        &self.token_manager
    }
}
