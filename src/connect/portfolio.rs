//! # Portfolio Module
//!
//! This module provides comprehensive portfolio management capabilities for the KiteConnect API v1.0.3,
//! offering both real-time portfolio tracking and detailed analysis of holdings and positions.
//!
//! ## Overview
//!
//! The portfolio module is the central component for managing your trading and investment portfolio.
//! It provides access to holdings (long-term investments), positions (trading activities), margins,
//! and portfolio analytics with both legacy JSON-based and modern strongly-typed APIs.
//!
//! ## Key Features
//!
//! ### 🔄 **Dual API Support**
//! - **Legacy API**: Returns `JsonValue` for backward compatibility
//! - **Typed API**: Returns structured types with compile-time safety (methods ending in `_typed`)
//!
//! ### 📊 **Comprehensive Portfolio Data**
//! - **Holdings**: Long-term investments with P&L tracking
//! - **Positions**: Intraday and overnight trading positions
//! - **Margins**: Available funds and utilization across segments
//! - **Analytics**: Portfolio summaries and performance metrics
//!
//! ### 💡 **Advanced Features**
//! - **Real-time P&L**: Live profit/loss calculations
//! - **Position Analysis**: Day vs overnight position tracking
//! - **Risk Management**: Margin monitoring and limit checking
//! - **Portfolio Conversion**: Convert positions between product types
//!
//! ## Available Methods
//!
//! ### Holdings Management
//! - [`holdings()`](KiteConnect::holdings) / [`holdings_typed()`](KiteConnect::holdings_typed) - Get all stock holdings
//! - Portfolio analysis and P&L tracking
//! - T+1 quantity and sellable quantity calculations
//!
//! ### Positions Tracking
//! - [`positions()`](KiteConnect::positions) / [`positions_typed()`](KiteConnect::positions_typed) - Get current positions
//! - Day and net position separation
//! - Real-time P&L and M2M calculations
//!
//! ### Margin Management
//! - [`margins()`](KiteConnect::margins) / [`margins_typed()`](KiteConnect::margins_typed) - Get available margins
//! - Segment-wise margin tracking
//! - Utilization and available funds monitoring
//!
//! ## Usage Examples
//!
//! ### Holdings Analysis
//! ```rust,no_run
//! use kiteconnect_async_wasm::connect::KiteConnect;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = KiteConnect::new("api_key", "access_token");
//!
//! // Get all holdings (typed API - recommended)
//! let holdings = client.holdings_typed().await?;
//!
//! let mut total_investment = 0.0;
//! let mut total_value = 0.0;
//! let mut total_pnl = 0.0;
//!
//! println!("Holdings Portfolio Analysis:");
//! println!("============================");
//!
//! for holding in &holdings {
//!     let investment = holding.investment_value();
//!     let current_value = holding.market_value();
//!     let pnl_pct = holding.pnl_percentage();
//!     
//!     total_investment += investment;
//!     total_value += current_value;
//!     total_pnl += holding.pnl;
//!     
//!     println!("📈 {}: {} shares", holding.trading_symbol, holding.quantity);
//!     println!("   💰 Investment: ₹{:.2} | Current: ₹{:.2}", investment, current_value);
//!     println!("   📊 P&L: ₹{:.2} ({:.2}%)", holding.pnl, pnl_pct);
//!     
//!     // Check trading availability
//!     if holding.can_sell_today() {
//!         println!("   ✅ Can sell {} shares today", holding.sellable_today());
//!     }
//!     
//!     println!();
//! }
//!
//! let overall_pnl_pct = (total_pnl / total_investment) * 100.0;
//! println!("🎯 Portfolio Summary:");
//! println!("   Total Investment: ₹{:.2}", total_investment);
//! println!("   Current Value: ₹{:.2}", total_value);
//! println!("   Total P&L: ₹{:.2} ({:.2}%)", total_pnl, overall_pnl_pct);
//! # Ok(())
//! # }
//! ```
//!
//! ### Positions Monitoring
//! ```rust,no_run
//! use kiteconnect_async_wasm::connect::KiteConnect;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = KiteConnect::new("api_key", "access_token");
//!
//! // Get all positions (typed API)
//! let positions = client.positions_typed().await?;
//!
//! println!("Active Trading Positions:");
//! println!("========================");
//!
//! let mut day_pnl = 0.0;
//! let mut total_pnl = 0.0;
//!
//! for position in &positions {
//!     if !position.is_flat() {
//!         let direction = if position.is_long() { "LONG" } else { "SHORT" };
//!         let pnl_pct = position.pnl_percentage();
//!         
//!         day_pnl += position.day_pnl();
//!         total_pnl += position.pnl;
//!         
//!         println!("📊 {}: {} {} shares",
//!             position.trading_symbol,
//!             position.abs_quantity(),
//!             direction);
//!         println!("   💵 Avg: ₹{:.2} | LTP: ₹{:.2}",
//!             position.average_price, position.last_price);
//!         println!("   📈 P&L: ₹{:.2} ({:.2}%)", position.pnl, pnl_pct);
//!         
//!         if position.is_day_position() {
//!             println!("   🔄 Intraday position");
//!         } else if position.is_overnight_position() {
//!             println!("   🌙 Overnight position ({})", position.overnight_quantity);
//!         }
//!         
//!         println!();
//!     }
//! }
//!
//! println!("📊 Trading Summary:");
//! println!("   Day P&L: ₹{:.2}", day_pnl);
//! println!("   Total P&L: ₹{:.2}", total_pnl);
//! # Ok(())
//! # }
//! ```
//!
//! ### Margin Analysis
//! ```rust,no_run
//! use kiteconnect_async_wasm::connect::KiteConnect;
//! use kiteconnect_async_wasm::models::auth::TradingSegment;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = KiteConnect::new("api_key", "access_token");
//!
//! // Get margin data (typed API)
//! let margins = client.margins_typed(None).await?;
//!
//! println!("Margin Analysis:");
//! println!("===============");
//!
//! if let Some(equity_margin) = margins.equity {
//!     let available = equity_margin.available.cash;
//!     let net = equity_margin.net;
//!     let utilisation_pct = equity_margin.utilisation_percentage();
//!     
//!     println!("💰 Equity Segment:");
//!     println!("   Available Cash: ₹{:.2}", available);
//!     println!("   Net Margin: ₹{:.2}", net);
//!     println!("   Utilisation: {:.1}%", utilisation_pct);
//!     
//!     // Check if sufficient margin for trading
//!     let required_margin = 50000.0; // Example
//!     if equity_margin.can_place_order(required_margin) {
//!         println!("   ✅ Sufficient margin for ₹{:.0} order", required_margin);
//!     } else {
//!         println!("   ❌ Insufficient margin for ₹{:.0} order", required_margin);
//!     }
//! }
//!
//! if let Some(commodity_margin) = margins.commodity {
//!     println!("🌾 Commodity Segment:");
//!     println!("   Available Cash: ₹{:.2}", commodity_margin.available.cash);
//!     println!("   Net Margin: ₹{:.2}", commodity_margin.net);
//! }
//!
//! // Overall margin check
//! let total_cash = margins.total_cash();
//! let total_net = margins.total_net_margin();
//! println!("🎯 Total Available: ₹{:.2} | Net: ₹{:.2}", total_cash, total_net);
//! # Ok(())
//! # }
//! ```
//!
//! ### Portfolio Risk Analysis
//! ```rust,no_run
//! use kiteconnect_async_wasm::connect::KiteConnect;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = KiteConnect::new("api_key", "access_token");
//!
//! // Get holdings and positions for comprehensive analysis
//! let (holdings, positions) = tokio::try_join!(
//!     client.holdings_typed(),
//!     client.positions_typed()
//! )?;
//!
//! println!("Portfolio Risk Analysis:");
//! println!("=======================");
//!
//! // Holdings analysis
//! let profitable_holdings = holdings.iter().filter(|h| h.is_profitable()).count();
//! let loss_holdings = holdings.iter().filter(|h| h.is_loss()).count();
//! let holdings_win_rate = (profitable_holdings as f64 / holdings.len() as f64) * 100.0;
//!
//! println!("📊 Holdings (Long-term):");
//! println!("   Total Holdings: {}", holdings.len());
//! println!("   Profitable: {} | Loss-making: {}", profitable_holdings, loss_holdings);
//! println!("   Win Rate: {:.1}%", holdings_win_rate);
//!
//! // Positions analysis
//! let active_positions: Vec<_> = positions.iter().filter(|p| !p.is_flat()).collect();
//! let profitable_positions = active_positions.iter().filter(|p| p.is_profitable()).count();
//! let loss_positions = active_positions.iter().filter(|p| p.is_loss()).count();
//!
//! if !active_positions.is_empty() {
//!     let positions_win_rate = (profitable_positions as f64 / active_positions.len() as f64) * 100.0;
//!     
//!     println!("📈 Active Positions (Trading):");
//!     println!("   Active Positions: {}", active_positions.len());
//!     println!("   Profitable: {} | Loss-making: {}", profitable_positions, loss_positions);
//!     println!("   Win Rate: {:.1}%", positions_win_rate);
//! }
//!
//! // Risk metrics
//! let total_holdings_value: f64 = holdings.iter().map(|h| h.market_value()).sum();
//! let total_position_exposure: f64 = active_positions.iter()
//!     .map(|p| p.market_value())
//!     .sum();
//!
//! println!("⚖️ Risk Exposure:");
//! println!("   Holdings Value: ₹{:.2}", total_holdings_value);
//! println!("   Position Exposure: ₹{:.2}", total_position_exposure);
//! println!("   Total Exposure: ₹{:.2}", total_holdings_value + total_position_exposure);
//! # Ok(())
//! # }
//! ```
//!
//! ## Data Models
//!
//! ### Holdings
//! The [`Holding`] struct represents long-term investments with comprehensive tracking:
//! - **Investment tracking**: Average price, current price, P&L calculations
//! - **Quantity management**: Total, T+1, realised, and pledged quantities
//! - **Trading availability**: Check what can be sold today vs tomorrow
//! - **Portfolio analytics**: Market value, investment value, percentage returns
//!
//! ### Positions
//! The [`Position`] struct represents active trading positions:
//! - **Direction tracking**: Long vs short positions
//! - **Day vs Net**: Separate intraday and overnight positions
//! - **P&L breakdown**: Realised, unrealised, and M2M calculations
//! - **Risk metrics**: Exposure, margin requirements
//!
//! ### Margins
//! The [`MarginData`] struct provides fund information:
//! - **Segment-wise**: Equity and commodity margins separately
//! - **Available funds**: Cash, collateral, and total available
//! - **Utilisation**: Used margin and exposure tracking
//! - **Order capacity**: Check if sufficient margin for new orders
//!
//! ## Error Handling
//!
//! All methods return `Result<T>` with comprehensive error information:
//!
//! ```rust,no_run
//! use kiteconnect_async_wasm::models::common::KiteError;
//!
//! # #[tokio::main]
//! # async fn main() {
//! # let client = kiteconnect_async_wasm::connect::KiteConnect::new("", "");
//! match client.holdings_typed().await {
//!     Ok(holdings) => {
//!         println!("Portfolio loaded: {} holdings", holdings.len());
//!         // Process holdings...
//!     }
//!     Err(KiteError::Authentication(msg)) => {
//!         eprintln!("Authentication failed: {}", msg);
//!         // Handle re-authentication
//!     }
//!     Err(KiteError::Api { status, message, .. }) => {
//!         eprintln!("API Error {}: {}", status, message);
//!         // Handle API errors
//!     }
//!     Err(e) => eprintln!("Other error: {}", e),
//! }
//! # }
//! ```
//!
//! ## Performance Considerations
//!
//! ### Efficient Data Access
//! - **Batch Operations**: Get holdings and positions together with `tokio::try_join!`
//! - **Typed APIs**: Use `*_typed()` methods for better performance and type safety
//! - **Selective Updates**: Update only necessary data for real-time monitoring
//!
//! ### Memory Usage
//! - **Structured Data**: Typed APIs use less memory than JSON parsing
//! - **Efficient Calculations**: Built-in helper methods reduce computation overhead
//! - **Lazy Evaluation**: Calculate metrics only when needed
//!
//! ## Rate Limiting
//!
//! The module automatically handles rate limiting according to KiteConnect API guidelines:
//! - **Portfolio APIs**: 3 requests per second for holdings, positions, margins
//! - **Automatic Retry**: Built-in retry mechanism with exponential backoff
//! - **Connection Pooling**: HTTP connections are reused for better performance
//!
//! ## Thread Safety
//!
//! All methods are thread-safe and can be called concurrently:
//! ```rust,no_run
//! # use kiteconnect_async_wasm::connect::KiteConnect;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! # let client = KiteConnect::new("", "");
//! // Concurrent portfolio data retrieval
//! let (holdings, positions, margins) = tokio::try_join!(
//!     client.holdings_typed(),
//!     client.positions_typed(),
//!     client.margins_typed(None)
//! )?;
//!
//! // All data retrieved concurrently for maximum efficiency
//! # Ok(())
//! # }
//! ```
//!
//! ## Migration from v1.0.2
//!
//! All existing methods continue to work. New typed methods provide enhanced features:
//! - Replace `holdings()` with `holdings_typed()` for structured data
//! - Use `positions_typed()` and `margins_typed()` for type safety
//! - Legacy methods remain available for backward compatibility
//! - Enhanced helper methods on all model structs for better analytics

