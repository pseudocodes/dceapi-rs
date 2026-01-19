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

/// Deserialize a string or number to i64
fn deserialize_string_or_i64<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;
    use serde_json::Value;

    let value = Value::deserialize(deserializer)?;
    match value {
        Value::Number(n) => n.as_i64().ok_or_else(|| D::Error::custom("invalid number")),
        Value::String(s) => s
            .parse::<i64>()
            .map_err(|_| D::Error::custom("invalid string number")),
        Value::Null => Ok(0),
        _ => Err(D::Error::custom("expected string or number")),
    }
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
    /// Article version.
    #[serde(default, deserialize_with = "deserialize_nullable_string")]
    pub version: String,
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
    /// Source ID.
    #[serde(default, deserialize_with = "deserialize_nullable_string")]
    pub source_id: String,
    /// Display date.
    #[serde(default, deserialize_with = "deserialize_nullable_string")]
    pub show_date: String,
    /// Release date.
    #[serde(default, deserialize_with = "deserialize_nullable_string")]
    pub release_date: String,
    /// Article content.
    #[serde(default, deserialize_with = "deserialize_nullable_string")]
    pub content: String,
    /// Keywords.
    #[serde(default, deserialize_with = "deserialize_nullable_string")]
    pub keywords: String,
    /// Entity type (e.g., "HTML").
    #[serde(default, deserialize_with = "deserialize_nullable_string")]
    pub entity_type: String,
    /// Title image URL.
    #[serde(default, deserialize_with = "deserialize_nullable_string")]
    pub title_image_url: String,
    /// Article static URL.
    #[serde(default, deserialize_with = "deserialize_nullable_string")]
    pub article_static_url: String,
    /// Article dynamic URL.
    #[serde(default, deserialize_with = "deserialize_nullable_string")]
    pub article_dynamic_url: String,
    /// Page name.
    #[serde(default, deserialize_with = "deserialize_nullable_string")]
    pub page_name: String,
    /// Creation date.
    #[serde(default, deserialize_with = "deserialize_nullable_string")]
    pub create_date: String,
}

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
    /// Status code.
    #[serde(default, deserialize_with = "deserialize_nullable_string")]
    pub status: String,
    /// Status information.
    #[serde(default, deserialize_with = "deserialize_nullable_string")]
    pub status_info: String,
    /// List of articles.
    pub result_list: Vec<Article>,
    /// Total count of articles.
    pub total_count: i32,
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
    /// Variety name.
    #[serde(default, deserialize_with = "deserialize_nullable_string")]
    pub variety: String,
    /// Variety order/code.
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
    /// Last clearing price (前结算价).
    #[serde(
        rename = "lastClear",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub last_clear: String,
    /// Last price (最新价, for night quotes).
    #[serde(
        rename = "lastPrice",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub last_price: String,
    /// Clearing/settlement price (结算价).
    #[serde(
        rename = "clearPrice",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub clear_price: String,
    /// Price difference (涨跌).
    #[serde(default, deserialize_with = "deserialize_nullable_string")]
    pub diff: String,
    /// Price difference 1 (涨跌1).
    #[serde(default, deserialize_with = "deserialize_nullable_string")]
    pub diff1: String,
    /// Declare price (买价/卖价, for night quotes).
    #[serde(
        rename = "declarePrice",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub declare_price: String,
    /// Volume (成交量).
    #[serde(rename = "volumn", default)]
    pub volume: i64,
    /// Open interest (持仓量).
    #[serde(rename = "openInterest", default)]
    pub open_interest: i64,
    /// Open interest difference (持仓量变化).
    #[serde(rename = "diffI", default)]
    pub diff_i: i64,
    /// Turnover (成交额).
    #[serde(default, deserialize_with = "deserialize_nullable_string")]
    pub turnover: String,
    /// Variety name in English.
    #[serde(
        rename = "varietyEn",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub variety_en: String,
    /// Turnover in English format.
    #[serde(
        rename = "turnoverEn",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub turnover_en: String,
    /// Delta (期权).
    #[serde(default, deserialize_with = "deserialize_nullable_string")]
    pub delta: String,
    /// Exercise quantity sum (行权量).
    #[serde(rename = "matchQtySum", default)]
    pub match_qty_sum: i64,
    /// Turnover difference.
    #[serde(
        rename = "diffT",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub diff_t: String,
    /// Volume rate (期权期货成交比).
    #[serde(
        rename = "volumnRate",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub volumn_rate: String,
    /// Open interest rate (期权期货持仓比).
    #[serde(
        rename = "openInterestRate",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub open_interest_rate: String,
    /// Period over period change.
    #[serde(
        rename = "periodOverPeriodChg",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub period_over_period_chg: String,
    /// Volume difference.
    #[serde(rename = "diffV", default)]
    pub diff_v: Option<i64>,
    /// Implied volatility (隐含波动率).
    #[serde(
        rename = "impliedVolatility",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub implied_volatility: String,
    /// Series ID (期权系列).
    #[serde(
        rename = "seriesId",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub series_id: String,
    /// Average open interest (日均持仓量).
    #[serde(rename = "avgOpenInterest", default)]
    pub avg_open_interest: i64,
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

