//! DCE API Client Library
//!
//! A Rust client library for the Dalian Commodity Exchange (DCE) API.
//!
//! # Overview
//!
//! This library provides access to:
//! - **News**: Articles and announcements
//! - **Common**: Trade dates and variety (commodity) information
//! - **Market**: Quotes and market data (day, night, week, month)
//! - **Delivery**: Delivery data, warehouse receipts, costs
//! - **Member**: Member trading rankings
//! - **Trade**: Trading parameters and contract information
//! - **Settlement**: Settlement parameters
//!
//! # Quick Start
//!
//! ```no_run
//! use dceapi::{Client, Config};
//!
//! #[tokio::main]
//! async fn main() -> dceapi::Result<()> {
//!     // Create client with credentials
//!     let config = Config::new()
//!         .with_api_key("your-api-key")
//!         .with_secret("your-secret");
//!     
//!     let client = Client::new(config)?;
//!     
//!     // Get current trade date
//!     let trade_date = client.common.get_curr_trade_date(None).await?;
//!     println!("Current trade date: {}", trade_date.date);
//!     
//!     // Get variety list
//!     let varieties = client.common.get_variety_list(None).await?;
//!     for v in varieties {
//!         println!("Variety: {} ({})", v.name, v.code);
//!     }
//!     
//!     Ok(())
//! }
//! ```
//!
//! # Using Environment Variables
//!
//! You can also create a client from environment variables:
//!
//! ```no_run
//! use dceapi::Client;
//!
//! # async fn example() -> dceapi::Result<()> {
//! // Set DCE_API_KEY and DCE_SECRET environment variables
//! let client = Client::from_env()?;
//! # Ok(())
//! # }
//! ```
//!
//! # Error Handling
//!
//! All API methods return `Result<T, Error>`. The error types include:
//! - `Error::Api` - API returned an error response
//! - `Error::Auth` - Authentication failed
//! - `Error::Network` - Network or HTTP error
//! - `Error::Validation` - Invalid request parameters
//! - `Error::Parse` - Failed to parse response
//!
//! # Request Options
//!
//! Most methods accept optional `RequestOptions` to override defaults:
//!
//! ```no_run
//! use dceapi::{Client, Config, RequestOptions};
//!
//! # async fn example() -> dceapi::Result<()> {
//! # let client = Client::new(Config::new().with_api_key("k").with_secret("s"))?;
//! let opts = RequestOptions::new()
//!     .with_trade_type(2)  // Options instead of futures
//!     .with_lang("en");    // English language
//!
//! let varieties = client.common.get_variety_list(Some(opts)).await?;
//! # Ok(())
//! # }
//! ```

#![warn(missing_docs)]
#![warn(rust_2018_idioms)]

mod client;
mod config;
mod error;
mod http;
mod models;
mod services;
mod token;

// Re-export main types
pub use client::Client;
pub use config::{Config, DEFAULT_BASE_URL, DEFAULT_LANG, DEFAULT_TIMEOUT_SECS, DEFAULT_TRADE_TYPE};
pub use error::{Error, ErrorCode, Result};
pub use http::RequestOptions;
pub use token::TokenManager;

// Re-export all models
pub use models::*;

// Re-export services for direct access
pub use services::{
    CommonService, DeliveryService, MarketService, MemberService, NewsService, SettleService,
    TradeService,
};

// Re-export news helper
pub use services::news::is_valid_column_id;
