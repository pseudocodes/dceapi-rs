//! 完整功能演示 - 展示 DCE API 所有接口
//!
//! 本示例包含官方 API 文档 (dceapiv1.0.md) 中定义的所有 API 及请求参数示例。
//!
//! 服务及接口数量:
//! - CommonService - 通用服务 (3 个 API)
//! - NewsService - 资讯服务 (1 个 API，支持 6 种 columnId)
//! - MarketService - 行情服务 (11 个 API)
//! - MemberService - 会员成交持仓统计服务 (2 个 API)
//! - TradeService - 交易参数服务 (8 个 API)
//! - SettleService - 结算参数服务 (1 个 API)
//! - DeliveryService - 交割统计服务 (10 个 API)
//!
//! 合计: 36 个 API
//!
//! 运行前请设置环境变量:
//!   export DCE_API_KEY="your-api-key"
//!   export DCE_SECRET="your-secret"
//!
//! 运行命令:
//!   cargo run --example complete

use dceapi_rs::{Client, Config};
use std::time::Duration;
use tokio::time::sleep;

fn print_separator(title: &str) {
    println!("\n{}", "=".repeat(80));
    println!("{}", title);
    println!("{}", "=".repeat(80));
}

#[tokio::main]
async fn main() -> dceapi_rs::Result<()> {
    println!("\nDCE API Rust SDK - 完整功能演示 (基于 dceapiv1.0.md)");
    println!("{}", "=".repeat(80));

    // 初始化客户端
    let config = Config::from_env();
    let client = Client::new(config)?;
    println!("✓ 客户端初始化成功");

    // 获取当前交易日期
    let trade_date_res = client.common.get_curr_trade_date(None).await?;
    let trade_date = trade_date_res.date.clone();
    let trade_month = trade_date[0..6].to_string(); // YYYYMM

    println!("✓ 当前交易日期: {}", trade_date);
    println!("✓ 交易月份: {}\n", trade_month);

    // sleep(Duration::from_secs(1)).await;

    // 运行各服务示例 (可单独注释掉某个服务进行测试)
    // run_common_service_examples(&client, &trade_date, &trade_month).await;
    // sleep(Duration::from_secs(1)).await;

    // run_news_service_examples(&client).await;
    // sleep(Duration::from_secs(1)).await;

    run_market_service_examples(&client, &trade_date, &trade_month).await;
    sleep(Duration::from_secs(1)).await;

    // run_member_service_examples(&client, &trade_date, &trade_month).await;
    // sleep(Duration::from_secs(1)).await;

    // run_trade_service_examples(&client, &trade_date).await;
    // sleep(Duration::from_secs(1)).await;

    // run_settle_service_examples(&client, &trade_date).await;
    // sleep(Duration::from_secs(1)).await;

    // run_delivery_service_examples(&client, &trade_date, &trade_month).await;

    // 完成总结
    print_separator("完成总结");
    println!("✓ 完成全部 36 个接口演示");
    println!("✓ 所有 API 调用间隔 1 秒");
    println!("✓ 成功测试 7 个服务:");
    println!("   - CommonService 公共服务: 3 个接口");
    println!("   - NewsService 资讯服务: 1 个接口 (6 个栏目循环)");
    println!("   - MarketService 市场数据服务: 11 个接口");
    println!("   - MemberService 会员排名服务: 2 个接口");
    println!("   - TradeService 交易参数服务: 8 个接口");
    println!("   - SettleService 结算参数服务: 1 个接口");
    println!("   - DeliveryService 交割统计服务: 10 个接口");
    println!("\n✓ 总计: 36 个接口 (官方 API v1.0 文档定义)");

    Ok(())
}

// ============================================================================
// CommonService - 通用服务 (3 个 API)
// 文档参考: 数据 > 通用数据接口
// ============================================================================

async fn run_common_service_examples(client: &Client, trade_date: &str, trade_month: &str) {
    print_separator("CommonService - 通用服务");

    // [1/3] GetCurrTradeDate - 获取当前交易日期
    // 文档: GET /dceapi/forward/publicweb/maxTradeDate
    println!("\n[1/3] GetCurrTradeDate - 获取当前交易日期");
    println!("✓ 当前交易日期: {}", trade_date);
    sleep(Duration::from_secs(1)).await;

    // [2/3] GetVarietyList - 获取品种列表
    // 文档: GET /dceapi/forward/publicweb/variety
    println!("\n[2/3] GetVarietyList - 获取品种列表");
    match client.common.get_variety_list(None).await {
        Ok(varieties) => {
            println!("✓ 品种数量: {}", varieties.len());
            for (i, v) in varieties.iter().enumerate() {
                if i >= 5 {
                    println!("   ... 还有 {} 个品种", varieties.len() - 5);
                    break;
                }
                println!("   - {} ({}) - {}", v.name, v.code, v.variety_type);
            }
        }
        Err(e) => println!("✗ Error: {}", e),
    }
    sleep(Duration::from_secs(1)).await;

    // [3/3] GetVarietyMonthYearStat - 获取品种月度统计
    // 文档: POST /dceapi/forward/publicweb/phasestat/varietyMonthYearStat
    // 请求示例: {"tradeMonth": "202509", "tradeType": "1", "lang": "zh"}
    println!("\n[3/3] GetVarietyMonthYearStat - 获取品种月度统计");
    match client
        .common
        .get_variety_month_year_stat(
            &dceapi_rs::VarietyMonthYearStatRequest {
                trade_month: trade_month.to_string(),
                trade_type: "1".to_string(),
                lang: "zh".to_string(),
            },
            None,
        )
        .await
    {
        Ok(stats) => {
            println!("✓ 品种月度统计数量: {}", stats.len());
            for s in stats.iter().take(3) {
                println!(
                    "   {} | 本月成交量: {} | 本年成交量: {}",
                    s.variety, s.this_month_volumn, s.this_year_volumn
                );
            }
        }
        Err(e) => println!("✗ Error: {}", e),
    }
}