// ============================================================================
// Delivery Data Models (交割数据模型)
// ============================================================================

/// Delivery data.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryData {
    /// Variety name.
    #[serde(default, deserialize_with = "deserialize_nullable_string")]
    pub variety: String,
    /// Contract ID.
    #[serde(default, deserialize_with = "deserialize_nullable_string")]
    pub contract_id: String,
    /// Delivery date.
    #[serde(default, deserialize_with = "deserialize_nullable_string")]
    pub delivery_date: String,
    /// Delivery quantity.
    #[serde(default)]
    pub delivery_qty: i64,
    /// Delivery amount.
    #[serde(default, deserialize_with = "deserialize_nullable_string")]
    pub delivery_amt: String,
}

/// Request for delivery data.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryDataRequest {
    /// Variety ID.
    pub variety_id: String,
    /// Start month (YYYYMM format).
    pub start_month: String,
    /// End month (YYYYMM format).
    pub end_month: String,
    /// Variety type ("0" = physical delivery, "1" = average price delivery).
    pub variety_type: String,
}

/// Delivery match data.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryMatch {
    /// Contract ID.
    #[serde(default, deserialize_with = "deserialize_nullable_string")]
    pub contract_id: String,
    /// Match date.
    #[serde(default, deserialize_with = "deserialize_nullable_string")]
    pub match_date: String,
    /// Buy member ID.
    #[serde(default, deserialize_with = "deserialize_nullable_string")]
    pub buy_member_id: String,
    /// Sell member ID.
    #[serde(default, deserialize_with = "deserialize_nullable_string")]
    pub sell_member_id: String,
    /// Delivery quantity.
    #[serde(default)]
    pub delivery_qty: i64,
    /// Delivery price.
    #[serde(default, deserialize_with = "deserialize_nullable_string")]
    pub delivery_price: String,
}

/// Request for delivery match data.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryMatchRequest {
    /// Variety ID.
    pub variety_id: String,
    /// Contract ID ("all" for all contracts).
    pub contract_id: String,
    /// Start month (YYYYMM format).
    pub start_month: String,
    /// End month (YYYYMM format).
    pub end_month: String,
}

/// Warehouse receipt daily report response.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WarehouseReceipt {
    /// Entity list containing warehouse receipt details.
    #[serde(rename = "entityList", default)]
    pub entity_list: Vec<WarehouseReceiptDetail>,
    /// Whether has agio flag.
    #[serde(
        rename = "ifAgioFlag",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub if_agio_flag: String,
    /// Agio delivery type.
    #[serde(
        rename = "agioDeliType",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub agio_deli_type: String,
    /// Whether has agio brand flag.
    #[serde(
        rename = "ifAgioBrandFlag",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub if_agio_brand_flag: String,
}

/// Warehouse receipt detail entry.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WarehouseReceiptDetail {
    /// Variety order.
    #[serde(
        rename = "varietyOrder",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub variety_order: String,
    /// Group code order.
    #[serde(
        rename = "groupCodeOrder",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub group_code_order: String,
    /// Warehouse code order.
    #[serde(
        rename = "whCodeOrder",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub wh_code_order: String,
    /// Warehouse type.
    #[serde(
        rename = "whType",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub wh_type: String,
    /// Variety name.
    #[serde(default, deserialize_with = "deserialize_nullable_string")]
    pub variety: String,
    /// Generation date.
    #[serde(
        rename = "genDate",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub gen_date: String,
    /// Warehouse abbreviation.
    #[serde(
        rename = "whAbbr",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub wh_abbr: String,
    /// Delivery abbreviation.
    #[serde(
        rename = "deliveryAbbr",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub delivery_abbr: String,
    /// Yesterday's warehouse bill quantity (lots).
    #[serde(rename = "lastWbillQty", default)]
    pub last_wbill_qty: i64,
    /// Registered warehouse bill quantity.
    #[serde(rename = "regWbillQty", default)]
    pub reg_wbill_qty: i64,
    /// Logout warehouse bill quantity.
    #[serde(rename = "logoutWbillQty", default)]
    pub logout_wbill_qty: i64,
    /// Today's warehouse bill quantity (lots).
    #[serde(rename = "wbillQty", default)]
    pub wbill_qty: i64,
    /// Difference (lots).
    #[serde(default)]
    pub diff: i64,
}

