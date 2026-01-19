//! Market service for quote and market data APIs.

use crate::error::Result;
use crate::http::{BaseClient, RequestOptions};
use crate::models::{
    ContractMonthMaxOpeni, ContractMonthMaxPrice, ContractMonthMaxRequest,
    ContractMonthMaxTurnover, ContractMonthMaxVolume, DivisionPriceInfo, DivisionPriceInfoRequest,
    Quote, QuotesRequest, RiseFallEvent, RiseFallEventRequest,
    WarehouseReceipt, WarehouseReceiptRequest,
};

/// API endpoint for night quotes.
const PATH_GET_NIGHT_QUOTES: &str = "/dceapi/forward/publicweb/dailystat/tiNightQuotes";

/// API endpoint for day quotes.
const PATH_GET_DAY_QUOTES: &str = "/dceapi/forward/publicweb/dailystat/dayQuotes";

/// API endpoint for week quotes.
const PATH_GET_WEEK_QUOTES: &str = "/dceapi/forward/publicweb/dailystat/weekQuotes";

/// API endpoint for month quotes.
const PATH_GET_MONTH_QUOTES: &str = "/dceapi/forward/publicweb/dailystat/monthQuotes";

/// API endpoint for contract monthly max statistics.
const PATH_GET_CONTRACT_MONTH_MAX: &str = "/dceapi/forward/publicweb/phasestat/contractMonthMax";

/// API endpoint for rise/fall event (trading limit) query.
const PATH_GET_RISE_FALL_EVENT: &str = "/dceapi/forward/publicweb/phasestat/riseFallEvent";

/// API endpoint for division price info.
const PATH_GET_DIVISION_PRICE_INFO: &str = "/dceapi/forward/publicweb/dailystat/divisionPriceInfo";

/// API endpoint for warehouse receipt (daily report).
const PATH_GET_WAREHOUSE_RECEIPT: &str = "/dceapi/forward/publicweb/dailystat/wbillWeeklyQuotes";

/// Market service for accessing quote and market data.
#[derive(Debug, Clone)]
pub struct MarketService {
    client: BaseClient,
}

impl MarketService {
    /// Create a new market service.
    pub fn new(client: BaseClient) -> Self {
        MarketService { client }
    }

    /// Get night session quotes.
    ///
    /// # Arguments
    /// * `req` - Request with variety and trade date
    /// * `opts` - Optional request options
    pub async fn get_night_quotes(
        &self,
        req: &QuotesRequest,
        opts: Option<RequestOptions>,
    ) -> Result<Vec<Quote>> {
        self.client.do_post(PATH_GET_NIGHT_QUOTES, req, opts).await
    }

    /// Get day session quotes.
    ///
    /// # Arguments
    /// * `req` - Request with variety and trade date
    /// * `opts` - Optional request options
    pub async fn get_day_quotes(
        &self,
        req: &QuotesRequest,
        opts: Option<RequestOptions>,
    ) -> Result<Vec<Quote>> {
        self.client.do_post(PATH_GET_DAY_QUOTES, req, opts).await
    }

    /// Get weekly quotes.
    ///
    /// # Arguments
    /// * `req` - Request with variety and trade date (same as day quotes)
    /// * `opts` - Optional request options
    pub async fn get_week_quotes(
        &self,
        req: &QuotesRequest,
        opts: Option<RequestOptions>,
    ) -> Result<Vec<Quote>> {
        self.client.do_post(PATH_GET_WEEK_QUOTES, req, opts).await
    }

    /// Get monthly quotes.
    ///
    /// # Arguments
    /// * `req` - Request with variety and trade date (same as day quotes)
    /// * `opts` - Optional request options
    pub async fn get_month_quotes(
        &self,
        req: &QuotesRequest,
        opts: Option<RequestOptions>,
    ) -> Result<Vec<Quote>> {
        self.client.do_post(PATH_GET_MONTH_QUOTES, req, opts).await
    }

    /// Get contract monthly max statistics (volume).
    ///
    /// # Arguments
    /// * `req` - Request with stat_content="0" for volume statistics
    /// * `opts` - Optional request options
    pub async fn get_contract_month_max_volume(
        &self,
        req: &ContractMonthMaxRequest,
        opts: Option<RequestOptions>,
    ) -> Result<Vec<ContractMonthMaxVolume>> {
        self.client
            .do_post(PATH_GET_CONTRACT_MONTH_MAX, req, opts)
            .await
    }

    /// Get contract monthly max statistics (turnover).
    ///
    /// # Arguments
    /// * `req` - Request with stat_content="1" for turnover statistics
    /// * `opts` - Optional request options
    pub async fn get_contract_month_max_turnover(
        &self,
        req: &ContractMonthMaxRequest,
        opts: Option<RequestOptions>,
    ) -> Result<Vec<ContractMonthMaxTurnover>> {
        self.client
            .do_post(PATH_GET_CONTRACT_MONTH_MAX, req, opts)
            .await
    }

    /// Get contract monthly max statistics (open interest).
    ///
    /// # Arguments
    /// * `req` - Request with stat_content="2" for open interest statistics
    /// * `opts` - Optional request options
    pub async fn get_contract_month_max_openi(
        &self,
        req: &ContractMonthMaxRequest,
        opts: Option<RequestOptions>,
    ) -> Result<Vec<ContractMonthMaxOpeni>> {
        self.client
            .do_post(PATH_GET_CONTRACT_MONTH_MAX, req, opts)
            .await
    }

    /// Get contract monthly max statistics (price).
    ///
    /// # Arguments
    /// * `req` - Request with stat_content="3" for price statistics
    /// * `opts` - Optional request options
    pub async fn get_contract_month_max_price(
        &self,
        req: &ContractMonthMaxRequest,
        opts: Option<RequestOptions>,
    ) -> Result<Vec<ContractMonthMaxPrice>> {
        self.client
            .do_post(PATH_GET_CONTRACT_MONTH_MAX, req, opts)
            .await
    }

    /// Get rise/fall events (trading limit occurrences).
    ///
    /// # Arguments
    /// * `req` - Request with date range and variety
    /// * `opts` - Optional request options
    pub async fn get_rise_fall_event(
        &self,
        req: &RiseFallEventRequest,
        opts: Option<RequestOptions>,
    ) -> Result<Vec<RiseFallEvent>> {
        self.client
            .do_post(PATH_GET_RISE_FALL_EVENT, req, opts)
            .await
    }

    /// Get division price information (settlement reference price by time).
    ///
    /// # Arguments
    /// * `req` - Request with variety, trade date, and trade type
    /// * `opts` - Optional request options
    pub async fn get_division_price_info(
        &self,
        req: &DivisionPriceInfoRequest,
        opts: Option<RequestOptions>,
    ) -> Result<Vec<DivisionPriceInfo>> {
        self.client
            .do_post(PATH_GET_DIVISION_PRICE_INFO, req, opts)
            .await
    }

    /// Get warehouse receipt daily report.
    ///
    /// # Arguments
    /// * `req` - Request with variety and trade date
    /// * `opts` - Optional request options
    pub async fn get_warehouse_receipt(
        &self,
        req: &WarehouseReceiptRequest,
        opts: Option<RequestOptions>,
    ) -> Result<WarehouseReceipt> {
        self.client
            .do_post(PATH_GET_WAREHOUSE_RECEIPT, req, opts)
            .await
    }
}
