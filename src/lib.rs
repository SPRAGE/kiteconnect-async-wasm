//! # kiteconnect-async-wasm
//! 
//! A modern, async Rust implementation of the Zerodha KiteConnect API with WASM support.
//! This library provides comprehensive access to KiteConnect's REST APIs for trading, 
//! portfolio management, and market data.
//! 
//! ## Features
//! 
//! - **🚀 Async/Await**: Built with modern Rust async patterns using `tokio`
//! - **🌐 WASM Compatible**: Run in browsers with WebAssembly support
//! - **🔄 Cross-Platform**: Native (Linux, macOS, Windows) and Web targets
//! - **📦 Modern Dependencies**: Updated to latest Rust ecosystem libraries
//! - **🧪 Well Tested**: Comprehensive test coverage with mocked responses
//! - **⚡ High Performance**: Efficient HTTP client with connection pooling
//! - **🛡️ Type Safe**: Leverages Rust's type system for safer API interactions
//! 
//! ## Feature Flags
//! 
//! This crate supports multiple feature flags to enable platform-specific functionality:
//! 
//! - **`native`** (default): Enables native platform support with tokio, file I/O, and CSV parsing
//!   - Includes: `tokio`, `sha2`, `csv` dependencies
//!   - Best for: Desktop applications, servers, CLI tools
//! 
//! - **`wasm`**: Enables WebAssembly support with browser APIs and CSV parsing
//!   - Includes: `wasm-bindgen`, `web-sys`, `js-sys`, `gloo-utils`, `csv-core` dependencies
//!   - Features: Browser-compatible CSV parsing using csv-core
//!   - Best for: Browser applications, web workers
//! 
//! - **`debug`**: Enables additional logging and debugging features
//!   - Includes: Enhanced `log` output
//!   - Best for: Development and troubleshooting
//! 
//! ## Quick Start
//! 
//! Add to your `Cargo.toml`:
//! 
//! ```toml
//! # For native applications (default)
//! [dependencies]
//! kiteconnect-async-wasm = "0.1.0"
//! 
//! # For WASM/browser applications
//! [dependencies]
//! kiteconnect-async-wasm = { version = "0.1.0", default-features = false, features = ["wasm"] }
//! 
//! # For development with debugging
//! [dependencies]
//! kiteconnect-async-wasm = { version = "0.1.0", features = ["native", "debug"] }
//! ```
//! 
//! ## Basic Usage
//! 
//! ```rust,no_run
//! use kiteconnect_async_wasm::connect::KiteConnect;
//! use serde_json::Value as JsonValue;
//! 
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Initialize KiteConnect client
//!     let mut kiteconnect = KiteConnect::new("<YOUR-API-KEY>", "");
//! 
//!     // Step 1: Get login URL
//!     let login_url = kiteconnect.login_url();
//!     println!("Login URL: {}", login_url);
//!     
//!     // Step 2: After user login, generate session with request token
//!     let session_response = kiteconnect
//!         .generate_session("<REQUEST-TOKEN>", "<API-SECRET>")
//!         .await?;
//!     println!("Session: {:?}", session_response);
//! 
//!     // Step 3: Use the API (access token is automatically set)
//!     let holdings = kiteconnect.holdings().await?;
//!     println!("Holdings: {:?}", holdings);
//! 
//!     Ok(())
//! }
//! ```
//! 
//! ## Modular Architecture
//! 
//! The library is organized into focused modules for better maintainability:
//! 
//! ### Core Modules
//! - **`connect::client`** - Main KiteConnect struct and authentication
//! - **`connect::portfolio`** - Portfolio operations (holdings, positions, margins)
//! - **`connect::orders`** - Order management and trading operations  
//! - **`connect::market`** - Market data and instrument information
//! - **`connect::mutual_funds`** - Mutual fund operations and SIPs
//! - **`connect::gtt`** - Good Till Triggered order management
//! - **`connect::request`** - HTTP request handling abstraction
//! - **`connect::utils`** - CSV parsing utilities for WASM compatibility
//! 
//! ### Model Types
//! - **`model`** - Comprehensive type definitions for all API responses
//!   - Portfolio models (Holdings, Positions, Margins)
//!   - Order models (Order, Trade, OrderResponse) 
//!   - Market data models (Quote, Instrument, Historical data)
//!   - Mutual fund models (MFOrder, MFSIP, MFHolding)
//!   - Error handling types (KiteResponse, KiteErrorResponse)
//! 
//! ## Available APIs
//! 
//! The library provides access to all KiteConnect REST APIs through a clean, modular structure:
//! 
//! ### Authentication
//! - `login_url()` - Generate login URL for user authentication
//! - `generate_session()` - Create session with request token and API secret
//! - `invalidate_session()` - Logout user and invalidate access token
//! - `set_access_token()` - Manually set access token
//! - `set_session_expiry_hook()` - Set callback for session expiry events
//! 
//! ### Portfolio Management
//! - `profile()` - Get user profile information
//! - `holdings()` - Get user holdings with P&L
//! - `positions()` - Get user positions (day and net)
//! - `margins()` - Get account margins for all segments
//! - `instruments_margins()` - Get margins for specific segment
//! 
//! ### Order Management  
//! - `orders()` - Get all orders for the day
//! - `order_history()` - Get order history for specific order ID
//! - `place_order()` - Place new orders (regular, bracket, cover, etc.)
//! - `modify_order()` - Modify existing pending orders
//! - `cancel_order()` - Cancel pending orders
//! - `convert_position()` - Convert position between product types
//! - `trades()` - Get all executed trades
//! - `order_trades()` - Get trades for specific order
//! 
//! ### Market Data
//! - `instruments()` - Get complete instrument list (CSV parsed to structs)
//! - `historical_data()` - Get historical OHLCV candlestick data
//! - `quote()` - Get real-time quotes with market depth
//! - `ohlc()` - Get OHLC data for instruments
//! - `ltp()` - Get last traded price for instruments
//! - `trigger_range()` - Get valid price ranges for stop-loss orders
//! 
//! ### Mutual Funds
//! - `mf_instruments()` - Get mutual fund instrument list
//! - `mf_orders()` - Get mutual fund orders
//! - `place_mf_order()` - Place mutual fund buy/sell orders
//! - `cancel_mf_order()` - Cancel pending mutual fund orders
//! - `mf_holdings()` - Get mutual fund holdings
//! - `mf_sips()` - Get SIP (Systematic Investment Plan) details
//! - `place_mf_sip()` - Create new SIP
//! - `modify_mf_sip()` - Modify existing SIP
//! - `cancel_mf_sip()` - Cancel SIP
//! 
//! ### GTT (Good Till Triggered)
//! - `gtts()` - Get all GTT orders
//! - `place_gtt()` - Place new GTT order
//! - `modify_gtt()` - Modify existing GTT order
//! - `delete_gtt()` - Delete GTT order
//! 
//! ## Error Handling
//! 
//! The library uses `anyhow::Result` for comprehensive error handling:
//! 
//! ```rust,no_run
//! # use kiteconnect_async_wasm::connect::KiteConnect;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! # let kiteconnect = KiteConnect::new("", "");
//! match kiteconnect.holdings().await {
//!     Ok(holdings) => {
//!         println!("Holdings: {:?}", holdings);
//!         // Access typed data directly
//!         for holding in &holdings {
//!             println!("Symbol: {}, P&L: {}", holding.tradingsymbol, holding.pnl);
//!         }
//!     },
//!     Err(e) => eprintln!("Error fetching holdings: {}", e),
//! }
//! # Ok(())
//! # }
//! ```
//! 
//! ## Platform-Specific Features
//! 
//! ### Native (Tokio)
//! - Full CSV parsing for instruments with typed structs
//! - Complete async/await support with `tokio` runtime
//! - High-performance HTTP client with connection pooling
//! - Native cryptographic functions for authentication
//! - File I/O support for caching and logging
//! 
//! ### WASM (Browser)
//! - All APIs supported with full CSV parsing capability
//! - CSV parsing using `csv-core` for browser compatibility
//! - Returns structured typed data (same as native)
//! - Compatible with web frameworks (React, Vue, etc.)
//! - Works in web workers for background processing
//! 
//! ## Examples
//! 
//! See the `examples/` directory for comprehensive usage examples:
//! - `connect_sample.rs` - Basic API usage and authentication flow
//! - `async_connect_example.rs` - Advanced async patterns and error handling
//! - `comprehensive_example.rs` - Complete API demonstration
//! - `model_usage_example.rs` - Working with typed response models
//! - `response_handling_example.rs` - Error handling and response parsing
//! 
//! ## Migration from Original Library
//! 
//! This library maintains API compatibility while providing significant improvements:
//! 
//! - **Type Safety**: All responses are now strongly typed structs instead of JSON
//! - **Async/Await**: Modern async patterns replace blocking calls
//! - **Modular Design**: Clean separation of concerns across modules  
//! - **WASM Support**: Works in browsers with full CSV parsing capability
//! - **Better Error Handling**: Comprehensive error types and `anyhow::Result`
//! - **Performance**: Connection pooling and efficient HTTP client
//! 
//! ## Thread Safety
//! 
//! The `KiteConnect` struct is `Clone + Send + Sync`, making it safe to use across
//! multiple threads and async tasks. The underlying HTTP client uses connection
//! pooling for optimal performance.
//! 
//! ```rust,no_run
//! # use kiteconnect_async_wasm::connect::KiteConnect;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let kiteconnect = KiteConnect::new("<API-KEY>", "<ACCESS-TOKEN>");
//! 
//! // Clone for use in different tasks
//! let kc1 = kiteconnect.clone();
//! let kc2 = kiteconnect.clone();
//! 
//! // Use in concurrent tasks
//! let (holdings, positions) = tokio::try_join!(
//!     kc1.holdings(),
//!     kc2.positions()
//! )?;
//! # Ok(())
//! # }
//! ```
//!
#[cfg(test)]
extern crate mockito;

