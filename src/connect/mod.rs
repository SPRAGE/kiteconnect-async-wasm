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
use std::sync::{Arc, atomic::AtomicU64};
use std::time::Duration;
use serde::de::DeserializeOwned;

// Import our typed models
use crate::models::common::{KiteError, KiteResult};

// Cache imports
use std::sync::Mutex;
use std::time::{SystemTime, Duration as StdDuration};

// WASM platform imports  
#[cfg(all(feature = "wasm", target_arch = "wasm32"))]
use web_sys::console;

// Import sub-modules
pub mod utils;
pub mod auth;
pub mod portfolio;
pub mod orders;
pub mod market_data;
pub mod mutual_funds;
pub mod gtt;
pub mod endpoints;
pub mod rate_limiter;

// Re-export commonly used utilities
pub use utils::{RequestHandler, URL};
pub use endpoints::{KiteEndpoint, HttpMethod, RateLimitCategory, Endpoint};
pub use rate_limiter::{RateLimiter, RateLimiterStats, CategoryStats};

/// Configuration for retry behavior
#[derive(Debug, Clone)]
pub struct RetryConfig {
    pub max_retries: u32,
    pub base_delay: Duration,
    pub max_delay: Duration,
    pub exponential_backoff: bool,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            base_delay: Duration::from_millis(200),
            max_delay: Duration::from_secs(5),
            exponential_backoff: true,
        }
    }
}

/// Configuration for response caching
#[derive(Debug, Clone)]
pub struct CacheConfig {
    pub enable_instruments_cache: bool,
    pub cache_ttl_minutes: u64,
    pub max_cache_size: usize,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            enable_instruments_cache: true,
            cache_ttl_minutes: 60, // 1 hour
            max_cache_size: 1000,
        }
    }
}

/// Simple in-memory cache for API responses
#[derive(Debug)]
struct ResponseCache {
    instruments_cache: Option<(JsonValue, SystemTime)>,
    ttl_minutes: u64,
}

impl ResponseCache {
    fn new(ttl_minutes: u64) -> Self {
        Self {
            instruments_cache: None,
            ttl_minutes,
        }
    }

    fn get_instruments(&self) -> Option<JsonValue> {
        if let Some((data, timestamp)) = &self.instruments_cache {
            let elapsed = timestamp.elapsed().ok()?;
            if elapsed < StdDuration::from_secs(self.ttl_minutes * 60) {
                return Some(data.clone());
            }
        }
        None
    }

    fn set_instruments(&mut self, data: JsonValue) {
        self.instruments_cache = Some((data, SystemTime::now()));
    }
}

/// Configuration for KiteConnect client
#[derive(Debug, Clone)]
pub struct KiteConnectConfig {
    pub base_url: String,
    pub timeout: u64,
    pub retry_config: RetryConfig,
    pub cache_config: Option<CacheConfig>,
    pub max_idle_connections: usize,
    pub idle_timeout: u64,
    pub enable_rate_limiting: bool,
}

impl Default for KiteConnectConfig {
    fn default() -> Self {
        Self {
            base_url: "https://api.kite.trade".to_string(),
            timeout: 30,
            retry_config: RetryConfig::default(),
            cache_config: Some(CacheConfig::default()),
            max_idle_connections: 10,
            idle_timeout: 30,
            enable_rate_limiting: true,
        }
    }
}

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
    /// Base URL for API requests
    pub(crate) root: String,
    /// Request timeout in seconds
    pub(crate) timeout: u64,
    /// Optional callback for session expiry handling
    pub(crate) session_expiry_hook: Option<fn() -> ()>,
    /// HTTP client for making requests (shared and reusable)
    pub(crate) client: reqwest::Client,
    
    // New fields for v1.0.0
    /// Retry configuration for failed requests
    pub(crate) retry_config: RetryConfig,
    /// Cache configuration for response caching
    pub(crate) cache_config: Option<CacheConfig>,
    /// Request counter for debugging and monitoring
    pub(crate) request_counter: Arc<AtomicU64>,
    /// Response cache for performance optimization
    pub(crate) response_cache: Arc<Mutex<Option<ResponseCache>>>,
    /// Rate limiter for API compliance
    pub(crate) rate_limiter: rate_limiter::RateLimiter,
}

impl Default for KiteConnect {
    fn default() -> Self {
        KiteConnect {
            api_key: "<API-KEY>".to_string(),
            access_token: "<ACCESS-TOKEN>".to_string(),
            root: URL.to_string(),
            timeout: 30,
            session_expiry_hook: None,
            client: reqwest::Client::new(),
            retry_config: RetryConfig::default(),
            cache_config: Some(CacheConfig::default()),
            request_counter: Arc::new(AtomicU64::new(0)),
            response_cache: Arc::new(Mutex::new(None)),
            rate_limiter: rate_limiter::RateLimiter::new(true),
        }
    }
}

