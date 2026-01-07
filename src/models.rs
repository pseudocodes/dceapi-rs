//! Data models for the DCE API.
//!
//! This module contains all request and response structures used by the API.

use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;

/// Helper function to deserialize a nullable string as an empty string.
fn deserialize_nullable_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let opt: Option<String> = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or_default())
}

// ============================================================================
// Common Response Models
// ============================================================================

/// API common response wrapper.
#[derive(Debug, Clone, Deserialize)]
pub struct ApiResponse {
    /// Response code.
    pub code: i32,
    /// Response message (field name is "msg" in the API).
    #[serde(alias = "message", default)]
    pub msg: String,
    /// Response data (raw JSON).
    #[serde(default)]
    pub data: Value,
}

/// Token response from authentication endpoint.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenResponse {
    /// Token type (e.g., "Bearer").
    pub token_type: String,
    /// Access token.
    #[serde(rename = "token")]
    pub access_token: String,
    /// Token expiry time in seconds.
    pub expires_in: i32,
}

// ============================================================================
// News/Article Models (资讯数据模型)
// ============================================================================

/// Article information.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Article {
    /// Article ID.
    #[serde(default, deserialize_with = "deserialize_nullable_string")]
    pub id: String,
    /// Article title.
    #[serde(default, deserialize_with = "deserialize_nullable_string")]
    pub title: String,
    /// Article subtitle.
    #[serde(default, deserialize_with = "deserialize_nullable_string")]
    pub sub_title: String,
    /// Article summary.
    #[serde(
        rename = "infoSummary",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub summary: String,
    /// Display date.
    #[serde(default, deserialize_with = "deserialize_nullable_string")]
    pub show_date: String,
    /// Creation date.
    #[serde(default, deserialize_with = "deserialize_nullable_string")]
    pub create_date: String,
    /// Article content.
    #[serde(default, deserialize_with = "deserialize_nullable_string")]
    pub content: String,
    /// Keywords.
    #[serde(default, deserialize_with = "deserialize_nullable_string")]
    pub keywords: String,
    /// Page name.
    #[serde(default, deserialize_with = "deserialize_nullable_string")]
    pub page_name: String,
}

/// Article detail (same as Article).
pub type ArticleDetail = Article;

/// Request for paginated article list.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetArticleByPageRequest {
    /// Column ID (e.g., "244" for announcements).
    pub column_id: String,
    /// Page number (1-indexed).
    pub page_no: i32,
    /// Page size.
    pub page_size: i32,
    /// Site ID. Defaults to 5.
    pub site_id: i32,
}

/// Response for paginated article list.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetArticleByPageResponse {
    /// Column ID.
    pub column_id: String,
    /// Total count of articles.
    pub total_count: i32,
    /// List of articles.
    pub result_list: Vec<Article>,
}

// ============================================================================
// Common Data Models (通用数据模型)
// ============================================================================

/// Trade date information.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradeDate {
    /// Trade date string.
    #[serde(
        rename = "tradeDate",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub date: String,
}

/// Variety (commodity) information.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Variety {
    /// Variety code/ID.
    #[serde(
        rename = "varietyId",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub code: String,
    /// Variety name in Chinese.
    #[serde(
        rename = "varietyName",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub name: String,
    /// Variety name in English.
    #[serde(
        rename = "varietyEnglishName",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub english_name: String,
    /// Picture URL.
    #[serde(default, deserialize_with = "deserialize_nullable_string")]
    pub pic: String,
    /// Variety type.
    #[serde(
        rename = "varietyType",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub variety_type: String,
}

// ============================================================================
// Market Data Models (行情数据模型)
// ============================================================================

/// Quote data for a contract.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Quote {
    /// Variety code.
    #[serde(default, deserialize_with = "deserialize_nullable_string")]
    pub variety: String,
    /// Contract ID.
    #[serde(
        rename = "contractId",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub contract_id: String,
    /// Delivery month (for night quotes).
    #[serde(
        rename = "delivMonth",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub deliv_month: String,
    /// Open price.
    #[serde(default, deserialize_with = "deserialize_nullable_string")]
    pub open: String,
    /// High price.
    #[serde(default, deserialize_with = "deserialize_nullable_string")]
    pub high: String,
    /// Low price.
    #[serde(default, deserialize_with = "deserialize_nullable_string")]
    pub low: String,
    /// Close price.
    #[serde(default, deserialize_with = "deserialize_nullable_string")]
    pub close: String,
    /// Last clearing price.
    #[serde(
        rename = "lastClear",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub last_clear: String,
    /// Last price (for night quotes).
    #[serde(
        rename = "lastPrice",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub last_price: String,
    /// Clearing/settlement price.
    #[serde(
        rename = "clearPrice",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub clear_price: String,
    /// Price difference.
    #[serde(default, deserialize_with = "deserialize_nullable_string")]
    pub diff: String,
    /// Price difference 1.
    #[serde(default, deserialize_with = "deserialize_nullable_string")]
    pub diff1: String,
    /// Volume.
    #[serde(rename = "volumn", default)]
    pub volume: i64,
    /// Open interest.
    #[serde(rename = "openInterest", default)]
    pub open_interest: i64,
    /// Open interest difference.
    #[serde(rename = "diffI", default)]
    pub diff_i: i64,
    /// Turnover.
    #[serde(default, deserialize_with = "deserialize_nullable_string")]
    pub turnover: String,
}

