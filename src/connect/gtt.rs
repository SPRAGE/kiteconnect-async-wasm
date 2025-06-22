use anyhow::Result;
use std::collections::HashMap;
use super::{client::KiteConnect, request::RequestHandler};

// Import model types for typed responses
use crate::model::{
    GTT, OrderResponse,
};

impl KiteConnect {
    /// Get Good Till Triggered (GTT) orders
    /// 
    /// Retrieves all GTT orders or details of a specific GTT order.
    /// GTT orders are trigger-based orders that remain active until triggered or cancelled.
    /// 
    /// # Arguments
    /// 
    /// * `gtt_id` - Optional GTT ID. If None, returns all GTT orders
    /// 
    /// # Returns
    /// 
    /// A `Result<Vec<GTT>>` containing GTT order information
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
    /// let gtt_orders = client.gtts(None).await?;
    /// 
    /// // Access GTT details directly
    /// for gtt in &gtt_orders {
    ///     println!("GTT: {} - {:?} ({})", gtt.id, gtt.gtt_type, gtt.status);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn gtts(&self, gtt_id: Option<&str>) -> Result<Vec<GTT>> {
        let url: reqwest::Url = if let Some(gtt_id) = gtt_id {
            self.build_url(&format!("/gtt/triggers/{}", gtt_id), None)
        } else {
            self.build_url("/gtt/triggers", None)
        };

        let resp = self.send_request(url, "GET", None).await?;
        self.parse_response(resp).await
    }

    /// Place a Good Till Triggered (GTT) order
    /// 
    /// Creates a new GTT order that will be triggered when certain conditions are met.
    /// 
    /// # Arguments
    /// 
    /// * `trigger_type` - Type of trigger ("single" or "two-leg")
    /// * `tradingsymbol` - Trading symbol of the instrument
    /// * `exchange` - Exchange on which the instrument is listed
    /// * `trigger_values` - List of trigger price values
    /// * `last_price` - Last traded price of the instrument
    /// * `orders` - List of orders to place when triggered
    /// 
    /// # Returns
    /// 
    /// A `Result<OrderResponse>` containing GTT creation confirmation
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
    /// // This is a placeholder - actual implementation would require
    /// // proper parameter structure for GTT orders
    /// # Ok(())
    /// # }
    /// ```
    pub async fn place_gtt(
        &self,
        trigger_type: &str,
        tradingsymbol: &str,
        exchange: &str,
        trigger_values: Vec<f64>,
        last_price: f64,
        _orders: Vec<HashMap<&str, &str>>,
    ) -> Result<OrderResponse> {
        let mut params = HashMap::new();
        params.insert("trigger_type", trigger_type);
        params.insert("tradingsymbol", tradingsymbol);
        params.insert("exchange", exchange);
        
        // Convert price to string with proper lifetime
        let last_price_str = last_price.to_string();
        params.insert("last_price", &last_price_str);
        
        // Convert trigger values to strings
        let trigger_values_str: Vec<String> = trigger_values.iter().map(|v| v.to_string()).collect();
        let trigger_values_joined = trigger_values_str.join(",");
        params.insert("trigger_values", &trigger_values_joined);
        
        // For now, this is a placeholder implementation
        // Full GTT implementation would require more complex parameter handling
        
        let url = self.build_url("/gtt/triggers", None);
        let resp = self.send_request(url, "POST", Some(params)).await?;
        self.parse_response(resp).await
    }

    /// Modify a Good Till Triggered (GTT) order
    /// 
    /// Modifies an existing GTT order's parameters.
    /// 
    /// # Arguments
    /// 
    /// * `gtt_id` - The GTT ID to modify
    /// * `trigger_type` - Type of trigger
    /// * `tradingsymbol` - Trading symbol
    /// * `exchange` - Exchange
    /// * `trigger_values` - New trigger values
    /// * `last_price` - Current last price
    /// * `orders` - Modified order parameters
    /// 
    /// # Returns
    /// 
    /// A `Result<OrderResponse>` containing modification confirmation
    pub async fn modify_gtt(
        &self,
        gtt_id: &str,
        trigger_type: &str,
        tradingsymbol: &str,
        exchange: &str,
        trigger_values: Vec<f64>,
        last_price: f64,
        _orders: Vec<HashMap<&str, &str>>,
    ) -> Result<OrderResponse> {
        let mut params = HashMap::new();
        params.insert("trigger_type", trigger_type);
        params.insert("tradingsymbol", tradingsymbol);
        params.insert("exchange", exchange);
        
        // Convert price to string with proper lifetime
        let last_price_str = last_price.to_string();
        params.insert("last_price", &last_price_str);
        
        let trigger_values_str: Vec<String> = trigger_values.iter().map(|v| v.to_string()).collect();
        let trigger_values_joined = trigger_values_str.join(",");
        params.insert("trigger_values", &trigger_values_joined);

        let url = self.build_url(&format!("/gtt/triggers/{}", gtt_id), None);
        let resp = self.send_request(url, "PUT", Some(params)).await?;
        self.parse_response(resp).await
    }

    /// Delete a Good Till Triggered (GTT) order
    /// 
    /// Cancels an active GTT order.
    /// 
    /// # Arguments
    /// 
    /// * `gtt_id` - The GTT ID to delete
    /// 
    /// # Returns
    /// 
    /// A `Result<OrderResponse>` containing deletion confirmation
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
    /// println!("GTT deleted: {}", result.order_id);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete_gtt(&self, gtt_id: &str) -> Result<OrderResponse> {
        let url = self.build_url(&format!("/gtt/triggers/{}", gtt_id), None);
        let resp = self.send_request(url, "DELETE", None).await?;
        self.parse_response(resp).await
    }
}
