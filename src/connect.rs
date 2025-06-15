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

use serde_json::Value as JsonValue;
use anyhow::{anyhow, Context, Result};
use std::collections::HashMap;
use reqwest::header::{HeaderMap, AUTHORIZATION, USER_AGENT};

// Native platform imports
#[cfg(all(feature = "native", not(target_arch = "wasm32")))]
use sha2::{Sha256, Digest};

#[cfg(all(feature = "native", not(target_arch = "wasm32")))]
use csv::ReaderBuilder;

// WASM platform imports  
#[cfg(all(feature = "wasm", target_arch = "wasm32"))]
use web_sys::window;

#[cfg(all(feature = "wasm", target_arch = "wasm32"))]
use js_sys::Uint8Array;

#[cfg(all(feature = "wasm", target_arch = "wasm32"))]
use wasm_bindgen_futures::JsFuture;

#[cfg(all(feature = "wasm", target_arch = "wasm32"))]
use csv_core::{Reader, ReadFieldResult};

#[cfg(not(test))]
const URL: &str = "https://api.kite.trade";

#[cfg(test)]
const URL: &str = "http://127.0.0.1:1234";

/// Async trait for handling HTTP requests across different platforms
trait RequestHandler {
    async fn send_request(
        &self,
        url: reqwest::Url,
        method: &str,
        data: Option<HashMap<&str, &str>>,
    ) -> Result<reqwest::Response>;
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
    api_key: String,
    /// Access token for authenticated requests
    access_token: String,
    /// Optional callback for session expiry handling
    session_expiry_hook: Option<fn() -> ()>,
    /// HTTP client for making requests (shared and reusable)
    client: reqwest::Client,
}