/// Request for day/night quotes.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuotesRequest {
    /// Variety ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variety_id: Option<String>,
    /// Variety code (for night quotes).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variety: Option<String>,
    /// Trade date (YYYYMMDD format).
    pub trade_date: String,
    /// Trade type ("1" = futures, "2" = options).
    pub trade_type: String,
    /// Language.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
    /// Statistics type for options: 0=contract, 1=series, 2=variety.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistics_type: Option<i32>,
}

/// Request for weekly quotes.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WeekQuotesRequest {
    /// Variety code.
    pub variety_code: String,
    /// Year.
    pub year: i32,
    /// Week number.
    pub week: i32,
}

/// Request for monthly quotes.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MonthQuotesRequest {
    /// Variety code.
    pub variety_code: String,
    /// Year.
    pub year: i32,
    /// Month.
    pub month: i32,
}

/// Request for contract statistics.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContractStatRequest {
    /// Contract code.
    pub contract_code: String,
    /// Start date.
    pub start_date: String,
    /// End date.
    pub end_date: String,
}

/// Contract statistics response.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContractStat {
    /// Contract code.
    pub contract_code: String,
    /// Total volume.
    pub total_volume: i64,
    /// Average price.
    pub avg_price: f64,
}

// ============================================================================
// Delivery Data Models (交割数据模型)
// ============================================================================

/// Delivery data.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryData {
    /// Variety code.
    #[serde(default, deserialize_with = "deserialize_nullable_string")]
    pub variety_code: String,
    /// Delivery month.
    #[serde(default, deserialize_with = "deserialize_nullable_string")]
    pub delivery_month: String,
    /// Delivery volume.
    pub delivery_volume: i64,
    /// Delivery amount.
    pub delivery_amount: f64,
}

/// Request for delivery data.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryDataRequest {
    /// Variety code.
    pub variety_code: String,
    /// Trade date.
    pub trade_date: String,
}

/// Delivery match data.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryMatch {
    /// Variety code.
    #[serde(default, deserialize_with = "deserialize_nullable_string")]
    pub variety_code: String,
    /// Buy member.
    #[serde(default, deserialize_with = "deserialize_nullable_string")]
    pub buy_member: String,
    /// Sell member.
    #[serde(default, deserialize_with = "deserialize_nullable_string")]
    pub sell_member: String,
    /// Volume.
    pub volume: i64,
}

/// Request for delivery match data.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryMatchRequest {
    /// Variety code.
    pub variety_code: String,
    /// Trade date.
    pub trade_date: String,
}

/// Warehouse receipt data.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WarehouseReceipt {
    /// Variety code.
    #[serde(default, deserialize_with = "deserialize_nullable_string")]
    pub variety_code: String,
    /// Warehouse name.
    #[serde(default, deserialize_with = "deserialize_nullable_string")]
    pub warehouse_name: String,
    /// Quantity.
    pub quantity: i64,
    /// Trade date.
    #[serde(default, deserialize_with = "deserialize_nullable_string")]
    pub trade_date: String,
}

/// Request for warehouse receipt data.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WarehouseReceiptRequest {
    /// Variety code.
    pub variety_code: String,
    /// Trade date.
    pub trade_date: String,
}

/// Delivery cost data.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryCost {
    /// Variety code.
    #[serde(default, deserialize_with = "deserialize_nullable_string")]
    pub variety_code: String,
    /// Delivery fee.
    pub delivery_fee: f64,
    /// Inspection fee.
    pub inspection_fee: f64,
    /// Storage fee.
    pub storage_fee: f64,
}

