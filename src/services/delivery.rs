//! Delivery service for delivery data APIs.

use crate::error::{Error, Result};
use crate::http::{BaseClient, RequestOptions};
use crate::models::{
    BondedDelivery, BondedDeliveryRequest, DeliveryCost, DeliveryData, DeliveryDataRequest,
    DeliveryMatch, DeliveryMatchRequest, FactorySpotAgio, FactorySpotAgioRequest,
    PlywoodDeliveryCommodity, PlywoodDeliveryCommodityRequest, RollDeliverySellerIntention,
    RollDeliverySellerIntentionRequest, TcCongregateDelivery, TcCongregateDeliveryRequest,
    TdBondedDelivery, TdBondedDeliveryRequest, WarehousePremiumResponse,
};

/// API endpoint for delivery data.
const PATH_GET_DELIVERY_DATA: &str = "/dceapi/forward/publicweb/deliverystat/delivery";

/// API endpoint for delivery match data.
const PATH_GET_DELIVERY_MATCH: &str = "/dceapi/forward/publicweb/deliverystat/deliveryMatch";

/// API endpoint for delivery costs.
const PATH_GET_DELIVERY_COST: &str = "/dceapi/forward/publicweb/deliverypara/deliveryCosts";

/// API endpoint for warehouse premium.
const PATH_GET_WAREHOUSE_PREMIUM: &str = "/dceapi/forward/publicweb/deliverypara/floatingAgio";

/// API endpoint for TC congregate delivery statistics.
const PATH_GET_TC_CONGREGATE_DELIVERY: &str =
    "/dceapi/forward/publicweb/DeliveryStatistics/tcCongregateDeliveryQuotes";

/// API endpoint for roll delivery seller intention.
const PATH_GET_ROLL_DELIVERY_SELLER_INTENTION: &str =
    "/dceapi/forward/publicweb/DeliveryStatistics/rollDeliverySellerIntention";

/// API endpoint for bonded delivery settlement price.
const PATH_GET_BONDED_DELIVERY: &str = "/dceapi/forward/publicweb/quotesdata/bondedDelivery";

/// API endpoint for TD bonded delivery settlement price.
const PATH_GET_TD_BONDED_DELIVERY: &str = "/dceapi/forward/publicweb/quotesdata/tdBondedDelivery";

/// API endpoint for factory spot premium (basis spread for fiberboard).
const PATH_GET_FACTORY_SPOT_AGIO: &str =
    "/dceapi/forward/publicweb/quotesdata/queryFactorySpotAgioQuotes";

/// API endpoint for plywood delivery commodity information.
const PATH_GET_PLYWOOD_DELIVERY_COMMODITY: &str =
    "/dceapi/forward/publicweb/deliverystat/queryPlywoodDeliveryCommodity";

/// Delivery service for accessing delivery-related data.
#[derive(Debug, Clone)]
pub struct DeliveryService {
    client: BaseClient,
}

impl DeliveryService {
    /// Create a new delivery service.
    pub fn new(client: BaseClient) -> Self {
        DeliveryService { client }
    }

    /// Get delivery data.
    ///
    /// # Arguments
    /// * `req` - Request with variety code and trade date
    /// * `opts` - Optional request options
    pub async fn get_delivery_data(
        &self,
        req: &DeliveryDataRequest,
        opts: Option<RequestOptions>,
    ) -> Result<Vec<DeliveryData>> {
        self.client.do_post(PATH_GET_DELIVERY_DATA, req, opts).await
    }

    /// Get delivery match data.
    ///
    /// # Arguments
    /// * `req` - Request with variety code and trade date
    /// * `opts` - Optional request options
    pub async fn get_delivery_match(
        &self,
        req: &DeliveryMatchRequest,
        opts: Option<RequestOptions>,
    ) -> Result<Vec<DeliveryMatch>> {
        self.client
            .do_post(PATH_GET_DELIVERY_MATCH, req, opts)
            .await
    }

    /// Get delivery cost for a variety.
    ///
    /// # Arguments
    /// * `variety_id` - Variety ID ("all" for all varieties)
    /// * `variety_type` - Variety type ("0" = physical delivery, "1" = average price delivery)
    /// * `opts` - Optional request options
    pub async fn get_delivery_cost(
        &self,
        variety_id: &str,
        variety_type: &str,
        opts: Option<RequestOptions>,
    ) -> Result<Vec<DeliveryCost>> {
        if variety_id.is_empty() {
            return Err(Error::validation("variety_id", "variety_id is required"));
        }

        #[derive(serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Request<'a> {
            variety_id: &'a str,
            variety_type: &'a str,
            lang: &'a str,
        }