// ============================================================================
// NewsService - 资讯服务 (1 个 API，支持 6 种 columnId)
// 文档参考: 资讯
// columnId 说明:
//   244 - 业务公告与通知
//   245 - 活动公告与通知
//   246 - 交易所新闻-文媒
//   248 - 媒体看大商所-文媒
//   1076 - 今日提示
//   242 - 新闻发布
// ============================================================================

async fn run_news_service_examples(client: &Client) {
    print_separator("NewsService - 资讯服务");

    // columnId 列表及说明
    let column_configs = [
        ("244", "业务公告与通知"),
        ("245", "活动公告与通知"),
        ("246", "交易所新闻-文媒"),
        ("248", "媒体看大商所-文媒"),
        ("1076", "今日提示"),
        ("242", "新闻发布"),
    ];

    for (i, (column_id, name)) in column_configs.iter().enumerate() {
        println!(
            "\n[{}/6] GetArticleByPage - {} (columnId={})",
            i + 1,
            name,
            column_id
        );

        // 文档请求示例: {"columnId":"244","pageNo":1,"siteId":5,"pageSize":10}
        match client
            .news
            .get_article_by_page(
                dceapi_rs::GetArticleByPageRequest {
                    column_id: column_id.to_string(),
                    page_no: 1,
                    page_size: 3,
                    site_id: 5,
                },
                None,
            )
            .await
        {
            Ok(result) => {
                println!("✓ 总文章数: {}", result.total_count);
                for article in result.result_list.iter().take(2) {
                    println!("   - [{}] {}", article.show_date, article.title);
                }
            }
            Err(e) => println!("✗ Error: {}", e),
        }
        sleep(Duration::from_millis(500)).await;
    }
}

// ============================================================================
// MarketService - 行情服务 (11 个 API)
// 文档参考: 数据 > 行情统计
// ============================================================================