/// Request for warehouse receipt data (daily report).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WarehouseReceiptRequest {
    /// Variety ID ("all" for all varieties).
    pub variety_id: String,
    /// Trade date (YYYYMMDD format).
    pub trade_date: String,
}

/// Delivery cost data.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryCost {
    /// Variety name.
    #[serde(default, deserialize_with = "deserialize_nullable_string")]
    pub variety: String,
    /// Earnest rate.
    #[serde(default, deserialize_with = "deserialize_nullable_string")]
    pub earnest_rate: String,
    /// Unit.
    #[serde(default, deserialize_with = "deserialize_nullable_string")]
    pub unit: String,
    /// Delivery fee.
    #[serde(default, deserialize_with = "deserialize_nullable_string")]
    pub delivery_fee: String,
    /// Fee rate.
    #[serde(default, deserialize_with = "deserialize_nullable_string")]
    pub fee_rate: String,
    /// Start date.
    #[serde(default, deserialize_with = "deserialize_nullable_string")]
    pub start_date: String,
    /// End date.
    #[serde(default, deserialize_with = "deserialize_nullable_string")]
    pub end_date: String,
}

/// Warehouse premium data.
/// Warehouse premium response wrapper.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WarehousePremiumResponse {
    /// Entity list containing warehouse premium details.
    #[serde(rename = "entityList", default)]
    pub entity_list: Vec<WarehousePremium>,
    /// Whether has agio flag.
    #[serde(
        rename = "ifAgioFlag",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub if_agio_flag: String,
}

/// Warehouse premium data.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WarehousePremium {
    /// Variety ID.
    #[serde(default, deserialize_with = "deserialize_nullable_string")]
    pub variety_id: String,
    /// Variety name.
    #[serde(default, deserialize_with = "deserialize_nullable_string")]
    pub variety_name: String,
    /// Valid date.
    #[serde(default, deserialize_with = "deserialize_nullable_string")]
    pub valid_date: String,
    /// Warehouse code.
    #[serde(
        rename = "whCode",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub wh_code: String,
    /// Warehouse name.
    #[serde(
        rename = "whName",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub wh_name: String,
    /// Average premium (元/吨).
    #[serde(
        rename = "avgAgio",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub avg_agio: String,
    /// Warehouse group abbreviation.
    #[serde(
        rename = "whGroupAbbr",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub wh_group_abbr: String,
    /// Brand abbreviation.
    #[serde(
        rename = "brandAbbr",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub brand_abbr: String,
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
    /// Position limit style.
    #[serde(default, deserialize_with = "deserialize_nullable_string")]
    pub style: String,
    /// Non-futures company member futures position quota.
    #[serde(rename = "selfTotBuyPosiQuota", default)]
    pub self_tot_buy_posi_quota: Option<f64>,
    /// Non-futures company member options position quota.
    #[serde(rename = "selfTotBuyPosiQuotaSerLimit", default)]
    pub self_tot_buy_posi_quota_ser_limit: Option<f64>,
    /// Client futures position quota.
    #[serde(rename = "clientBuyPosiQuota", default)]
    pub client_buy_posi_quota: Option<f64>,
    /// Client options position quota.
    #[serde(rename = "clientBuyPosiQuotaSerLimit", default)]
    pub client_buy_posi_quota_ser_limit: Option<f64>,
    /// Contract limit.
    #[serde(
        rename = "contractLimit",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub contract_limit: String,
    /// Variety limit.
    #[serde(
        rename = "varietyLimit",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub variety_limit: String,
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

// ============================================================================
// Extended Market Models (扩展行情数据模型)
// ============================================================================

/// Request for variety month/year statistics.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VarietyMonthYearStatRequest {
    /// Trade month (YYYYMM format).
    pub trade_month: String,
    /// Trade type ("1" = futures, "2" = options).
    pub trade_type: String,
    /// Language ("zh" or "en").
    pub lang: String,
}

