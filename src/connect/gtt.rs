//! # GTT (Good Till Triggered) Module
//! 
//! This module contains GTT-related methods for the KiteConnect API.

use serde_json::Value as JsonValue;
use anyhow::Result;
use std::collections::HashMap;
use crate::connect::endpoints::KiteEndpoint;
use crate::connect::KiteConnect;

impl KiteConnect {
    /// Get all GTT orders or details of a specific GTT
    /// 
    /// Retrieves all Good Till Triggered (GTT) orders or details of a specific GTT order.
    /// GTT orders are conditional orders that get executed when certain trigger conditions are met.
    /// 
    /// # Arguments
    /// 
    /// * `gtt_id` - Optional GTT ID. If None, returns all GTT orders
    /// 
    /// # Returns
    /// 
    /// A `Result<JsonValue>` containing GTT orders data
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
    /// // Get all GTT orders
    /// let all_gtts = client.get_gtts(None).await?;
    /// println!("All GTTs: {:?}", all_gtts);
    /// 
    /// // Get specific GTT
    /// let gtt_details = client.get_gtts(Some("123456")).await?;
    /// println!("GTT details: {:?}", gtt_details);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_gtts(&self, gtt_id: Option<&str>) -> Result<JsonValue> {
        let resp = if let Some(id) = gtt_id {
            self.send_request_with_rate_limiting_and_retry(
                KiteEndpoint::GTTInfo,
                &[id],
                None,
                None,
            ).await.map_err(|e| anyhow::anyhow!("Failed to get GTT info: {}", e))?
        } else {
            self.send_request_with_rate_limiting_and_retry(
                KiteEndpoint::GTTs,
                &[],
                None,
                None,
            ).await.map_err(|e| anyhow::anyhow!("Failed to get GTTs: {}", e))?
        };
        
        self.raise_or_return_json(resp).await
    }

    /// Place a GTT order
    /// 
    /// Creates a new Good Till Triggered order that will be executed when 
    /// the specified trigger conditions are met.
    /// 
    /// # Arguments
    /// 
    /// * `trigger_type` - Type of trigger ("single" or "two-leg")
    /// * `tradingsymbol` - Trading symbol of the instrument
    /// * `exchange` - Exchange where the instrument is traded
    /// * `trigger_values` - Trigger price values
    /// * `last_price` - Current market price of the instrument
    /// * `orders` - List of orders to be placed when triggered
    /// 
    /// # Returns
    /// 
    /// A `Result<JsonValue>` containing GTT order creation response
    /// 
    /// # Errors
    /// 
    /// Returns an error if the GTT placement fails or parameters are invalid
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
    /// let gtt = client.place_gtt(
    ///     "single",
    ///     "RELIANCE",
    ///     "NSE",
    ///     &[2500.0], // Trigger when price hits 2500
    ///     2450.0,    // Current price
    ///     &[serde_json::json!({
    ///         "transaction_type": "SELL",
    ///         "quantity": 10,
    ///         "order_type": "LIMIT",
    ///         "product": "CNC",
    ///         "price": 2500.0
    ///     })]
    /// ).await?;
    /// 
    /// println!("GTT placed: {:?}", gtt);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn place_gtt(
        &self,
        trigger_type: &str,
        tradingsymbol: &str,
        exchange: &str,
        trigger_values: &[f64],
        last_price: f64,
        orders: &[JsonValue],
    ) -> Result<JsonValue> {
        let mut params = HashMap::new();
        params.insert("type", trigger_type);
        params.insert("tradingsymbol", tradingsymbol);
        params.insert("exchange", exchange);
        
        // Convert trigger values to comma-separated string
        let trigger_values_str = trigger_values.iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(",");
        params.insert("trigger_values", &trigger_values_str);
        
        let last_price_str = last_price.to_string();
        params.insert("last_price", &last_price_str);
        
        // Convert orders to JSON string
        let orders_json = serde_json::to_string(orders)?;
        params.insert("orders", &orders_json);

        let resp = self.send_request_with_rate_limiting_and_retry(
            KiteEndpoint::PlaceGTT,
            &[],
            None,
            Some(params),
        ).await.map_err(|e| anyhow::anyhow!("Failed to place GTT: {}", e))?;
        
        self.raise_or_return_json(resp).await
    }

    /// Modify a GTT order
    /// 
    /// Updates an existing GTT order with new trigger conditions or orders.
    /// 
    /// # Arguments
    /// 
    /// * `gtt_id` - GTT order ID to modify
    /// * `trigger_type` - Type of trigger ("single" or "two-leg")
    /// * `tradingsymbol` - Trading symbol of the instrument
    /// * `exchange` - Exchange where the instrument is traded
    /// * `trigger_values` - New trigger price values
    /// * `last_price` - Current market price of the instrument
    /// * `orders` - Updated list of orders to be placed when triggered
    /// 
    /// # Returns
    /// 
    /// A `Result<JsonValue>` containing GTT order modification response
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
    /// let result = client.modify_gtt(
    ///     "123456",  // GTT ID
    ///     "single",
    ///     "RELIANCE",
    ///     "NSE",
    ///     &[2600.0], // New trigger price
    ///     2450.0,    // Current price
    ///     &[serde_json::json!({
    ///         "transaction_type": "SELL",
    ///         "quantity": 20,  // Updated quantity
    ///         "order_type": "LIMIT",
    ///         "product": "CNC",
    ///         "price": 2600.0
    ///     })]
    /// ).await?;
    /// 
    /// println!("GTT modified: {:?}", result);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn modify_gtt(
        &self,
        gtt_id: &str,
        trigger_type: &str,
        tradingsymbol: &str,
        exchange: &str,
        trigger_values: &[f64],
        last_price: f64,
        orders: &[JsonValue],
    ) -> Result<JsonValue> {
        let mut params = HashMap::new();
        params.insert("type", trigger_type);
        params.insert("tradingsymbol", tradingsymbol);
        params.insert("exchange", exchange);
        
        // Convert trigger values to comma-separated string
        let trigger_values_str = trigger_values.iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(",");
        params.insert("trigger_values", &trigger_values_str);
        
        let last_price_str = last_price.to_string();
        params.insert("last_price", &last_price_str);
        
        // Convert orders to JSON string
        let orders_json = serde_json::to_string(orders)?;
        params.insert("orders", &orders_json);

        let resp = self.send_request_with_rate_limiting_and_retry(
            KiteEndpoint::ModifyGTT,
            &[gtt_id],
            None,
            Some(params),
        ).await.map_err(|e| anyhow::anyhow!("Failed to modify GTT: {}", e))?;
        
        self.raise_or_return_json(resp).await
    }

    /// Delete a GTT order
    /// 
    /// Cancels an existing GTT order. Once deleted, the GTT will no longer
    /// monitor for trigger conditions.
    /// 
    /// # Arguments
    /// 
    /// * `gtt_id` - GTT order ID to delete
    /// 
    /// # Returns
    /// 
    /// A `Result<JsonValue>` containing deletion confirmation
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
    /// let result = client.delete_gtt("123456").await?;
    /// println!("GTT deleted: {:?}", result);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete_gtt(&self, gtt_id: &str) -> Result<JsonValue> {
        let resp = self.send_request_with_rate_limiting_and_retry(
            KiteEndpoint::CancelGTT,
            &[gtt_id],
            None,
            None,
        ).await.map_err(|e| anyhow::anyhow!("Failed to delete GTT: {}", e))?;
        
        self.raise_or_return_json(resp).await
    }
}