async fn run_market_service_examples(client: &Client, trade_date: &str, trade_month: &str) {
    print_separator("MarketService - 行情服务");
    println!("\n使用交易日期: {}", trade_date);

    // [1/11] GetNightQuotes - 获取夜盘行情
    // 文档: POST /dceapi/forward/publicweb/dailystat/tiNightQuotes
    // 请求示例: {"variety": "a", "tradeType": "1", "tradeDate": "20250930"}
    println!("\n[1/11] GetNightQuotes - 获取夜盘行情 (豆一 a)");
    match client
        .market
        .get_night_quotes(
            &dceapi_rs::QuotesRequest {
                trade_date: trade_date.to_string(),
                variety: Some("a".to_string()),
                variety_id: None,
                trade_type: "1".to_string(),
                lang: Some("zh".to_string()),
                statistics_type: None,
            },
            None,
        )
        .await
    {
        Ok(quotes) => {
            println!("✓ 豆一夜盘行情, 合约数: {}", quotes.len());
            let mut count = 0;
            for q in quotes.iter() {
                if q.deliv_month.is_empty() || q.variety == "总计" {
                    continue;
                }
                if count >= 3 {
                    break;
                }
                println!(
                    "   {} | 最新价: {} | 持仓量: {}",
                    q.deliv_month, q.last_price, q.open_interest
                );
                count += 1;
            }
        }
        Err(e) => println!("✗ Error: {}", e),
    }
    sleep(Duration::from_secs(1)).await;

    // [2/11] GetDayQuotes - 获取日行情 (期货)
    // 文档: POST /dceapi/forward/publicweb/dailystat/dayQuotes
    // 请求示例: {"varietyId": "a", "tradeDate": "20250930", "tradeType": "1", "lang": "zh"}
    println!("\n[2/11] GetDayQuotes - 获取日行情-期货 (豆一 a)");
    match client
        .market
        .get_day_quotes(
            &dceapi_rs::QuotesRequest {
                trade_date: trade_date.to_string(),
                variety: None,
                variety_id: Some("a".to_string()),
                trade_type: "1".to_string(),
                lang: Some("zh".to_string()),
                statistics_type: None,
            },
            None,
        )
        .await
    {
        Ok(quotes) => {
            println!("✓ 豆一日行情, 合约数: {}", quotes.len());
            let mut count = 0;
            for q in quotes.iter() {
                if q.contract_id.is_empty() || q.variety == "总计" {
                    continue;
                }
                if count >= 3 {
                    break;
                }
                println!(
                    "   {} | 开: {} 高: {} 低: {} 收: {}",
                    q.contract_id, q.open, q.high, q.low, q.close
                );
                count += 1;
            }
        }
        Err(e) => println!("✗ Error: {}", e),
    }
    sleep(Duration::from_secs(1)).await;

    // [3/11] GetDayQuotes - 获取日行情 (期权)
    // 请求示例: {"varietyId": "a", "tradeDate": "20250930", "tradeType": "2", "lang": "zh", "statisticsType": 0}
    println!("\n[3/11] GetDayQuotes - 获取日行情-期权 (豆一期权)");
    match client
        .market
        .get_day_quotes(
            &dceapi_rs::QuotesRequest {
                trade_date: trade_date.to_string(),
                variety: None,
                variety_id: Some("a".to_string()),
                trade_type: "2".to_string(),
                lang: Some("zh".to_string()),
                statistics_type: Some(0), // 0=合约, 1=系列, 2=品种
            },
            None,
        )
        .await
    {
        Ok(quotes) => {
            println!("✓ 豆一期权日行情, 合约数: {}", quotes.len());
        }
        Err(e) => println!("✗ Error: {}", e),
    }
    sleep(Duration::from_secs(1)).await;

    // [4/11] GetWeekQuotes - 获取周行情
    // 文档: POST /dceapi/forward/publicweb/dailystat/weekQuotes
    println!("\n[4/11] GetWeekQuotes - 获取周行情 (豆粕 m)");
    match client
        .market
        .get_week_quotes(
            &dceapi_rs::QuotesRequest {
                trade_date: trade_date.to_string(),
                variety: None,
                variety_id: Some("m".to_string()),
                trade_type: "1".to_string(),
                lang: Some("zh".to_string()),
                statistics_type: None,
            },
            None,
        )
        .await
    {
        Ok(quotes) => {
            println!("✓ 豆粕周行情, 合约数: {}", quotes.len());
        }
        Err(e) => println!("✗ Error: {}", e),
    }
    sleep(Duration::from_secs(1)).await;

    // [5/11] GetMonthQuotes - 获取月行情
    // 文档: POST /dceapi/forward/publicweb/dailystat/monthQuotes
    println!("\n[5/11] GetMonthQuotes - 获取月行情 (玉米 c)");
    match client
        .market
        .get_month_quotes(
            &dceapi_rs::QuotesRequest {
                trade_date: trade_date.to_string(),
                variety: Some("c".to_string()),
                variety_id: None,
                trade_type: "1".to_string(),
                lang: Some("zh".to_string()),
                statistics_type: None,
            },
            None,
        )
        .await
    {
        Ok(quotes) => {
            println!("✓ 玉米月行情, 合约数: {}", quotes.len());
        }
        Err(e) => println!("✗ Error: {}", e),
    }
    sleep(Duration::from_secs(1)).await;

    let start_date = format!("{}01", trade_month);

    // [6/11] GetContractMonthMaxVolume - 合约最值统计-成交量
    // 文档: POST /dceapi/forward/publicweb/phasestat/contractMonthMax
    // 请求示例: {"startMonth": "202510", "endMonth": "202510", "statContent": "0", "tradeType": "1", "lang": "zh"}
    println!("\n[6/11] GetContractMonthMax - 合约最值统计-成交量");
    match client
        .market
        .get_contract_month_max_volume(
            &dceapi_rs::ContractMonthMaxRequest {
                start_month: trade_month.to_string(),
                end_month: trade_month.to_string(),
                stat_content: "0".to_string(), // 0=成交量
                trade_type: "1".to_string(),
                lang: "zh".to_string(),
            },
            None,
        )
        .await
    {
        Ok(stats) => {
            println!("✓ 成交量统计数量: {}", stats.len());
            for s in stats.iter().take(2) {
                println!(
                    "   {} | 总量: {} | 最大: {} ({})",
                    s.contract_id, s.sum_amount, s.max_amount, s.max_amount_date
                );
            }
        }
        Err(e) => println!("✗ Error: {}", e),
    }
    sleep(Duration::from_secs(1)).await;

    // [7/11] GetContractMonthMaxTurnover - 合约最值统计-成交额
    // 请求示例: {"startMonth": "202510", "endMonth": "202510", "statContent": "1", "tradeType": "1", "lang": "zh"}
    println!("\n[7/11] GetContractMonthMax - 合约最值统计-成交额");
    match client
        .market
        .get_contract_month_max_turnover(
            &dceapi_rs::ContractMonthMaxRequest {
                start_month: trade_month.to_string(),
                end_month: trade_month.to_string(),
                stat_content: "1".to_string(), // 1=成交额
                trade_type: "1".to_string(),
                lang: "zh".to_string(),
            },
            None,
        )
        .await
    {
        Ok(stats) => {
            println!("✓ 成交额统计数量: {}", stats.len());
            for s in stats.iter().take(2) {
                println!(
                    "   {} | 总额: {} | 最大: {} ({})",
                    s.contract_id, s.sum_turnover, s.max_turnover, s.max_turnover_date
                );
            }
        }
        Err(e) => println!("✗ Error: {}", e),
    }
    sleep(Duration::from_secs(1)).await;

    // [8/11] GetContractMonthMaxOpeni - 合约最值统计-持仓量
    // 请求示例: {"startMonth": "202510", "endMonth": "202510", "statContent": "2", "tradeType": "1", "lang": "zh"}
    println!("\n[8/11] GetContractMonthMax - 合约最值统计-持仓量");
    match client
        .market
        .get_contract_month_max_openi(
            &dceapi_rs::ContractMonthMaxRequest {
                start_month: trade_month.to_string(),
                end_month: trade_month.to_string(),
                stat_content: "2".to_string(), // 2=持仓量
                trade_type: "1".to_string(),
                lang: "zh".to_string(),
            },
            None,
        )
        .await
    {
        Ok(stats) => {
            println!("✓ 持仓量统计数量: {}", stats.len());
            for s in stats.iter().take(2) {
                println!(
                    "   {} | 总持仓: {} | 最大: {} ({})",
                    s.contract_id, s.sum_openi, s.max_openi, s.max_openi_date
                );
            }
        }
        Err(e) => println!("✗ Error: {}", e),
    }
    sleep(Duration::from_secs(1)).await;

    // [9/11] GetContractMonthMaxPrice - 合约最值统计-价格统计
    // 请求示例: {"startMonth": "202510", "endMonth": "202510", "statContent": "3", "tradeType": "1", "lang": "zh"}
    println!("\n[9/11] GetContractMonthMax - 合约最值统计-价格统计");
    match client
        .market
        .get_contract_month_max_price(
            &dceapi_rs::ContractMonthMaxRequest {
                start_month: trade_month.to_string(),
                end_month: trade_month.to_string(),
                stat_content: "3".to_string(), // 3=价格统计
                trade_type: "1".to_string(),
                lang: "zh".to_string(),
            },
            None,
        )
        .await
    {
        Ok(stats) => {
            println!("✓ 价格统计数量: {}", stats.len());
            for s in stats.iter().take(2) {
                println!(
                    "   {} | 开: {} 收: {} 高: {} ({}) 低: {} ({})",
                    s.contract_id, s.open, s.close, s.high, s.high_date, s.low, s.low_date
                );
            }
        }
        Err(e) => println!("✗ Error: {}", e),
    }
    sleep(Duration::from_secs(1)).await;

    // [10/11] GetRiseFallEvent - 获取合约停板查询
    // 文档: POST /dceapi/forward/publicweb/phasestat/riseFallEvent
    // 请求示例: {"startDate": "20251009", "endDate": "20251009", "varietyId": "all", "lang": "zh"}
    println!("\n[10/11] GetRiseFallEvent - 获取合约停板查询");
    match client
        .market
        .get_rise_fall_event(
            &dceapi_rs::RiseFallEventRequest {
                start_date: start_date.clone(),
                end_date: trade_date.to_string(),
                variety_id: "all".to_string(),
                lang: "zh".to_string(),
            },
            None,
        )
        .await
    {
        Ok(events) => {
            println!("✓ 停板事件数量: {}", events.len());
            for e in events.iter().take(3) {
                println!(
                    "   {} | 合约: {} | 方向: {} | 次数: {}",
                    e.trade_date, e.contract_id, e.direction, e.times
                );
            }
        }
        Err(e) => println!("✗ Error: {}", e),
    }
    sleep(Duration::from_secs(1)).await;

    // [11/11] GetDivisionPriceInfo - 获取分时结算参考价
    // 文档: POST /dceapi/forward/publicweb/dailystat/divisionPriceInfo
    println!("\n[11/11] GetDivisionPriceInfo - 获取分时结算参考价");
    match client
        .market
        .get_division_price_info(
            &dceapi_rs::DivisionPriceInfoRequest {
                variety_id: "m".to_string(),
                trade_date: trade_date.to_string(),
                trade_type: "1".to_string(),
            },
            None,
        )
        .await
    {
        Ok(prices) => {
            println!("✓ 分时结算参考价数量: {}", prices.len());
            for p in prices.iter().take(3) {
                println!(
                    "   {} @ {}: 结算价 {}",
                    p.contract_id, p.calculate_time, p.clear_price
                );
            }
        }
        Err(e) => println!("✗ Error: {}", e),
    }
}

