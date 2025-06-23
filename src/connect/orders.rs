//! # Orders Module
//! 
//! This module contains order management methods for the KiteConnect API.

use serde_json::Value as JsonValue;
use anyhow::Result;
use std::collections::HashMap;
use crate::connect::utils::RequestHandler;

use crate::connect::KiteConnect;

impl KiteConnect {
    /// Place an order
    pub async fn place_order(
        &self,
        variety: &str,
        exchange: &str,
        tradingsymbol: &str,
        transaction_type: &str,
        quantity: &str,
        product: Option<&str>,
        order_type: Option<&str>,
        price: Option<&str>,
        validity: Option<&str>,
        disclosed_quantity: Option<&str>,
        trigger_price: Option<&str>,
        squareoff: Option<&str>,
        stoploss: Option<&str>,
        trailing_stoploss: Option<&str>,
        tag: Option<&str>,
    ) -> Result<JsonValue> {
        let mut params = HashMap::new();
        params.insert("variety", variety);
        params.insert("exchange", exchange);
        params.insert("tradingsymbol", tradingsymbol);
        params.insert("transaction_type", transaction_type);
        params.insert("quantity", quantity);
        
        if let Some(product) = product { params.insert("product", product); }
        if let Some(order_type) = order_type { params.insert("order_type", order_type); }
        if let Some(price) = price { params.insert("price", price); }
        if let Some(validity) = validity { params.insert("validity", validity); }
        if let Some(disclosed_quantity) = disclosed_quantity { params.insert("disclosed_quantity", disclosed_quantity); }
        if let Some(trigger_price) = trigger_price { params.insert("trigger_price", trigger_price); }
        if let Some(squareoff) = squareoff { params.insert("squareoff", squareoff); }
        if let Some(stoploss) = stoploss { params.insert("stoploss", stoploss); }
        if let Some(trailing_stoploss) = trailing_stoploss { params.insert("trailing_stoploss", trailing_stoploss); }
        if let Some(tag) = tag { params.insert("tag", tag); }

        let url = self.build_url(&format!("/orders/{}", variety), None);
        let resp = self.send_request(url, "POST", Some(params)).await?;
        self.raise_or_return_json(resp).await
    }

    /// Modify an open order
    pub async fn modify_order(
        &self,
        order_id: &str,
        variety: &str,
        quantity: Option<&str>,
        price: Option<&str>,
        order_type: Option<&str>,
        validity: Option<&str>,
        disclosed_quantity: Option<&str>,
        trigger_price: Option<&str>,
        parent_order_id: Option<&str>,
    ) -> Result<JsonValue> {
        let mut params = HashMap::new();
        params.insert("order_id", order_id);
        params.insert("variety", variety);
        
        if let Some(quantity) = quantity { params.insert("quantity", quantity); }
        if let Some(price) = price { params.insert("price", price); }
        if let Some(order_type) = order_type { params.insert("order_type", order_type); }
        if let Some(validity) = validity { params.insert("validity", validity); }
        if let Some(disclosed_quantity) = disclosed_quantity { params.insert("disclosed_quantity", disclosed_quantity); }
        if let Some(trigger_price) = trigger_price { params.insert("trigger_price", trigger_price); }
        if let Some(parent_order_id) = parent_order_id { params.insert("parent_order_id", parent_order_id); }

        let url = self.build_url(&format!("/orders/{}/{}", variety, order_id), None);
        let resp = self.send_request(url, "PUT", Some(params)).await?;
        self.raise_or_return_json(resp).await
    }

    /// Cancel an order
    pub async fn cancel_order(
        &self,
        order_id: &str,
        variety: &str,
        parent_order_id: Option<&str>,
    ) -> Result<JsonValue> {
        let mut params = HashMap::new();
        params.insert("order_id", order_id);
        params.insert("variety", variety);
        if let Some(parent_order_id) = parent_order_id {
            params.insert("parent_order_id", parent_order_id);
        }

        let url = self.build_url(&format!("/orders/{}/{}", variety, order_id), None);
        let resp = self.send_request(url, "DELETE", Some(params)).await?;
        self.raise_or_return_json(resp).await
    }

    /// Exit a BO/CO order
    pub async fn exit_order(
        &self,
        order_id: &str,
        variety: &str,
        parent_order_id: Option<&str>,
    ) -> Result<JsonValue> {
        self.cancel_order(order_id, variety, parent_order_id).await
    }

    /// Retrieves a list of all orders for the current trading day
    /// 
    /// Returns all orders placed by the user for the current trading day,
    /// including pending, completed, rejected, and cancelled orders.
    /// 
    /// # Returns
    /// 
    /// A `Result<JsonValue>` containing orders data with fields like:
    /// - `order_id` - Unique order identifier
    /// - `tradingsymbol` - Trading symbol
    /// - `quantity` - Order quantity
    /// - `price` - Order price
    /// - `status` - Order status (OPEN, COMPLETE, CANCELLED, REJECTED)
    /// - `order_type` - Order type (MARKET, LIMIT, SL, SL-M)
    /// - `product` - Product type (MIS, CNC, NRML)
    /// 
    /// # Errors
    /// 
    /// Returns an error if the API request fails or the user is not authenticated.
    /// 
    /// # Example
    /// 
    /// ```rust,no_run
    /// use kiteconnect_async_wasm::connect::KiteConnect;
    /// 
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = KiteConnect::new("api_key", "access_token");
    /// 
    /// let orders = client.orders().await?;
    /// println!("Orders: {:?}", orders);
    /// 
    /// // Check order statuses
    /// if let Some(data) = orders["data"].as_array() {
    ///     for order in data {
    ///         println!("Order {}: {} - {}", 
    ///             order["order_id"], 
    ///             order["tradingsymbol"], 
    ///             order["status"]);
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn orders(&self) -> Result<JsonValue> {
        let url = self.build_url("/orders", None);
        let resp = self.send_request(url, "GET", None).await?;
        self.raise_or_return_json(resp).await
    }

    /// Get the list of order history
    pub async fn order_history(&self, order_id: &str) -> Result<JsonValue> {
        let params = vec![("order_id", order_id)];
        let url = self.build_url("/orders", Some(params));
        let resp = self.send_request(url, "GET", None).await?;
        self.raise_or_return_json(resp).await
    }

    /// Get all trades
    pub async fn trades(&self) -> Result<JsonValue> {
        let url = self.build_url("/trades", None);
        let resp = self.send_request(url, "GET", None).await?;
        self.raise_or_return_json(resp).await
    }

    /// Get all trades for a specific order
    pub async fn order_trades(&self, order_id: &str) -> Result<JsonValue> {
        let url = self.build_url(&format!("/orders/{}/trades", order_id), None);
        let resp = self.send_request(url, "GET", None).await?;
        self.raise_or_return_json(resp).await
    }

    /// Modify an open position product type
    pub async fn convert_position(
        &self,
        exchange: &str,
        tradingsymbol: &str,
        transaction_type: &str,
        position_type: &str,
        quantity: &str,
        old_product: &str,
        new_product: &str,
    ) -> Result<JsonValue> {
        let mut params = HashMap::new();
        params.insert("exchange", exchange);
        params.insert("tradingsymbol", tradingsymbol);
        params.insert("transaction_type", transaction_type);
        params.insert("position_type", position_type);
        params.insert("quantity", quantity);
        params.insert("old_product", old_product);
        params.insert("new_product", new_product);

        let url = self.build_url("/portfolio/positions", None);
        let resp = self.send_request(url, "PUT", Some(params)).await?;
        self.raise_or_return_json(resp).await
    }
}