use crate::connect::endpoints::KiteEndpoint;
use anyhow::Result;
use serde_json::Value as JsonValue;
// Import typed models for dual API support
use crate::models::auth::MarginData;
use crate::models::common::KiteResult;
use crate::models::portfolio::{ConversionRequest, Holding, Position};

use crate::connect::KiteConnect;

impl KiteConnect {
    // === LEGACY API METHODS (JSON responses) ===

    /// Retrieves account balance and margin details
    ///
    /// Returns margin information for trading segments including available cash,
    /// used margins, and available margins for different product types.
    ///
    /// # Arguments
    ///
    /// * `segment` - Optional trading segment ("equity" or "commodity"). If None, returns all segments
    ///
    /// # Returns
    ///
    /// A `Result<JsonValue>` containing margin data with fields like:
    /// - `available` - Available margin for trading
    /// - `utilised` - Currently utilized margin
    /// - `net` - Net available margin
    /// - `enabled` - Whether the segment is enabled
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
    /// // Get margins for all segments
    /// let all_margins = client.margins(None).await?;
    /// println!("All margins: {:?}", all_margins);
    ///
    /// // Get margins for specific segment
    /// let equity_margins = client.margins(Some("equity".to_string())).await?;
    /// println!("Equity available margin: {}",
    ///     equity_margins["data"]["available"]["live_balance"]);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn margins(&self, segment: Option<String>) -> Result<JsonValue> {
        if let Some(segment) = segment {
            let resp = self
                .send_request_with_rate_limiting_and_retry(
                    KiteEndpoint::MarginsSegment,
                    &[&segment],
                    None,
                    None,
                )
                .await
                .map_err(|e| anyhow::anyhow!("Get margins failed: {:?}", e))?;
            self.raise_or_return_json(resp).await
        } else {
            let resp = self
                .send_request_with_rate_limiting_and_retry(KiteEndpoint::Margins, &[], None, None)
                .await
                .map_err(|e| anyhow::anyhow!("Get margins failed: {:?}", e))?;
            self.raise_or_return_json(resp).await
        }
    }

    /// Get user profile details
    pub async fn profile(&self) -> Result<JsonValue> {
        let resp = self
            .send_request_with_rate_limiting_and_retry(KiteEndpoint::Profile, &[], None, None)
            .await
            .map_err(|e| anyhow::anyhow!("Get profile failed: {:?}", e))?;
        self.raise_or_return_json(resp).await
    }

    /// Retrieves the user's holdings (stocks held in demat account)
    ///
    /// Holdings represent stocks that are held in the user's demat account.
    /// This includes information about quantity, average price, current market value,
    /// profit/loss, and more.
    ///
    /// # Returns
    ///
    /// A `Result<JsonValue>` containing holdings data with fields like:
    /// - `tradingsymbol` - Trading symbol of the instrument
    /// - `quantity` - Total quantity held
    /// - `average_price` - Average buying price
    /// - `last_price` - Current market price
    /// - `pnl` - Profit and loss
    /// - `product` - Product type (CNC, MIS, etc.)
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
    /// let holdings = client.holdings().await?;
    /// println!("Holdings: {:?}", holdings);
    ///
    /// // Access specific fields
    /// if let Some(data) = holdings["data"].as_array() {
    ///     for holding in data {
    ///         println!("Symbol: {}, Quantity: {}",
    ///             holding["tradingsymbol"], holding["quantity"]);
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn holdings(&self) -> Result<JsonValue> {
        let resp = self
            .send_request_with_rate_limiting_and_retry(KiteEndpoint::Holdings, &[], None, None)
            .await
            .map_err(|e| anyhow::anyhow!("Get holdings failed: {:?}", e))?;
        self.raise_or_return_json(resp).await
    }

    /// Retrieves the user's positions (open positions for the day)
    ///
    /// Positions represent open trading positions for the current trading day.
    /// This includes both intraday and carry-forward positions with details about
    /// profit/loss, margin requirements, and position status.
    ///
    /// # Returns
    ///
    /// A `Result<JsonValue>` containing positions data with fields like:
    /// - `tradingsymbol` - Trading symbol of the instrument
    /// - `quantity` - Net position quantity
    /// - `buy_quantity` - Total buy quantity
    /// - `sell_quantity` - Total sell quantity
    /// - `average_price` - Average position price
    /// - `pnl` - Realized and unrealized P&L
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
    /// let positions = client.positions().await?;
    /// println!("Positions: {:?}", positions);
    ///
    /// // Check for open positions
    /// if let Some(day_positions) = positions["data"]["day"].as_array() {
    ///     for position in day_positions {
    ///         if position["quantity"].as_i64().unwrap_or(0) != 0 {
    ///             println!("Open position: {} qty {}",
    ///                 position["tradingsymbol"], position["quantity"]);
    ///         }
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn positions(&self) -> Result<JsonValue> {
        let resp = self
            .send_request_with_rate_limiting_and_retry(KiteEndpoint::Positions, &[], None, None)
            .await
            .map_err(|e| anyhow::anyhow!("Get positions failed: {:?}", e))?;
        self.raise_or_return_json(resp).await
    }

    // === TYPED API METHODS (v1.0.0) ===

    /// Get user margins with typed response
    ///
    /// Returns strongly typed margin data instead of JsonValue.
    ///
    /// # Arguments
    ///
    /// * `segment` - Optional trading segment ("equity" or "commodity")
    ///
    /// # Returns
    ///
    /// A `KiteResult<MarginData>` containing typed margin information
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
    /// let margins = client.margins_typed(None).await?;
    /// println!("Available equity margin: {}", margins.equity.unwrap().available.cash);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn margins_typed(&self, segment: Option<&str>) -> KiteResult<MarginData> {
        if let Some(segment) = segment {
            let resp = self
                .send_request_with_rate_limiting_and_retry(
                    KiteEndpoint::MarginsSegment,
                    &[segment],
                    None,
                    None,
                )
                .await?;
            let json_response = self.raise_or_return_json_typed(resp).await?;
            self.parse_response(json_response)
        } else {
            let resp = self
                .send_request_with_rate_limiting_and_retry(KiteEndpoint::Margins, &[], None, None)
                .await?;
            let json_response = self.raise_or_return_json_typed(resp).await?;
            self.parse_response(json_response)
        }
    }

    /// Get user holdings with typed response
    ///
    /// Returns a vector of strongly typed holding objects instead of JsonValue.
    ///
    /// # Returns
    ///
    /// A `KiteResult<Vec<Holding>>` containing typed holdings data
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
    /// let holdings = client.holdings_typed().await?;
    /// for holding in holdings {
    ///     println!("Symbol: {}, Quantity: {}, P&L: {}",
    ///         holding.trading_symbol, holding.quantity, holding.pnl);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn holdings_typed(&self) -> KiteResult<Vec<Holding>> {
        let resp = self
            .send_request_with_rate_limiting_and_retry(KiteEndpoint::Holdings, &[], None, None)
            .await?;
        let json_response = self.raise_or_return_json_typed(resp).await?;

        // Extract the "data" field and parse as Vec<Holding>
        if let Some(data) = json_response.get("data") {
            self.parse_response(data.clone())
        } else {
            // If no "data" field, try parsing the entire response
            self.parse_response(json_response)
        }
    }

    /// Get user positions with typed response
    ///
    /// Returns structured position data instead of JsonValue.
    ///
    /// # Returns
    ///
    /// A `KiteResult<Vec<Position>>` containing typed positions data
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
    /// let positions = client.positions_typed().await?;
    /// for position in &positions {
    ///     if position.quantity != 0 {
    ///         println!("Open position: {} qty {}",
    ///             position.trading_symbol, position.quantity);
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn positions_typed(&self) -> KiteResult<Vec<Position>> {
        let resp = self
            .send_request_with_rate_limiting_and_retry(KiteEndpoint::Positions, &[], None, None)
            .await?;
        let json_response = self.raise_or_return_json_typed(resp).await?;

        // KiteConnect returns positions in nested structure: { "data": { "day": [...], "net": [...] } }
        // We'll flatten both day and net positions into a single vector
        let mut all_positions = Vec::new();

        if let Some(data) = json_response.get("data") {
            if let Some(day_positions) = data.get("day").and_then(|v| v.as_array()) {
                for pos_json in day_positions {
                    if let Ok(position) = self.parse_response::<Position>(pos_json.clone()) {
                        all_positions.push(position);
                    }
                }
            }

            if let Some(net_positions) = data.get("net").and_then(|v| v.as_array()) {
                for pos_json in net_positions {
                    if let Ok(position) = self.parse_response::<Position>(pos_json.clone()) {
                        all_positions.push(position);
                    }
                }
            }
        }

        Ok(all_positions)
    }

    /// Convert positions between product types (typed)
    ///
    /// Converts a position from one product type to another (e.g., MIS to CNC).
    ///
    /// # Arguments
    ///
    /// * `request` - Conversion request details
    ///
    /// # Returns
    ///
    /// A `KiteResult<bool>` indicating success
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use kiteconnect_async_wasm::connect::KiteConnect;
    /// use kiteconnect_async_wasm::models::portfolio::ConversionRequest;
    /// use kiteconnect_async_wasm::models::common::{Exchange, Product, TransactionType};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = KiteConnect::new("api_key", "access_token");
    ///
    /// let conversion = ConversionRequest {
    ///     exchange: Exchange::NSE,
    ///     trading_symbol: "RELIANCE".to_string(),
    ///     transaction_type: TransactionType::BUY,
    ///     quantity: 10,
    ///     from_product: Product::MIS,
    ///     to_product: Product::CNC,
    /// };
    ///
    /// let success = client.convert_position_typed(&conversion).await?;
    /// println!("Conversion successful: {}", success);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn convert_position_typed(&self, request: &ConversionRequest) -> KiteResult<bool> {
        let mut params = std::collections::HashMap::new();
        let exchange_str = request.exchange.to_string();
        let transaction_str = request.transaction_type.to_string();
        let quantity_str = request.quantity.to_string();
        let from_product_str = request.from_product.to_string();
        let to_product_str = request.to_product.to_string();

        params.insert("exchange", exchange_str.as_str());
        params.insert("tradingsymbol", request.trading_symbol.as_str());
        params.insert("transaction_type", transaction_str.as_str());
        params.insert("quantity", quantity_str.as_str());
        params.insert("old_product", from_product_str.as_str());
        params.insert("new_product", to_product_str.as_str());

        let resp = self
            .send_request_with_rate_limiting_and_retry(
                KiteEndpoint::ConvertPosition,
                &[],
                None,
                Some(params),
            )
            .await?;
        let json_response = self.raise_or_return_json_typed(resp).await?;

        // Check if conversion was successful
        Ok(json_response.get("status").and_then(|v| v.as_str()) == Some("success"))
    }
}