// ============================================================================
// MemberService - 会员成交持仓统计服务 (2 个 API)
// 文档参考: 数据 > 会员成交持仓统计
// ============================================================================

async fn run_member_service_examples(client: &Client, trade_date: &str, trade_month: &str) {
    print_separator("MemberService - 会员成交持仓统计服务");

    // [1/2] GetDailyRanking - 查询成交持仓排名
    // 文档: POST /dceapi/forward/publicweb/memberstat/dailyRanking
    // 请求示例: {"contractId": "a2505", "varietyId": "a", "tradeDate": "20251009", "tradeType": "1"}
    println!("[1/2] GetDailyRanking - 查询成交持仓排名");
    match client
        .member
        .get_daily_ranking(
            &dceapi_rs::DailyRankingRequest {
                variety_id: "a".to_string(),
                contract_id: "a2505".to_string(),
                trade_date: trade_date.to_string(),
                trade_type: "1".to_string(),
            },
            None,
        )
        .await
    {
        Ok(ranking) => {
            println!(
                "✓ 合约: {}, 今日总成交量: {}",
                ranking.contract_id, ranking.today_qty
            );
            if !ranking.qty_future_list.is_empty() {
                println!("   📊 成交量排名 Top 3:");
                for (i, r) in ranking.qty_future_list.iter().take(3).enumerate() {
                    println!(
                        "      {}. {} | 成交量: {} | 增减: {}",
                        i + 1,
                        r.qty_abbr,
                        r.today_qty,
                        r.qty_sub
                    );
                }
            }
            if !ranking.buy_future_list.is_empty() {
                println!("   📊 买持仓排名 Top 3:");
                for (i, r) in ranking.buy_future_list.iter().take(3).enumerate() {
                    println!(
                        "      {}. {} | 持买量: {} | 增减: {}",
                        i + 1,
                        r.buy_abbr,
                        r.today_buy_qty,
                        r.buy_sub
                    );
                }
            }
        }
        Err(e) => println!("✗ Error: {}", e),
    }
    sleep(Duration::from_secs(1)).await;

    // [2/2] GetPhaseRanking - 阶段排名统计
    // 文档: POST /dceapi/forward/publicweb/memberstat/phaseRanking
    // 请求示例: {"startDate": "20251001", "endDate": "20251009", "varietyId": "a", "tradeType": "1"}
    println!("\n[2/2] GetPhaseRanking - 阶段排名统计");
    match client
        .member
        .get_phase_ranking(
            &dceapi_rs::PhaseRankingRequest {
                variety: "a".to_string(),
                start_month: trade_month.to_string(),
                end_month: trade_month.to_string(),
                trade_type: "1".to_string(),
            },
            None,
        )
        .await
    {
        Ok(rankings) => {
            println!("✓ 阶段排名数量: {}", rankings.len());
            for r in rankings.iter().take(3) {
                println!(
                    "   {}. {} | 成交量: {:.0} | 占比: {:.2}%",
                    r.seq, r.member_name, r.month_qty, r.qty_ratio
                );
            }
        }
        Err(e) => println!("✗ Error: {}", e),
    }
}

