//! # GTT (Good Till Triggered) Module
//!
//! This module provides comprehensive GTT (Good Till Triggered) order management functionality
//! for the KiteConnect API. GTT orders are advanced conditional orders that remain active until
//! triggered by specific market conditions or manually cancelled.
//!
//! ## Overview
//!
//! GTT orders allow you to:
//! - Set stop-loss orders that trigger when price moves against your position
//! - Set target orders that trigger when price reaches your profit target
//! - Create bracket orders (OCO - One Cancels Other) with both stop-loss and target
//! - Monitor and manage conditional orders without constant market watching
//!
//! ## GTT Types
//!
//! ### Single Trigger GTT
//! A single trigger GTT executes one order when a specific price level is reached.
//!
//! ### Two-Leg GTT (OCO - One Cancels Other)
//! A two-leg GTT contains two orders where execution of one automatically cancels the other.
//! This is commonly used for bracket orders with both stop-loss and target prices.
//!
//! ## Basic Usage
//!
//! ### Creating a Simple Stop-Loss GTT
//!
//! ```rust,no_run
//! use kiteconnect_async_wasm::connect::KiteConnect;
//! use serde_json::json;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = KiteConnect::new("api_key", "access_token");
//!
//! // Place a stop-loss GTT - sell 10 shares of RELIANCE when price drops to ₹2000
//! let stop_loss_gtt = client.place_gtt(
//!     "single",           // Single trigger type
//!     "RELIANCE",         // Trading symbol
//!     "NSE",              // Exchange
//!     &[2000.0],          // Trigger price
//!     2100.0,             // Current market price
//!     &[json!({
//!         "transaction_type": "SELL",
//!         "quantity": 10,
//!         "order_type": "MARKET",
//!         "product": "CNC"
//!     })]
//! ).await?;
//!
//! println!("Stop-loss GTT placed: {:?}", stop_loss_gtt);
//! # Ok(())
//! # }
//! ```
//!
//! ### Creating a Target GTT
//!
//! ```rust,no_run
//! use kiteconnect_async_wasm::connect::KiteConnect;
//! use serde_json::json;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = KiteConnect::new("api_key", "access_token");
//!
//! // Place a target GTT - sell 10 shares of RELIANCE when price reaches ₹2200
//! let target_gtt = client.place_gtt(
//!     "single",           // Single trigger type
//!     "RELIANCE",         // Trading symbol
//!     "NSE",              // Exchange
//!     &[2200.0],          // Target price
//!     2100.0,             // Current market price
//!     &[json!({
//!         "transaction_type": "SELL",
//!         "quantity": 10,
//!         "order_type": "LIMIT",
//!         "product": "CNC",
//!         "price": 2200.0
//!     })]
//! ).await?;
//!
//! println!("Target GTT placed: {:?}", target_gtt);
//! # Ok(())
//! # }
//! ```
//!
//! ### Creating a Bracket GTT (OCO - One Cancels Other)
//!
//! ```rust,no_run
//! use kiteconnect_async_wasm::connect::KiteConnect;
//! use serde_json::json;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = KiteConnect::new("api_key", "access_token");
//!
//! // Place a bracket GTT with both stop-loss and target
//! let bracket_gtt = client.place_gtt(
//!     "two-leg",          // Two-leg trigger type (OCO)
//!     "RELIANCE",         // Trading symbol
//!     "NSE",              // Exchange
//!     &[2000.0, 2200.0],  // Stop-loss and target prices
//!     2100.0,             // Current market price
//!     &[
//!         // Stop-loss order (market order)
//!         json!({
//!             "transaction_type": "SELL",
//!             "quantity": 10,
//!             "order_type": "MARKET",
//!             "product": "CNC"
//!         }),
//!         // Target order (limit order)
//!         json!({
//!             "transaction_type": "SELL",
//!             "quantity": 10,
//!             "order_type": "LIMIT",
//!             "product": "CNC",
//!             "price": 2200.0
//!         })
//!     ]
//! ).await?;
//!
//! println!("Bracket GTT placed: {:?}", bracket_gtt);
//! # Ok(())
//! # }
//! ```
//!
//! ## Advanced Usage with Builder Patterns
//!
//! For more complex GTT orders, you can use the builder patterns provided in the models:
//!
//! ```rust,no_run
//! use kiteconnect_async_wasm::models::gtt::{StopLossGTTBuilder, BracketGTTBuilder};
//! use kiteconnect_async_wasm::models::common::{Exchange, TransactionType, Product};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Create a stop-loss GTT using the builder pattern
//! let stop_loss_gtt = StopLossGTTBuilder::new()
//!     .exchange(Exchange::NSE)
//!     .trading_symbol("RELIANCE")
//!     .transaction_type(TransactionType::SELL)
//!     .product(Product::CNC)
//!     .quantity(10)
//!     .trigger_price(2000.0)
//!     .current_price(2100.0)
//!     .build_market()?;  // Creates market order on trigger
//!
//! // Create a bracket GTT using the builder pattern
//! let bracket_gtt = BracketGTTBuilder::new()
//!     .exchange(Exchange::NSE)
//!     .trading_symbol("RELIANCE")
//!     .transaction_type(TransactionType::SELL)
//!     .product(Product::CNC)
//!     .quantity(10)
//!     .stop_loss_price(2000.0)
//!     .target_price(2200.0)
//!     .current_price(2100.0)
//!     .build()?;
//!
//! println!("GTT parameters ready for placement");
//! # Ok(())
//! # }
//! ```
//!
//! ## GTT Management Operations
//!
//! ### Retrieving GTT Orders
//!
//! ```rust,no_run
//! use kiteconnect_async_wasm::connect::KiteConnect;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = KiteConnect::new("api_key", "access_token");
//!
//! // Get all GTT orders
//! let all_gtts = client.get_gtts(None).await?;
//! println!("All GTT orders: {:?}", all_gtts);
//!
//! // Get specific GTT order details
//! let gtt_details = client.get_gtts(Some("123456")).await?;
//! println!("GTT 123456 details: {:?}", gtt_details);
//! # Ok(())
//! # }
//! ```
//!
//! ### Modifying GTT Orders
//!
//! ```rust,no_run
//! use kiteconnect_async_wasm::connect::KiteConnect;
//! use serde_json::json;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = KiteConnect::new("api_key", "access_token");
//!
//! // Modify an existing GTT order
//! let modified_gtt = client.modify_gtt(
//!     "123456",           // GTT ID to modify
//!     "single",           // Trigger type
//!     "RELIANCE",         // Trading symbol
//!     "NSE",              // Exchange
//!     &[1950.0],          // New trigger price (lowered from 2000)
//!     2100.0,             // Current market price
//!     &[json!({
//!         "transaction_type": "SELL",
//!         "quantity": 15,  // Increased quantity
//!         "order_type": "MARKET",
//!         "product": "CNC"
//!     })]
//! ).await?;
//!
//! println!("GTT modified: {:?}", modified_gtt);
//! # Ok(())
//! # }
//! ```
//!
//! ### Cancelling GTT Orders
//!
//! ```rust,no_run
//! use kiteconnect_async_wasm::connect::KiteConnect;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = KiteConnect::new("api_key", "access_token");
//!
//! // Cancel a GTT order
//! let cancellation_result = client.delete_gtt("123456").await?;
//! println!("GTT cancelled: {:?}", cancellation_result);
//! # Ok(())
//! # }
//! ```
//!
//! ## Best Practices
//!
//! ### 1. Risk Management
//!
//! - **Always set stop-losses**: Use GTT orders to automatically limit losses
//! - **Position sizing**: Never risk more than 1-2% of your capital per trade
//! - **Multiple GTTs**: Use bracket orders to capture profits and limit losses simultaneously
//!
//! ### 2. Price Setting Guidelines
//!
//! - **Stop-loss placement**: Set 3-5% below entry price for swing trades
//! - **Target placement**: Use risk-reward ratio of at least 1:2 (target = 2x stop-loss distance)
//! - **Market conditions**: Adjust trigger levels based on volatility and support/resistance levels
//!
//! ### 3. Order Types
//!
//! - **Market orders**: Use for stop-losses when quick execution is priority
//! - **Limit orders**: Use for targets to ensure specific price execution
//! - **Slippage consideration**: Account for potential slippage in volatile markets
//!
//! ### 4. Monitoring and Maintenance
//!
//! ```rust,no_run
//! use kiteconnect_async_wasm::connect::KiteConnect;
//! use std::time::Duration;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = KiteConnect::new("api_key", "access_token");
//!
//! // Regular GTT monitoring loop
//! loop {
//!     let gtts = client.get_gtts(None).await?;
//!     
//!     // Process GTT status and take action if needed
//!     // - Check for triggered GTTs
//!     // - Update stop-losses based on favorable price movement
//!     // - Cancel outdated GTTs
//!     
//!     println!("GTT status check completed");
//!     
//!     // Wait before next check (respect rate limits)
//!     tokio::time::sleep(Duration::from_secs(300)).await; // Check every 5 minutes
//! }
//! # }
//! ```
//!
//! ## Error Handling
//!
//! GTT operations can fail for various reasons. Always implement proper error handling:
//!
//! ```rust,no_run
//! use kiteconnect_async_wasm::connect::KiteConnect;
//! use serde_json::json;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = KiteConnect::new("api_key", "access_token");
//!
//! match client.place_gtt(
//!     "single",
//!     "RELIANCE",
//!     "NSE",
//!     &[2000.0],
//!     2100.0,
//!     &[json!({
//!         "transaction_type": "SELL",
//!         "quantity": 10,
//!         "order_type": "MARKET",
//!         "product": "CNC"
//!     })]
//! ).await {
//!     Ok(response) => {
//!         println!("✅ GTT placed successfully: {:?}", response);
//!     },
//!     Err(e) => {
//!         eprintln!("❌ GTT placement failed: {}", e);
//!         // Handle specific error cases:
//!         // - Invalid price levels
//!         // - Insufficient margin
//!         // - Invalid instrument
//!         // - Rate limiting
//!     }
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## Platform Compatibility
//!
//! This GTT module works seamlessly across platforms:
//! - **Native (Desktop/Server)**: Full functionality with optimal performance
//! - **WASM (Browser)**: Complete GTT management in web applications
//! - **Cross-platform**: Identical API surface and behavior
//!
//! ## Rate Limiting
//!
//! GTT operations are subject to API rate limits:
//! - **Standard category**: 10 requests per second
//! - **Automatic handling**: Built-in rate limiting with retry logic
//! - **Best practice**: Batch multiple operations when possible
//!
//! For high-frequency GTT management, consider implementing local caching and
//! batching strategies to minimize API calls.

