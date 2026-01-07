//! Complete usage example demonstrating all services of the dceapi-rs crate.

use std::error::Error as StdError;
use std::time::Duration;

use dceapi_rs::{
    Client, Config, ContractInfoRequest, DailyRankingRequest, DayTradeParamRequest,
    DeliveryDataRequest, Error as DceError, GetArticleByPageRequest, PhaseRankingRequest,
    QuotesRequest, Result, SettleParamRequest, WarehouseReceiptRequest,
};

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn StdError>> {
    // Create client
    let client = match create_client() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to create client: {}", e);
            std::process::exit(1);
        }
    };

    println!("DCE API client created successfully!");

    // Run examples for each service
    run_common_service_examples(&client).await?;
    run_news_service_examples(&client).await?;
    run_market_service_examples(&client).await?;
    run_trade_service_examples(&client).await?;
    run_settle_service_examples(&client).await?;
    run_member_service_examples(&client).await?;
    run_delivery_service_examples(&client).await?;

    Ok(())
}

fn create_client() -> Result<Client> {
    let mut config = Config::from_env();

    // Validate we have credentials
    if config.api_key.is_empty() || config.secret.is_empty() {
        return Err(DceError::validation(
            "api_key/secret",
            "Please set DCE_API_KEY and DCE_SECRET environment variables",
        ));
    }

    // Set some explicit timeouts
    config.timeout = Duration::from_secs(30);

    Client::new(config)
}

fn print_separator(title: &str) {
    println!("\n{}", "=".repeat(60));
    println!("{}", title);
    println!("{}", "=".repeat(60));
}

// ============================================================================
// CommonService Examples
// ============================================================================

