//! # KiteConnect API Client Module
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

use serde_json::Value as JsonValue;
use anyhow::{anyhow, Context, Result};
use std::collections::HashMap;
use reqwest::header::{HeaderMap, AUTHORIZATION, USER_AGENT};

// Import sub-modules
pub mod utils;
pub mod auth;
pub mod portfolio;
pub mod orders;
pub mod market_data;
pub mod mutual_funds;

// Re-export commonly used utilities
pub use utils::{RequestHandler, URL};

/// Main client for interacting with the KiteConnect API
/// 
/// This struct provides async methods for all KiteConnect REST API endpoints.
/// It handles authentication, request formatting, and response parsing automatically.
/// 
/// ## Thread Safety
/// 
/// `KiteConnect` implements `Clone + Send + Sync`, making it safe to use across
/// multiple threads and async tasks. The underlying HTTP client uses connection
/// pooling for optimal performance.
/// 
/// ## Example
/// 
/// ```rust,no_run
/// use kiteconnect_async_wasm::connect::KiteConnect;
/// 
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// // Create a new client
/// let mut client = KiteConnect::new("your_api_key", "");
/// 
/// // Set access token (usually done via generate_session)
/// client.set_access_token("your_access_token");
/// 
/// // Use the API
/// let holdings = client.holdings().await?;
/// println!("Holdings: {:?}", holdings);
/// # Ok(())
/// # }
/// ```
/// 
/// ## Cloning for Concurrent Use
/// 
/// ```rust,no_run
/// use kiteconnect_async_wasm::connect::KiteConnect;
/// 
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let client = KiteConnect::new("api_key", "access_token");
/// 
/// // Clone for use in different tasks
/// let client1 = client.clone();
/// let client2 = client.clone();
/// 
/// // Fetch data concurrently
/// let (holdings, positions) = tokio::try_join!(
///     client1.holdings(),
///     client2.positions()
/// )?;
/// # Ok(())
/// # }
/// ```
#[derive(Clone, Debug)]
pub struct KiteConnect {
    /// API key for authentication
    pub(crate) api_key: String,
    /// Access token for authenticated requests
    pub(crate) access_token: String,
    /// Optional callback for session expiry handling
    pub(crate) session_expiry_hook: Option<fn() -> ()>,
    /// HTTP client for making requests (shared and reusable)
    pub(crate) client: reqwest::Client,
}

impl Default for KiteConnect {
    fn default() -> Self {
        KiteConnect {
            api_key: "<API-KEY>".to_string(),
            access_token: "<ACCESS-TOKEN>".to_string(),
            session_expiry_hook: None,
            client: reqwest::Client::new(),
        }
    }
}

impl KiteConnect {
    /// Constructs url for the given path and query params
    pub(crate) fn build_url(&self, path: &str, param: Option<Vec<(&str, &str)>>) -> reqwest::Url {
        let url: &str = &format!("{}/{}", URL, &path[1..]);
        let mut url = reqwest::Url::parse(url).unwrap();

        if let Some(data) = param {
            url.query_pairs_mut().extend_pairs(data.iter());
        }
        url
    }

    /// Creates a new KiteConnect client instance
    /// 
    /// # Arguments
    /// 
    /// * `api_key` - Your KiteConnect API key
    /// * `access_token` - Access token (can be empty string if using `generate_session`)
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use kiteconnect_async_wasm::connect::KiteConnect;
    /// 
    /// // Create client for authentication flow
    /// let mut client = KiteConnect::new("your_api_key", "");
    /// 
    /// // Or create with existing access token
    /// let client = KiteConnect::new("your_api_key", "your_access_token");
    /// ```
    pub fn new(api_key: &str, access_token: &str) -> Self {
        Self {
            api_key: api_key.to_string(),
            access_token: access_token.to_string(),
            client: reqwest::Client::new(),
            ..Default::default()
        }
    }

    /// Helper method to raise or return json response for async responses
    pub(crate) async fn raise_or_return_json(&self, resp: reqwest::Response) -> Result<JsonValue> {
        if resp.status().is_success() {
            let jsn: JsonValue = resp.json().await.with_context(|| "Serialization failed")?;
            Ok(jsn)
        } else {
            let error_text = resp.text().await?;
            Err(anyhow!(error_text))
        }
    }