/// Main KiteConnect API client and authentication module
/// 
/// This module contains the core [`KiteConnect`] struct and all API methods
/// organized into focused submodules for better maintainability:
/// 
/// - [`client`] - Core struct, authentication, and session management
/// - [`portfolio`] - Portfolio operations (holdings, positions, margins)  
/// - [`orders`] - Order management and trading operations
/// - [`market`] - Market data and instrument information
/// - [`mutual_funds`] - Mutual fund operations and SIPs
/// - [`gtt`] - Good Till Triggered order management
/// 
/// [`KiteConnect`]: connect::KiteConnect
/// [`client`]: connect::client
/// [`portfolio`]: connect::portfolio  
/// [`orders`]: connect::orders
/// [`market`]: connect::market
/// [`mutual_funds`]: connect::mutual_funds
/// [`gtt`]: connect::gtt
pub mod connect;

/// Comprehensive data models for all KiteConnect API responses
/// 
/// This module provides strongly-typed Rust structs for all API responses,
/// enabling type-safe interaction with the KiteConnect API. Models are
/// organized by functionality:
/// 
/// - Portfolio models ([`Holdings`], [`Positions`], [`Margins`])
/// - Order models ([`Order`], [`Trade`], [`OrderResponse`])
/// - Market data models ([`Quote`], [`Instrument`], [`Historical data`])
/// - Mutual fund models ([`MFOrder`], [`MFSIP`], [`MFHolding`])
/// - Response wrappers ([`KiteResponse`], [`KiteErrorResponse`])
/// 
/// [`Holdings`]: model::portfolio::Holdings
/// [`Positions`]: model::portfolio::Positions
/// [`Margins`]: model::margin::Margins
/// [`Order`]: model::orders::Order
/// [`Trade`]: model::orders::Trade
/// [`OrderResponse`]: model::orders::OrderResponse
/// [`Quote`]: model::market::Quote
/// [`Instrument`]: model::market::Instrument
/// [`MFOrder`]: model::mutualfunds::MFOrder
/// [`MFSIP`]: model::mutualfunds::MFSIP
/// [`MFHolding`]: model::mutualfunds::MFHolding
/// [`KiteResponse`]: model::response::KiteResponse
/// [`KiteErrorResponse`]: model::errors::KiteErrorResponse
pub mod model;
