//! # kiteconnect-async-wasm v1.0.3
//!
//! A modern, async Rust implementation of the Zerodha KiteConnect API with dual API support,
//! enhanced error handling, and full WASM compatibility. This library provides comprehensive
//! access to KiteConnect's REST APIs for trading, portfolio management, and market data.
//!
//! ## ✨ Features v1.0.3
//!
//! - **🚀 Enhanced Historical Data API**: New `HistoricalDataRequest` struct with `NaiveDateTime` precision
//! - **🔄 Dual Serde Support**: Flexible Interval enum accepting both strings and integers
//! - **📦 Organized Enum System**: Modular enum structure for better maintainability
//! - **🎯 Dual API Support**: Legacy JSON + new strongly-typed APIs with automatic retry logic
//! - **🌐 WASM Compatible**: Run in browsers with WebAssembly support
//! - **🔄 Cross-Platform**: Native (Linux, macOS, Windows) and Web targets
//! - **📦 Modern Dependencies**: Updated to latest Rust ecosystem libraries
//! - **🧪 Well Tested**: Comprehensive test coverage (30 unit + 11 integration + 58 doc tests)
//! - **⚡ High Performance**: Efficient HTTP client with connection pooling and caching
//! - **🛡️ Type Safe**: Enhanced type system with better error handling patterns
//! - **📈 Professional Quality**: Clippy-optimized and consistently formatted code
//!
//! ## 🎯 Quick Start
//!
//! Add to your `Cargo.toml`:
//!
//! ```toml
//! # For native applications (default)
//! [dependencies]
//! kiteconnect-async-wasm = "1.0.3"
//!
//! # For WASM/browser applications
//! [dependencies]
//! kiteconnect-async-wasm = { version = "1.0.3", features = ["wasm"] }
//!
//! # For development with debugging
//! [dependencies]
//! kiteconnect-async-wasm = { version = "1.0.3", features = ["native", "debug"] }
//! ```
//!
//! ## 🚀 Basic Usage
//!
//! ### Legacy API (Backward Compatible)
//! ```rust,no_run
//! use kiteconnect_async_wasm::connect::KiteConnect;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = KiteConnect::new("api_key", "access_token");
//!     
//!     // Legacy API - returns JsonValue (still works)
//!     let holdings = client.holdings().await?;
//!     println!("Holdings: {:?}", holdings);
//!     
//!     Ok(())
//! }
//! ```
//!
//! ### Enhanced Typed API (Recommended for v1.0.3)
//! ```rust,no_run
//! use kiteconnect_async_wasm::connect::KiteConnect;
//! use kiteconnect_async_wasm::models::prelude::*;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = KiteConnect::new("api_key", "access_token");
//!     
//!     // New typed API with enhanced error handling
//!     let holdings: Vec<Holding> = client.holdings_typed().await?;
//!     let positions: Vec<Position> = client.positions_typed().await?;
//!     
//!     println!("Found {} holdings and {} positions", holdings.len(), positions.len());
//!     
//!     Ok(())
//! }
//! ```
//!
//! ### Enhanced Historical Data API (v1.0.3)
//! ```rust,no_run
//! use kiteconnect_async_wasm::connect::KiteConnect;
//! use kiteconnect_async_wasm::models::market_data::HistoricalDataRequest;
//! use kiteconnect_async_wasm::models::common::Interval;
//! use chrono::NaiveDateTime;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = KiteConnect::new("api_key", "access_token");
//!     
//!     // New structured approach with precise datetime handling
//!     let request = HistoricalDataRequest::new(
//!         738561,  // RELIANCE instrument token
//!         NaiveDateTime::parse_from_str("2023-11-01 09:15:00", "%Y-%m-%d %H:%M:%S")?,
//!         NaiveDateTime::parse_from_str("2023-11-30 15:30:00", "%Y-%m-%d %H:%M:%S")?,
//!         Interval::Day,
//!     ).continuous(false).with_oi(true);
//!     
//!     let historical_data = client.historical_data_typed(request).await?;
//!     println!("Received {} candles", historical_data.candles.len());
//!     
//!     Ok(())
//! }
//! ```
//!
//! ## 🔧 Feature Flags
//!
//! This crate supports multiple feature flags for platform-specific functionality:
//!
//! - **`native`** (default): Enables native platform support with tokio and full CSV parsing
//!   - Includes: `tokio`, `sha2`, `csv` dependencies
//!   - Best for: Desktop applications, servers, CLI tools
//!
//! - **`wasm`**: Enables WebAssembly support with browser APIs
//!   - Includes: `wasm-bindgen`, `web-sys`, `js-sys`, `gloo-utils` dependencies
//!   - Features: Browser-compatible operations with Web APIs
//!   - Best for: Browser applications, web workers
//!
//! - **`debug`**: Enables additional logging and debugging features
//!   - Includes: Enhanced `log` output and debugging utilities
//!   - Best for: Development and troubleshooting
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
//!     let holdings: JsonValue = kiteconnect.holdings().await?;
//!     println!("Holdings: {:?}", holdings);
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Available APIs
//!
//! The library provides access to all KiteConnect REST APIs:
//!
//! ### Authentication
//! - `login_url()` - Generate login URL
//! - `generate_session()` - Create session with request token
//! - `invalidate_session()` - Logout user
//!
//! ### Portfolio
//! - `holdings()` - Get user holdings
//! - `positions()` - Get user positions
//! - `margins()` - Get account margins
//!
//! ### Orders
//! - `orders()` - Get all orders
//! - `order_trades()` - Get trades for specific order
//! - `trades()` - Get all trades
//!
//! ### Market Data
//! - `instruments()` - Get instrument list
//! - `trigger_range()` - Get trigger range for instruments
//!
//! ### Mutual Funds
//! - `mf_orders()` - Get mutual fund orders
//! - `mf_instruments()` - Get mutual fund instruments
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
//!     Ok(holdings) => println!("Holdings: {:?}", holdings),
//!     Err(e) => eprintln!("Error fetching holdings: {}", e),
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## Platform-Specific Features
//!
//! ### Native (Tokio)
//! - Full CSV parsing for instruments
//! - Complete async/await support
//! - High-performance HTTP client
//!
//! ### WASM (Browser)
//! - All APIs supported with full CSV parsing
//! - CSV parsing using csv-core for browser compatibility
//! - Returns structured JSON data (same as native)
//! - Compatible with web frameworks
//!
//! ## Examples
//!
//! See the `examples/` directory for comprehensive usage examples:
//! - `connect_sample.rs` - Basic API usage
//! - `async_connect_example.rs` - Advanced async patterns
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

pub mod connect;
pub mod models;
