//! kiteconnect//! 
//! # Basic Operation
//! 
//! kiteconnect-rs provides async HTTP REST API access to KiteConnect APIs
//! 
//! 
//! # Async HTTP APIa rust implementation of the KiteConnect library with async support.
//! 
//! The crate is called `kiteconnect` and you can depend of it via cargo:
//! 
//! ```ini
//! [dependencies.kiteconnect]
//! version = "*"
//! ```
//! 
//! If you want to use the git version:
//! 
//! ```ini
//! [dependencies.kiteconnect]
//! git = "https://github.com/zerodhatech/kiteconnect-rs"
//! ```
//! 
//! # Basic Operation
//! 
//! kiteconnect-rs is a Rust implementation of the KiteConnect REST API with async support
//! 
//! 
//! # Async HTTP API
//! 
//! 
//! 
//! ```rust,no_run
//! # extern crate kiteconnect;
//! extern crate serde_json as json;
//! 
//! use kiteconnect::connect::KiteConnect;
//! 
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let mut kiteconnect = KiteConnect::new("<API-KEY>", "");
//! 
//!     // Open browser with this URL and get the request token from the callback
//!     let loginurl = kiteconnect.login_url();
//!     println!("{:?}", loginurl);
//! 
//!     // Generate access token with the above request token
//!     let resp = kiteconnect.generate_session("<REQUEST-TOKEN>", "<API-SECRET>").await?;
//!     // `generate_session` internally sets the access token from the response
//!     println!("{:?}", resp);
//! 
//!     let holdings: json::Value = kiteconnect.holdings().await?;
//!     println!("{:?}", holdings);
//! 
//!     Ok(())
//! }
//! ```
//!
#[cfg(test)]
extern crate mockito;

pub mod connect;
