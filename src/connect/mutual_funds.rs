//! # Mutual Funds Module
//!
//! This module provides comprehensive mutual fund investment and management functionality
//! for the KiteConnect API. It supports all major mutual fund operations including orders,
//! SIPs (Systematic Investment Plans), holdings management, and fund discovery.
//!
//! ## Overview
//!
//! The mutual funds module enables you to:
//! - **Place Orders**: Buy and sell mutual fund units
//! - **Manage SIPs**: Set up and manage systematic investment plans
//! - **Track Holdings**: Monitor your mutual fund portfolio
//! - **Fund Discovery**: Access comprehensive mutual fund database
//! - **Performance Analysis**: Track returns and analyze fund performance
//!
//! ## Mutual Fund Operations
//!
//! ### 1. Fund Discovery and Information
//!
//! ```rust,no_run
//! use kiteconnect_async_wasm::connect::KiteConnect;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = KiteConnect::new("api_key", "access_token");
//!
//! // Get all available mutual funds
//! let all_funds = client.mf_instruments().await?;
//! println!("Available mutual funds: {}", all_funds.len());
//!
//! // Search for specific fund categories
//! // Filter by AMC, category, risk level, etc.
//! # Ok(())
//! # }
//! ```
//!
//! ### 2. Placing Mutual Fund Orders
//!
//! #### Buy Orders (Investment)
//!
//! ```rust,no_run
//! use kiteconnect_async_wasm::connect::KiteConnect;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = KiteConnect::new("api_key", "access_token");
//!
//! // Method 1: Buy by amount (recommended for most cases)
//! let buy_order = client.place_mf_order(
//!     "INF090I01239",     // Fund ISIN (e.g., SBI Blue Chip Fund)
//!     "BUY",              // Transaction type
//!     None,               // Quantity (not needed for amount-based orders)
//!     Some(5000.0),       // Investment amount in rupees
//!     None                // Tag (optional)
//! ).await?;
//!
//! println!("MF buy order placed: {:?}", buy_order);
//!
//! // Method 2: Buy by units (if you know exact units needed)
//! let unit_buy_order = client.place_mf_order(
//!     "INF090I01239",     // Fund ISIN
//!     "BUY",              // Transaction type
//!     Some(100.0),        // Exact units to buy
//!     None,               // Amount (not needed for unit-based orders)
//!     Some("INVESTMENT")  // Optional tag for tracking
//! ).await?;
//!
//! println!("MF unit buy order placed: {:?}", unit_buy_order);
//! # Ok(())
//! # }
//! ```
//!
//! #### Sell Orders (Redemption)
//!
//! ```rust,no_run
//! use kiteconnect_async_wasm::connect::KiteConnect;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = KiteConnect::new("api_key", "access_token");
//!
//! // Sell specific units
//! let sell_order = client.place_mf_order(
//!     "INF090I01239",     // Fund ISIN
//!     "SELL",             // Transaction type
//!     Some(50.0),         // Units to sell
//!     None,               // Amount (not used for sell orders)
//!     Some("PARTIAL_REDEMPTION") // Optional tag
//! ).await?;
//!
//! println!("MF sell order placed: {:?}", sell_order);
//!
//! // Full redemption (sell all units)
//! let full_redemption = client.place_mf_order(
//!     "INF090I01239",     // Fund ISIN
//!     "SELL",             // Transaction type
//!     Some(0.0),          // 0 units = full redemption
//!     None,               // Amount not used
//!     Some("FULL_REDEMPTION") // Tag for tracking
//! ).await?;
//!
//! println!("Full redemption order placed: {:?}", full_redemption);
//! # Ok(())
//! # }
//! ```
//!
//! ### 3. Order Management and Tracking
//!
//! ```rust,no_run
//! use kiteconnect_async_wasm::connect::KiteConnect;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = KiteConnect::new("api_key", "access_token");
//!
//! // Get all MF orders
//! let all_orders = client.mf_orders(None).await?;
//! println!("All MF orders: {:?}", all_orders);
//!
//! // Get specific order details
//! let order_details = client.mf_orders(Some("MF_ORDER_ID")).await?;
//! println!("Order details: {:?}", order_details);
//!
//! // Cancel a pending order
//! let cancellation = client.cancel_mf_order("MF_ORDER_ID").await?;
//! println!("Order cancelled: {:?}", cancellation);
//! # Ok(())
//! # }
//! ```
//!
//! ### 4. SIP (Systematic Investment Plan) Management
//!
//! #### Setting Up SIPs
//!
//! ```rust,no_run
//! use kiteconnect_async_wasm::connect::KiteConnect;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = KiteConnect::new("api_key", "access_token");
//!
//! // Create a monthly SIP
//! let sip_order = client.place_mf_sip(
//!     "INF090I01239",     // Fund ISIN
//!     5000.0,             // Monthly amount
//!     12,                 // Number of installments
//!     "monthly",          // Frequency
//!     None,               // Initial amount (optional)
//!     None                // Tag (optional)
//! ).await?;
//!
//! println!("SIP created: {:?}", sip_order);
//!
//! // Create a weekly SIP with initial investment
//! let weekly_sip = client.place_mf_sip(
//!     "INF090I01247",     // Different fund ISIN
//!     1000.0,             // Weekly amount
//!     52,                 // 52 weeks = 1 year
//!     "weekly",           // Frequency
//!     Some(10000.0),      // Initial lump sum investment
//!     Some("WEEKLY_SIP")  // Tag for tracking
//! ).await?;
//!
//! println!("Weekly SIP with initial investment created: {:?}", weekly_sip);
//! # Ok(())
//! # }
//! ```
//!
//! #### Managing Existing SIPs
//!
//! ```rust,no_run
//! use kiteconnect_async_wasm::connect::KiteConnect;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = KiteConnect::new("api_key", "access_token");
//!
//! // Get all SIPs
//! let all_sips = client.mf_sips(None).await?;
//! println!("All SIPs: {:?}", all_sips);
//!
//! // Get specific SIP details
//! let sip_details = client.mf_sips(Some("SIP_ID")).await?;
//! println!("SIP details: {:?}", sip_details);
//!
//! // Modify a SIP (change amount or frequency)
//! let modified_sip = client.modify_mf_sip(
//!     "SIP_ID",
//!     Some(7500.0),       // New amount (increased from 5000)
//!     None,               // Status (keep unchanged)
//!     None,               // Frequency (keep unchanged)
//!     None                // Installments (keep unchanged)
//! ).await?;
//!
//! println!("SIP modified: {:?}", modified_sip);
//!
//! // Pause a SIP
//! let paused_sip = client.modify_mf_sip(
//!     "SIP_ID",
//!     None,               // Amount (keep unchanged)
//!     Some("paused"),     // Pause the SIP
//!     None,               // Frequency (keep unchanged)
//!     None                // Installments (keep unchanged)
//! ).await?;
//!
//! println!("SIP paused: {:?}", paused_sip);
//!
//! // Cancel a SIP permanently
//! let cancelled_sip = client.cancel_mf_sip("SIP_ID").await?;
//! println!("SIP cancelled: {:?}", cancelled_sip);
//! # Ok(())
//! # }
//! ```
//!
//! ### 5. Portfolio Holdings and Performance Analysis
//!
//! ```rust,no_run
//! use kiteconnect_async_wasm::connect::KiteConnect;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = KiteConnect::new("api_key", "access_token");
//!
//! // Get all mutual fund holdings
//! let holdings = client.mf_holdings().await?;
//! println!("MF Holdings: {:?}", holdings);
//!
//! // Example: Calculate portfolio metrics
//! if let Ok(holdings_array) = holdings.as_array() {
//!     let mut total_investment = 0.0;
//!     let mut total_current_value = 0.0;
//!     
//!     for holding in holdings_array {
//!         if let (Some(avg_price), Some(quantity), Some(last_price)) = (
//!             holding["average_price"].as_f64(),
//!             holding["quantity"].as_f64(),
//!             holding["last_price"].as_f64()
//!         ) {
//!             let investment = avg_price * quantity;
//!             let current_value = last_price * quantity;
//!             
//!             total_investment += investment;
//!             total_current_value += current_value;
//!             
//!             let pnl = current_value - investment;
//!             let pnl_percent = (pnl / investment) * 100.0;
//!             
//!             println!("Fund: {} | Investment: ₹{:.2} | Current: ₹{:.2} | P&L: ₹{:.2} ({:.2}%)",
//!                 holding["tradingsymbol"].as_str().unwrap_or("Unknown"),
//!                 investment, current_value, pnl, pnl_percent
//!             );
//!         }
//!     }
//!     
//!     let total_pnl = total_current_value - total_investment;
//!     let total_pnl_percent = (total_pnl / total_investment) * 100.0;
//!     
//!     println!("\n=== Portfolio Summary ===");
//!     println!("Total Investment: ₹{:.2}", total_investment);
//!     println!("Current Value: ₹{:.2}", total_current_value);
//!     println!("Total P&L: ₹{:.2} ({:.2}%)", total_pnl, total_pnl_percent);
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## Advanced Usage with Typed APIs
//!
//! For better type safety and IDE support, use the typed API methods:
//!
//! ```rust,no_run
//! use kiteconnect_async_wasm::connect::KiteConnect;
//! use kiteconnect_async_wasm::models::mutual_funds::MFOrderParams;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = KiteConnect::new("api_key", "access_token");
//!
//! // Using typed APIs for better error handling and type safety
//! let mf_orders = client.mf_orders_typed().await?;
//! println!("Total MF orders: {}", mf_orders.len());
//!
//! let mf_holdings = client.mf_holdings_typed().await?;
//! println!("Total MF holdings: {}", mf_holdings.len());
//!
//! let sips = client.mf_sips_typed().await?;
//! println!("Active SIPs: {}", sips.len());
//!
//! // Type-safe order placement
//! let order_params = MFOrderParams {
//!     tradingsymbol: "INF090I01239".to_string(),
//!     transaction_type: "BUY".to_string(),
//!     amount: Some(5000.0),
//!     quantity: None,
//!     tag: Some("MONTHLY_INVESTMENT".to_string()),
//! };
//!
//! let order_response = client.place_mf_order_typed(&order_params).await?;
//! println!("Order placed with ID: {}", order_response.order_id);
//! # Ok(())
//! # }
//! ```
//!
//! ## Investment Strategies and Best Practices
//!
//! ### 1. Diversification Strategy
//!
//! ```rust,no_run
//! # use kiteconnect_async_wasm::connect::KiteConnect;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = KiteConnect::new("api_key", "access_token");
//!
//! // Diversified portfolio approach
//! let funds = vec![
//!     ("INF090I01239", 3000.0, "Large Cap"),      // 30% - Large cap stability
//!     ("INF090I01247", 2000.0, "Mid Cap"),        // 20% - Mid cap growth
//!     ("INF090I01255", 1500.0, "Small Cap"),      // 15% - Small cap high growth
//!     ("INF090I01263", 2000.0, "Debt Fund"),      // 20% - Debt for stability
//!     ("INF090I01271", 1500.0, "International"),  // 15% - International exposure
//! ];
//!
//! for (isin, amount, category) in funds {
//!     match client.place_mf_order(isin, "BUY", None, Some(amount), Some(category)).await {
//!         Ok(order) => println!("✅ {} investment of ₹{} placed", category, amount),
//!         Err(e) => eprintln!("❌ Failed to place {} investment: {}", category, e),
//!     }
//!     
//!     // Add delay to respect rate limits
//!     tokio::time::sleep(std::time::Duration::from_millis(500)).await;
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ### 2. SIP Automation Strategy
//!
//! ```rust,no_run
//! # use kiteconnect_async_wasm::connect::KiteConnect;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = KiteConnect::new("api_key", "access_token");
//!
//! // Automated SIP setup for long-term wealth building
//! let sip_strategy = vec![
//!     ("INF090I01239", 2000.0, "EQUITY_GROWTH"),   // Equity for growth
//!     ("INF090I01247", 1000.0, "DEBT_STABILITY"),  // Debt for stability
//!     ("INF090I01255", 500.0, "INTERNATIONAL"),    // International diversification
//! ];
//!
//! for (isin, monthly_amount, tag) in sip_strategy {
//!     match client.place_mf_sip(
//!         isin,
//!         monthly_amount,
//!         60,              // 5 years (60 months)
//!         "monthly",
//!         None,
//!         Some(tag)
//!     ).await {
//!         Ok(sip) => println!("✅ SIP of ₹{}/month set up for {}", monthly_amount, tag),
//!         Err(e) => eprintln!("❌ Failed to set up SIP: {}", e),
//!     }
//!     
//!     tokio::time::sleep(std::time::Duration::from_millis(500)).await;
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ### 3. Portfolio Rebalancing
//!
//! ```rust,no_run
//! # use kiteconnect_async_wasm::connect::KiteConnect;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = KiteConnect::new("api_key", "access_token");
//!
//! // Periodic portfolio rebalancing
//! async fn rebalance_portfolio(client: &KiteConnect) -> Result<(), Box<dyn std::error::Error>> {
//!     let holdings = client.mf_holdings().await?;
//!     
//!     // Calculate current allocation percentages
//!     // Identify over/under allocated categories
//!     // Place buy/sell orders to rebalance
//!     
//!     println!("Portfolio rebalancing completed");
//!     Ok(())
//! }
//!
//! // Run rebalancing monthly
//! rebalance_portfolio(&client).await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Error Handling Best Practices
//!
//! ```rust,no_run
//! use kiteconnect_async_wasm::connect::KiteConnect;
//! use kiteconnect_async_wasm::models::common::KiteError;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = KiteConnect::new("api_key", "access_token");
//!
//! // Comprehensive error handling for MF operations
//! match client.place_mf_order("INF090I01239", "BUY", None, Some(5000.0), None).await {
//!     Ok(order) => {
//!         println!("✅ Order placed successfully: {:?}", order);
//!     },
//!     Err(e) => {
//!         // Handle specific error types
//!         eprintln!("❌ Order placement failed: {}", e);
//!         
//!         // Common error scenarios:
//!         // - Insufficient balance
//!         // - Invalid ISIN
//!         // - Market closed
//!         // - Minimum investment amount not met
//!         // - Daily investment limit exceeded
//!         // Implement retry logic or alternative actions
//!     }
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## Performance Monitoring and Analytics
//!
//! ```rust,no_run
//! use kiteconnect_async_wasm::connect::KiteConnect;
//! use std::time::Duration;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = KiteConnect::new("api_key", "access_token");
//!
//! // Monthly performance analysis
//! async fn analyze_performance(client: &KiteConnect) -> Result<(), Box<dyn std::error::Error>> {
//!     let holdings = client.mf_holdings().await?;
//!     let orders = client.mf_orders(None).await?;
//!     
//!     // Calculate returns, volatility, and risk metrics
//!     // Generate performance reports
//!     // Send alerts for significant changes
//!     
//!     println!("Performance analysis completed");
//!     Ok(())
//! }
//!
//! // Automated monitoring loop
//! loop {
//!     analyze_performance(&client).await?;
//!     tokio::time::sleep(Duration::from_secs(24 * 60 * 60)).await; // Daily analysis
//! }
//! # }
//! ```
//!
//! ## Rate Limiting and API Compliance
//!
//! Mutual fund operations are subject to specific rate limits:
//! - **Order operations**: 10 requests per second (standard category)
//! - **Data retrieval**: Built-in rate limiting with automatic retry
//! - **Best practices**: Batch operations and implement exponential backoff
//!
//! The module automatically handles rate limiting, but for high-frequency operations,
//! implement additional delay between requests to ensure compliance.
//!
//! ## Platform Compatibility
//!
//! This mutual funds module provides identical functionality across all platforms:
//! - **Native (Desktop/Server)**: Full performance with local caching
//! - **WASM (Browser)**: Complete functionality in web applications  
//! - **Mobile**: Optimized for mobile trading applications
//!
//! All APIs maintain consistent behavior and error handling across platforms.