async fn run_common_service_examples(client: &Client) -> Result<()> {
    print_separator("CommonService Examples");

    println!("\n--- GetCurrTradeDate ---");
    match client.common.get_curr_trade_date(None).await {
        Ok(date) => println!("Current trade date: {}", date.date),
        Err(e) => eprintln!("Error: {}", e),
    }

    println!("\n--- GetVarietyList (Futures) ---");
    match client.common.get_variety_list(None).await {
        Ok(varieties) => {
            println!("Variety count: {}", varieties.len());
            for v in varieties.iter().take(5) {
                println!("  - {} ({}) - {}", v.name, v.code, v.variety_type);
            }
            if varieties.len() > 5 {
                println!("  ... and {} more", varieties.len() - 5);
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    Ok(())
}

// ============================================================================
// NewsService Examples
// ============================================================================

async fn run_news_service_examples(client: &Client) -> Result<()> {
    print_separator("NewsService Examples");

    println!("\n--- GetArticleByPage (Exchange Announcements, columnId=244) ---");
    let req = GetArticleByPageRequest {
        column_id: "244".to_string(),
        page_no: 1,
        page_size: 5,
        site_id: 5,
    };
    match client.news.get_article_by_page(req, None).await {
        Ok(resp) => {
            println!(
                "Total articles: {}, Current page: {}",
                resp.total_count,
                resp.result_list.len()
            );
            for article in resp.result_list {
                println!("  - [{}] {}", article.show_date, article.title);
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    println!("\n--- GetArticleByPage (Exchange Notices, columnId=245) ---");
    let req = GetArticleByPageRequest {
        column_id: "245".to_string(),
        page_no: 1,
        page_size: 3,
        site_id: 5,
    };
    match client.news.get_article_by_page(req, None).await {
        Ok(resp) => {
            println!("Total notices: {}", resp.total_count);
            for article in resp.result_list {
                println!("  - [{}] {}", article.show_date, article.title);
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    Ok(())
}

// ============================================================================
// MarketService Examples
// ============================================================================

async fn run_market_service_examples(client: &Client) -> Result<()> {
    print_separator("MarketService Examples");

    // Use a fixed date for historical data consistency in example
    let target_date = "20240101".to_string();

    println!("\n--- GetDayQuotes (Soybean Meal m) ---");
    let req = QuotesRequest {
        variety_id: Some("m".to_string()),
        variety: None,
        trade_date: target_date.clone(),
        trade_type: "1".to_string(),
        lang: Some("zh".to_string()),
        statistics_type: None,
    };
    match client.market.get_day_quotes(&req, None).await {
        Ok(quotes) => {
            println!("Soybean Meal day quotes, contract count: {}", quotes.len());
            for q in quotes
                .iter()
                .filter(|q| !q.contract_id.is_empty() && q.variety != "总计")
                .take(3)
            {
                println!(
                    "  Contract: {} | Open: {} High: {} Low: {} Close: {} | Volume: {}",
                    q.contract_id, q.open, q.high, q.low, q.close, q.volume
                );
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    println!("\n--- GetNightQuotes (Iron Ore i) ---");
    let req = QuotesRequest {
        variety_id: None,
        variety: Some("i".to_string()),
        trade_date: target_date,
        trade_type: "1".to_string(),
        lang: Some("zh".to_string()),
        statistics_type: None,
    };
    match client.market.get_night_quotes(&req, None).await {
        Ok(quotes) => {
            println!("Iron Ore night quotes, contract count: {}", quotes.len());
            for q in quotes
                .iter()
                .filter(|q| !q.deliv_month.is_empty() && q.variety != "总计")
                .take(3)
            {
                println!(
                    "  Contract: {} | Last Price: {} | Open Interest: {}",
                    q.deliv_month, q.last_price, q.open_interest
                );
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    Ok(())
}

// ============================================================================
// TradeService Examples
// ============================================================================

async fn run_trade_service_examples(client: &Client) -> Result<()> {
    print_separator("TradeService Examples");

    println!("\n--- GetDayTradeParam (Soybean Meal m) ---");
    let req = DayTradeParamRequest {
        variety_id: "m".to_string(),
        trade_type: "1".to_string(),
        lang: "zh".to_string(),
    };
    match client.trade.get_day_trade_param(&req, None).await {
        Ok(params) => {
            println!("Trade parameters count: {}", params.len());
            for p in params.iter().take(3) {
                println!(
                    "  Contract: {} | Spec Buy Rate: {:.2}% | Rise Limit: {} | Fall Limit: {}",
                    p.contract_id,
                    p.spec_buy_rate * 100.0,
                    p.rise_limit,
                    p.fall_limit
                );
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    println!("\n--- GetContractInfo (Corn c) ---");
    let req = ContractInfoRequest {
        variety_id: "c".to_string(),
        trade_type: "1".to_string(),
        lang: "zh".to_string(),
    };
    match client.trade.get_contract_info(&req, None).await {
        Ok(contracts) => {
            println!("Contract count: {}", contracts.len());
            for c in contracts.iter().take(3) {
                println!(
                    "  Contract: {} | Variety: {} | Unit: {} | End Trade Date: {}",
                    c.contract_id, c.variety, c.unit, c.end_trade_date
                );
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    println!("\n--- GetArbitrageContract (zh) ---");
    match client.trade.get_arbitrage_contract(Some("zh"), None).await {
        Ok(arbi_contracts) => {
            println!("Arbitrage contract count: {}", arbi_contracts.len());
            for a in arbi_contracts.iter().take(3) {
                println!(
                    "  {} | {} | {} | Max Hand: {}",
                    a.arbi_name, a.variety_name, a.arbi_contract_id, a.max_hand
                );
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    Ok(())
}

// ============================================================================
// SettleService Examples
// ============================================================================

async fn run_settle_service_examples(client: &Client) -> Result<()> {
    print_separator("SettleService Examples");

    println!("\n--- GetSettleParam (Soybean Meal m) ---");
    let req = SettleParamRequest {
        variety_id: "m".to_string(),
        trade_date: "20240101".to_string(),
        trade_type: "1".to_string(),
        lang: "zh".to_string(),
    };
    match client.settle.get_settle_param(&req, None).await {
        Ok(params) => {
            println!("Settle parameters count: {}", params.len());
            for s in params.iter().take(3) {
                println!(
                    "  Contract: {} | Settle Price: {} | Spec Buy Rate: {} | Open Fee: {}",
                    s.contract_id, s.clear_price, s.spec_buy_rate, s.open_fee
                );
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    Ok(())
}

// ============================================================================
// MemberService Examples
// ============================================================================

async fn run_member_service_examples(client: &Client) -> Result<()> {
    print_separator("MemberService Examples");

    println!("\n--- GetDailyRanking (Soybean No.1 a) ---");
    let req = DailyRankingRequest {
        variety_id: "a".to_string(),
        contract_id: "a2405".to_string(),
        trade_date: "20240101".to_string(),
        trade_type: "1".to_string(),
    };
    match client.member.get_daily_ranking(&req, None).await {
        Ok(rankings) => {
            if !rankings.qty_future_list.is_empty() {
                println!("Volume Ranking (Top 3):");
                for r in rankings.qty_future_list.iter().take(3) {
                    println!(
                        "  {}. {} | Volume: {} | Change: {:+}",
                        r.rank, r.qty_abbr, r.today_qty, r.qty_sub
                    );
                }
            }
            if !rankings.buy_future_list.is_empty() {
                println!("Buy Ranking (Top 3):");
                for r in rankings.buy_future_list.iter().take(3) {
                    println!(
                        "  {}. {} | Buy Volume: {} | Change: {:+}",
                        r.rank, r.buy_abbr, r.today_buy_qty, r.buy_sub
                    );
                }
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    println!("\n--- GetPhaseRanking (Soybean No.1 a) ---");
    let req = PhaseRankingRequest {
        variety: "a".to_string(),
        start_month: "202401".to_string(),
        end_month: "202401".to_string(),
        trade_type: "1".to_string(),
    };
    match client.member.get_phase_ranking(&req, None).await {
        Ok(rankings) => {
            println!("Phase ranking count: {}", rankings.len());
            for r in rankings.iter().take(3) {
                println!(
                    "  {}. {} | Monthly Volume: {:.0} | Ratio: {:.2}%",
                    r.seq, r.member_name, r.month_qty, r.qty_ratio
                );
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    Ok(())
}

// ============================================================================
// DeliveryService Examples
// ============================================================================

async fn run_delivery_service_examples(client: &Client) -> Result<()> {
    print_separator("DeliveryService Examples");

    println!("\n--- GetDeliveryData (Soybean No.1 a) ---");
    let req = DeliveryDataRequest {
        variety_code: "a".to_string(),
        trade_date: "20240101".to_string(),
    };
    match client.delivery.get_delivery_data(&req, None).await {
        Ok(data) => {
            println!("Delivery data count: {}", data.len());
            for d in data.iter().take(3) {
                println!(
                    "  Variety: {} | Month: {} | Volume: {}",
                    d.variety_code, d.delivery_month, d.delivery_volume
                );
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    println!("\n--- GetWarehouseReceipt (Soybean No.1 a) ---");
    let req = WarehouseReceiptRequest {
        variety_code: "a".to_string(),
        trade_date: "20240101".to_string(),
    };
    match client.delivery.get_warehouse_receipt(&req, None).await {
        Ok(receipts) => {
            println!("Warehouse receipt count: {}", receipts.len());
            for r in receipts.iter().take(3) {
                println!(
                    "  Variety: {} | Warehouse: {} | Qty: {}",
                    r.variety_code, r.warehouse_name, r.quantity
                );
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    println!("\n--- GetDeliveryCost (Soybean No.1 a) ---");
    match client.delivery.get_delivery_cost("a", None).await {
        Ok(cost) => {
            println!(
                "  Variety: {} | Deliv Fee: {:.2} | Insp Fee: {:.2} | Storage Fee: {:.2}",
                cost.variety_code, cost.delivery_fee, cost.inspection_fee, cost.storage_fee
            );
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    println!("\n--- GetWarehousePremium (Corn c) ---");
    match client.delivery.get_warehouse_premium("c", None).await {
        Ok(premiums) => {
            println!("Warehouse premium count: {}", premiums.len());
            for p in premiums.iter().take(3) {
                println!(
                    "  Variety: {} | Warehouse: {} | Premium: {:.2}",
                    p.variety_code, p.warehouse_name, p.premium
                );
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    Ok(())
}
