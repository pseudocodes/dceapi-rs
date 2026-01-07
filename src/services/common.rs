//! Common service for general API endpoints.

use crate::error::Result;
use crate::http::{BaseClient, RequestOptions};
use crate::models::{TradeDate, Variety};

/// API endpoint for current trade date.
const PATH_GET_CURR_TRADE_DATE: &str = "/dceapi/forward/publicweb/maxTradeDate";

/// API endpoint for variety list.
const PATH_GET_VARIETY_LIST: &str = "/dceapi/forward/publicweb/variety";

/// Common service for general operations.
#[derive(Debug, Clone)]
pub struct CommonService {
    client: BaseClient,
}

impl CommonService {
    /// Create a new common service.
    pub fn new(client: BaseClient) -> Self {
        CommonService { client }
    }

    /// Get the current (latest) trade date.
    ///
    /// # Arguments
    /// * `opts` - Optional request options
    pub async fn get_curr_trade_date(&self, opts: Option<RequestOptions>) -> Result<TradeDate> {
        self.client.do_get(PATH_GET_CURR_TRADE_DATE, opts).await
    }

    /// Get the list of available varieties (commodities).
    ///
    /// # Arguments
    /// * `opts` - Optional request options (use trade_type to filter futures/options)
    pub async fn get_variety_list(&self, opts: Option<RequestOptions>) -> Result<Vec<Variety>> {
        self.client.do_get(PATH_GET_VARIETY_LIST, opts).await
    }
}