/// Warehouse premium data.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WarehousePremium {
    /// Variety code.
    #[serde(default, deserialize_with = "deserialize_nullable_string")]
    pub variety_code: String,
    /// Warehouse name.
    #[serde(default, deserialize_with = "deserialize_nullable_string")]
    pub warehouse_name: String,
    /// Premium amount.
    pub premium: f64,
}

// ============================================================================
// Member Data Models (会员数据模型)
// ============================================================================

/// Ranking data entry.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ranking {
    /// Rank position.
    #[serde(default, deserialize_with = "deserialize_nullable_string")]
    pub rank: String,
    /// Volume member abbreviation.
    #[serde(
        rename = "qtyAbbr",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub qty_abbr: String,
    /// Today's volume.
    #[serde(rename = "todayQty", default)]
    pub today_qty: i64,
    /// Volume change.
    #[serde(rename = "qtySub", default)]
    pub qty_sub: i64,
    /// Buy member abbreviation.
    #[serde(
        rename = "buyAbbr",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub buy_abbr: String,
    /// Today's buy quantity.
    #[serde(rename = "todayBuyQty", default)]
    pub today_buy_qty: i64,
    /// Buy quantity change.
    #[serde(rename = "buySub", default)]
    pub buy_sub: i64,
    /// Sell member abbreviation.
    #[serde(
        rename = "sellAbbr",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub sell_abbr: String,
    /// Today's sell quantity.
    #[serde(rename = "todaySellQty", default)]
    pub today_sell_qty: i64,
    /// Sell quantity change.
    #[serde(rename = "sellSub", default)]
    pub sell_sub: i64,
}

/// Request for daily ranking.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DailyRankingRequest {
    /// Variety ID.
    pub variety_id: String,
    /// Contract ID.
    pub contract_id: String,
    /// Trade date.
    pub trade_date: String,
    /// Trade type ("1" = futures, "2" = options).
    pub trade_type: String,
}

/// Response for daily ranking.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DailyRankingResponse {
    /// Contract ID.
    #[serde(
        rename = "contractId",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub contract_id: String,
    /// Today's total volume.
    #[serde(rename = "todayQty", default)]
    pub today_qty: i64,
    /// Volume change.
    #[serde(rename = "qtySub", default)]
    pub qty_sub: i64,
    /// Today's buy quantity.
    #[serde(rename = "todayBuyQty", default)]
    pub today_buy_qty: i64,
    /// Buy quantity change.
    #[serde(rename = "buySub", default)]
    pub buy_sub: i64,
    /// Today's sell quantity.
    #[serde(rename = "todaySellQty", default)]
    pub today_sell_qty: i64,
    /// Sell quantity change.
    #[serde(rename = "sellSub", default)]
    pub sell_sub: i64,
    /// Volume ranking list.
    #[serde(rename = "qtyFutureList", default)]
    pub qty_future_list: Vec<Ranking>,
    /// Buy ranking list.
    #[serde(rename = "buyFutureList", default)]
    pub buy_future_list: Vec<Ranking>,
    /// Sell ranking list.
    #[serde(rename = "sellFutureList", default)]
    pub sell_future_list: Vec<Ranking>,
}

/// Request for phase ranking.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PhaseRankingRequest {
    /// Variety code.
    pub variety: String,
    /// Start month.
    pub start_month: String,
    /// End month.
    pub end_month: String,
    /// Trade type.
    pub trade_type: String,
}

/// Phase ranking data.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PhaseRanking {
    /// Sequence number.
    #[serde(default, deserialize_with = "deserialize_nullable_string")]
    pub seq: String,
    /// Member ID.
    #[serde(
        rename = "memberId",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub member_id: String,
    /// Member name.
    #[serde(
        rename = "memberName",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub member_name: String,
    /// Monthly volume.
    #[serde(rename = "monthQty", default)]
    pub month_qty: f64,
    /// Volume ratio.
    #[serde(rename = "qtyRatio", default)]
    pub qty_ratio: f64,
    /// Monthly amount.
    #[serde(rename = "monthAmt", default)]
    pub month_amt: f64,
    /// Amount ratio.
    #[serde(rename = "amtRatio", default)]
    pub amt_ratio: f64,
}

// ============================================================================
// Trade Parameter Models (交易参数数据模型)
// ============================================================================