use crate::connect::endpoints::KiteEndpoint;
use crate::connect::KiteConnect;
use anyhow::Result;
use serde_json::Value as JsonValue;
use std::collections::HashMap;

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
            self.send_request_with_rate_limiting_and_retry(KiteEndpoint::GTTInfo, &[id], None, None)
                .await
                .map_err(|e| anyhow::anyhow!("Failed to get GTT info: {}", e))?
        } else {
            self.send_request_with_rate_limiting_and_retry(KiteEndpoint::GTTs, &[], None, None)
                .await
                .map_err(|e| anyhow::anyhow!("Failed to get GTTs: {}", e))?
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
        let trigger_values_str = trigger_values
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(",");
        params.insert("trigger_values", &trigger_values_str);

        let last_price_str = last_price.to_string();
        params.insert("last_price", &last_price_str);

        // Convert orders to JSON string
        let orders_json = serde_json::to_string(orders)?;
        params.insert("orders", &orders_json);

        let resp = self
            .send_request_with_rate_limiting_and_retry(
                KiteEndpoint::PlaceGTT,
                &[],
                None,
                Some(params),
            )
            .await
            .map_err(|e| anyhow::anyhow!("Failed to place GTT: {}", e))?;

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
    #[allow(clippy::too_many_arguments)]
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
        let trigger_values_str = trigger_values
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(",");
        params.insert("trigger_values", &trigger_values_str);

        let last_price_str = last_price.to_string();
        params.insert("last_price", &last_price_str);

        // Convert orders to JSON string
        let orders_json = serde_json::to_string(orders)?;
        params.insert("orders", &orders_json);

        let resp = self
            .send_request_with_rate_limiting_and_retry(
                KiteEndpoint::ModifyGTT,
                &[gtt_id],
                None,
                Some(params),
            )
            .await
            .map_err(|e| anyhow::anyhow!("Failed to modify GTT: {}", e))?;

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
        let resp = self
            .send_request_with_rate_limiting_and_retry(
                KiteEndpoint::CancelGTT,
                &[gtt_id],
                None,
                None,
            )
            .await
            .map_err(|e| anyhow::anyhow!("Failed to delete GTT: {}", e))?;

        self.raise_or_return_json(resp).await
    }
}