/// Variety monthly/yearly statistics.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VarietyMonthYearStat {
    /// Variety name.
    #[serde(default, deserialize_with = "deserialize_nullable_string")]
    pub variety: String,
    /// This month volume.
    #[serde(rename = "thisMonthVolumn", default)]
    pub this_month_volumn: i64,
    /// Volume year-over-year comparison.
    #[serde(
        rename = "volumnBalance",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub volumn_balance: String,
    /// Volume month-over-month comparison.
    #[serde(
        rename = "volumnChain",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub volumn_chain: String,
    /// This year volume.
    #[serde(rename = "thisYearVolumn", default)]
    pub this_year_volumn: i64,
    /// Year volume year-over-year comparison.
    #[serde(
        rename = "yearVolumnChain",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub year_volumn_chain: String,
    /// This month turnover.
    #[serde(
        rename = "thisMonthTurnover",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub this_month_turnover: String,
    /// Turnover year-over-year comparison.
    #[serde(
        rename = "turnoverBalance",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub turnover_balance: String,
    /// Turnover month-over-month comparison.
    #[serde(
        rename = "turnoverChain",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub turnover_chain: String,
    /// This year turnover.
    #[serde(
        rename = "thisYearTurnover",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub this_year_turnover: String,
    /// Year turnover year-over-year comparison.
    #[serde(
        rename = "yearTurnoverChain",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub year_turnover_chain: String,
    /// This month open interest.
    #[serde(rename = "thisMonthOpeni", default)]
    pub this_month_openi: i64,
    /// Open interest year-over-year comparison.
    #[serde(
        rename = "openiBalance",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub openi_balance: String,
    /// Open interest month-over-month comparison.
    #[serde(
        rename = "openiChain",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub openi_chain: String,
}

/// Request for contract monthly max statistics.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContractMonthMaxRequest {
    /// Start month (YYYYMM format).
    pub start_month: String,
    /// End month (YYYYMM format).
    pub end_month: String,
    /// Statistics content: "0"=volume, "1"=turnover, "2"=open_interest, "3"=price.
    pub stat_content: String,
    /// Trade type ("1" = futures, "2" = options).
    pub trade_type: String,
    /// Language ("zh" or "en").
    pub lang: String,
}

/// Contract monthly max - Volume statistics.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContractMonthMaxVolume {
    /// Contract ID.
    #[serde(
        rename = "contractId",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub contract_id: String,
    /// Total volume.
    #[serde(rename = "sumAmount", default)]
    pub sum_amount: i64,
    /// Maximum volume.
    #[serde(rename = "maxAmount", default)]
    pub max_amount: i64,
    /// Date of maximum volume.
    #[serde(
        rename = "maxAmountDate",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub max_amount_date: String,
    /// Minimum volume.
    #[serde(rename = "minAmount", default)]
    pub min_amount: i64,
    /// Date of minimum volume.
    #[serde(
        rename = "minAmountDate",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub min_amount_date: String,
    /// Average daily volume.
    #[serde(rename = "avgAmount", default)]
    pub avg_amount: i64,
}

/// Contract monthly max - Turnover statistics.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContractMonthMaxTurnover {
    /// Contract ID.
    #[serde(
        rename = "contractId",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub contract_id: String,
    /// Total turnover.
    #[serde(
        rename = "sumTurnover",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub sum_turnover: String,
    /// Maximum turnover.
    #[serde(
        rename = "maxTurnover",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub max_turnover: String,
    /// Date of maximum turnover.
    #[serde(
        rename = "maxTurnoverDate",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub max_turnover_date: String,
    /// Minimum turnover.
    #[serde(
        rename = "minTurnover",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub min_turnover: String,
    /// Date of minimum turnover.
    #[serde(
        rename = "minTurnoverDate",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub min_turnover_date: String,
    /// Average daily turnover.
    #[serde(
        rename = "avgTurnover",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub avg_turnover: String,
}

/// Contract monthly max - Open Interest statistics.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContractMonthMaxOpeni {
    /// Contract ID.
    #[serde(
        rename = "contractId",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub contract_id: String,
    /// Total open interest.
    #[serde(rename = "sumOpeni", default)]
    pub sum_openi: i64,
    /// Maximum open interest.
    #[serde(rename = "maxOpeni", default)]
    pub max_openi: i64,
    /// Date of maximum open interest.
    #[serde(
        rename = "maxOpeniDate",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub max_openi_date: String,
    /// Minimum open interest.
    #[serde(rename = "minOpeni", default)]
    pub min_openi: i64,
    /// Date of minimum open interest.
    #[serde(
        rename = "minOpeniDate",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub min_openi_date: String,
    /// Average daily open interest.
    #[serde(rename = "avgOpeni", default)]
    pub avg_openi: i64,
}