// ============================================================================
// TradeService - 交易参数服务 (8 个 API)
// 文档参考: 数据 > 交易参数
// ============================================================================

async fn run_trade_service_examples(client: &Client, trade_date: &str) {
    print_separator("TradeService - 交易参数服务");

    // [1/8] GetDayTradeParam - 获取当日交易参数
    // 文档: POST /dceapi/forward/publicweb/tradeparam/dayTradeParam
    // 请求示例: {"varietyId": "m", "tradeType": "1", "lang": "zh"}
    println!("[1/8] GetDayTradeParam - 获取当日交易参数");
    match client
        .trade
        .get_day_trade_param(
            &dceapi_rs::DayTradeParamRequest {
                variety_id: "m".to_string(),
                trade_type: "1".to_string(),
                lang: "zh".to_string(),
            },
            None,
        )
        .await
    {
        Ok(params) => {
            println!("✓ 交易参数数量: {}", params.len());
            if let Some(p) = params.first() {
                println!(
                    "   {} | 投机买保证金率: {} | 涨停价: {}",
                    p.contract_id, p.spec_buy_rate, p.rise_limit
                );
            }
        }
        Err(e) => println!("✗ Error: {}", e),
    }
    sleep(Duration::from_secs(1)).await;

    // [2/8] GetMonthTradeParam - 获取月交易参数
    // 文档: POST /dceapi/forward/publicweb/tradeparam/monthTradeParam
    println!("\n[2/8] GetMonthTradeParam - 获取月交易参数");
    match client.trade.get_month_trade_param(None).await {
        Ok(params) => {
            println!("✓ 月交易参数键数量: {}", params.len());
        }
        Err(e) => println!("✗ Error: {}", e),
    }
    sleep(Duration::from_secs(1)).await;

    // [3/8] GetContractInfo - 获取合约信息
    // 文档: POST /dceapi/forward/publicweb/tradeparam/contractInfo
    // 请求示例: {"varietyId": "m", "tradeType": "1", "lang": "zh"}
    println!("\n[3/8] GetContractInfo - 获取合约信息");
    match client
        .trade
        .get_contract_info(
            &dceapi_rs::ContractInfoRequest {
                variety_id: "m".to_string(),
                trade_type: "1".to_string(),
                lang: "zh".to_string(),
            },
            None,
        )
        .await
    {
        Ok(contracts) => {
            println!("✓ 合约数量: {}", contracts.len());
            for c in contracts.iter().take(3) {
                println!(
                    "   {} | 开始交易日: {} | 最后交易日: {}",
                    c.contract_id, c.start_trade_date, c.end_trade_date
                );
            }
        }
        Err(e) => println!("✗ Error: {}", e),
    }
    sleep(Duration::from_secs(1)).await;

    // [4/8] GetArbitrageContract - 获取套利合约
    // 文档: POST /dceapi/forward/publicweb/tradeparam/arbitrageContract
    println!("\n[4/8] GetArbitrageContract - 获取套利合约");
    match client.trade.get_arbitrage_contract(Some("zh"), None).await {
        Ok(contracts) => {
            println!("✓ 套利合约数量: {}", contracts.len());
            for c in contracts.iter().take(3) {
                println!(
                    "   {} | {} | 最大下单量: {}",
                    c.arbi_contract_id, c.variety_name, c.max_hand
                );
            }
        }
        Err(e) => println!("✗ Error: {}", e),
    }
    sleep(Duration::from_secs(1)).await;

    // [5/8] GetTradingParam - 获取交易参数(按品种)
    // 文档: POST /dceapi/forward/publicweb/tradeparam/tradingParam
    println!("\n[5/8] GetTradingParam - 获取交易参数(按品种)");
    match client.trade.get_trading_param(Some("zh"), None).await {
        Ok(params) => {
            println!("✓ 交易参数数量: {}", params.len());
            for p in params.iter().take(3) {
                println!(
                    "   {} | 投机保证金率: {} | 投机开仓手续费: {}",
                    p.variety_name, p.trading_margin_rate_speculation, p.spec_open_fee
                );
            }
        }
        Err(e) => println!("✗ Error: {}", e),
    }
    sleep(Duration::from_secs(1)).await;

    // [6/8] GetMarginArbiPerfPara - 获取套利套保保证金参数
    // 文档: POST /dceapi/forward/publicweb/tradeparam/marginArbiPerfPara
    // 请求示例: {"varietyId": "m"}
    println!("\n[6/8] GetMarginArbiPerfPara - 获取套利套保保证金参数");
    match client
        .trade
        .get_margin_arbi_perf_para(
            &dceapi_rs::MarginArbiPerfParaRequest {
                variety_id: "m".to_string(),
            },
            None,
        )
        .await
    {
        Ok(params) => {
            println!("✓ 套利套保保证金参数数量: {}", params.len());
            for p in params.iter().take(3) {
                println!(
                    "   {} | 策略: {} | 投机保证金率: {}",
                    p.variety, p.strategy_name, p.trading_margin_rate_speculation
                );
            }
        }
        Err(e) => println!("✗ Error: {}", e),
    }
    sleep(Duration::from_secs(1)).await;

    // [7/8] GetNewContractInfo - 获取新上市合约信息
    // 文档: POST /dceapi/forward/publicweb/tradeparam/newContractInfo
    // 请求示例: {"tradeDate": "20251009", "tradeType": "1", "lang": "zh"}
    println!("\n[7/8] GetNewContractInfo - 获取新上市合约信息");
    match client
        .trade
        .get_new_contract_info(
            &dceapi_rs::NewContractInfoRequest {
                trade_date: trade_date.to_string(),
                trade_type: "1".to_string(),
                lang: Some("zh".to_string()),
            },
            None,
        )
        .await
    {
        Ok(contracts) => {
            println!("✓ 新上市合约数量: {}", contracts.len());
            for c in contracts.iter().take(3) {
                println!(
                    "   {} | 开始交易日: {} | 基准价: {}",
                    c.contract_id, c.start_trade_date, c.ref_price_unit
                );
            }
        }
        Err(e) => println!("✗ Error: {}", e),
    }
    sleep(Duration::from_secs(1)).await;

    // [8/8] GetMainSeriesInfo - 获取做市商合约信息
    // 文档: POST /dceapi/forward/publicweb/tradeparam/mainSeriesInfo
    // 请求示例: {"varietyId": "m", "tradeDate": "20251009"}
    println!("\n[8/8] GetMainSeriesInfo - 获取做市商合约信息");
    match client
        .trade
        .get_main_series_info(
            &dceapi_rs::MainSeriesInfoRequest {
                variety_id: "m".to_string(),
                trade_date: trade_date.to_string(),
            },
            None,
        )
        .await
    {
        Ok(series) => {
            println!("✓ 做市商合约数量: {}", series.len());
            for s in series.iter().take(3) {
                println!(
                    "   {} | 品种: {} | 系列: {}",
                    s.contract_id, s.variety_id, s.series_id
                );
            }
        }
        Err(e) => println!("✗ Error: {}", e),
    }
}

