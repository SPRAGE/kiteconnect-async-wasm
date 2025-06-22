//! # KiteConnect API Client
//! 
//! This module provides the main [`KiteConnect`] struct and associated methods for
//! interacting with the Zerodha KiteConnect REST API.
//! 
//! ## Overview
//! 
//! The KiteConnect API allows you to build trading applications and manage portfolios
//! programmatically. This module provides async methods for all supported endpoints.
//! 
//! ## Authentication Flow
//! 
//! 1. **Get Login URL**: Use [`KiteConnect::login_url`] to generate a login URL
//! 2. **User Login**: Direct user to the URL to complete login
//! 3. **Generate Session**: Use [`KiteConnect::generate_session`] with the request token
//! 4. **API Access**: Use any API method with the authenticated client
//! 
//! ## Example Usage
//! 
//! ```rust,no_run
//! use kiteconnect_async_wasm::connect::KiteConnect;
//! 
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let mut client = KiteConnect::new("your_api_key", "");
//! 
//! // Authentication
//! let login_url = client.login_url();
//! // ... user completes login and returns request_token ...
//! 
//! let session = client.generate_session("request_token", "api_secret").await?;
//! 
//! // Portfolio operations
//! let holdings = client.holdings().await?;
//! let positions = client.positions().await?;
//! let margins = client.margins(None).await?;
//! 
//! // Order operations  
//! let orders = client.orders().await?;
//! let trades = client.trades().await?;
//! 
//! // Market data
//! let instruments = client.instruments(None).await?;
//! # Ok(())
//! # }
//! ```

mod client;
mod portfolio;
mod orders;
mod market;
mod mutual_funds;
mod gtt;
mod request;
mod utils;

// Re-export the main KiteConnect struct and common types
pub use client::KiteConnect;
pub use request::RequestHandler;

// Re-export platform-specific utility functions for external use if needed
#[cfg(all(feature = "wasm", target_arch = "wasm32"))]
pub use utils::{parse_csv_with_core, parse_csv_to_instruments};