/// Contract monthly max - Price statistics.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContractMonthMaxPrice {
    /// Contract ID.
    #[serde(
        rename = "contractId",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub contract_id: String,
    /// Opening price at period start.
    #[serde(default, deserialize_with = "deserialize_nullable_string")]
    pub open: String,
    /// Closing price at period end.
    #[serde(default, deserialize_with = "deserialize_nullable_string")]
    pub close: String,
    /// Highest price.
    #[serde(default, deserialize_with = "deserialize_nullable_string")]
    pub high: String,
    /// Date of highest price.
    #[serde(
        rename = "highDate",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub high_date: String,
    /// Lowest price.
    #[serde(default, deserialize_with = "deserialize_nullable_string")]
    pub low: String,
    /// Date of lowest price.
    #[serde(
        rename = "lowDate",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub low_date: String,
    /// Settlement price at period end.
    #[serde(
        rename = "clearPrice",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub clear_price: String,
}

/// Request for rise/fall event (trading limit) query.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RiseFallEventRequest {
    /// Start date (YYYYMMDD format).
    pub start_date: String,
    /// End date (YYYYMMDD format).
    pub end_date: String,
    /// Variety ID ("all" for all varieties).
    pub variety_id: String,
    /// Language ("zh" or "en").
    pub lang: String,
}

/// Rise/fall event (trading limit) information.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RiseFallEvent {
    /// Trade date.
    #[serde(
        rename = "tradeDate",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub trade_date: String,
    /// Contract ID.
    #[serde(
        rename = "contractId",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub contract_id: String,
    /// Direction (limit up/down).
    #[serde(default, deserialize_with = "deserialize_nullable_string")]
    pub direction: String,
    /// Number of times.
    #[serde(default)]
    pub times: i32,
}

/// Request for division price info.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DivisionPriceInfoRequest {
    /// Variety ID.
    pub variety_id: String,
    /// Trade date (YYYYMMDD format).
    pub trade_date: String,
    /// Trade type ("1" = futures, "2" = options).
    pub trade_type: String,
}

/// Division price information (分时结算参考价).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DivisionPriceInfo {
    /// Calculate date (交易日期).
    #[serde(
        rename = "calculateDate",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub calculate_date: String,
    /// Calculate time (计算时间).
    #[serde(
        rename = "calculateTime",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub calculate_time: String,
    /// Variety name (品种名称).
    #[serde(
        rename = "varietyName",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub variety_name: String,
    /// Variety name in English.
    #[serde(
        rename = "varietyEnName",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub variety_en_name: String,
    /// Contract ID (合约).
    #[serde(
        rename = "contractId",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub contract_id: String,
    /// Settlement reference price (结算参考价).
    #[serde(rename = "clearPrice", default)]
    pub clear_price: f64,
    /// Series ID (期权系列).
    #[serde(
        rename = "seriesId",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub series_id: String,
    /// Volatility (结算参考隐含波动率).
    #[serde(default)]
    pub volatility: f64,
}

// ============================================================================
// Extended Trade Models (扩展交易参数模型)
// ============================================================================

/// Request for trading parameters by variety.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TradingParamRequest {
    /// Language ("zh" or "en").
    pub lang: String,
}

