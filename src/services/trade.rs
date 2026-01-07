//! Trade service for trading parameter APIs.

use std::collections::HashMap;

use crate::error::Result;
use crate::http::{BaseClient, RequestOptions};
use crate::models::{
    ArbitrageContract, ArbitrageContractRequest, ContractInfo, ContractInfoRequest,
    DayTradeParamRequest, TradeParam,
};

/// API endpoint for day trade parameters.
const PATH_GET_DAY_TRADE_PARAM: &str = "/dceapi/forward/publicweb/tradepara/dayTradPara";

/// API endpoint for month trade parameters.
const PATH_GET_MONTH_TRADE_PARAM: &str = "/dceapi/forward/publicweb/tradepara/monthTradPara";

/// API endpoint for contract information.
const PATH_GET_CONTRACT_INFO: &str = "/dceapi/forward/publicweb/tradepara/contractInfo";

/// API endpoint for arbitrage contracts.
const PATH_GET_ARBITRAGE_CONTRACT: &str = "/dceapi/forward/publicweb/tradepara/arbitrageContract";

/// Trade service for accessing trading parameters.
#[derive(Debug, Clone)]
pub struct TradeService {
    client: BaseClient,
}

impl TradeService {
    /// Create a new trade service.
    pub fn new(client: BaseClient) -> Self {
        TradeService { client }
    }

    /// Get daily trading parameters.
    ///
    /// Returns margin rates, price limits, etc. for a variety.
    ///
    /// # Arguments
    /// * `req` - Request with variety_id, trade_type, and lang
    /// * `opts` - Optional request options
    pub async fn get_day_trade_param(
        &self,
        req: &DayTradeParamRequest,
        opts: Option<RequestOptions>,
    ) -> Result<Vec<TradeParam>> {
        self.client.do_post(PATH_GET_DAY_TRADE_PARAM, req, opts).await
    }

    /// Get monthly trading parameters.
    ///
    /// # Arguments
    /// * `opts` - Optional request options
    pub async fn get_month_trade_param(
        &self,
        opts: Option<RequestOptions>,
    ) -> Result<HashMap<String, serde_json::Value>> {
        #[derive(serde::Serialize)]
        struct EmptyRequest {}

        self.client
            .do_post(PATH_GET_MONTH_TRADE_PARAM, &EmptyRequest {}, opts)
            .await
    }

    /// Get contract information.
    ///
    /// Returns contract details including trading dates, unit, tick, etc.
    ///
    /// # Arguments
    /// * `req` - Request with variety_id, trade_type, and lang
    /// * `opts` - Optional request options
    pub async fn get_contract_info(
        &self,
        req: &ContractInfoRequest,
        opts: Option<RequestOptions>,
    ) -> Result<Vec<ContractInfo>> {
        self.client.do_post(PATH_GET_CONTRACT_INFO, req, opts).await
    }

    /// Get arbitrage contracts.
    ///
    /// Returns available spread/arbitrage trading contracts.
    ///
    /// # Arguments
    /// * `lang` - Language ("zh" or "en"), defaults to "zh"
    /// * `opts` - Optional request options
    pub async fn get_arbitrage_contract(
        &self,
        lang: Option<&str>,
        opts: Option<RequestOptions>,
    ) -> Result<Vec<ArbitrageContract>> {
        let req = ArbitrageContractRequest {
            lang: lang.unwrap_or("zh").to_string(),
        };
        self.client.do_post(PATH_GET_ARBITRAGE_CONTRACT, &req, opts).await
    }
}