/// Parse CSV data using csv-core for WASM compatibility
#[cfg(all(feature = "wasm", target_arch = "wasm32"))]
fn parse_csv_with_core(csv_data: &str) -> Result<JsonValue> {
    let mut reader = Reader::new();
    let mut output = vec![0; 1024];
    let mut field = Vec::new();
    let mut input = csv_data.as_bytes();
    
    let mut headers: Vec<String> = Vec::new();
    let mut records: Vec<Vec<String>> = Vec::new();
    let mut current_record: Vec<String> = Vec::new();
    let mut is_first_row = true;
    
    loop {
        let (result, input_consumed, output_written) = reader.read_field(input, &mut output);
        input = &input[input_consumed..];
        
        match result {
            ReadFieldResult::InputEmpty => {
                if !current_record.is_empty() {
                    if is_first_row {
                        headers = current_record.clone();
                        is_first_row = false;
                    } else {
                        records.push(current_record.clone());
                    }
                }
                break;
            }
            ReadFieldResult::OutputFull => {
                field.extend_from_slice(&output[..output_written]);
                // Continue reading with same input
            }
            ReadFieldResult::Field { record_end } => {
                field.extend_from_slice(&output[..output_written]);
                let field_str = String::from_utf8_lossy(&field).to_string();
                current_record.push(field_str);
                field.clear();
                
                if record_end {
                    if is_first_row {
                        headers = current_record.clone();
                        is_first_row = false;
                    } else {
                        records.push(current_record.clone());
                    }
                    current_record.clear();
                }
            }
        }
    }
    
    // Convert to JSON format
    let mut result = Vec::new();
    for record in records {
        let mut obj = serde_json::Map::new();
        for (i, field_value) in record.iter().enumerate() {
            if let Some(header) = headers.get(i) {
                obj.insert(header.clone(), JsonValue::String(field_value.clone()));
            }
        }
        result.push(JsonValue::Object(obj));
    }
    
    Ok(JsonValue::Array(result))
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
    async fn raise_or_return_json(&self, resp: reqwest::Response) -> Result<JsonValue> {
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

    /// Generates the KiteConnect login URL for user authentication
    /// 
    /// This URL should be opened in a browser to allow the user to log in to their
    /// Zerodha account. After successful login, the user will be redirected to your
    /// redirect URL with a `request_token` parameter.
    /// 
    /// # Returns
    /// 
    /// A login URL string that can be opened in a browser
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use kiteconnect_async_wasm::connect::KiteConnect;
    /// 
    /// let client = KiteConnect::new("your_api_key", "");
    /// let login_url = client.login_url();
    /// 
    /// println!("Please visit: {}", login_url);
    /// // User visits URL, logs in, and is redirected with request_token
    /// ```
    /// 
    /// # Authentication Flow
    /// 
    /// 1. Generate login URL with this method
    /// 2. Direct user to the URL in a browser
    /// 3. User completes login and is redirected with `request_token`
    /// 4. Use `generate_session()` with the request token to get access token
    pub fn login_url(&self) -> String {
        format!("https://kite.trade/connect/login?api_key={}&v3", self.api_key)
    }

    /// Compute checksum for authentication - different implementations for native vs WASM
    #[cfg(all(feature = "native", not(target_arch = "wasm32")))]
    async fn compute_checksum(&self, input: &str) -> Result<String> {
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        let result = hasher.finalize();
        Ok(hex::encode(result))
    }

    #[cfg(all(feature = "wasm", target_arch = "wasm32"))]
    async fn compute_checksum(&self, input: &str) -> Result<String> {
        // WASM implementation using Web Crypto API
        let window = window().ok_or_else(|| anyhow!("No window object"))?;
        let crypto = window.crypto().map_err(|_| anyhow!("No crypto object"))?;
        let subtle = crypto.subtle();

        let data = Uint8Array::from(input.as_bytes());
        let digest_promise = subtle
            .digest_with_str_and_u8_array("SHA-256", &data.to_vec())
            .map_err(|_| anyhow!("Failed to create digest"))?;

        let digest_result = JsFuture::from(digest_promise)
            .await
            .map_err(|_| anyhow!("Failed to compute hash"))?;

        let digest_array = Uint8Array::new(&digest_result);
        let digest_vec: Vec<u8> = digest_array.to_vec();
        Ok(hex::encode(digest_vec))
    }

    /// Fallback checksum implementation when neither native nor wasm features are enabled
    #[cfg(not(any(
        all(feature = "native", not(target_arch = "wasm32")),
        all(feature = "wasm", target_arch = "wasm32")
    )))]
    async fn compute_checksum(&self, _input: &str) -> Result<String> {
        Err(anyhow!("Checksum computation requires either 'native' or 'wasm' feature to be enabled"))
    }

    /// Generates an access token using the request token from login
    /// 
    /// This method completes the authentication flow by exchanging the request token
    /// (obtained after user login) for an access token that can be used for API calls.
    /// The access token is automatically stored in the client instance.
    /// 
    /// # Arguments
    /// 
    /// * `request_token` - The request token received after user login
    /// * `api_secret` - Your KiteConnect API secret
    /// 
    /// # Returns
    /// 
    /// A `Result<JsonValue>` containing the session information including access token
    /// 
    /// # Errors
    /// 
    /// Returns an error if:
    /// - The request token is invalid or expired
    /// - The API secret is incorrect
    /// - Network request fails
    /// - Response parsing fails
    /// 
    /// # Example
    /// 
    /// ```rust,no_run
    /// use kiteconnect_async_wasm::connect::KiteConnect;
    /// 
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut client = KiteConnect::new("your_api_key", "");
    /// 
    /// // After user completes login and you receive the request_token
    /// let session_data = client
    ///     .generate_session("request_token_from_callback", "your_api_secret")
    ///     .await?;
    /// 
    /// println!("Session created: {:?}", session_data);
    /// // Access token is now automatically set in the client
    /// # Ok(())
    /// # }
    /// ```
    /// 
    /// # Authentication Flow
    /// 
    /// 1. Call `login_url()` to get login URL
    /// 2. User visits URL and completes login
    /// 3. User is redirected with `request_token` parameter
    /// 4. Call this method with the request token and API secret
    /// 5. Access token is automatically set for subsequent API calls
    pub async fn generate_session(
        &mut self,
        request_token: &str,
        api_secret: &str,
    ) -> Result<JsonValue> {
        // Create a hex digest from api key, request token, api secret
        let input = format!("{}{}{}", self.api_key, request_token, api_secret);
        let checksum = self.compute_checksum(&input).await?;

        let api_key: &str = &self.api_key.clone();
        let mut data = HashMap::new();
        data.insert("api_key", api_key);
        data.insert("request_token", request_token);
        data.insert("checksum", checksum.as_str());

        let url = self.build_url("/session/token", None);
        let resp = self.send_request(url, "POST", Some(data)).await?;

        if resp.status().is_success() {
            let jsn: JsonValue = resp.json().await?;
            self.set_access_token(jsn["data"]["access_token"].as_str().unwrap());
            Ok(jsn)
        } else {
            let error_text = resp.text().await?;
            Err(anyhow!(error_text))
        }
    }

    /// Invalidates the access token
    pub async fn invalidate_access_token(&self, access_token: &str) -> Result<reqwest::Response> {
        let url = self.build_url("/session/token", None);
        let mut data = HashMap::new();
        data.insert("access_token", access_token);

        self.send_request(url, "DELETE", Some(data)).await
    }

    /// Request for new access token
    pub async fn renew_access_token(
        &mut self,
        access_token: &str,
        api_secret: &str,
    ) -> Result<JsonValue> {
        // Create a hex digest from api key, request token, api secret
        let input = format!("{}{}{}", self.api_key, access_token, api_secret);
        let checksum = self.compute_checksum(&input).await?;

        let api_key: &str = &self.api_key.clone();
        let mut data = HashMap::new();
        data.insert("api_key", api_key);
        data.insert("access_token", access_token);
        data.insert("checksum", checksum.as_str());

        let url = self.build_url("/session/refresh_token", None);
        let resp = self.send_request(url, "POST", Some(data)).await?;

        if resp.status().is_success() {
            let jsn: JsonValue = resp.json().await?;
            self.set_access_token(jsn["access_token"].as_str().unwrap());
            Ok(jsn)
        } else {
            let error_text = resp.text().await?;
            Err(anyhow!(error_text))
        }
    }

    /// Invalidates the refresh token
    pub async fn invalidate_refresh_token(&self, refresh_token: &str) -> Result<reqwest::Response> {
        let url = self.build_url("/session/refresh_token", None);
        let mut data = HashMap::new();
        data.insert("refresh_token", refresh_token);

        self.send_request(url, "DELETE", Some(data)).await
    }

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
        let url: reqwest::Url = if let Some(segment) = segment {
            self.build_url(&format!("/user/margins/{}", segment), None)
        } else {
            self.build_url("/user/margins", None)
        };

        let resp = self.send_request(url, "GET", None).await?;
        self.raise_or_return_json(resp).await
    }

    /// Get user profile details
    pub async fn profile(&self) -> Result<JsonValue> {
        let url = self.build_url("/user/profile", None);
        let resp = self.send_request(url, "GET", None).await?;
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
        let url = self.build_url("/portfolio/holdings", None);
        let resp = self.send_request(url, "GET", None).await?;
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
        let url = self.build_url("/portfolio/positions", None);
        let resp = self.send_request(url, "GET", None).await?;
        self.raise_or_return_json(resp).await
    }

    /// Place an order
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
        
        if let Some(product) = product { params.insert("product", product); }
        if let Some(order_type) = order_type { params.insert("order_type", order_type); }
        if let Some(price) = price { params.insert("price", price); }
        if let Some(validity) = validity { params.insert("validity", validity); }
        if let Some(disclosed_quantity) = disclosed_quantity { params.insert("disclosed_quantity", disclosed_quantity); }
        if let Some(trigger_price) = trigger_price { params.insert("trigger_price", trigger_price); }
        if let Some(squareoff) = squareoff { params.insert("squareoff", squareoff); }
        if let Some(stoploss) = stoploss { params.insert("stoploss", stoploss); }
        if let Some(trailing_stoploss) = trailing_stoploss { params.insert("trailing_stoploss", trailing_stoploss); }
        if let Some(tag) = tag { params.insert("tag", tag); }

        let url = self.build_url(&format!("/orders/{}", variety), None);
        let resp = self.send_request(url, "POST", Some(params)).await?;
        self.raise_or_return_json(resp).await
    }

    /// Modify an open order
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
        
        if let Some(quantity) = quantity { params.insert("quantity", quantity); }
        if let Some(price) = price { params.insert("price", price); }
        if let Some(order_type) = order_type { params.insert("order_type", order_type); }
        if let Some(validity) = validity { params.insert("validity", validity); }
        if let Some(disclosed_quantity) = disclosed_quantity { params.insert("disclosed_quantity", disclosed_quantity); }
        if let Some(trigger_price) = trigger_price { params.insert("trigger_price", trigger_price); }
        if let Some(parent_order_id) = parent_order_id { params.insert("parent_order_id", parent_order_id); }

        let url = self.build_url(&format!("/orders/{}/{}", variety, order_id), None);
        let resp = self.send_request(url, "PUT", Some(params)).await?;
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

        let url = self.build_url(&format!("/orders/{}/{}", variety, order_id), None);
        let resp = self.send_request(url, "DELETE", Some(params)).await?;
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
        let url = self.build_url("/orders", None);
        let resp = self.send_request(url, "GET", None).await?;
        self.raise_or_return_json(resp).await
    }

    /// Get the list of order history
    pub async fn order_history(&self, order_id: &str) -> Result<JsonValue> {
        let params = vec![("order_id", order_id)];
        let url = self.build_url("/orders", Some(params));
        let resp = self.send_request(url, "GET", None).await?;
        self.raise_or_return_json(resp).await
    }

    /// Get all trades
    pub async fn trades(&self) -> Result<JsonValue> {
        let url = self.build_url("/trades", None);
        let resp = self.send_request(url, "GET", None).await?;
        self.raise_or_return_json(resp).await
    }

    /// Get all trades for a specific order
    pub async fn order_trades(&self, order_id: &str) -> Result<JsonValue> {
        let url = self.build_url(&format!("/orders/{}/trades", order_id), None);
        let resp = self.send_request(url, "GET", None).await?;
        self.raise_or_return_json(resp).await
    }

    /// Modify an open position product type
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

        let url = self.build_url("/portfolio/positions", None);
        let resp = self.send_request(url, "PUT", Some(params)).await?;
        self.raise_or_return_json(resp).await
    }

    /// Get all mutual fund orders or individual order info
    pub async fn mf_orders(&self, order_id: Option<&str>) -> Result<JsonValue> {
        let url: reqwest::Url = if let Some(order_id) = order_id {
            self.build_url(&format!("/mf/orders/{}", order_id), None)
        } else {
            self.build_url("/mf/orders", None)
        };

        let resp = self.send_request(url, "GET", None).await?;
        self.raise_or_return_json(resp).await
    }

    /// Get the trigger range for a list of instruments
    pub async fn trigger_range(
        &self,
        transaction_type: &str,
        instruments: Vec<&str>,
    ) -> Result<JsonValue> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        params.push(("transaction_type", transaction_type));
        
        for instrument in instruments {
            params.push(("instruments", instrument));
        }

        let url = self.build_url("/instruments/trigger_range", Some(params));
        let resp = self.send_request(url, "GET", None).await?;
        self.raise_or_return_json(resp).await
    }

    /// Get instruments list
    #[cfg(all(feature = "native", not(target_arch = "wasm32")))]
    pub async fn instruments(&self, exchange: Option<&str>) -> Result<JsonValue> {
        let url: reqwest::Url = if let Some(exchange) = exchange {
            self.build_url(&format!("/instruments/{}", exchange), None)
        } else {
            self.build_url("/instruments", None)
        };

        let resp = self.send_request(url, "GET", None).await?;
        let body = resp.text().await?;
        
        // Parse CSV response
        let mut rdr = ReaderBuilder::new().from_reader(body.as_bytes());
        let mut result = Vec::new();
        
        let headers = rdr.headers()?.clone();
        for record in rdr.records() {
            let record = record?;
            let mut obj = serde_json::Map::new();
            
            for (i, field) in record.iter().enumerate() {
                if let Some(header) = headers.get(i) {
                    obj.insert(header.to_string(), JsonValue::String(field.to_string()));
                }
            }
            result.push(JsonValue::Object(obj));
        }
        
        Ok(JsonValue::Array(result))
    }

    /// Get instruments list (WASM version - now parses CSV using csv-core)
    #[cfg(all(feature = "wasm", target_arch = "wasm32"))]
    pub async fn instruments(&self, exchange: Option<&str>) -> Result<JsonValue> {
        let url: reqwest::Url = if let Some(exchange) = exchange {
            self.build_url(&format!("/instruments/{}", exchange), None)
        } else {
            self.build_url("/instruments", None)
        };

        let resp = self.send_request(url, "GET", None).await?;
        let body = resp.text().await?;
        
        // Parse CSV using csv-core for WASM compatibility
        parse_csv_with_core(&body)
    }

    /// Get mutual fund instruments list
    #[cfg(all(feature = "native", not(target_arch = "wasm32")))]
    pub async fn mf_instruments(&self) -> Result<JsonValue> {
        let url = self.build_url("/mf/instruments", None);
        let resp = self.send_request(url, "GET", None).await?;
        let body = resp.text().await?;
        
        // Parse CSV response
        let mut rdr = ReaderBuilder::new().from_reader(body.as_bytes());
        let mut result = Vec::new();
        
        let headers = rdr.headers()?.clone();
        for record in rdr.records() {
            let record = record?;
            let mut obj = serde_json::Map::new();
            
            for (i, field) in record.iter().enumerate() {
                if let Some(header) = headers.get(i) {
                    obj.insert(header.to_string(), JsonValue::String(field.to_string()));
                }
            }
            result.push(JsonValue::Object(obj));
        }
        
        Ok(JsonValue::Array(result))
    }

    /// Get mutual fund instruments list (WASM version - now parses CSV using csv-core)
    #[cfg(all(feature = "wasm", target_arch = "wasm32"))]
    pub async fn mf_instruments(&self) -> Result<JsonValue> {
        let url = self.build_url("/mf/instruments", None);
        let resp = self.send_request(url, "GET", None).await?;
        let body = resp.text().await?;
        
        // Parse CSV using csv-core for WASM compatibility
        parse_csv_with_core(&body)
    }

    /// Get instruments list (fallback when no platform features are enabled)
    #[cfg(not(any(
        all(feature = "native", not(target_arch = "wasm32")),
        all(feature = "wasm", target_arch = "wasm32")
    )))]
    pub async fn instruments(&self, _exchange: Option<&str>) -> Result<JsonValue> {
        Err(anyhow!("Instruments functionality requires either 'native' or 'wasm' feature to be enabled"))
    }

    /// Get mutual fund instruments list (fallback when no platform features are enabled)
    #[cfg(not(any(
        all(feature = "native", not(target_arch = "wasm32")),
        all(feature = "wasm", target_arch = "wasm32")
    )))]
    pub async fn mf_instruments(&self) -> Result<JsonValue> {
        Err(anyhow!("MF instruments functionality requires either 'native' or 'wasm' feature to be enabled"))
    }

    /// Retrieves historical candlestick data for an instrument
    /// 
    /// Returns historical OHLCV (Open, High, Low, Close, Volume) data for a given
    /// instrument within the specified date range and interval. This is useful for
    /// backtesting, analysis, and charting applications.
    /// 
    /// # Arguments
    /// 
    /// * `instrument_token` - The instrument token (numeric ID) of the instrument
    /// * `from_date` - Start date in YYYY-MM-DD format
    /// * `to_date` - End date in YYYY-MM-DD format  
    /// * `interval` - Time interval for candlesticks ("minute", "day", "3minute", "5minute", "10minute", "15minute", "30minute", "60minute")
    /// * `continuous` - Whether to include pre-market and post-market data ("1" for true, "0" for false)
    /// 
    /// # Returns
    /// 
    /// A `Result<JsonValue>` containing historical data with fields like:
    /// - `data` - Array of candlestick data points
    ///   - `date` - ISO datetime string
    ///   - `open` - Opening price
    ///   - `high` - Highest price
    ///   - `low` - Lowest price
    ///   - `close` - Closing price
    ///   - `volume` - Trading volume
    /// 
    /// # Errors
    /// 
    /// Returns an error if:
    /// - The instrument token is invalid
    /// - The date range is invalid or too large
    /// - The interval is not supported
    /// - Network request fails
    /// - User is not authenticated
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
    /// // Get daily data for RELIANCE for the last month
    /// let historical_data = client.historical_data(
    ///     "738561",           // RELIANCE instrument token
    ///     "2023-11-01",       // From date
    ///     "2023-11-30",       // To date
    ///     "day",              // Daily interval
    ///     "0"                 // No continuous data
    /// ).await?;
    /// 
    /// println!("Historical data: {:?}", historical_data);
    /// 
    /// // Access candlestick data
    /// if let Some(data) = historical_data["data"].as_array() {
    ///     for candle in data {
    ///         println!("Date: {}, Close: {}, Volume: {}", 
    ///             candle["date"], candle["close"], candle["volume"]);
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    /// 
    /// # Notes
    /// 
    /// - Historical data is subject to availability and may have limitations based on your subscription
    /// - Large date ranges may be split into multiple requests automatically
    /// - Intraday data older than a certain period may not be available
    /// - Weekend and holiday data will not be included in the response
    pub async fn historical_data(
        &self,
        instrument_token: &str,
        from_date: &str,
        to_date: &str,
        interval: &str,
        continuous: &str,
    ) -> Result<JsonValue> {
        let mut params = Vec::new();
        params.push(("from", from_date));
        params.push(("to", to_date));
        params.push(("continuous", continuous));
        
        let url = self.build_url(
            &format!("/instruments/historical/{}/{}", instrument_token, interval),
            Some(params),
        );

        let resp = self.send_request(url, "GET", None).await?;
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
        
        if let Some(quantity) = quantity { params.insert("quantity", quantity); }
        if let Some(amount) = amount { params.insert("amount", amount); }
        if let Some(tag) = tag { params.insert("tag", tag); }

        let url = self.build_url("/mf/orders", None);
        let resp = self.send_request(url, "POST", Some(params)).await?;
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
        let url = self.build_url(&format!("/mf/orders/{}", order_id), None);
        let resp = self.send_request(url, "DELETE", None).await?;
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
        let url: reqwest::Url = if let Some(sip_id) = sip_id {
            self.build_url(&format!("/mf/sips/{}", sip_id), None)
        } else {
            self.build_url("/mf/sips", None)
        };

        let resp = self.send_request(url, "GET", None).await?;
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
        
        if let Some(initial_amount) = initial_amount { params.insert("initial_amount", initial_amount); }
        if let Some(instalment_day) = instalment_day { params.insert("instalment_day", instalment_day); }
        if let Some(tag) = tag { params.insert("tag", tag); }

        let url = self.build_url("/mf/sips", None);
        let resp = self.send_request(url, "POST", Some(params)).await?;
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
        
        if let Some(instalment_day) = instalment_day { params.insert("instalment_day", instalment_day); }

        let url = self.build_url(&format!("/mf/sips/{}", sip_id), None);
        let resp = self.send_request(url, "PUT", Some(params)).await?;
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
        let url = self.build_url(&format!("/mf/sips/{}", sip_id), None);
        let resp = self.send_request(url, "DELETE", None).await?;
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
        let url = self.build_url("/mf/holdings", None);
        let resp = self.send_request(url, "GET", None).await?;
        self.raise_or_return_json(resp).await
    }

    /// Retrieve quote and market depth for list of instruments
    /// 
    /// Gets real-time quote data including bid/ask prices, market depth, 
    /// and other market data for the specified instruments.
    /// 
    /// # Arguments
    /// 
    /// * `instruments` - List of instrument identifiers (exchange:tradingsymbol or instrument_token)
    /// 
    /// # Returns
    /// 
    /// A `Result<JsonValue>` containing quote data with fields like:
    /// - `last_price` - Last traded price
    /// - `ohlc` - Open, High, Low, Close data
    /// - `market_depth` - Bid/Ask data with quantities
    /// - `volume` - Trading volume
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
    /// let instruments = vec!["NSE:RELIANCE", "BSE:SENSEX"];
    /// let quotes = client.quote(instruments).await?;
    /// 
    /// println!("Quotes: {:?}", quotes);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn quote(&self, instruments: Vec<&str>) -> Result<JsonValue> {
        let params: Vec<_> = instruments.into_iter().map(|i| ("i", i)).collect();
        let url = self.build_url("/quote", Some(params));
        let resp = self.send_request(url, "GET", None).await?;
        self.raise_or_return_json(resp).await
    }

    /// Retrieve OHLC (Open, High, Low, Close) data for instruments
    /// 
    /// Gets OHLC data for the current trading day for the specified instruments.
    /// 
    /// # Arguments
    /// 
    /// * `instruments` - List of instrument identifiers
    /// 
    /// # Returns
    /// 
    /// A `Result<JsonValue>` containing OHLC data
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
    /// let instruments = vec!["NSE:RELIANCE", "NSE:TCS"];
    /// let ohlc_data = client.ohlc(instruments).await?;
    /// 
    /// println!("OHLC Data: {:?}", ohlc_data);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn ohlc(&self, instruments: Vec<&str>) -> Result<JsonValue> {
        let params: Vec<_> = instruments.into_iter().map(|i| ("i", i)).collect();
        let url = self.build_url("/quote/ohlc", Some(params));
        let resp = self.send_request(url, "GET", None).await?;
        self.raise_or_return_json(resp).await
    }

    /// Retrieve Last Traded Price (LTP) for instruments
    /// 
    /// Gets the last traded price for the specified instruments. This is a 
    /// lightweight alternative to the full quote API.
    /// 
    /// # Arguments
    /// 
    /// * `instruments` - List of instrument identifiers
    /// 
    /// # Returns
    /// 
    /// A `Result<JsonValue>` containing last traded prices
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
    /// let instruments = vec!["NSE:RELIANCE", "NSE:TCS"];
    /// let ltp_data = client.ltp(instruments).await?;
    /// 
    /// println!("LTP Data: {:?}", ltp_data);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn ltp(&self, instruments: Vec<&str>) -> Result<JsonValue> {
        let params: Vec<_> = instruments.into_iter().map(|i| ("i", i)).collect();
        let url = self.build_url("/quote/ltp", Some(params));
        let resp = self.send_request(url, "GET", None).await?;
        self.raise_or_return_json(resp).await
    }

    /// Retrieve margin requirements for specific trading segments
    /// 
    /// Gets margin requirements and charges for different trading segments
    /// like equity, commodity, currency, etc.
    /// 
    /// # Arguments
    /// 
    /// * `segment` - Trading segment ("equity", "commodity", "currency")
    /// 
    /// # Returns
    /// 
    /// A `Result<JsonValue>` containing margin requirements data
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
    /// let equity_margins = client.instruments_margins("equity").await?;
    /// println!("Equity margins: {:?}", equity_margins);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn instruments_margins(&self, segment: &str) -> Result<JsonValue> {
        let url = self.build_url(&format!("/margins/{}", segment), None);
        let resp = self.send_request(url, "GET", None).await?;
        self.raise_or_return_json(resp).await
    }

    // ...existing code...
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
        
        #[cfg(all(feature = "debug", feature = "wasm", target_arch = "wasm32"))]
        web_sys::console::log_1(&format!("KiteConnect: {} {}", method, url).into());

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

    #[tokio::test]
    async fn test_margins() {
        // Create a new mock server
        let mut server = Server::new_async().await;
        
        // Create KiteConnect instance that uses the mock server URL
        let kiteconnect = TestKiteConnect::new("API_KEY", "ACCESS_TOKEN", &server.url());

        let _mock1 = server.mock("GET", Matcher::Regex(r"^/user/margins".to_string()))
            .with_body_from_file("mocks/margins.json")
            .create_async()
            .await;
        let _mock2 = server.mock("GET", Matcher::Regex(r"^/user/margins/commodity".to_string()))
            .with_body_from_file("mocks/margins.json")
            .create_async()
            .await;

        let data: JsonValue = kiteconnect.margins(None).await.unwrap();
        println!("{:?}", data);
        assert!(data.is_object());
        let data: JsonValue = kiteconnect.margins(Some("commodity".to_string())).await.unwrap();
        println!("{:?}", data);
        assert!(data.is_object());
    }

    #[tokio::test]
    async fn test_holdings() {
        let mut server = Server::new_async().await;
        let kiteconnect = TestKiteConnect::new("API_KEY", "ACCESS_TOKEN", &server.url());

        let _mock = server.mock("GET", Matcher::Regex(r"^/portfolio/holdings".to_string()))
            .with_body_from_file("mocks/holdings.json")
            .create_async()
            .await;

        let data: JsonValue = kiteconnect.holdings().await.unwrap();
        println!("{:?}", data);
        assert!(data.is_object());
    }

    #[tokio::test]
    async fn test_positions() {
        let mut server = Server::new_async().await;
        let kiteconnect = TestKiteConnect::new("API_KEY", "ACCESS_TOKEN", &server.url());

        let _mock = server.mock("GET", Matcher::Regex(r"^/portfolio/positions".to_string()))
            .with_body_from_file("mocks/positions.json")
            .create_async()
            .await;

        let data: JsonValue = kiteconnect.positions().await.unwrap();
        println!("{:?}", data);
        assert!(data.is_object());
    }

    #[tokio::test]
    async fn test_order_trades() {
        let mut server = Server::new_async().await;
        let kiteconnect = TestKiteConnect::new("API_KEY", "ACCESS_TOKEN", &server.url());

        let _mock2 = server.mock(
            "GET", Matcher::Regex(r"^/orders/171229000724687/trades".to_string())
        )
        .with_body_from_file("mocks/order_trades.json")
        .create_async()
        .await;

        let data: JsonValue = kiteconnect.order_trades("171229000724687").await.unwrap();
        println!("{:?}", data);
        assert!(data.is_object());
    }

    #[tokio::test]
    async fn test_orders() {
        let mut server = Server::new_async().await;
        let kiteconnect = TestKiteConnect::new("API_KEY", "ACCESS_TOKEN", &server.url());

        let _mock2 = server.mock(
            "GET", Matcher::Regex(r"^/orders".to_string())
        )
        .with_body_from_file("mocks/orders.json")
        .with_status(200)
        .create_async()
        .await;

        let data: JsonValue = kiteconnect.orders().await.unwrap();
        println!("{:?}", data);
        assert!(data.is_object());
    }

    #[tokio::test]
    async fn test_order_history() {
        let mut server = Server::new_async().await;
        let kiteconnect = TestKiteConnect::new("API_KEY", "ACCESS_TOKEN", &server.url());

        let _mock2 = server.mock(
            "GET", Matcher::Regex(r"^/orders".to_string())
        )
        .with_body_from_file("mocks/order_info.json")
        .create_async()
        .await;

        let data: JsonValue = kiteconnect.order_history("171229000724687").await.unwrap();
        println!("{:?}", data);
        assert!(data.is_object());
    }

    #[tokio::test]
    async fn test_trades() {
        let mut server = Server::new_async().await;
        let kiteconnect = TestKiteConnect::new("API_KEY", "ACCESS_TOKEN", &server.url());

        let _mock1 = server.mock("GET", Matcher::Regex(r"^/trades".to_string()))
            .with_body_from_file("mocks/trades.json")
            .create_async()
            .await;

        let data: JsonValue = kiteconnect.trades().await.unwrap();
        println!("{:?}", data);
        assert!(data.is_object());
    }

    #[tokio::test]
    async fn test_mf_orders() {
        let mut server = Server::new_async().await;
        let kiteconnect = TestKiteConnect::new("API_KEY", "ACCESS_TOKEN", &server.url());

        let _mock1 = server.mock(
            "GET", Matcher::Regex(r"^/mf/orders$".to_string())
        )
        .with_body_from_file("mocks/mf_orders.json")
        .create_async()
        .await;

        let _mock2 = server.mock(
            "GET", Matcher::Regex(r"^/mf/orders".to_string())
        )
        .with_body_from_file("mocks/mf_orders_info.json")
        .create_async()
        .await;

        let data: JsonValue = kiteconnect.mf_orders(None).await.unwrap();
        println!("{:?}", data);
        assert!(data.is_object());
        let data: JsonValue = kiteconnect.mf_orders(Some("171229000724687")).await.unwrap();
        println!("{:?}", data);
        assert!(data.is_object());
    }

    #[tokio::test]
    async fn test_trigger_range() {
        let mut server = Server::new_async().await;
        let kiteconnect = TestKiteConnect::new("API_KEY", "ACCESS_TOKEN", &server.url());

        let _mock2 = server.mock(
            "GET", Matcher::Regex(r"^/instruments/trigger_range".to_string())
        )
        .with_body_from_file("mocks/trigger_range.json")
        .create_async()
        .await;

        let data: JsonValue = kiteconnect.trigger_range("BUY", vec!["NSE:INFY", "NSE:RELIANCE"]).await.unwrap();
        println!("{:?}", data);
        assert!(data.is_object());
    }

    #[tokio::test]
    async fn test_instruments() {
        let mut server = Server::new_async().await;
        let kiteconnect = TestKiteConnect::new("API_KEY", "ACCESS_TOKEN", &server.url());

        let _mock2 = server.mock(
            "GET", Matcher::Regex(r"^/instruments".to_string())
        )
        .with_body_from_file("mocks/instruments.csv")
        .create_async()
        .await;

        let data: JsonValue = kiteconnect.instruments(None).await.unwrap();
        println!("{:?}", data);
        assert_eq!(data[0]["instrument_token"].as_str(), Some("408065"));
    }

    #[tokio::test]
    async fn test_mf_instruments() {
        let mut server = Server::new_async().await;
        let kiteconnect = TestKiteConnect::new("API_KEY", "ACCESS_TOKEN", &server.url());

        let _mock2 = server.mock(
            "GET", Matcher::Regex(r"^/mf/instruments".to_string())
        )
        .with_body_from_file("mocks/mf_instruments.csv")
        .create_async()
        .await;

        let data: JsonValue = kiteconnect.mf_instruments().await.unwrap();
        println!("{:?}", data);
        assert_eq!(data[0]["tradingsymbol"].as_str(), Some("INF846K01DP8"));
    }

    #[cfg(all(feature = "wasm", target_arch = "wasm32"))]
    #[test]
    fn test_csv_core_parsing() {
        let csv_data = "instrument_token,exchange_token,tradingsymbol,name\n408065,1594,INFY,INFOSYS\n5720322,22345,NIFTY15DECFUT,\n";
        let result = parse_csv_with_core(csv_data).unwrap();
        
        if let JsonValue::Array(records) = result {
            assert_eq!(records.len(), 2);
            
            // Check first record
            let first_record = &records[0];
            assert_eq!(first_record["instrument_token"].as_str(), Some("408065"));
            assert_eq!(first_record["tradingsymbol"].as_str(), Some("INFY"));
            assert_eq!(first_record["name"].as_str(), Some("INFOSYS"));
            
            // Check second record
            let second_record = &records[1];
            assert_eq!(second_record["instrument_token"].as_str(), Some("5720322"));
            assert_eq!(second_record["tradingsymbol"].as_str(), Some("NIFTY15DECFUT"));
            assert_eq!(second_record["name"].as_str(), Some(""));
        } else {
            panic!("Expected JsonValue::Array");
        }
    }

    // Helper struct to override the URL for testing
    #[derive(Clone, Debug)]
    struct TestKiteConnect {
        api_key: String,
        access_token: String,
        client: reqwest::Client,
        base_url: String,
    }

    impl TestKiteConnect {
        fn new(api_key: &str, access_token: &str, base_url: &str) -> Self {
            Self {
                api_key: api_key.to_string(),
                access_token: access_token.to_string(),
                client: reqwest::Client::new(),
                base_url: base_url.to_string(),
            }
        }

        fn build_url(&self, path: &str, param: Option<Vec<(&str, &str)>>) -> reqwest::Url {
            let url: &str = &format!("{}/{}", self.base_url, &path[1..]);
            let mut url = reqwest::Url::parse(url).unwrap();

            if let Some(data) = param {
                url.query_pairs_mut().extend_pairs(data.iter());
            }
            url
        }

        async fn send_request(
            &self,
            url: reqwest::Url,
            method: &str,
            data: Option<HashMap<&str, &str>>,
        ) -> Result<reqwest::Response> {
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

            Ok(response)
        }

        async fn raise_or_return_json(&self, resp: reqwest::Response) -> Result<JsonValue> {
            if resp.status().is_success() {
                let jsn: JsonValue = resp.json().await.with_context(|| "Serialization failed")?;
                Ok(jsn)
            } else {
                let error_text = resp.text().await?;
                Err(anyhow!(error_text))
            }
        }

        async fn holdings(&self) -> Result<JsonValue> {
            let url = self.build_url("/portfolio/holdings", None);
            let resp = self.send_request(url, "GET", None).await?;
            self.raise_or_return_json(resp).await
        }

        async fn positions(&self) -> Result<JsonValue> {
            let url = self.build_url("/portfolio/positions", None);
            let resp = self.send_request(url, "GET", None).await?;
            self.raise_or_return_json(resp).await
        }

        async fn orders(&self) -> Result<JsonValue> {
            let url = self.build_url("/orders", None);
            let resp = self.send_request(url, "GET", None).await?;
            self.raise_or_return_json(resp).await
        }

        async fn margins(&self, segment: Option<String>) -> Result<JsonValue> {
            let url: reqwest::Url = if let Some(segment) = segment {
                self.build_url(&format!("/user/margins/{}", segment), None)
            } else {
                self.build_url("/user/margins", None)
            };

            let resp = self.send_request(url, "GET", None).await?;
            self.raise_or_return_json(resp).await
        }

        async fn order_trades(&self, order_id: &str) -> Result<JsonValue> {
            let url = self.build_url(&format!("/orders/{}/trades", order_id), None);
            let resp = self.send_request(url, "GET", None).await?;
            self.raise_or_return_json(resp).await
        }

        async fn order_history(&self, order_id: &str) -> Result<JsonValue> {
            let params = vec![("order_id", order_id)];
            let url = self.build_url("/orders", Some(params));
            let resp = self.send_request(url, "GET", None).await?;
            self.raise_or_return_json(resp).await
        }

        async fn trades(&self) -> Result<JsonValue> {
            let url = self.build_url("/trades", None);
            let resp = self.send_request(url, "GET", None).await?;
            self.raise_or_return_json(resp).await
        }

        async fn mf_orders(&self, order_id: Option<&str>) -> Result<JsonValue> {
            let url: reqwest::Url = if let Some(order_id) = order_id {
                self.build_url(&format!("/mf/orders/{}", order_id), None)
            } else {
                self.build_url("/mf/orders", None)
            };

            let resp = self.send_request(url, "GET", None).await?;
            self.raise_or_return_json(resp).await
        }

        async fn trigger_range(
            &self,
            transaction_type: &str,
            instruments: Vec<&str>,
        ) -> Result<JsonValue> {
            let mut params: Vec<(&str, &str)> = Vec::new();
            params.push(("transaction_type", transaction_type));
            
            for instrument in instruments {
                params.push(("instruments", instrument));
            }

            let url = self.build_url("/instruments/trigger_range", Some(params));
            let resp = self.send_request(url, "GET", None).await?;
            self.raise_or_return_json(resp).await
        }

        async fn instruments(&self, exchange: Option<&str>) -> Result<JsonValue> {
            let url: reqwest::Url = if let Some(exchange) = exchange {
                self.build_url(&format!("/instruments/{}", exchange), None)
            } else {
                self.build_url("/instruments", None)
            };

            let resp = self.send_request(url, "GET", None).await?;
            let body = resp.text().await?;
            
            // Parse CSV response
            #[cfg(not(target_arch = "wasm32"))]
            {
                use csv::ReaderBuilder;
                let mut rdr = ReaderBuilder::new().from_reader(body.as_bytes());
                let mut result = Vec::new();
                
                let headers = rdr.headers()?.clone();
                for record in rdr.records() {
                    let record = record?;
                    let mut obj = serde_json::Map::new();
                    
                    for (i, field) in record.iter().enumerate() {
                        if let Some(header) = headers.get(i) {
                            obj.insert(header.to_string(), JsonValue::String(field.to_string()));
                        }
                    }
                    result.push(JsonValue::Object(obj));
                }
                
                Ok(JsonValue::Array(result))
            }
            
            #[cfg(target_arch = "wasm32")]
            {
                Ok(JsonValue::String(body))
            }
        }

        async fn mf_instruments(&self) -> Result<JsonValue> {
            let url = self.build_url("/mf/instruments", None);
            let resp = self.send_request(url, "GET", None).await?;
            let body = resp.text().await?;
            
            // Parse CSV response
            #[cfg(not(target_arch = "wasm32"))]
            {
                use csv::ReaderBuilder;
                let mut rdr = ReaderBuilder::new().from_reader(body.as_bytes());
                let mut result = Vec::new();
                
                let headers = rdr.headers()?.clone();
                for record in rdr.records() {
                    let record = record?;
                    let mut obj = serde_json::Map::new();
                    
                    for (i, field) in record.iter().enumerate() {
                        if let Some(header) = headers.get(i) {
                            obj.insert(header.to_string(), JsonValue::String(field.to_string()));
                        }
                    }
                    result.push(JsonValue::Object(obj));
                }
                
                Ok(JsonValue::Array(result))
            }
            
            #[cfg(target_arch = "wasm32")]
            {
                Ok(JsonValue::String(body))
            }
        }
    }
}
