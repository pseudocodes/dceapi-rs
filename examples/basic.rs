//! Basic usage example for the DCE API client.

use dceapi_rs::{Client, Config, GetArticleByPageRequest, QuotesRequest, RequestOptions};

#[tokio::main]
async fn main() -> dceapi_rs::Result<()> {
    // Create client from environment variables
    // Set DCE_API_KEY and DCE_SECRET before running
    let config = Config::from_env();

    // Or create with explicit credentials:
    // let config = Config::new()
    //     .with_api_key("your-api-key")
    //     .with_secret("your-secret");

    let client = Client::new(config)?;
    println!("DCE API client created successfully!");

    // Example 1: Get current trade date
    println!("\n--- Getting current trade date ---");
    match client.common.get_curr_trade_date(None).await {
        Ok(trade_date) => println!("Current trade date: {}", trade_date.date),
        Err(e) => println!("Error getting trade date: {}", e),
    }

    // Example 2: Get variety list
    println!("\n--- Getting variety list ---");
    match client.common.get_variety_list(None).await {
        Ok(varieties) => {
            println!("Found {} varieties:", varieties.len());
            for v in varieties.iter().take(5) {
                println!("  - {} ({})", v.name, v.code);
            }
            if varieties.len() > 5 {
                println!("  ... and {} more", varieties.len() - 5);
            }
        }
        Err(e) => println!("Error getting varieties: {}", e),
    }

    // Example 3: Get articles
    println!("\n--- Getting exchange announcements ---");
    let article_req = GetArticleByPageRequest {
        column_id: "244".to_string(), // Exchange announcements
        page_no: 1,
        page_size: 5,
        site_id: 5,
    };
    match client.news.get_article_by_page(article_req, None).await {
        Ok(response) => {
            println!("Found {} articles:", response.total_count);
            for article in response.result_list.iter().take(3) {
                println!("  - {}", article.title);
            }
        }
        Err(e) => println!("Error getting articles: {}", e),
    }

    // Example 4: Get day quotes (with options)
    println!("\n--- Getting day quotes ---");
    let quotes_req = QuotesRequest {
        variety_id: Some("a".to_string()), // Soybean No.1
        variety: None,
        trade_date: "20240115".to_string(),
        trade_type: "1".to_string(), // Futures
        lang: Some("zh".to_string()),
        statistics_type: None,
    };

    let opts = RequestOptions::new().with_trade_type(1);

    match client.market.get_day_quotes(&quotes_req, Some(opts)).await {
        Ok(quotes) => {
            println!("Found {} quotes:", quotes.len());
            for quote in quotes.iter().take(3) {
                println!(
                    "  - {} | Open: {} | High: {} | Low: {} | Close: {}",
                    quote.contract_id, quote.open, quote.high, quote.low, quote.close
                );
            }
        }
        Err(e) => println!("Error getting quotes: {}", e),
    }

    println!("\nDone!");
    Ok(())
}