/// Trading parameters for a variety.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradingParam {
    /// Variety ID.
    #[serde(
        rename = "varietyId",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub variety_id: String,
    /// Variety name.
    #[serde(
        rename = "varietyName",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub variety_name: String,
    /// Trading margin rate (speculation).
    #[serde(
        rename = "tradingMarginRateSpeculation",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub trading_margin_rate_speculation: String,
    /// Trading margin rate (hedging).
    #[serde(
        rename = "tradingMarginRateHedging",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub trading_margin_rate_hedging: String,
    /// Price limit for existing contracts.
    #[serde(
        rename = "priceLimitExistingContract",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub price_limit_existing_contract: String,
    /// Price limit for new contracts.
    #[serde(
        rename = "priceLimitNewContract",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub price_limit_new_contract: String,
    /// Price limit for delivery month.
    #[serde(
        rename = "priceLimitDeliveryMonth",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub price_limit_delivery_month: String,
    /// Trading margin rate (speculation) - Day N.
    #[serde(
        rename = "tradingMarginRateSpeculationN",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub trading_margin_rate_speculation_n: String,
    /// Trading margin rate (hedging) - Day N.
    #[serde(
        rename = "tradingMarginRateHedgingN",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub trading_margin_rate_hedging_n: String,
    /// Settlement margin rate (speculation/hedging) - Day N.
    #[serde(
        rename = "settlementMarginRateHedgingN",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub settlement_margin_rate_hedging_n: String,
    /// Price limit - Day N.
    #[serde(
        rename = "priceLimitN",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub price_limit_n: String,
    /// Trading margin rate (speculation/hedging) - Day N+1.
    #[serde(
        rename = "tradingMarginRateN1",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub trading_margin_rate_n1: String,
    /// Settlement margin rate (speculation/hedging) - Day N+1.
    #[serde(
        rename = "settlementMarginRateHedgingN1",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub settlement_margin_rate_hedging_n1: String,
    /// Price limit - Day N+1.
    #[serde(
        rename = "priceLimitN1",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub price_limit_n1: String,
    /// Trading margin rate (speculation/hedging) - Day N+2.
    #[serde(
        rename = "tradingMarginRateN2",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub trading_margin_rate_n2: String,
    /// Price limit - Day N+2.
    #[serde(
        rename = "priceLimitN2",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub price_limit_n2: String,
    /// Trading limit.
    #[serde(
        rename = "tradingLimit",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub trading_limit: String,
    /// Speculative open fee.
    #[serde(
        rename = "specOpenFee",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub spec_open_fee: String,
    /// Speculative offset fee.
    #[serde(
        rename = "specOffsetFee",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub spec_offset_fee: String,
    /// Speculative short open fee (intraday).
    #[serde(
        rename = "specShortOpenFee",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub spec_short_open_fee: String,
    /// Speculative short offset fee (intraday).
    #[serde(
        rename = "specShortOffsetFee",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub spec_short_offset_fee: String,
    /// Hedge open fee.
    #[serde(
        rename = "hedgeOpenFee",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub hedge_open_fee: String,
    /// Hedge offset fee.
    #[serde(
        rename = "hedgeOffsetFee",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub hedge_offset_fee: String,
    /// Hedge short open fee (intraday).
    #[serde(
        rename = "hedgeShortOpenFee",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub hedge_short_open_fee: String,
    /// Hedge short offset fee (intraday).
    #[serde(
        rename = "hedgeShortOffsetFee",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub hedge_short_offset_fee: String,
    /// Fee style.
    #[serde(
        rename = "feeStyle",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub fee_style: String,
    /// Fee style (English).
    #[serde(
        rename = "feeStyleEn",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub fee_style_en: String,
    /// Delivery fee.
    #[serde(
        rename = "deliveryFee",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub delivery_fee: String,
    /// Maximum hand (order size).
    #[serde(
        rename = "maxHand",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub max_hand: String,
}

/// Request for margin arbitrage performance parameters.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MarginArbiPerfParaRequest {
    /// Variety ID.
    pub variety_id: String,
}

/// Margin arbitrage performance parameters.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarginArbiPerfPara {
    /// Variety.
    #[serde(default, deserialize_with = "deserialize_nullable_string")]
    pub variety: String,
    /// Strategy name.
    #[serde(
        rename = "strategyName",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub strategy_name: String,
    /// Trading margin rate (speculation).
    #[serde(
        rename = "tradingMarginRateSpeculation",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub trading_margin_rate_speculation: String,
    /// Trading margin rate (hedging).
    #[serde(
        rename = "tradingMarginRateHedging",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub trading_margin_rate_hedging: String,
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
    /// Short open fee.
    #[serde(
        rename = "shortOpenFee",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub short_open_fee: String,
    /// Short offset fee.
    #[serde(
        rename = "shortOffsetFee",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub short_offset_fee: String,
}

/// Request for new contract information.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewContractInfoRequest {
    /// Trade date (YYYYMMDD format).
    pub trade_date: String,
    /// Trade type ("1" = futures, "2" = options).
    pub trade_type: String,
    /// Language.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
}

/// New contract information.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewContractInfo {
    /// Trade type.
    #[serde(
        rename = "tradeType",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub trade_type: String,
    /// Variety.
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
    /// Start trade date.
    #[serde(
        rename = "startTradeDate",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub start_trade_date: String,
    /// Reference price unit.
    #[serde(
        rename = "refPriceUnit",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub ref_price_unit: String,
    /// No rise limit.
    #[serde(
        rename = "noRiseLimit",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub no_rise_limit: String,
    /// No fall limit.
    #[serde(
        rename = "noFallLimit",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub no_fall_limit: String,
}

