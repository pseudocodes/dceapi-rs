//! Trade service for trading parameter APIs.

use std::collections::HashMap;

use crate::error::Result;
use crate::http::{BaseClient, RequestOptions};
use crate::models::{
    ArbitrageContract, ArbitrageContractRequest, ContractInfo, ContractInfoRequest,
    DayTradeParamRequest, MainSeriesInfo, MainSeriesInfoRequest, MarginArbiPerfPara,
    MarginArbiPerfParaRequest, NewContractInfo, NewContractInfoRequest, TradeParam, TradingParam,
    TradingParamRequest,
};

/// API endpoint for day trade parameters.
const PATH_GET_DAY_TRADE_PARAM: &str = "/dceapi/forward/publicweb/tradepara/dayTradPara";

/// API endpoint for month trade parameters.
const PATH_GET_MONTH_TRADE_PARAM: &str = "/dceapi/forward/publicweb/tradepara/monthTradPara";

/// API endpoint for contract information.
const PATH_GET_CONTRACT_INFO: &str = "/dceapi/forward/publicweb/tradepara/contractInfo";

/// API endpoint for arbitrage contracts.
const PATH_GET_ARBITRAGE_CONTRACT: &str = "/dceapi/forward/publicweb/tradepara/arbitrageContract";

/// API endpoint for trading parameters by variety.
const PATH_GET_TRADING_PARAM: &str = "/dceapi/forward/publicweb/tradepara/tradingParam";

/// API endpoint for margin arbitrage performance parameters.
const PATH_GET_MARGIN_ARBI_PERF_PARA: &str =
    "/dceapi/forward/publicweb/tradepara/marginArbiPerfPara";

/// API endpoint for new contract information.
const PATH_GET_NEW_CONTRACT_INFO: &str = "/dceapi/forward/publicweb/tradepara/newContractInfo";

/// API endpoint for main series information (market maker contracts).
const PATH_GET_MAIN_SERIES_INFO: &str = "/dceapi/forward/publicweb/tradepara/mainSeriesInfo";

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
        self.client
            .do_post(PATH_GET_DAY_TRADE_PARAM, req, opts)
            .await
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
        self.client
            .do_post(PATH_GET_ARBITRAGE_CONTRACT, &req, opts)
            .await
    }

    /// Get trading parameters by variety.
    ///
    /// Returns comprehensive trading parameters including margins, fees, limits for all varieties.
    ///
    /// # Arguments
    /// * `lang` - Language ("zh" or "en"), defaults to "zh"
    /// * `opts` - Optional request options
    pub async fn get_trading_param(
        &self,
        lang: Option<&str>,
        opts: Option<RequestOptions>,
    ) -> Result<Vec<TradingParam>> {
        let req = TradingParamRequest {
            lang: lang.unwrap_or("zh").to_string(),
        };
        self.client
            .do_post(PATH_GET_TRADING_PARAM, &req, opts)
            .await
    }

    /// Get margin arbitrage performance parameters.
    ///
    /// Returns margin requirements and fees for arbitrage strategies.
    ///
    /// # Arguments
    /// * `req` - Request with variety_id
    /// * `opts` - Optional request options
    pub async fn get_margin_arbi_perf_para(
        &self,
        req: &MarginArbiPerfParaRequest,
        opts: Option<RequestOptions>,
    ) -> Result<Vec<MarginArbiPerfPara>> {
        self.client
            .do_post(PATH_GET_MARGIN_ARBI_PERF_PARA, req, opts)
            .await
    }

    /// Get new contract information (newly listed contracts).
    ///
    /// Returns information about futures/options contracts that were recently added.
    ///
    /// # Arguments
    /// * `req` - Request with variety_id, trade_type, and contract_month
    /// * `opts` - Optional request options
    pub async fn get_new_contract_info(
        &self,
        req: &NewContractInfoRequest,
        opts: Option<RequestOptions>,
    ) -> Result<Vec<NewContractInfo>> {
        self.client
            .do_post(PATH_GET_NEW_CONTRACT_INFO, req, opts)
            .await
    }

    /// Get main series information (market maker continuous quote contracts).
    ///
    /// Returns contracts designated for market maker continuous quoting.
    ///
    /// # Arguments
    /// * `req` - Request with variety_id and trade_type
    /// * `opts` - Optional request options
    pub async fn get_main_series_info(
        &self,
        req: &MainSeriesInfoRequest,
        opts: Option<RequestOptions>,
    ) -> Result<Vec<MainSeriesInfo>> {
        self.client
            .do_post(PATH_GET_MAIN_SERIES_INFO, req, opts)
            .await
    }
}
