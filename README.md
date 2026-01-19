# DCE API Rust Client

A high-performance, asynchronous Rust client library for the Dalian Commodity Exchange (DCE) API.

Ported from the [dceapi-go](https://github.com/pseudocodes/dceapi-go) library, this SDK provides a type-safe and efficient way to interact with DCE market data, news, delivery, and trading services.

## Features

- **Asynchronous**: Built on `tokio` and `reqwest` for modern async/await workflows.
- **Auto-Token Management**: Handles API token acquisition and automatic refresh before expiry.
- **Retry Mechanism**: Automatically retries requests on token expiration (HTTP 402).
- **Compression Support**: Supports Gzip, Brotli, and Deflate for faster data transfer.
- **Type Safety**: Comprehensive Rust models for all request and response structures.
- **Error Handling**: Detailed error types using `thiserror`.
- **Complete Coverage**: 100% API endpoint coverage (37 methods across 34 unique endpoints)

## Services

### News Service (2 endpoints)
- Get articles, announcements, and notices
- Retrieve detailed news content

### Common Service (3 endpoints)
- Retrieve current trade dates and variety (commodity) lists
- Get variety month/year statistics

### Market Service (11 methods, 8 endpoints)
- Fetch day, night, week, and month quotes
- Contract statistics (max volume, turnover, open interest, price)
- Rise/fall event (limit up/down) queries
- Division price information
- Warehouse receipt data

### Trade Service (8 endpoints)
- Get trading parameters (day/month)
- Contract information (standard, new, arbitrage)
- Margin and performance parameters
- Market maker continuous quote series

### Settle Service (1 endpoint)
- Retrieve settlement parameters

### Member Service (2 endpoints)
- Access member trading and volume rankings (daily and phase)

### Delivery Service (10 endpoints)
- Get delivery data and match information
- Warehouse receipts and premiums
- TC congregate delivery and roll delivery
- Bonded delivery (standard and T+D)
- Factory spot basis spreads
- Plywood delivery commodities

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
dceapi = { path = "./path/to/dceapi-rs" } # Adjust the path accordingly
tokio = { version = "1", features = ["full"] }
```

## Quick Start

### Set Environment Variables

```bash
export DCE_API_KEY="your-api-key"
export DCE_SECRET="your-secret"
```

### Basic Usage

```rust
use dceapi::{Client, Config};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client from environment variables
    let client = Client::from_env()?;

    // Get current trade date
    let trade_date = client.common.get_curr_trade_date(None).await?;
    println!("Current trade date: {}", trade_date.date);

    // Get variety list
    let varieties = client.common.get_variety_list(None).await?;
    for v in varieties.iter().take(5) {
        println!("Variety: {} ({})", v.name, v.code);
    }

    Ok(())
}
```

## Examples

The project includes two examples:

1. **Basic**: Simple demonstration of trade date and variety list retrieval.
   ```bash
   cargo run --example basic
   ```

2. **Complete**: Comprehensive demonstration of all 36+ API endpoints across 7 services:
   - Common Service (3 endpoints)
   - News Service (2 endpoints)
   - Market Service (10 endpoints)
   - Trade Service (8 endpoints)
   - Settle Service (3 endpoints)
   - Member Service (2 endpoints)
   - Delivery Service (11 endpoints)
   
   All API calls are spaced 1 second apart to respect rate limits.
   ```bash
   cargo run --example complete
   ```

## Configuration

You can also configure the client manually:

```rust
use dceapi::{Client, Config};
use std::time::Duration;

let config = Config::new()
    .with_api_key("your-api-key")
    .with_secret("your-secret")
    .with_timeout(Duration::from_secs(30))
    .with_lang("zh");

let client = Client::new(config)?;
```

## License

MIT / Apache-2.0