    /// Sets a session expiry callback hook for this instance
    /// 
    /// This hook will be called when a session expires, allowing you to handle
    /// re-authentication or cleanup logic.
    /// 
    /// # Arguments
    /// 
    /// * `method` - Callback function to execute on session expiry
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use kiteconnect_async_wasm::connect::KiteConnect;
    /// 
    /// fn handle_session_expiry() {
    ///     println!("Session expired! Please re-authenticate.");
    /// }
    /// 
    /// let mut client = KiteConnect::new("api_key", "access_token");
    /// client.set_session_expiry_hook(handle_session_expiry);
    /// ```
    pub fn set_session_expiry_hook(&mut self, method: fn() -> ()) {
        self.session_expiry_hook = Some(method);
    }

    /// Gets the current session expiry hook
    /// 
    /// Returns the session expiry callback function if one has been set.
    /// 
    /// # Returns
    /// 
    /// `Option<fn() -> ()>` - The callback function, or `None` if not set
    pub fn session_expiry_hook(&self) -> Option<fn() -> ()> {
        self.session_expiry_hook
    }

    /// Sets the access token for authenticated API requests
    /// 
    /// This is typically called automatically by `generate_session`, but can
    /// be used manually if you have a pre-existing access token.
    /// 
    /// # Arguments
    /// 
    /// * `access_token` - The access token string
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use kiteconnect_async_wasm::connect::KiteConnect;
    /// 
    /// let mut client = KiteConnect::new("api_key", "");
    /// client.set_access_token("your_access_token");
    /// ```
    pub fn set_access_token(&mut self, access_token: &str) {
        self.access_token = access_token.to_string();
    }

    /// Gets the access token for this instance
    pub fn access_token(&self) -> &str {
        &self.access_token
    }
}

/// Implement the async request handler for KiteConnect struct
impl RequestHandler for KiteConnect {
    async fn send_request(
        &self,
        url: reqwest::Url,
        method: &str,
        data: Option<HashMap<&str, &str>>,
    ) -> Result<reqwest::Response> {
        #[cfg(feature = "debug")]
        log::debug!("Sending {} request to: {}", method, url);
        
        #[cfg(all(feature = "debug", feature = "wasm"))]
        console::log_1(&format!("KiteConnect: {} {}", method, url).into());

        let mut headers = HeaderMap::new();
        headers.insert("XKiteVersion", "3".parse().unwrap());
        headers.insert(
            AUTHORIZATION,
            format!("token {}:{}", self.api_key, self.access_token)
                .parse()
                .unwrap(),
        );
        headers.insert(USER_AGENT, "Rust".parse().unwrap());

        let response = match method {
            "GET" => self.client.get(url).headers(headers).send().await?,
            "POST" => self.client.post(url).headers(headers).form(&data).send().await?,
            "DELETE" => self.client.delete(url).headers(headers).json(&data).send().await?,
            "PUT" => self.client.put(url).headers(headers).form(&data).send().await?,
            _ => return Err(anyhow!("Unknown method!")),
        };

        #[cfg(feature = "debug")]
        log::debug!("Response status: {}", response.status());

        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::{Server, Matcher};

    #[tokio::test]
    async fn test_build_url() {
        let kiteconnect = KiteConnect::new("key", "token");
        let url = kiteconnect.build_url("/my-holdings", None);
        assert_eq!(url.as_str(), format!("{}/my-holdings", URL).as_str());

        let mut params: Vec<(&str, &str)> = Vec::new();
        params.push(("one", "1"));
        let url = kiteconnect.build_url("/my-holdings", Some(params));
        assert_eq!(url.as_str(), format!("{}/my-holdings?one=1", URL).as_str());
    }

    #[tokio::test]
    async fn test_set_access_token() {
        let mut kiteconnect = KiteConnect::new("key", "token");
        assert_eq!(kiteconnect.access_token(), "token");
        kiteconnect.set_access_token("my_token");
        assert_eq!(kiteconnect.access_token(), "my_token");
    }

    #[tokio::test]
    async fn test_session_expiry_hook() {
        let mut kiteconnect = KiteConnect::new("key", "token");
        assert_eq!(kiteconnect.session_expiry_hook(), None);

        fn mock_hook() { 
            println!("Session expired");
        }

        kiteconnect.set_session_expiry_hook(mock_hook);
        assert_ne!(kiteconnect.session_expiry_hook(), None);
    }

    #[tokio::test]
    async fn test_login_url() {
        let kiteconnect = KiteConnect::new("key", "token");
        assert_eq!(kiteconnect.login_url(), "https://kite.trade/connect/login?api_key=key&v3");
    }

    // Test implementations for the various modules can be added here
    // For now, keeping it minimal to focus on the module structure
}
