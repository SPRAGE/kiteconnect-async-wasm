//! # Orders Module
//!
//! This module contains order management methods for the KiteConnect API.

use crate::connect::endpoints::KiteEndpoint;
use anyhow::Result;
use serde_json::Value as JsonValue;
use std::collections::HashMap;

// Import typed models for dual API support
use crate::models::common::KiteResult;
use crate::models::orders::{Order, OrderParams, OrderResponse, Trade};

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

        if let Some(product) = product {
            params.insert("product", product);
        }
        if let Some(order_type) = order_type {
            params.insert("order_type", order_type);
        }
        if let Some(price) = price {
            params.insert("price", price);
        }
        if let Some(validity) = validity {
            params.insert("validity", validity);
        }
        if let Some(disclosed_quantity) = disclosed_quantity {
            params.insert("disclosed_quantity", disclosed_quantity);
        }
        if let Some(trigger_price) = trigger_price {
            params.insert("trigger_price", trigger_price);
        }
        if let Some(squareoff) = squareoff {
            params.insert("squareoff", squareoff);
        }
        if let Some(stoploss) = stoploss {
            params.insert("stoploss", stoploss);
        }
        if let Some(trailing_stoploss) = trailing_stoploss {
            params.insert("trailing_stoploss", trailing_stoploss);
        }
        if let Some(tag) = tag {
            params.insert("tag", tag);
        }

        let resp = self
            .send_request_with_rate_limiting_and_retry(
                KiteEndpoint::PlaceOrder,
                &[variety],
                None,
                Some(params),
            )
            .await
            .map_err(|e| anyhow::anyhow!("Place order failed: {:?}", e))?;
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

        if let Some(quantity) = quantity {
            params.insert("quantity", quantity);
        }
        if let Some(price) = price {
            params.insert("price", price);
        }
        if let Some(order_type) = order_type {
            params.insert("order_type", order_type);
        }
        if let Some(validity) = validity {
            params.insert("validity", validity);
        }
        if let Some(disclosed_quantity) = disclosed_quantity {
            params.insert("disclosed_quantity", disclosed_quantity);
        }
        if let Some(trigger_price) = trigger_price {
            params.insert("trigger_price", trigger_price);
        }
        if let Some(parent_order_id) = parent_order_id {
            params.insert("parent_order_id", parent_order_id);
        }

        let resp = self
            .send_request_with_rate_limiting_and_retry(
                KiteEndpoint::ModifyOrder,
                &[variety, order_id],
                None,
                Some(params),
            )
            .await
            .map_err(|e| anyhow::anyhow!("Modify order failed: {:?}", e))?;
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

        let resp = self
            .send_request_with_rate_limiting_and_retry(
                KiteEndpoint::CancelOrder,
                &[variety, order_id],
                None,
                Some(params),
            )
            .await
            .map_err(|e| anyhow::anyhow!("Cancel order failed: {:?}", e))?;
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
        let resp = self
            .send_request_with_rate_limiting_and_retry(KiteEndpoint::Orders, &[], None, None)
            .await
            .map_err(|e| anyhow::anyhow!("Get orders failed: {:?}", e))?;
        self.raise_or_return_json(resp).await
    }

    /// Get the list of order history
    pub async fn order_history(&self, order_id: &str) -> Result<JsonValue> {
        let params = vec![("order_id", order_id)];
        let resp = self
            .send_request_with_rate_limiting_and_retry(
                KiteEndpoint::OrderHistory,
                &[],
                Some(params),
                None,
            )
            .await
            .map_err(|e| anyhow::anyhow!("Get order history failed: {:?}", e))?;
        self.raise_or_return_json(resp).await
    }

    /// Get all trades
    pub async fn trades(&self) -> Result<JsonValue> {
        let resp = self
            .send_request_with_rate_limiting_and_retry(KiteEndpoint::Trades, &[], None, None)
            .await
            .map_err(|e| anyhow::anyhow!("Get trades failed: {:?}", e))?;
        self.raise_or_return_json(resp).await
    }

    /// Get all trades for a specific order
    pub async fn order_trades(&self, order_id: &str) -> Result<JsonValue> {
        let resp = self
            .send_request_with_rate_limiting_and_retry(
                KiteEndpoint::OrderTrades,
                &[order_id],
                None,
                None,
            )
            .await
            .map_err(|e| anyhow::anyhow!("Get order trades failed: {:?}", e))?;
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

        let resp = self
            .send_request_with_rate_limiting_and_retry(
                KiteEndpoint::ConvertPosition,
                &[],
                None,
                Some(params),
            )
            .await
            .map_err(|e| anyhow::anyhow!("Convert position failed: {:?}", e))?;
        self.raise_or_return_json(resp).await
    }

    // === TYPED API METHODS (v1.0.0) ===

    /// Place an order with typed response
    ///
    /// Returns strongly typed order response instead of JsonValue.
    /// This is the preferred method for new applications.
    ///
    /// # Arguments
    ///
    /// * `variety` - Order variety ("regular", "bo", "co", etc.)
    /// * `order_params` - Typed order parameters struct
    ///
    /// # Returns
    ///
    /// A `KiteResult<OrderResponse>` containing the order ID
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
    ///     trading_symbol: "INFY".to_string(),
    ///     exchange: Exchange::NSE,
    ///     transaction_type: TransactionType::BUY,
    ///     quantity: 1,
    ///     order_type: OrderType::LIMIT,
    ///     product: Product::CNC,
    ///     price: Some(1500.0),
    ///     validity: Some(Validity::DAY),
    ///     disclosed_quantity: None,
    ///     trigger_price: None,
    ///     squareoff: None,
    ///     stoploss: None,
    ///     trailing_stoploss: None,
    ///     market_protection: None,
    ///     iceberg_legs: None,
    ///     iceberg_quantity: None,
    ///     auction_number: None,
    ///     tag: None,
    /// };
    ///
    /// let order_response = client.place_order_typed("regular", &params).await?;
    /// println!("Order ID: {}", order_response.order_id);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn place_order_typed(
        &self,
        variety: &str,
        order_params: &OrderParams,
    ) -> KiteResult<OrderResponse> {
        // Create all string conversions upfront to avoid lifetime issues
        let exchange_str = order_params.exchange.to_string();
        let transaction_type_str = order_params.transaction_type.to_string();
        let quantity_str = order_params.quantity.to_string();
        let product_str = order_params.product.to_string();
        let order_type_str = order_params.order_type.to_string();

        let price_str = order_params.price.map(|p| p.to_string());
        let validity_str = order_params.validity.as_ref().map(|v| v.to_string());
        let disclosed_str = order_params.disclosed_quantity.map(|d| d.to_string());
        let trigger_str = order_params.trigger_price.map(|t| t.to_string());

        let mut params = HashMap::new();
        params.insert("variety", variety);
        params.insert("exchange", exchange_str.as_str());
        params.insert("tradingsymbol", order_params.trading_symbol.as_str());
        params.insert("transaction_type", transaction_type_str.as_str());
        params.insert("quantity", quantity_str.as_str());
        params.insert("product", product_str.as_str());
        params.insert("order_type", order_type_str.as_str());

        if let Some(ref price) = price_str {
            params.insert("price", price.as_str());
        }
        if let Some(ref validity) = validity_str {
            params.insert("validity", validity.as_str());
        }
        if let Some(ref disclosed) = disclosed_str {
            params.insert("disclosed_quantity", disclosed.as_str());
        }
        if let Some(ref trigger) = trigger_str {
            params.insert("trigger_price", trigger.as_str());
        }
        if let Some(ref tag) = order_params.tag {
            params.insert("tag", tag.as_str());
        }

        let resp = self
            .send_request_with_rate_limiting_and_retry(
                KiteEndpoint::PlaceOrder,
                &[variety],
                None,
                Some(params),
            )
            .await?;
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
        let resp = self
            .send_request_with_rate_limiting_and_retry(KiteEndpoint::Orders, &[], None, None)
            .await?;
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
    ///         trade.average_price);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn trades_typed(&self) -> KiteResult<Vec<Trade>> {
        let resp = self
            .send_request_with_rate_limiting_and_retry(KiteEndpoint::Trades, &[], None, None)
            .await?;
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
    ///     println!("Trade executed: {} @ {}", trade.quantity, trade.average_price);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn order_trades_typed(&self, order_id: &str) -> KiteResult<Vec<Trade>> {
        let resp = self
            .send_request_with_rate_limiting_and_retry(
                KiteEndpoint::OrderTrades,
                &[order_id],
                None,
                None,
            )
            .await?;
        let json_response = self.raise_or_return_json_typed(resp).await?;

        // Extract the data field from response
        let data = json_response["data"].clone();
        self.parse_response(data)
    }
}
