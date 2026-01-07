//! Settlement service for settlement parameter APIs.

use crate::error::Result;
use crate::http::{BaseClient, RequestOptions};
use crate::models::{SettleParam, SettleParamRequest};

/// API endpoint for settlement parameters.
const PATH_GET_SETTLE_PARAM: &str = "/dceapi/forward/publicweb/tradepara/futAndOptSettle";

/// Settlement service for accessing settlement parameters.
#[derive(Debug, Clone)]
pub struct SettleService {
    client: BaseClient,
}

impl SettleService {
    /// Create a new settlement service.
    pub fn new(client: BaseClient) -> Self {
        SettleService { client }
    }

    /// Get settlement parameters.
    ///
    /// Returns settlement prices, fees, and margin rates for contracts.
    ///
    /// # Arguments
    /// * `req` - Request with variety_id, trade_date, trade_type, and lang
    /// * `opts` - Optional request options
    pub async fn get_settle_param(
        &self,
        req: &SettleParamRequest,
        opts: Option<RequestOptions>,
    ) -> Result<Vec<SettleParam>> {
        self.client.do_post(PATH_GET_SETTLE_PARAM, req, opts).await
    }
}