        let req = Request {
            variety_id,
            variety_type,
            lang: "zh",
        };
        self.client
            .do_post(PATH_GET_DELIVERY_COST, &req, opts)
            .await
    }

    /// Get warehouse premium for a variety.
    ///
    /// # Arguments
    /// * `variety_id` - Variety ID
    /// * `trade_date` - Trade date (YYYYMMDD format)
    /// * `opts` - Optional request options
    pub async fn get_warehouse_premium(
        &self,
        variety_id: &str,
        trade_date: &str,
        opts: Option<RequestOptions>,
    ) -> Result<WarehousePremiumResponse> {
        if variety_id.is_empty() {
            return Err(Error::validation("variety_id", "variety_id is required"));
        }

        #[derive(serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Request<'a> {
            variety_id: &'a str,
            trade_date: &'a str,
        }

        let req = Request {
            variety_id,
            trade_date,
        };
        self.client
            .do_post(PATH_GET_WAREHOUSE_PREMIUM, &req, opts)
            .await
    }

    /// Get TC (two-way delivery) congregate delivery statistics.
    ///
    /// Returns aggregated delivery information for varieties supporting two-way delivery.
    ///
    /// # Arguments
    /// * `req` - Request with variety_id and trade_date
    /// * `opts` - Optional request options
    pub async fn get_tc_congregate_delivery(
        &self,
        req: &TcCongregateDeliveryRequest,
        opts: Option<RequestOptions>,
    ) -> Result<Vec<TcCongregateDelivery>> {
        self.client
            .do_post(PATH_GET_TC_CONGREGATE_DELIVERY, req, opts)
            .await
    }

    /// Get roll delivery seller intention.
    ///
    /// Returns seller's delivery intentions for rolling delivery contracts.
    ///
    /// # Arguments
    /// * `req` - Request with variety_id and trade_date
    /// * `opts` - Optional request options
    pub async fn get_roll_delivery_seller_intention(
        &self,
        req: &RollDeliverySellerIntentionRequest,
        opts: Option<RequestOptions>,
    ) -> Result<Vec<RollDeliverySellerIntention>> {
        self.client
            .do_post(PATH_GET_ROLL_DELIVERY_SELLER_INTENTION, req, opts)
            .await
    }

    /// Get bonded delivery data.
    ///
    /// Returns delivery statistics for bonded warehouse delivery mode.
    ///
    /// # Arguments
    /// * `req` - Request with variety_id and trade_date
    /// * `opts` - Optional request options
    pub async fn get_bonded_delivery(
        &self,
        req: &BondedDeliveryRequest,
        opts: Option<RequestOptions>,
    ) -> Result<Vec<BondedDelivery>> {
        self.client
            .do_post(PATH_GET_BONDED_DELIVERY, req, opts)
            .await
    }

    /// Get TD (two-day) bonded delivery data.
    ///
    /// Returns bonded delivery data with two-day settlement mode.
    ///
    /// # Arguments
    /// * `req` - Request with variety_id and trade_date
    /// * `opts` - Optional request options
    pub async fn get_td_bonded_delivery(
        &self,
        req: &TdBondedDeliveryRequest,
        opts: Option<RequestOptions>,
    ) -> Result<Vec<TdBondedDelivery>> {
        self.client
            .do_post(PATH_GET_TD_BONDED_DELIVERY, req, opts)
            .await
    }

    /// Get factory spot premium (basis spread).
    ///
    /// Returns the difference between factory spot price and futures price.
    ///
    /// # Arguments
    /// * `req` - Request with variety_id
    /// * `opts` - Optional request options
    pub async fn get_factory_spot_agio(
        &self,
        req: &FactorySpotAgioRequest,
        opts: Option<RequestOptions>,
    ) -> Result<Vec<FactorySpotAgio>> {
        self.client
            .do_post(PATH_GET_FACTORY_SPOT_AGIO, req, opts)
            .await
    }

    /// Get plywood delivery commodity information.
    ///
    /// Returns delivery specifications and parameters specific to plywood contracts.
    ///
    /// # Arguments
    /// * `req` - Request with variety_id
    /// * `opts` - Optional request options
    pub async fn get_plywood_delivery_commodity(
        &self,
        req: &PlywoodDeliveryCommodityRequest,
        opts: Option<RequestOptions>,
    ) -> Result<Vec<PlywoodDeliveryCommodity>> {
        self.client
            .do_post(PATH_GET_PLYWOOD_DELIVERY_COMMODITY, req, opts)
            .await
    }
}
