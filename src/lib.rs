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
//! ## Quick Start
//! 
//! Add to your `Cargo.toml`:
//! 
//! ```toml
//! [dependencies]
//! kiteconnect-async-wasm = { version = "0.1.0", features = ["native"] }
//! 
//! # For WASM targets
//! # kiteconnect-async-wasm = { version = "0.1.0", features = ["wasm"] }
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
//! - All APIs supported
//! - Raw CSV returned for client-side parsing
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
