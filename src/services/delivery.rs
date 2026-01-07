//! Delivery service for delivery data APIs.

use crate::error::{Error, Result};
use crate::http::{BaseClient, RequestOptions};
use crate::models::{
    DeliveryCost, DeliveryData, DeliveryDataRequest, DeliveryMatch, DeliveryMatchRequest,
    WarehousePremium, WarehouseReceipt, WarehouseReceiptRequest,
};

/// API endpoint for delivery data.
const PATH_GET_DELIVERY_DATA: &str = "/dceapi/forward/publicweb/deliverystat/delivery";

/// API endpoint for delivery match data.
const PATH_GET_DELIVERY_MATCH: &str = "/dceapi/forward/publicweb/deliverystat/deliveryMatch";

/// API endpoint for warehouse receipt data.
const PATH_GET_WAREHOUSE_RECEIPT: &str = "/dceapi/forward/publicweb/deliverystat/warehouseReceipt";

/// API endpoint for delivery costs.
const PATH_GET_DELIVERY_COST: &str = "/dceapi/forward/publicweb/deliverypara/deliveryCosts";

/// API endpoint for warehouse premium.
const PATH_GET_WAREHOUSE_PREMIUM: &str = "/dceapi/forward/publicweb/deliverypara/floatingAgio";

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
        self.client.do_post(PATH_GET_DELIVERY_MATCH, req, opts).await
    }

    /// Get warehouse receipt data.
    ///
    /// # Arguments
    /// * `req` - Request with variety code and trade date
    /// * `opts` - Optional request options
    pub async fn get_warehouse_receipt(
        &self,
        req: &WarehouseReceiptRequest,
        opts: Option<RequestOptions>,
    ) -> Result<Vec<WarehouseReceipt>> {
        self.client.do_post(PATH_GET_WAREHOUSE_RECEIPT, req, opts).await
    }

    /// Get delivery cost for a variety.
    ///
    /// # Arguments
    /// * `variety` - Variety code
    /// * `opts` - Optional request options
    pub async fn get_delivery_cost(
        &self,
        variety: &str,
        opts: Option<RequestOptions>,
    ) -> Result<DeliveryCost> {
        if variety.is_empty() {
            return Err(Error::validation("variety", "variety is required"));
        }

        #[derive(serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Request<'a> {
            variety_code: &'a str,
        }

        let req = Request { variety_code: variety };
        self.client.do_post(PATH_GET_DELIVERY_COST, &req, opts).await
    }

    /// Get warehouse premium for a variety.
    ///
    /// # Arguments
    /// * `variety` - Variety code
    /// * `opts` - Optional request options
    pub async fn get_warehouse_premium(
        &self,
        variety: &str,
        opts: Option<RequestOptions>,
    ) -> Result<Vec<WarehousePremium>> {
        if variety.is_empty() {
            return Err(Error::validation("variety", "variety is required"));
        }

        #[derive(serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Request<'a> {
            variety_code: &'a str,
        }

        let req = Request { variety_code: variety };
        self.client.do_post(PATH_GET_WAREHOUSE_PREMIUM, &req, opts).await
    }
}