impl KiteConnect {
    /// Constructs url for the given path and query params
    pub(crate) fn build_url(&self, path: &str, param: Option<Vec<(&str, &str)>>) -> reqwest::Url {
        let url: &str = &format!("{}/{}", self.root, &path[1..]);
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
            root: URL.to_string(),
            timeout: 30,
            session_expiry_hook: None,
            client: reqwest::Client::new(),
            retry_config: RetryConfig::default(),
            cache_config: Some(CacheConfig::default()),
            request_counter: Arc::new(AtomicU64::new(0)),
            response_cache: Arc::new(Mutex::new(None)),
            rate_limiter: rate_limiter::RateLimiter::new(true),
        }
    }

    /// Creates a new KiteConnect client with custom configuration
    /// 
    /// # Arguments
    /// 
    /// * `api_key` - Your KiteConnect API key
    /// * `config` - Configuration for the client
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use kiteconnect_async_wasm::connect::{KiteConnect, KiteConnectConfig, RetryConfig};
    /// use std::time::Duration;
    /// 
    /// let config = KiteConnectConfig {
    ///     retry_config: RetryConfig {
    ///         max_retries: 5,
    ///         base_delay: Duration::from_millis(100),
    ///         ..Default::default()
    ///     },
    ///     ..Default::default()
    /// };
    /// 
    /// let mut client = KiteConnect::new_with_config("your_api_key", config);
    /// client.set_access_token("your_access_token");
    /// ```
    pub fn new_with_config(api_key: &str, config: KiteConnectConfig) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(config.timeout))
            .pool_max_idle_per_host(config.max_idle_connections)
            .pool_idle_timeout(Duration::from_secs(config.idle_timeout))
            .user_agent(format!("kiteconnect-rust/{}", env!("CARGO_PKG_VERSION")))
            .build()
            .expect("Failed to create HTTP client");
            
        Self {
            api_key: api_key.to_string(),
            access_token: String::new(),
            root: config.base_url,
            timeout: config.timeout,
            session_expiry_hook: None,
            client,
            retry_config: config.retry_config,
            cache_config: config.cache_config.clone(),
            request_counter: Arc::new(AtomicU64::new(0)),
            response_cache: Arc::new(Mutex::new(
                config.cache_config.as_ref()
                    .map(|c| ResponseCache::new(c.cache_ttl_minutes))
            )),
            rate_limiter: rate_limiter::RateLimiter::new(config.enable_rate_limiting),
        }
    }

    /// Helper method to raise or return json response for async responses
    pub(crate) async fn raise_or_return_json(&self, resp: reqwest::Response) -> Result<JsonValue> {
        if resp.status().is_success() {
            let jsn: JsonValue = resp.json().await.with_context(|| "Serialization failed")?;
            Ok(jsn)
        } else {
            let status_code = resp.status().as_u16();
            let status = status_code.to_string();
            let error_text = resp.text().await?;
            
            // Try to parse as JSON to extract error details
            if let Ok(error_json) = serde_json::from_str::<JsonValue>(&error_text) {
                let message = error_json["message"].as_str()
                    .unwrap_or(&error_text)
                    .to_string();
                let error_type = error_json["error_type"].as_str()
                    .map(|s| s.to_string());
                
                let kite_error = KiteError::from_api_response(status_code, status, message, error_type);
                Err(anyhow::Error::new(kite_error))
            } else {
                let kite_error = KiteError::from_api_response(status_code, status, error_text, None);
                Err(anyhow::Error::new(kite_error))
            }
        }
    }

    /// Send request with retry logic and enhanced error handling
    pub(crate) async fn send_request_with_retry(
        &self,
        url: reqwest::Url,
        method: &str,
        data: Option<HashMap<&str, &str>>,
    ) -> KiteResult<reqwest::Response> {
        let mut last_error = None;
        
        for attempt in 0..=self.retry_config.max_retries {
            // Increment request counter
            self.request_counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            
            match self.send_request(url.clone(), method, data.clone()).await {
                Ok(response) => {
                    // Check if response indicates an error that should be retried
                    if response.status().is_server_error() || response.status() == 429 {
                        let status = response.status().as_u16().to_string();
                        let error_text = response.text().await
                            .unwrap_or_else(|_| "Unknown server error".to_string());
                        
                        let error = KiteError::Api {
                            status,
                            message: error_text,
                            error_type: Some("ServerError".to_string()),
                        };
                        
                        if attempt < self.retry_config.max_retries && self.should_retry(&error) {
                            last_error = Some(error);
                            let delay = self.calculate_retry_delay(attempt);
                            
                            #[cfg(feature = "debug")]
                            log::debug!("Request failed, retrying in {:?}. Attempt {}/{}", 
                                delay, attempt + 1, self.retry_config.max_retries);
                            
                            tokio::time::sleep(delay).await;
                            continue;
                        } else {
                            return Err(error);
                        }
                    }
                    
                    return Ok(response);
                }
                Err(e) => {
                    let kite_error = KiteError::Legacy(e);
                    
                    if attempt < self.retry_config.max_retries && self.should_retry(&kite_error) {
                        last_error = Some(kite_error);
                        let delay = self.calculate_retry_delay(attempt);
                        
                        #[cfg(feature = "debug")]
                        log::debug!("Request failed, retrying in {:?}. Attempt {}/{}", 
                            delay, attempt + 1, self.retry_config.max_retries);
                        
                        tokio::time::sleep(delay).await;
                        continue;
                    } else {
                        return Err(kite_error);
                    }
                }
            }
        }
        
        // If we've exhausted all retries, return the last error
        Err(last_error.unwrap_or_else(|| KiteError::General("All retry attempts failed".to_string())))
    }

    /// Enhanced JSON response handler with better error handling
    pub(crate) async fn raise_or_return_json_typed(&self, resp: reqwest::Response) -> KiteResult<JsonValue> {
        if resp.status().is_success() {
            resp.json().await.map_err(KiteError::Http)
        } else {
            let status_code = resp.status().as_u16();
            let status = status_code.to_string();
            let error_text = resp.text().await
                .map_err(KiteError::Http)?;
            
            // Try to parse as JSON to extract error details
            if let Ok(error_json) = serde_json::from_str::<JsonValue>(&error_text) {
                let message = error_json["message"].as_str()
                    .unwrap_or(&error_text)
                    .to_string();
                let error_type = error_json["error_type"].as_str()
                    .map(|s| s.to_string());
                
                Err(KiteError::from_api_response(status_code, status, message, error_type))
            } else {
                Err(KiteError::from_api_response(status_code, status, error_text, None))
            }
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

    /// Internal helper method for parsing JSON responses to typed models
    /// 
    /// This method converts JsonValue responses from legacy API methods
    /// into strongly typed model structs for the new typed API methods.
    fn parse_response<T: DeserializeOwned>(&self, response: JsonValue) -> KiteResult<T> {
        serde_json::from_value(response)
            .map_err(KiteError::Json)
    }

    /// Determines if a request should be retried based on the error type
    fn should_retry(&self, error: &KiteError) -> bool {
        error.is_retryable()
    }

    /// Calculates retry delay using exponential backoff or fixed delay
    fn calculate_retry_delay(&self, attempt: u32) -> Duration {
        if self.retry_config.exponential_backoff {
            let delay = self.retry_config.base_delay * 2_u32.pow(attempt);
            std::cmp::min(delay, self.retry_config.max_delay)
        } else {
            self.retry_config.base_delay
        }
    }

    /// Gets the current request count for monitoring
    pub fn request_count(&self) -> u64 {
        self.request_counter.load(std::sync::atomic::Ordering::Relaxed)
    }

    /// Get rate limiter statistics for monitoring
    pub async fn rate_limiter_stats(&self) -> rate_limiter::RateLimiterStats {
        self.rate_limiter.get_stats().await
    }

    /// Enable or disable rate limiting
    pub fn set_rate_limiting_enabled(&mut self, enabled: bool) {
        self.rate_limiter.set_enabled(enabled);
    }

    /// Check if rate limiting is enabled
    pub fn is_rate_limiting_enabled(&self) -> bool {
        self.rate_limiter.is_enabled()
    }

    /// Check if a request can be made without waiting
    pub async fn can_request_immediately(&self, endpoint: &KiteEndpoint) -> bool {
        self.rate_limiter.can_request_immediately(endpoint).await
    }

    /// Get the delay required before making a request
    pub async fn get_delay_for_request(&self, endpoint: &KiteEndpoint) -> std::time::Duration {
        self.rate_limiter.get_delay_for_request(endpoint).await
    }

    /// Wait for rate limit compliance before making a request
    pub async fn wait_for_request(&self, endpoint: &KiteEndpoint) {
        self.rate_limiter.wait_for_request(endpoint).await
    }

    /// Send request with rate limiting and retry logic
    async fn send_request_with_rate_limiting_and_retry(
        &self,
        endpoint: KiteEndpoint,
        path_segments: &[&str],
        query_params: Option<Vec<(&str, &str)>>,
        data: Option<HashMap<&str, &str>>,
    ) -> KiteResult<reqwest::Response> {
        // Apply rate limiting
        self.rate_limiter.wait_for_request(&endpoint).await;

        // Build URL with endpoint configuration
        let config = endpoint.config();
        let full_path = if path_segments.is_empty() {
            config.path.to_string()
        } else {
            format!("{}/{}", config.path, path_segments.join("/"))
        };
        
        let url = self.build_url(&full_path, query_params);
        
        // Use existing retry logic
        self.send_request_with_retry(url, config.method.as_str(), data).await
    }
}

/// Implement the async request handler for KiteConnect struct
impl RequestHandler for KiteConnect {
    fn send_request(
        &self,
        url: reqwest::Url,
        method: &str,
        data: Option<HashMap<&str, &str>>,
    ) -> impl std::future::Future<Output = Result<reqwest::Response>> + Send {
        async move {
            #[cfg(feature = "debug")]
            log::debug!("Sending {} request to: {}", method, url);
            
            #[cfg(all(feature = "debug", feature = "wasm", target_arch = "wasm32"))]
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_build_url() {
        let kiteconnect = KiteConnect::new("key", "token");
        let url = kiteconnect.build_url("/my-holdings", None);
        assert_eq!(url.as_str(), format!("{}/my-holdings", URL).as_str());

        let params: Vec<(&str, &str)> = vec![("one", "1")];
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