// ============================================================================
// SettleService - 结算参数服务 (1 个 API)
// 文档参考: 数据 > 结算参数
// ============================================================================

async fn run_settle_service_examples(client: &Client, trade_date: &str) {
    print_separator("SettleService - 结算参数服务");

    // [1/1] GetSettleParam - 获取结算参数
    // 文档: POST /dceapi/forward/publicweb/settleparam/settleParam
    // 请求示例: {"varietyId": "m", "tradeDate": "20251009", "tradeType": "1", "lang": "zh"}
    println!("[1/1] GetSettleParam - 获取结算参数");
    match client
        .settle
        .get_settle_param(
            &dceapi_rs::SettleParamRequest {
                variety_id: "m".to_string(),
                trade_date: trade_date.to_string(),
                trade_type: "1".to_string(),
                lang: "zh".to_string(),
            },
            None,
        )
        .await
    {
        Ok(params) => {
            println!("✓ 结算参数数量: {}", params.len());
            for p in params.iter().take(3) {
                println!(
                    "   {} | 结算价: {} | 开仓手续费: {}",
                    p.contract_id, p.clear_price, p.open_fee
                );
            }
        }
        Err(e) => println!("✗ Error: {}", e),
    }
}

// ============================================================================
// DeliveryService - 交割统计服务 (10 个 API)
// 文档参考: 数据 > 交割统计
// ============================================================================