use crate::connect::endpoints::KiteEndpoint;
use anyhow::Result;
use serde_json::Value as JsonValue;
use std::collections::HashMap;

// Import typed models for dual API support
use crate::models::common::KiteResult;
use crate::models::mutual_funds::{
    MFHolding, MFOrder, MFOrderParams, MFOrderResponse, SIPParams, SIPResponse, SIP,
};

use crate::connect::KiteConnect;

impl KiteConnect {
    // === LEGACY API METHODS (JSON responses) ===

    /// Get all mutual fund orders or individual order info
    pub async fn mf_orders(&self, order_id: Option<&str>) -> Result<JsonValue> {
        let resp = if let Some(order_id) = order_id {
            self.send_request_with_rate_limiting_and_retry(
                KiteEndpoint::MFOrderInfo,
                &[order_id],
                None,
                None,
            )
            .await
            .map_err(|e| anyhow::anyhow!("Failed to get MF order info: {}", e))?
        } else {
            self.send_request_with_rate_limiting_and_retry(KiteEndpoint::MFOrders, &[], None, None)
                .await
                .map_err(|e| anyhow::anyhow!("Failed to get MF orders: {}", e))?
        };

        self.raise_or_return_json(resp).await
    }

    /// Place a mutual fund order
    ///
    /// Places a mutual fund buy or sell order. For buy orders, you can specify either
    /// quantity (units) or amount (monetary value). For sell orders, quantity is required.
    ///
    /// # Arguments
    ///
    /// * `tradingsymbol` - Trading symbol of the mutual fund
    /// * `transaction_type` - "BUY" or "SELL"
    /// * `quantity` - Quantity (units) for the order (optional for buy orders)
    /// * `amount` - Amount in rupees for buy orders (alternative to quantity)
    /// * `tag` - Optional tag to identify orders
    ///
    /// # Returns
    ///
    /// A `Result<JsonValue>` containing order confirmation with order_id
    ///
    /// # Errors
    ///
    /// Returns an error if the order placement fails or parameters are invalid
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
    /// // Buy order with amount
    /// let buy_order = client.place_mf_order(
    ///     "INF846K01DP8",    // MF trading symbol
    ///     "BUY",             // Transaction type
    ///     None,              // No quantity
    ///     Some("1000"),      // Amount in rupees
    ///     Some("my_tag")     // Optional tag
    /// ).await?;
    ///
    /// println!("Order placed: {:?}", buy_order);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn place_mf_order(
        &self,
        tradingsymbol: &str,
        transaction_type: &str,
        quantity: Option<&str>,
        amount: Option<&str>,
        tag: Option<&str>,
    ) -> Result<JsonValue> {
        let mut params = HashMap::new();
        params.insert("tradingsymbol", tradingsymbol);
        params.insert("transaction_type", transaction_type);

        if let Some(quantity) = quantity {
            params.insert("quantity", quantity);
        }
        if let Some(amount) = amount {
            params.insert("amount", amount);
        }
        if let Some(tag) = tag {
            params.insert("tag", tag);
        }

        let resp = self
            .send_request_with_rate_limiting_and_retry(
                KiteEndpoint::PlaceMFOrder,
                &[],
                None,
                Some(params),
            )
            .await
            .map_err(|e| anyhow::anyhow!("Failed to place MF order: {}", e))?;

        self.raise_or_return_json(resp).await
    }

    /// Cancel a mutual fund order
    ///
    /// Cancels a pending mutual fund order. Only orders in OPEN status can be cancelled.
    ///
    /// # Arguments
    ///
    /// * `order_id` - The mutual fund order ID to cancel
    ///
    /// # Returns
    ///
    /// A `Result<JsonValue>` containing cancellation confirmation
    ///
    /// # Errors
    ///
    /// Returns an error if the order cannot be cancelled or doesn't exist
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
    /// let result = client.cancel_mf_order("123456789").await?;
    /// println!("Cancellation result: {:?}", result);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn cancel_mf_order(&self, order_id: &str) -> Result<JsonValue> {
        let resp = self
            .send_request_with_rate_limiting_and_retry(
                KiteEndpoint::CancelMFOrder,
                &[order_id],
                None,
                None,
            )
            .await
            .map_err(|e| anyhow::anyhow!("Failed to cancel MF order: {}", e))?;

        self.raise_or_return_json(resp).await
    }

    /// Get mutual fund SIPs (Systematic Investment Plans)
    ///
    /// Retrieves all active SIPs or details of a specific SIP.
    ///
    /// # Arguments
    ///
    /// * `sip_id` - Optional SIP ID. If None, returns all SIPs
    ///
    /// # Returns
    ///
    /// A `Result<JsonValue>` containing SIP information
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails
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
    /// // Get all SIPs
    /// let all_sips = client.mf_sips(None).await?;
    ///
    /// // Get specific SIP
    /// let sip_details = client.mf_sips(Some("123456")).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn mf_sips(&self, sip_id: Option<&str>) -> Result<JsonValue> {
        let resp = if let Some(sip_id) = sip_id {
            self.send_request_with_rate_limiting_and_retry(
                KiteEndpoint::SIPInfo,
                &[sip_id],
                None,
                None,
            )
            .await
            .map_err(|e| anyhow::anyhow!("Failed to get SIP info: {}", e))?
        } else {
            self.send_request_with_rate_limiting_and_retry(KiteEndpoint::SIPs, &[], None, None)
                .await
                .map_err(|e| anyhow::anyhow!("Failed to get SIPs: {}", e))?
        };

        self.raise_or_return_json(resp).await
    }

    /// Place a mutual fund SIP (Systematic Investment Plan)
    ///
    /// Creates a new SIP for systematic investment in mutual funds.
    ///
    /// # Arguments
    ///
    /// * `tradingsymbol` - Trading symbol of the mutual fund
    /// * `amount` - SIP amount per installment
    /// * `instalments` - Total number of installments (max 99 for lifetime)
    /// * `frequency` - SIP frequency ("weekly", "monthly", "quarterly")
    /// * `initial_amount` - Optional initial lump sum amount
    /// * `instalment_day` - Day of month for monthly SIPs (1-28) or day of week for weekly
    /// * `tag` - Optional tag to identify the SIP
    ///
    /// # Returns
    ///
    /// A `Result<JsonValue>` containing SIP creation confirmation
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
    /// let sip = client.place_mf_sip(
    ///     "INF846K01DP8",    // MF trading symbol
    ///     "1000",            // Amount per installment
    ///     "12",              // 12 installments
    ///     "monthly",         // Monthly frequency
    ///     Some("5000"),      // Initial amount
    ///     Some("15"),        // 15th of every month
    ///     Some("retirement_sip") // Tag
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[allow(clippy::too_many_arguments)]
    pub async fn place_mf_sip(
        &self,
        tradingsymbol: &str,
        amount: &str,
        instalments: &str,
        frequency: &str,
        initial_amount: Option<&str>,
        instalment_day: Option<&str>,
        tag: Option<&str>,
    ) -> Result<JsonValue> {
        let mut params = HashMap::new();
        params.insert("tradingsymbol", tradingsymbol);
        params.insert("amount", amount);
        params.insert("instalments", instalments);
        params.insert("frequency", frequency);

        if let Some(initial_amount) = initial_amount {
            params.insert("initial_amount", initial_amount);
        }
        if let Some(instalment_day) = instalment_day {
            params.insert("instalment_day", instalment_day);
        }
        if let Some(tag) = tag {
            params.insert("tag", tag);
        }

        let resp = self
            .send_request_with_rate_limiting_and_retry(
                KiteEndpoint::PlaceSIP,
                &[],
                None,
                Some(params),
            )
            .await
            .map_err(|e| anyhow::anyhow!("Failed to place MF SIP: {}", e))?;

        self.raise_or_return_json(resp).await
    }

    /// Modify a mutual fund SIP
    ///
    /// Modifies an existing SIP's parameters like amount, frequency, or status.
    ///
    /// # Arguments
    ///
    /// * `sip_id` - The SIP ID to modify
    /// * `amount` - New SIP amount per installment
    /// * `status` - SIP status ("ACTIVE" or "PAUSED")
    /// * `instalments` - New total number of installments
    /// * `frequency` - New SIP frequency
    /// * `instalment_day` - New day for installments
    ///
    /// # Returns
    ///
    /// A `Result<JsonValue>` containing modification confirmation
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
    /// // Increase SIP amount and change frequency
    /// let result = client.modify_mf_sip(
    ///     "123456",          // SIP ID
    ///     "1500",            // New amount
    ///     "ACTIVE",          // Status
    ///     "24",              // New installment count
    ///     "monthly",         // Frequency
    ///     Some("20")         // New instalment day
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn modify_mf_sip(
        &self,
        sip_id: &str,
        amount: &str,
        status: &str,
        instalments: &str,
        frequency: &str,
        instalment_day: Option<&str>,
    ) -> Result<JsonValue> {
        let mut params = HashMap::new();
        params.insert("amount", amount);
        params.insert("status", status);
        params.insert("instalments", instalments);
        params.insert("frequency", frequency);

        if let Some(instalment_day) = instalment_day {
            params.insert("instalment_day", instalment_day);
        }

        let resp = self
            .send_request_with_rate_limiting_and_retry(
                KiteEndpoint::ModifySIP,
                &[sip_id],
                None,
                Some(params),
            )
            .await
            .map_err(|e| anyhow::anyhow!("Failed to modify MF SIP: {}", e))?;

        self.raise_or_return_json(resp).await
    }

    /// Cancel a mutual fund SIP
    ///
    /// Cancels an active SIP. This will stop all future installments.
    ///
    /// # Arguments
    ///
    /// * `sip_id` - The SIP ID to cancel
    ///
    /// # Returns
    ///
    /// A `Result<JsonValue>` containing cancellation confirmation
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
    /// let result = client.cancel_mf_sip("123456").await?;
    /// println!("SIP cancelled: {:?}", result);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn cancel_mf_sip(&self, sip_id: &str) -> Result<JsonValue> {
        let resp = self
            .send_request_with_rate_limiting_and_retry(
                KiteEndpoint::CancelSIP,
                &[sip_id],
                None,
                None,
            )
            .await
            .map_err(|e| anyhow::anyhow!("Failed to cancel MF SIP: {}", e))?;

        self.raise_or_return_json(resp).await
    }

    /// Get mutual fund holdings
    ///
    /// Retrieves the user's mutual fund holdings with current values and returns.
    ///
    /// # Returns
    ///
    /// A `Result<JsonValue>` containing mutual fund holdings data
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails
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
    /// let holdings = client.mf_holdings().await?;
    /// println!("MF Holdings: {:?}", holdings);
    ///
    /// // Access specific fields
    /// if let Some(data) = holdings["data"].as_array() {
    ///     for holding in data {
    ///         println!("Fund: {}, Units: {}, Current Value: {}",
    ///             holding["tradingsymbol"],
    ///             holding["quantity"],
    ///             holding["last_price"]);
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn mf_holdings(&self) -> Result<JsonValue> {
        let resp = self
            .send_request_with_rate_limiting_and_retry(KiteEndpoint::MFHoldings, &[], None, None)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to get MF holdings: {}", e))?;

        self.raise_or_return_json(resp).await
    }

    // === TYPED API METHODS (v1.0.0) ===

    /// Get mutual fund orders with typed response
    ///
    /// Returns strongly typed MF order data instead of JsonValue.
    ///
    /// # Arguments
    ///
    /// * `order_id` - Optional order ID. If None, returns all orders
    ///
    /// # Returns
    ///
    /// A `KiteResult<Vec<MFOrder>>` for all orders or `KiteResult<MFOrder>` for specific order
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
    /// // Get all MF orders
    /// let all_orders = client.mf_orders_typed(None).await?;
    /// for order in all_orders {
    ///     println!("Order ID: {}, Status: {:?}", order.order_id, order.status);
    /// }
    ///
    /// // Get specific order
    /// let specific_order = client.mf_order_typed("123456").await?;
    /// println!("Order: {:?}", specific_order);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn mf_orders_typed(&self, order_id: Option<&str>) -> KiteResult<Vec<MFOrder>> {
        let resp = if let Some(order_id) = order_id {
            self.send_request_with_rate_limiting_and_retry(
                KiteEndpoint::MFOrderInfo,
                &[order_id],
                None,
                None,
            )
            .await?
        } else {
            self.send_request_with_rate_limiting_and_retry(KiteEndpoint::MFOrders, &[], None, None)
                .await?
        };

        let json_response = self.raise_or_return_json_typed(resp).await?;

        // Extract the data field from response
        let data = json_response["data"].clone();
        self.parse_response(data)
    }

    /// Get single mutual fund order with typed response
    ///
    /// Returns a single strongly typed MF order.
    ///
    /// # Arguments
    ///
    /// * `order_id` - The order ID to fetch
    ///
    /// # Returns
    ///
    /// A `KiteResult<MFOrder>` containing the typed order data
    pub async fn mf_order_typed(&self, order_id: &str) -> KiteResult<MFOrder> {
        let resp = self
            .send_request_with_rate_limiting_and_retry(
                KiteEndpoint::MFOrderInfo,
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

    /// Place a mutual fund order with typed response
    ///
    /// Places a mutual fund order and returns typed response.
    ///
    /// # Arguments
    ///
    /// * `order_params` - Typed order parameters
    ///
    /// # Returns
    ///
    /// A `KiteResult<MFOrderResponse>` containing order confirmation
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use kiteconnect_async_wasm::connect::KiteConnect;
    /// use kiteconnect_async_wasm::models::mutual_funds::MFOrderParams;
    /// use kiteconnect_async_wasm::models::common::TransactionType;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = KiteConnect::new("api_key", "access_token");
    ///
    /// let order_params = MFOrderParams {
    ///     trading_symbol: "INF846K01DP8".to_string(),
    ///     transaction_type: TransactionType::BUY,
    ///     amount: Some(1000.0),
    ///     quantity: None,
    ///     tag: Some("my_tag".to_string()),
    /// };
    ///
    /// let response = client.place_mf_order_typed(&order_params).await?;
    /// println!("Order placed with ID: {}", response.order_id);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn place_mf_order_typed(
        &self,
        order_params: &MFOrderParams,
    ) -> KiteResult<MFOrderResponse> {
        // Create all string conversions upfront to avoid lifetime issues
        let transaction_type_str = order_params.transaction_type.to_string();
        let amount_str = order_params.amount.map(|a| a.to_string());
        let quantity_str = order_params.quantity.map(|q| q.to_string());

        let mut params = HashMap::new();
        params.insert("tradingsymbol", order_params.trading_symbol.as_str());
        params.insert("transaction_type", transaction_type_str.as_str());

        if let Some(ref amount) = amount_str {
            params.insert("amount", amount.as_str());
        }
        if let Some(ref quantity) = quantity_str {
            params.insert("quantity", quantity.as_str());
        }
        if let Some(ref tag) = order_params.tag {
            params.insert("tag", tag.as_str());
        }

        let resp = self
            .send_request_with_rate_limiting_and_retry(
                KiteEndpoint::PlaceMFOrder,
                &[],
                None,
                Some(params),
            )
            .await?;

        let json_response = self.raise_or_return_json_typed(resp).await?;

        // Extract the data field from response
        let data = json_response["data"].clone();
        self.parse_response(data)
    }

    /// Get mutual fund SIPs with typed response
    ///
    /// Returns strongly typed SIP data instead of JsonValue.
    ///
    /// # Arguments
    ///
    /// * `sip_id` - Optional SIP ID. If None, returns all SIPs
    ///
    /// # Returns
    ///
    /// A `KiteResult<Vec<SIP>>` for all SIPs or `KiteResult<SIP>` for specific SIP
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
    /// // Get all SIPs
    /// let all_sips = client.mf_sips_typed(None).await?;
    /// for sip in all_sips {
    ///     println!("SIP ID: {}, Status: {:?}", sip.sip_id, sip.status);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn mf_sips_typed(&self, sip_id: Option<&str>) -> KiteResult<Vec<SIP>> {
        let resp = if let Some(sip_id) = sip_id {
            self.send_request_with_rate_limiting_and_retry(
                KiteEndpoint::SIPInfo,
                &[sip_id],
                None,
                None,
            )
            .await?
        } else {
            self.send_request_with_rate_limiting_and_retry(KiteEndpoint::SIPs, &[], None, None)
                .await?
        };

        let json_response = self.raise_or_return_json_typed(resp).await?;

        // Extract the data field from response
        let data = json_response["data"].clone();
        self.parse_response(data)
    }

    /// Get single mutual fund SIP with typed response
    ///
    /// Returns a single strongly typed SIP.
    ///
    /// # Arguments
    ///
    /// * `sip_id` - The SIP ID to fetch
    ///
    /// # Returns
    ///
    /// A `KiteResult<SIP>` containing the typed SIP data
    pub async fn mf_sip_typed(&self, sip_id: &str) -> KiteResult<SIP> {
        let resp = self
            .send_request_with_rate_limiting_and_retry(KiteEndpoint::SIPInfo, &[sip_id], None, None)
            .await?;

        let json_response = self.raise_or_return_json_typed(resp).await?;

        // Extract the data field from response
        let data = json_response["data"].clone();
        self.parse_response(data)
    }

    /// Place a mutual fund SIP with typed response
    ///
    /// Creates a new SIP and returns typed response.
    ///
    /// # Arguments
    ///
    /// * `sip_params` - Typed SIP parameters
    ///
    /// # Returns
    ///
    /// A `KiteResult<SIPResponse>` containing SIP creation confirmation
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use kiteconnect_async_wasm::connect::KiteConnect;
    /// use kiteconnect_async_wasm::models::mutual_funds::{SIPParams, SIPFrequency};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = KiteConnect::new("api_key", "access_token");
    ///
    /// let sip_params = SIPParams {
    ///     trading_symbol: "INF846K01DP8".to_string(),
    ///     amount: 1000.0,
    ///     instalments: Some(12),
    ///     frequency: SIPFrequency::Monthly,
    ///     initial_amount: Some(5000.0),
    ///     tag: Some("retirement_sip".to_string()),
    /// };
    ///
    /// let response = client.place_mf_sip_typed(&sip_params).await?;
    /// println!("SIP created with ID: {}", response.sip_id);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn place_mf_sip_typed(&self, sip_params: &SIPParams) -> KiteResult<SIPResponse> {
        // Create all string conversions upfront to avoid lifetime issues
        let amount_str = sip_params.amount.to_string();
        let instalments_str = sip_params.instalments.map(|i| i.to_string());
        let frequency_str = sip_params.frequency.to_string(); // Convert enum to string using Display trait
        let initial_amount_str = sip_params.initial_amount.map(|a| a.to_string());

        let mut params = HashMap::new();
        params.insert("tradingsymbol", sip_params.trading_symbol.as_str());
        params.insert("amount", amount_str.as_str());
        params.insert("frequency", frequency_str.as_str());

        if let Some(ref instalments) = instalments_str {
            params.insert("instalments", instalments.as_str());
        }
        if let Some(ref initial_amount) = initial_amount_str {
            params.insert("initial_amount", initial_amount.as_str());
        }
        if let Some(ref tag) = sip_params.tag {
            params.insert("tag", tag.as_str());
        }

        let resp = self
            .send_request_with_rate_limiting_and_retry(
                KiteEndpoint::PlaceSIP,
                &[],
                None,
                Some(params),
            )
            .await?;

        let json_response = self.raise_or_return_json_typed(resp).await?;

        // Extract the data field from response
        let data = json_response["data"].clone();
        self.parse_response(data)
    }

    /// Get mutual fund holdings with typed response
    ///
    /// Returns strongly typed MF holdings data instead of JsonValue.
    ///
    /// # Returns
    ///
    /// A `KiteResult<Vec<MFHolding>>` containing typed holdings data
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
    /// let holdings = client.mf_holdings_typed().await?;
    /// for holding in holdings {
    ///     println!("Fund: {}, Units: {}, Current Value: {}",
    ///         holding.trading_symbol, holding.quantity, holding.last_price);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn mf_holdings_typed(&self) -> KiteResult<Vec<MFHolding>> {
        let resp = self
            .send_request_with_rate_limiting_and_retry(KiteEndpoint::MFHoldings, &[], None, None)
            .await?;

        let json_response = self.raise_or_return_json_typed(resp).await?;

        // Extract the data field from response
        let data = json_response["data"].clone();
        self.parse_response(data)
    }
}