/// Request for main series information (做市商持续报价合约).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MainSeriesInfoRequest {
    /// Variety ID ("all" for all varieties).
    pub variety_id: String,
    /// Trade date (YYYYMMDD format).
    pub trade_date: String,
}

/// Main series information (market maker contracts).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MainSeriesInfo {
    /// Trade date.
    #[serde(
        rename = "tradeDate",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub trade_date: String,
    /// Variety ID.
    #[serde(
        rename = "varietyId",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub variety_id: String,
    /// Series ID.
    #[serde(
        rename = "seriesId",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub series_id: String,
    /// Contract ID.
    #[serde(
        rename = "contractId",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub contract_id: String,
}

// ============================================================================
// Extended Delivery Models (扩展交割数据模型)
// ============================================================================

/// Request for TC congregate delivery (一次性交割卖方仓单查询).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TcCongregateDeliveryRequest {
    /// Variety code ("all" for all varieties).
    pub variety: String,
    /// Contract month (YYYYMM format).
    pub contract_month: String,
}

/// TC congregate delivery information.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TcCongregateDelivery {
    /// Variety ID.
    #[serde(
        rename = "varietyId",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub variety_id: String,
    /// Variety name.
    #[serde(
        rename = "varietyName",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub variety_name: String,
    /// Contract.
    #[serde(default, deserialize_with = "deserialize_nullable_string")]
    pub contract: String,
    /// Warehouse name.
    #[serde(
        rename = "warehouseName",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub warehouse_name: String,
    /// Warehouse bill quantity.
    #[serde(
        rename = "wbillQuantity",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub wbill_quantity: String,
    /// Agreeable place.
    #[serde(
        rename = "agreeablePlace",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub agreeable_place: String,
    /// Agreeable brand.
    #[serde(
        rename = "agreeableBrand",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub agreeable_brand: String,
    /// Agreeable quality.
    #[serde(
        rename = "agreeableQuality",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub agreeable_quality: String,
    /// Agreeable quantity.
    #[serde(
        rename = "agreeableQuantity",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub agreeable_quantity: String,
    /// Agreeable spread.
    #[serde(
        rename = "agreeableSpread",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub agreeable_spread: String,
    /// Contacts.
    #[serde(default, deserialize_with = "deserialize_nullable_string")]
    pub contracts: String,
    /// Contact way (method).
    #[serde(
        rename = "contractWay",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub contract_way: String,
    /// Warehouse group name.
    #[serde(
        rename = "whGroupName",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub wh_group_name: String,
}

/// Request for roll delivery seller intention (滚动交割卖方交割意向表).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RollDeliverySellerIntentionRequest {
    /// Variety code ("all" for all varieties).
    pub variety: String,
    /// Query date (YYYYMMDD format).
    pub date: String,
}

/// Roll delivery seller intention.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RollDeliverySellerIntention {
    /// Variety ID.
    #[serde(
        rename = "varietyId",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub variety_id: String,
    /// Variety name.
    #[serde(
        rename = "varietyName",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub variety_name: String,
    /// Contract.
    #[serde(default, deserialize_with = "deserialize_nullable_string")]
    pub contract: String,
    /// Type (e.g. 仓库/车板).
    #[serde(
        rename = "type",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub type_: String,
    /// Warehouse code.
    #[serde(
        rename = "warehouseCode",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub warehouse_code: String,
    /// Warehouse name.
    #[serde(
        rename = "warehouseName",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub warehouse_name: String,
    /// Quantity (string in doc).
    #[serde(default, deserialize_with = "deserialize_nullable_string")]
    pub quantity: String,
    /// Agreeable place.
    #[serde(
        rename = "agreeablePlace",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub agreeable_place: String,
    /// Agreeable brand.
    #[serde(
        rename = "agreeableBrand",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub agreeable_brand: String,
    /// Agreeable quality.
    #[serde(
        rename = "agreeableQuality",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub agreeable_quality: String,
    /// Agreeable quantity.
    #[serde(
        rename = "agreeableQuantity",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub agreeable_quantity: String,
    /// Agreeable spread.
    #[serde(
        rename = "agreeableSpread",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub agreeable_spread: String,
    /// Contacts.
    #[serde(default, deserialize_with = "deserialize_nullable_string")]
    pub contracts: String,
    /// Contact way (method).
    #[serde(
        rename = "contractWay",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub contract_way: String,
    /// Trade date.
    #[serde(
        rename = "tradeDate",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub trade_date: String,
    /// Warehouse group name.
    #[serde(
        rename = "whGroupName",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub wh_group_name: String,
    /// Delivery way.
    #[serde(
        rename = "deliveryWay",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub delivery_way: String,
}

