//! # Orders Module
//!
//! This module provides comprehensive order management capabilities for the KiteConnect API v1.0.3,
//! offering complete control over trade execution with both simple and advanced order types.
//!
//! ## Overview
//!
//! The orders module is the core component for executing trades on the KiteConnect platform.
//! It provides access to order placement, modification, cancellation, and monitoring with
//! both legacy JSON-based and modern strongly-typed APIs for enhanced developer experience.
//!
//! ## Key Features
//!
//! ### 🔄 **Dual API Support**
//! - **Legacy API**: Returns `JsonValue` for backward compatibility
//! - **Typed API**: Returns structured types with compile-time safety (methods ending in `_typed`)
//!
//! ### 📊 **Complete Order Management**
//! - **Order Placement**: Market, limit, stop-loss, and bracket orders
//! - **Order Modification**: Update price, quantity, and order parameters
//! - **Order Cancellation**: Cancel pending orders and exit positions
//! - **Order Monitoring**: Real-time status updates and fill information
//!
//! ### 💡 **Advanced Order Types**
//! - **Regular Orders**: Basic buy/sell orders
//! - **Bracket Orders**: Auto stop-loss and profit booking
//! - **Cover Orders**: Built-in stop-loss protection
//! - **Iceberg Orders**: Large order execution in smaller chunks
//! - **GTT Orders**: Good Till Triggered conditional orders
//!
//! ### 🛠️ **Builder Patterns**
//! - **OrderBuilder**: Fluent API for constructing orders
//! - **BracketOrderBuilder**: Specialized builder for bracket orders
//! - **Type Safety**: Compile-time validation of order parameters
//!
//! ## Available Methods
//!
//! ### Order Placement
//! - [`place_order()`](KiteConnect::place_order) / [`place_order_typed()`](KiteConnect::place_order_typed) - Place new orders
//! - [`modify_order()`](KiteConnect::modify_order) - Modify existing orders
//! - [`cancel_order()`](KiteConnect::cancel_order) - Cancel pending orders
//!
//! ### Order Information
//! - [`orders()`](KiteConnect::orders) / [`orders_typed()`](KiteConnect::orders_typed) - Get all orders
//! - [`order_history()`](KiteConnect::order_history) - Get order execution history
//! - [`trades()`](KiteConnect::trades) / [`trades_typed()`](KiteConnect::trades_typed) - Get trade book
//!
//! ### Position Management
//! - [`convert_position()`](KiteConnect::convert_position) - Convert product types
//! - Position tracking and P&L monitoring
//!
//! ## Usage Examples
//!
//! ### Basic Order Placement
//! ```rust,no_run
//! use kiteconnect_async_wasm::connect::KiteConnect;
//! use kiteconnect_async_wasm::models::orders::OrderParams;
//! use kiteconnect_async_wasm::models::common::{Exchange, TransactionType, OrderType, Product, Validity};
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = KiteConnect::new("api_key", "access_token");
//!
//! // Method 1: Direct order parameters
//! let order_params = OrderParams {
//!     exchange: Exchange::NSE,
//!     trading_symbol: "RELIANCE".to_string(),
//!     transaction_type: TransactionType::BUY,
//!     quantity: 10,
//!     order_type: OrderType::LIMIT,
//!     product: Product::CNC,
//!     price: Some(2500.0),
//!     validity: Some(Validity::DAY),
//!     disclosed_quantity: None,
//!     trigger_price: None,
//!     tag: Some("MyOrder".to_string()),
//!     squareoff: None,
//!     stoploss: None,
//!     trailing_stoploss: None,
//!     market_protection: None,
//!     iceberg_legs: None,
//!     iceberg_quantity: None,
//!     auction_number: None,
//! };
//!
//! let response = client.place_order_typed("regular", &order_params).await?;
//! println!("🎯 Order placed successfully!");
//! println!("   Order ID: {}", response.order_id);
//! # Ok(())
//! # }
//! ```
//!
//! ### Using Order Builder (Recommended)
//! ```rust,no_run
//! use kiteconnect_async_wasm::connect::KiteConnect;
//! use kiteconnect_async_wasm::models::orders::OrderBuilder;
//! use kiteconnect_async_wasm::models::common::{Exchange, TransactionType, OrderType, Product, Validity};
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = KiteConnect::new("api_key", "access_token");
//!
//! // Fluent builder pattern for better ergonomics
//! let order = OrderBuilder::new()
//!     .trading_symbol("TCS")
//!     .exchange(Exchange::NSE)
//!     .transaction_type(TransactionType::BUY)
//!     .quantity(5)
//!     .order_type(OrderType::MARKET)
//!     .product(Product::MIS)
//!     .validity(Validity::DAY)
//!     .tag("QuickBuy")
//!     .build()?;
//!
//! let response = client.place_order_typed("regular", &order).await?;
//! println!("🚀 Market order executed!");
//! println!("   Order ID: {}", response.order_id);
//! # Ok(())
//! # }
//! ```
//!
//! ### Advanced Order Types
//! ```rust,no_run
//! use kiteconnect_async_wasm::connect::KiteConnect;
//! use kiteconnect_async_wasm::models::orders::{OrderBuilder, BracketOrderBuilder};
//! use kiteconnect_async_wasm::models::common::{Exchange, TransactionType, OrderType, Product};
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = KiteConnect::new("api_key", "access_token");
//!
//! // Stop-Loss Order
//! let sl_order = OrderBuilder::new()
//!     .trading_symbol("INFY")
//!     .exchange(Exchange::NSE)
//!     .transaction_type(TransactionType::SELL)
//!     .quantity(20)
//!     .order_type(OrderType::SL)
//!     .product(Product::MIS)
//!     .price(1450.0)
//!     .trigger_price(1440.0)
//!     .build()?;
//!
//! let sl_response = client.place_order_typed("regular", &sl_order).await?;
//! println!("🛡️ Stop-loss order placed: {}", sl_response.order_id);
//!
//! // Bracket Order (Buy with auto profit booking and stop-loss)
//! let bracket_order = BracketOrderBuilder::new()
//!     .trading_symbol("HDFC")
//!     .exchange(Exchange::NSE)
//!     .transaction_type(TransactionType::BUY)
//!     .quantity(10)
//!     .price(1600.0)
//!     .squareoff(1650.0)    // Take profit at +50
//!     .stoploss(1580.0)     // Stop loss at -20
//!     .trailing_stoploss(5.0) // Trail by 5 points
//!     .build()?;
//!
//! let bo_response = client.place_order_typed("bo", &bracket_order.order_params).await?;
//! println!("🎯 Bracket order placed: {}", bo_response.order_id);
//!
//! // Iceberg Order (Large order in small chunks)
//! let iceberg_order = OrderBuilder::new()
//!     .trading_symbol("SBIN")
//!     .exchange(Exchange::NSE)
//!     .transaction_type(TransactionType::BUY)
//!     .quantity(1000)        // Total quantity
//!     .order_type(OrderType::LIMIT)
//!     .product(Product::CNC)
//!     .price(250.0)
//!     .iceberg(10, 100)      // 10 legs of 100 shares each
//!     .build()?;
//!
//! let iceberg_response = client.place_order_typed("iceberg", &iceberg_order).await?;
//! println!("🧊 Iceberg order placed: {}", iceberg_response.order_id);
//! # Ok(())
//! # }
//! ```
//!
//! ### Order Monitoring and Management
//! ```rust,no_run
//! use kiteconnect_async_wasm::connect::KiteConnect;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = KiteConnect::new("api_key", "access_token");
//!
//! // Get all orders for the day
//! let orders = client.orders_typed().await?;
//!
//! println!("📋 Today's Orders ({}):", orders.len());
//! println!("================================");
//!
//! for order in &orders {
//!     let status_icon = match order.status.as_str() {
//!         "COMPLETE" => "✅",
//!         "OPEN" | "TRIGGER PENDING" => "⏳",
//!         "CANCELLED" => "❌",
//!         "REJECTED" => "🚫",
//!         _ => "❓",
//!     };
//!
//!     println!("{} {} ({})", status_icon, order.order_id, order.status);
//!     println!("   📊 {} {} {} @ ₹{:.2}",
//!         order.transaction_type,
//!         order.quantity,
//!         order.trading_symbol,
//!         order.price.unwrap_or(0.0));
//!
//!     if order.is_partially_filled() {
//!         println!("   📈 Partial fill: {}/{} ({:.1}%)",
//!             order.filled_quantity,
//!             order.quantity,
//!             order.fill_percentage());
//!     }
//!
//!     if order.is_complete() && order.filled_quantity > 0 {
//!         println!("   💰 Avg fill price: ₹{:.2}",
//!             order.average_price.unwrap_or(0.0));
//!     }
//!     println!();
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ### Order Modification and Cancellation
//! ```rust,no_run
//! use kiteconnect_async_wasm::connect::KiteConnect;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = KiteConnect::new("api_key", "access_token");
//!
//! let order_id = "210429000000001"; // Replace with actual order ID
//!
//! // Modify order price
//! let modify_result = client.modify_order(
//!     order_id,
//!     "regular",           // variety
//!     None,               // quantity (unchanged)
//!     Some("2550.0"),     // new price
//!     None,               // order_type (unchanged)
//!     None,               // validity (unchanged)
//!     None,               // disclosed_quantity
//!     None,               // trigger_price
//!     None,               // parent_order_id
//! ).await;
//!
//! match modify_result {
//!     Ok(_) => println!("✅ Order {} modified successfully", order_id),
//!     Err(e) => println!("❌ Failed to modify order: {}", e),
//! }
//!
//! // Cancel order if modification fails or no longer needed
//! let cancel_result = client.cancel_order(order_id, "regular", None).await;
//! match cancel_result {
//!     Ok(_) => println!("🗑️ Order {} cancelled successfully", order_id),
//!     Err(e) => println!("❌ Failed to cancel order: {}", e),
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ### Trade Book Analysis
//! ```rust,no_run
//! use kiteconnect_async_wasm::connect::KiteConnect;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = KiteConnect::new("api_key", "access_token");
//!
//! // Get all trades for the day
//! let trades = client.trades_typed().await?;
//!
//! println!("💼 Trade Book Analysis ({} trades):", trades.len());
//! println!("=====================================");
//!
//! let mut total_turnover = 0.0;
//! let mut buy_trades = 0;
//! let mut sell_trades = 0;
//!
//! for trade in &trades {
//!     let trade_value = trade.total_value();
//!     total_turnover += trade_value;
//!
//!     if trade.is_buy() {
//!         buy_trades += 1;
//!     } else {
//!         sell_trades += 1;
//!     }
//!
//!     println!("🔄 {} {}: {} @ ₹{:.2} (₹{:.2})",
//!         trade.fill_timestamp.format("%H:%M:%S"),
//!         trade.trading_symbol,
//!         if trade.is_buy() { "BUY" } else { "SELL" },
//!         trade.average_price,
//!         trade_value);
//! }
//!
//! println!();
//! println!("📊 Summary:");
//! println!("   Total trades: {} (Buy: {}, Sell: {})", trades.len(), buy_trades, sell_trades);
//! println!("   Total turnover: ₹{:.2}", total_turnover);
//! println!("   Average trade size: ₹{:.2}",
//!     if !trades.is_empty() { total_turnover / trades.len() as f64 } else { 0.0 });
//! # Ok(())
//! # }
//! ```
//!
//! ## Data Models
//!
//! ### Order Types
//! The [`Order`] struct represents order information with comprehensive status tracking:
//! - **Order Status**: Open, complete, cancelled, rejected states
//! - **Fill Information**: Partial and complete fill tracking
//! - **Execution Details**: Average price, timestamps, and exchange data
//! - **Order Analysis**: Helper methods for status checking and calculations
//!
//! ### Order Parameters
//! The [`OrderParams`] struct defines order placement requirements:
//! - **Required Fields**: Symbol, exchange, transaction type, quantity
//! - **Optional Fields**: Price, validity, disclosed quantity, tags
//! - **Advanced Features**: Stop-loss, iceberg, bracket order parameters
//! - **Validation**: Built-in parameter validation and error checking
//!
//! ### Trade Information
//! The [`Trade`] struct represents executed trades:
//! - **Execution Data**: Fill price, quantity, and timestamp
//! - **Order Linkage**: Connection to parent order information
//! - **Value Calculations**: Trade value and commission tracking
//! - **Direction Analysis**: Buy/sell identification and analysis
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
//! # let order_params = kiteconnect_async_wasm::models::orders::OrderParams {
//! #     exchange: kiteconnect_async_wasm::models::common::Exchange::NSE,
//! #     trading_symbol: "TEST".to_string(),
//! #     transaction_type: kiteconnect_async_wasm::models::common::TransactionType::BUY,
//! #     quantity: 1,
//! #     order_type: kiteconnect_async_wasm::models::common::OrderType::MARKET,
//! #     product: kiteconnect_async_wasm::models::common::Product::MIS,
//! #     price: None, validity: None, disclosed_quantity: None, trigger_price: None,
//! #     tag: None, squareoff: None, stoploss: None, trailing_stoploss: None,
//! #     market_protection: None, iceberg_legs: None, iceberg_quantity: None,
//! #     auction_number: None,
//! # };
//! match client.place_order_typed("regular", &order_params).await {
//!     Ok(response) => {
//!         println!("✅ Order placed: {}", response.order_id);
//!     }
//!     Err(KiteError::Authentication(msg)) => {
//!         eprintln!("🔐 Authentication failed: {}", msg);
//!         // Handle re-authentication
//!     }
//!     Err(KiteError::Api { status, message, .. }) => {
//!         eprintln!("🚫 Order rejected: {} - {}", status, message);
//!         // Handle order rejection (insufficient margin, invalid params, etc.)
//!     }
//!     Err(KiteError::RateLimit { retry_after, .. }) => {
//!         eprintln!("⏱️ Rate limited, retry after: {:?}", retry_after);
//!         // Handle rate limiting
//!     }
//!     Err(e) => eprintln!("❌ Other error: {}", e),
//! }
//! # }
//! ```
//!
//! ## Order Validation
//!
//! The module provides built-in validation for order parameters:
//!
//! ```rust,no_run
//! use kiteconnect_async_wasm::models::orders::OrderBuilder;
//! use kiteconnect_async_wasm::models::common::{Exchange, TransactionType, OrderType, Product};
//!
//! # fn example() -> Result<(), String> {
//! // This will fail validation - no price for LIMIT order
//! let invalid_order = OrderBuilder::new()
//!     .trading_symbol("RELIANCE")
//!     .exchange(Exchange::NSE)
//!     .transaction_type(TransactionType::BUY)
//!     .quantity(10)
//!     .order_type(OrderType::LIMIT)  // LIMIT order requires price
//!     .product(Product::CNC)
//!     .build(); // This returns Err("Price is required for LIMIT orders")
//!
//! match invalid_order {
//!     Ok(_) => println!("Order validated successfully"),
//!     Err(e) => println!("Validation error: {}", e),
//! }
//!
//! // Correct version with price
//! let valid_order = OrderBuilder::new()
//!     .trading_symbol("RELIANCE")
//!     .exchange(Exchange::NSE)
//!     .transaction_type(TransactionType::BUY)
//!     .quantity(10)
//!     .order_type(OrderType::LIMIT)
//!     .product(Product::CNC)
//!     .price(2500.0)  // Price provided for LIMIT order
//!     .build()?;
//!
//! println!("✅ Order validated and ready for placement");
//! # Ok(())
//! # }
//! ```
//!
//! ## Performance Considerations
//!
//! ### Efficient Order Management
//! - **Batch Operations**: Use `tokio::join!` for concurrent order operations
//! - **Typed APIs**: Use `*_typed()` methods for better performance and type safety
//! - **Builder Patterns**: Use builders for complex orders to avoid parameter errors
//!
//! ### Memory Usage
//! - **Structured Data**: Typed APIs use less memory than JSON parsing
//! - **Efficient Calculations**: Built-in helper methods reduce computation overhead
//! - **Order Filtering**: Filter orders client-side to reduce data processing
//!
//! ## Rate Limiting
//!
//! The module automatically handles rate limiting according to KiteConnect API guidelines:
//! - **Order APIs**: 10 requests per second for order placement and modification
//! - **Query APIs**: 3 requests per second for order and trade queries
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
//! // Concurrent order and trade data retrieval
//! let (orders, trades) = tokio::try_join!(
//!     client.orders_typed(),
//!     client.trades_typed()
//! )?;
//!
//! // Process both datasets concurrently
//! println!("Orders: {}, Trades: {}", orders.len(), trades.len());
//! # Ok(())
//! # }
//! ```
//!
//! ## Migration from v1.0.2
//!
//! All existing methods continue to work. New typed methods provide enhanced features:
//! - Replace `place_order()` with `place_order_typed()` for structured parameters
//! - Use `orders_typed()` and `trades_typed()` for type safety
//! - Leverage `OrderBuilder` and `BracketOrderBuilder` for complex orders
//! - Legacy methods remain available for backward compatibility

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
    #[allow(clippy::too_many_arguments)]
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
    #[allow(clippy::too_many_arguments)]
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
    #[allow(clippy::too_many_arguments)]
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
