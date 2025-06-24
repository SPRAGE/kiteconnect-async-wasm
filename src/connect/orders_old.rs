//! # Orders Module
//! 
//! This module contains order management methods for the KiteConnect API.

use serde_json::Value as JsonValue;
use anyhow::Result;
use std::collections::HashMap;
use crate::connect::utils::RequestHandler;

// Import typed models for dual API support
use crate::models::common::KiteResult;
use crate::models::orders::{Order, OrderParams, Trade, OrderResponse};

use crate::connect::KiteConnect;

impl KiteConnect {
    // === LEGACY API METHODS (JSON responses) ===
    
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

    // === TYPED API METHODS (v1.0.0) ===
    
    /// Place an order with typed response
    /// 
    /// Returns strongly typed order status data instead of JsonValue.
    /// This is the preferred method for new applications.
    /// 
    /// # Arguments
    /// 
    /// * `order_params` - Typed order parameters struct
    /// 
    /// # Returns
    /// 
    /// A `KiteResult<OrderStatusData>` containing typed order status information
    /// 
    /// # Example
    /// 
    /// ```rust,no_run
    /// use kiteconnect_async_wasm::connect::KiteConnect;
    /// use kiteconnect_async_wasm::models::orders::OrderParams;
    /// use kiteconnect_async_wasm::models::common::{Exchange, Product, OrderType, TransactionType, Validity};
    /// 
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = KiteConnect::new("api_key", "access_token");
    /// 
    /// let params = OrderParams {
    ///     tradingsymbol: "INFY".to_string(),
    ///     exchange: Exchange::NSE,
    ///     transaction_type: TransactionType::BUY,
    ///     quantity: 1,
    ///     order_type: OrderType::LIMIT,
    ///     product: Product::CNC,
    ///     price: Some(1500.0),
    ///     validity: Some(Validity::DAY),
    ///     ..Default::default()
    /// };
    /// 
    /// let order_status = client.place_order_typed("regular", &params).await?;
    /// println!("Order ID: {}", order_status.order_id);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn place_order_typed(
        &self,
        variety: &str,
        order_params: &OrderParams,
    ) -> KiteResult<OrderResponse> {
        // Convert typed params to HashMap for legacy API
        let mut params = HashMap::new();
        params.insert("variety", variety);
        params.insert("exchange", &order_params.exchange.to_string());
        params.insert("tradingsymbol", &order_params.trading_symbol);
        params.insert("transaction_type", &order_params.transaction_type.to_string());
        params.insert("quantity", &order_params.quantity.to_string());
        params.insert("product", &order_params.product.to_string());
        params.insert("order_type", &order_params.order_type.to_string());
        
        if let Some(price) = order_params.price {
            params.insert("price", &price.to_string());
        }
        if let Some(validity) = &order_params.validity {
            params.insert("validity", &validity.to_string());
        }
        if let Some(disclosed_quantity) = order_params.disclosed_quantity {
            params.insert("disclosed_quantity", &disclosed_quantity.to_string());
        }
        if let Some(trigger_price) = order_params.trigger_price {
            params.insert("trigger_price", &trigger_price.to_string());
        }
        if let Some(tag) = &order_params.tag {
            params.insert("tag", tag);
        }

        let url = self.build_url(&format!("/orders/{}", variety), None);
        let resp = self.send_request_with_retry(url, "POST", Some(params)).await?;
        let json_response = self.raise_or_return_json_typed(resp).await?;
        
        // Extract the data field from response
        let data = json_response["data"].clone();
        self.parse_response(data)
    }

    /// Get all orders with typed response
    /// 
    /// Returns strongly typed list of orders instead of JsonValue.
    /// 
    /// # Returns
    /// 
    /// A `KiteResult<Vec<Order>>` containing typed order information
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
    /// let orders = client.orders_typed().await?;
    /// for order in orders {
    ///     println!("Order {}: {} - {:?}", 
    ///         order.order_id, 
    ///         order.trading_symbol,
    ///         order.status);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn orders_typed(&self) -> KiteResult<Vec<Order>> {
        let url = self.build_url("/orders", None);
        let resp = self.send_request_with_retry(url, "GET", None).await?;
        let json_response = self.raise_or_return_json_typed(resp).await?;
        
        // Extract the data field from response
        let data = json_response["data"].clone();
        self.parse_response(data)
    }

    /// Get all trades with typed response
    /// 
    /// Returns strongly typed list of trades instead of JsonValue.
    /// 
    /// # Returns
    /// 
    /// A `KiteResult<Vec<Trade>>` containing typed trade information
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
    /// let trades = client.trades_typed().await?;
    /// for trade in trades {
    ///     println!("Trade {}: {} @ {}", 
    ///         trade.trade_id, 
    ///         trade.quantity,
    ///         trade.price);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn trades_typed(&self) -> KiteResult<Vec<Trade>> {
        let url = self.build_url("/trades", None);
        let resp = self.send_request_with_retry(url, "GET", None).await?;
        let json_response = self.raise_or_return_json_typed(resp).await?;
        
        // Extract the data field from response
        let data = json_response["data"].clone();
        self.parse_response(data)
    }

    /// Get trades for specific order with typed response
    /// 
    /// Returns strongly typed list of trades for a specific order instead of JsonValue.
    /// 
    /// # Arguments
    /// 
    /// * `order_id` - The order ID to get trades for
    /// 
    /// # Returns
    /// 
    /// A `KiteResult<Vec<Trade>>` containing typed trade information
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
    /// let trades = client.order_trades_typed("order_id").await?;
    /// for trade in trades {
    ///     println!("Trade executed: {} @ {}", trade.quantity, trade.price);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn order_trades_typed(&self, order_id: &str) -> KiteResult<Vec<Trade>> {
        let url = self.build_url(&format!("/orders/{}/trades", order_id), None);
        let resp = self.send_request_with_retry(url, "GET", None).await?;
        let json_response = self.raise_or_return_json_typed(resp).await?;
        
        // Extract the data field from response
        let data = json_response["data"].clone();
        self.parse_response(data)
    }
}