/// Request for bonded delivery settlement price (交割结算价).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BondedDeliveryRequest {
    /// Start date (YYYYMMDD format).
    pub start_date: String,
    /// End date (YYYYMMDD format).
    pub end_date: String,
}

/// Bonded delivery settlement price.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BondedDelivery {
    /// Delivery date.
    #[serde(
        rename = "deliveryDate",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub delivery_date: String,
    /// Delivery way.
    #[serde(
        rename = "deliveryWay",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub delivery_way: String,
    /// Variety ID (e.g. i-铁矿石).
    #[serde(
        rename = "varietyId",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub variety_id: String,
    /// Contract ID.
    #[serde(
        rename = "contractId",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub contract_id: String,
    /// Warehouse abbreviation.
    #[serde(
        rename = "whAbbr",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub wh_abbr: String,
    /// Bonded delivery price.
    #[serde(
        rename = "bondedDeliveryPrice",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub bonded_delivery_price: String,
    /// Delivery price.
    #[serde(
        rename = "deliveryPrice",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub delivery_price: String,
}

/// Request for TD bonded delivery settlement price (保税交割结算价).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TdBondedDeliveryRequest {
    /// Start date (YYYYMMDD format).
    pub start_date: String,
    /// End date (YYYYMMDD format).
    pub end_date: String,
}

/// TD bonded delivery settlement price (same structure as BondedDelivery).
pub type TdBondedDelivery = BondedDelivery;

/// Request for factory spot agio (basis spread).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FactorySpotAgioRequest {
    /// Variety ID.
    pub variety_id: String,
    /// Trade date (YYYYMMDD format).
    pub trade_date: String,
}

/// Factory spot agio (price difference for fiberboard).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FactorySpotAgio {
    /// Sequence number.
    #[serde(default, deserialize_with = "deserialize_nullable_string")]
    pub seq_no: String,
    /// Factory abbreviation (Warehouse abbreviation).
    #[serde(
        rename = "whAbbr",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub wh_abbr: String,
    /// Variety ID.
    #[serde(default, deserialize_with = "deserialize_nullable_string")]
    pub variety_id: String,
    /// Variety name.
    #[serde(default, deserialize_with = "deserialize_nullable_string")]
    pub variety_name: String,
    /// Warehouse code.
    #[serde(
        rename = "whCode",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub wh_code: String,
    /// Thickness (mm).
    #[serde(default, deserialize_with = "deserialize_nullable_string")]
    pub bh: String,
    /// Density min (g/cm3).
    #[serde(default, deserialize_with = "deserialize_nullable_string")]
    pub mdmin: String,
    /// Density max (g/cm3).
    #[serde(default, deserialize_with = "deserialize_nullable_string")]
    pub mdmax: String,
    /// Formaldehyde (mg/m3).
    #[serde(default, deserialize_with = "deserialize_nullable_string")]
    pub jq: String,
    /// Price difference (Agio).
    #[serde(default, deserialize_with = "deserialize_nullable_string")]
    pub agio: String,
    /// Min exchange amount.
    #[serde(
        rename = "minExchangeAmount",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub min_exchange_amount: String,
    /// Warehouse address.
    #[serde(
        rename = "whAddr",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub wh_addr: String,
    /// Contact person.
    #[serde(
        rename = "connectPerson",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub connect_person: String,
    /// Telephone.
    #[serde(default, deserialize_with = "deserialize_nullable_string")]
    pub tel: String,
}

/// Request for plywood delivery commodity.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlywoodDeliveryCommodityRequest {
    /// Variety ID.
    pub variety_id: String,
}

/// Plywood delivery commodity.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlywoodDeliveryCommodity {
    /// Apply ID.
    #[serde(default, deserialize_with = "deserialize_nullable_string")]
    pub apply_id: String,
    /// Warehouse name.
    #[serde(
        rename = "whName",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub wh_name: String,
    /// Warehouse abbreviation.
    #[serde(
        rename = "whAbbr",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub wh_abbr: String,
    /// Upload file ID.
    #[serde(
        rename = "uploadFileId",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub upload_file_id: String,
    /// File size.
    #[serde(default)]
    pub file_size: i64,
    /// Upload file name.
    #[serde(
        rename = "uploadFileName",
        default,
        deserialize_with = "deserialize_nullable_string"
    )]
    pub upload_file_name: String,
}