/// Trade parameter data.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradeParam {
    /// Contract ID.
    #[serde(
        rename = "contractId",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub contract_id: String,
    /// Speculative buy margin rate.
    #[serde(rename = "specBuyRate", default)]
    pub spec_buy_rate: f64,
    /// Speculative buy margin.
    #[serde(rename = "specBuy", default)]
    pub spec_buy: f64,
    /// Hedge buy margin rate.
    #[serde(rename = "hedgeBuyRate", default)]
    pub hedge_buy_rate: f64,
    /// Hedge buy margin.
    #[serde(rename = "hedgeBuy", default)]
    pub hedge_buy: f64,
    /// Rise limit rate.
    #[serde(rename = "riseLimitRate", default)]
    pub rise_limit_rate: f64,
    /// Rise limit price.
    #[serde(rename = "riseLimit", default)]
    pub rise_limit: f64,
    /// Fall limit price.
    #[serde(rename = "fallLimit", default)]
    pub fall_limit: f64,
    /// Trade date.
    #[serde(
        rename = "tradeDate",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub trade_date: String,
}

/// Request for day trade parameters.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DayTradeParamRequest {
    /// Variety ID.
    pub variety_id: String,
    /// Trade type.
    pub trade_type: String,
    /// Language.
    pub lang: String,
}

/// Contract information.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContractInfo {
    /// Contract ID.
    #[serde(
        rename = "contractId",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub contract_id: String,
    /// Variety code.
    #[serde(default, deserialize_with = "deserialize_nullable_string")]
    pub variety: String,
    /// Variety order.
    #[serde(
        rename = "varietyOrder",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub variety_order: String,
    /// Contract unit.
    #[serde(default)]
    pub unit: i32,
    /// Minimum tick.
    #[serde(default, deserialize_with = "deserialize_nullable_string")]
    pub tick: String,
    /// Start trade date.
    #[serde(
        rename = "startTradeDate",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub start_trade_date: String,
    /// End trade date.
    #[serde(
        rename = "endTradeDate",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub end_trade_date: String,
    /// End delivery date.
    #[serde(
        rename = "endDeliveryDate",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub end_delivery_date: String,
    /// Trade type.
    #[serde(
        rename = "tradeType",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub trade_type: String,
}

/// Request for contract information.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContractInfoRequest {
    /// Variety ID.
    pub variety_id: String,
    /// Trade type.
    pub trade_type: String,
    /// Language.
    pub lang: String,
}

/// Arbitrage contract information.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArbitrageContract {
    /// Arbitrage strategy name.
    #[serde(
        rename = "arbiName",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub arbi_name: String,
    /// Variety name.
    #[serde(
        rename = "varietyName",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub variety_name: String,
    /// Arbitrage contract ID.
    #[serde(
        rename = "arbiContractId",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub arbi_contract_id: String,
    /// Maximum order size.
    #[serde(rename = "maxHand", default)]
    pub max_hand: i32,
    /// Minimum tick.
    #[serde(default)]
    pub tick: f64,
}

/// Request for arbitrage contracts.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ArbitrageContractRequest {
    /// Language.
    pub lang: String,
}

// ============================================================================
// Settlement Parameter Models (结算参数数据模型)
// ============================================================================

/// Settlement parameter data.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SettleParam {
    /// Variety code.
    #[serde(default, deserialize_with = "deserialize_nullable_string")]
    pub variety: String,
    /// Variety order.
    #[serde(
        rename = "varietyOrder",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub variety_order: String,
    /// Contract ID.
    #[serde(
        rename = "contractId",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub contract_id: String,
    /// Clearing/settlement price.
    #[serde(
        rename = "clearPrice",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub clear_price: String,
    /// Open fee.
    #[serde(
        rename = "openFee",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub open_fee: String,
    /// Offset fee.
    #[serde(
        rename = "offsetFee",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub offset_fee: String,
    /// Short open fee (intraday).
    #[serde(
        rename = "shortOpenFee",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub short_open_fee: String,
    /// Short offset fee (intraday).
    #[serde(
        rename = "shortOffsetFee",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub short_offset_fee: String,
    /// Position limit style.
    #[serde(default, deserialize_with = "deserialize_nullable_string")]
    pub style: String,
    /// Speculative buy margin rate.
    #[serde(
        rename = "specBuyRate",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub spec_buy_rate: String,
    /// Speculative sell margin rate.
    #[serde(
        rename = "specSellRate",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub spec_sell_rate: String,
    /// Hedge buy margin rate.
    #[serde(
        rename = "hedgeBuyRate",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub hedge_buy_rate: String,
    /// Hedge sell margin rate.
    #[serde(
        rename = "hedgeSellRate",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub hedge_sell_rate: String,
}

/// Request for settlement parameters.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SettleParamRequest {
    /// Variety ID.
    pub variety_id: String,
    /// Trade date.
    pub trade_date: String,
    /// Trade type.
    pub trade_type: String,
    /// Language.
    pub lang: String,
}
