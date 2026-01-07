//! Market service for quote and market data APIs.

use crate::error::{Error, Result};
use crate::http::{BaseClient, RequestOptions};
use crate::models::{
    ContractStat, ContractStatRequest, MonthQuotesRequest, Quote, QuotesRequest, WeekQuotesRequest,
};

/// API endpoint for night quotes.
const PATH_GET_NIGHT_QUOTES: &str = "/dceapi/forward/publicweb/dailystat/tiNightQuotes";

/// API endpoint for day quotes.
const PATH_GET_DAY_QUOTES: &str = "/dceapi/forward/publicweb/dailystat/dayQuotes";

/// API endpoint for week quotes.
const PATH_GET_WEEK_QUOTES: &str = "/dceapi/forward/publicweb/dailystat/weekQuotes";

/// API endpoint for month quotes.
const PATH_GET_MONTH_QUOTES: &str = "/dceapi/forward/publicweb/dailystat/monthQuotes";

/// API endpoint for contract statistics.
const PATH_GET_CONTRACT_STAT: &str = "/dceapi/forward/publicweb/dailystat/contractStat";

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
    /// * `req` - Request with variety, year, and week number
    /// * `opts` - Optional request options
    pub async fn get_week_quotes(
        &self,
        req: &WeekQuotesRequest,
        opts: Option<RequestOptions>,
    ) -> Result<Vec<Quote>> {
        self.client.do_post(PATH_GET_WEEK_QUOTES, req, opts).await
    }

    /// Get monthly quotes.
    ///
    /// # Arguments
    /// * `req` - Request with variety, year, and month
    /// * `opts` - Optional request options
    pub async fn get_month_quotes(
        &self,
        req: &MonthQuotesRequest,
        opts: Option<RequestOptions>,
    ) -> Result<Vec<Quote>> {
        self.client.do_post(PATH_GET_MONTH_QUOTES, req, opts).await
    }

    /// Get contract statistics for a date range.
    ///
    /// # Arguments
    /// * `req` - Request with contract code and date range
    /// * `opts` - Optional request options
    pub async fn get_contract_stat(
        &self,
        req: &ContractStatRequest,
        opts: Option<RequestOptions>,
    ) -> Result<ContractStat> {
        if req.contract_code.is_empty() {
            return Err(Error::validation("contract_code", "contract_code is required"));
        }
        self.client.do_post(PATH_GET_CONTRACT_STAT, req, opts).await
    }
}