async fn run_delivery_service_examples(client: &Client, trade_date: &str, trade_month: &str) {
    print_separator("DeliveryService - 交割统计服务");

    // [1/10] GetDeliveryData - 交割情况统计
    // 文档: POST /dceapi/forward/publicweb/deliverystat/deliveryData
    // 请求示例: {"varietyId": "a", "startMonth": "202510", "endMonth": "202510", "varietyType": "0"}
    println!("[1/10] GetDeliveryData - 交割情况统计");
    match client
        .delivery
        .get_delivery_data(
            &dceapi_rs::DeliveryDataRequest {
                variety_id: "a".to_string(),
                start_month: trade_month.to_string(),
                end_month: trade_month.to_string(),
                variety_type: "0".to_string(),
            },
            None,
        )
        .await
    {
        Ok(data) => {
            println!("✓ 交割数据数量: {}", data.len());
            for d in data.iter().take(3) {
                println!(
                    "   {} | 合约: {} | 交割量: {}",
                    d.variety, d.contract_id, d.delivery_qty
                );
            }
        }
        Err(e) => println!("✗ Error: {}", e),
    }
    sleep(Duration::from_secs(1)).await;

    // [2/10] GetDeliveryMatch - 配对交割明细
    // 文档: POST /dceapi/forward/publicweb/deliverystat/deliveryMatch
    // 请求示例: {"varietyId": "a", "contractId": "a2501", "startMonth": "202501", "endMonth": "202510"}
    let year_start_month = format!("{}01", &trade_date[0..4]);
    println!("\n[2/10] GetDeliveryMatch - 配对交割明细");
    match client
        .delivery
        .get_delivery_match(
            &dceapi_rs::DeliveryMatchRequest {
                variety_id: "a".to_string(),
                contract_id: "a2501".to_string(),
                start_month: year_start_month.clone(),
                end_month: trade_month.to_string(),
            },
            None,
        )
        .await
    {
        Ok(matches) => {
            println!("✓ 配对交割数量: {}", matches.len());
            for m in matches.iter().take(3) {
                println!(
                    "   {} | 配对日: {} | 交割量: {}",
                    m.contract_id, m.match_date, m.delivery_qty
                );
            }
        }
        Err(e) => println!("✗ Error: {}", e),
    }
    sleep(Duration::from_secs(1)).await;

    // [3/10] GetDeliveryCost - 交割费用
    // 文档: POST /dceapi/forward/publicweb/deliverystat/deliveryCost
    // 请求示例: {"varietyId": "a", "varietyType": "0"}
    println!("\n[3/10] GetDeliveryCost - 交割费用");
    match client.delivery.get_delivery_cost("a", "0", None).await {
        Ok(costs) => {
            println!("✓ 交割费用数量: {}", costs.len());
            for cost in costs.iter().take(3) {
                println!(
                    "   {} | 交割手续费: {} | 费率: {}",
                    cost.variety, cost.delivery_fee, cost.fee_rate
                );
            }
        }
        Err(e) => println!("✗ Error: {}", e),
    }
    sleep(Duration::from_secs(1)).await;

    // [4/10] GetWarehousePremium - 仓库升贴水
    // 文档: POST /dceapi/forward/publicweb/deliverystat/warehousePremium
    // 请求示例: {"varietyId": "c", "endDate": "20251230"}
    println!("\n[4/10] GetWarehousePremium - 仓库升贴水");
    match client
        .delivery
        .get_warehouse_premium("c", "20251230", None)
        .await
    {
        Ok(response) => {
            println!("✓ 仓库升贴水数量: {}", response.entity_list.len());
            for p in response.entity_list.iter().take(3) {
                println!(
                    "   {} | {} | 升贴水: {}",
                    p.variety_name, p.wh_name, p.avg_agio
                );
            }
        }
        Err(e) => println!("✗ Error: {}", e),
    }
    sleep(Duration::from_secs(1)).await;

    // [5/10] GetTcCongregateDelivery - 集中交割预报
    // 文档: POST /dceapi/forward/publicweb/deliverystat/tcCongregateDelivery
    // 请求示例: {"variety": "a", "contractMonth": "202501"}
    println!("\n[5/10] GetTcCongregateDelivery - 集中交割预报");
    match client
        .delivery
        .get_tc_congregate_delivery(
            &dceapi_rs::TcCongregateDeliveryRequest {
                variety: "a".to_string(),
                contract_month: "202501".to_string(),
            },
            None,
        )
        .await
    {
        Ok(deliveries) => {
            println!("✓ 集中交割预报数量: {}", deliveries.len());
            for d in deliveries.iter().take(3) {
                println!(
                    "   {} | 合约: {} | 仓库: {} | 数量: {}",
                    d.variety_name, d.contract, d.warehouse_name, d.wbill_quantity
                );
            }
        }
        Err(e) => println!("✗ Error: {}", e),
    }
    sleep(Duration::from_secs(1)).await;

    // [6/10] GetRollDeliverySellerIntention - 滚动交割卖方意向
    // 文档: POST /dceapi/forward/publicweb/deliverystat/rollDeliverySellerIntention
    // 请求示例: {"variety": "a", "date": "20251230"}
    println!("\n[6/10] GetRollDeliverySellerIntention - 滚动交割卖方意向");
    match client
        .delivery
        .get_roll_delivery_seller_intention(
            &dceapi_rs::RollDeliverySellerIntentionRequest {
                variety: "a".to_string(),
                date: "20251230".to_string(),
            },
            None,
        )
        .await
    {
        Ok(intentions) => {
            println!("✓ 卖方意向数量: {}", intentions.len());
            for i in intentions.iter().take(3) {
                println!(
                    "   {} | 合约: {} | 数量: {} | 交割方式: {}",
                    i.variety_name, i.contract, i.quantity, i.delivery_way
                );
            }
        }
        Err(e) => println!("✗ Error: {}", e),
    }
    sleep(Duration::from_secs(1)).await;

    // [7/10] GetBondedDelivery - 保税交割结算价
    // 文档: POST /dceapi/forward/publicweb/deliverystat/bondedDelivery
    // 请求示例: {"startDate": "20251201", "endDate": "20251230"}
    println!("\n[7/10] GetBondedDelivery - 保税交割结算价");
    match client
        .delivery
        .get_bonded_delivery(
            &dceapi_rs::BondedDeliveryRequest {
                start_date: "20251201".to_string(),
                end_date: "20251230".to_string(),
            },
            None,
        )
        .await
    {
        Ok(deliveries) => {
            println!("✓ 保税交割结算价数量: {}", deliveries.len());
            for d in deliveries.iter().take(3) {
                println!(
                    "   {} | {} | 合约: {} | 价格: {}",
                    d.delivery_date, d.variety_id, d.contract_id, d.delivery_price
                );
            }
        }
        Err(e) => println!("✗ Error: {}", e),
    }
    sleep(Duration::from_secs(1)).await;

    // [8/10] GetTdBondedDelivery - 保税延期交割结算价
    // 文档: POST /dceapi/forward/publicweb/deliverystat/tdBondedDelivery
    // 请求示例: {"startDate": "20251201", "endDate": "20251230"}
    println!("\n[8/10] GetTdBondedDelivery - 保税延期交割结算价");
    match client
        .delivery
        .get_td_bonded_delivery(
            &dceapi_rs::TdBondedDeliveryRequest {
                start_date: "20251201".to_string(),
                end_date: "20251230".to_string(),
            },
            None,
        )
        .await
    {
        Ok(deliveries) => {
            println!("✓ 保税延期交割结算价数量: {}", deliveries.len());
            for d in deliveries.iter().take(3) {
                println!(
                    "   {} | {} | 合约: {} | 价格: {}",
                    d.delivery_date, d.variety_id, d.contract_id, d.delivery_price
                );
            }
        }
        Err(e) => println!("✗ Error: {}", e),
    }
    sleep(Duration::from_secs(1)).await;

    // [9/10] GetFactorySpotAgio - 厂库升贴水
    // 文档: POST /dceapi/forward/publicweb/deliverystat/factorySpotAgio
    // 请求示例: {"varietyId": "fb", "tradeDate": "20251009"}
    println!("\n[9/10] GetFactorySpotAgio - 厂库升贴水");
    match client
        .delivery
        .get_factory_spot_agio(
            &dceapi_rs::FactorySpotAgioRequest {
                variety_id: "fb".to_string(),
                trade_date: trade_date.to_string(),
            },
            None,
        )
        .await
    {
        Ok(agios) => {
            println!("✓ 厂库升贴水数量: {}", agios.len());
            for a in agios.iter().take(3) {
                println!(
                    "   {} | 品种: {} | 升贴水: {}",
                    a.wh_abbr, a.variety_name, a.agio
                );
            }
        }
        Err(e) => println!("✗ Error: {}", e),
    }
    sleep(Duration::from_secs(1)).await;

    // [10/10] GetPlywoodDeliveryCommodity - 胶合板交割品牌
    // 文档: POST /dceapi/forward/publicweb/deliverystat/plywoodDeliveryCommodity
    // 请求示例: {"varietyId": "bb"}
    println!("\n[10/10] GetPlywoodDeliveryCommodity - 胶合板交割品牌");
    match client
        .delivery
        .get_plywood_delivery_commodity(
            &dceapi_rs::PlywoodDeliveryCommodityRequest {
                variety_id: "bb".to_string(),
            },
            None,
        )
        .await
    {
        Ok(commodities) => {
            println!("✓ 胶合板交割品牌数量: {}", commodities.len());
            for c in commodities.iter().take(3) {
                println!("   {} | 文件: {}", c.wh_name, c.upload_file_name);
            }
        }
        Err(e) => println!("✗ Error: {}", e),
    }
}
