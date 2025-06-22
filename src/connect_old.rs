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
use chrono::Utc;

// Import model types for typed responses
use crate::model::{
    // User and session models
    UserSession, UserProfile, FullUserProfile, AllMargins, Margins,
    // Portfolio models
    Holding, Holdings, Position, Positions,
    // Order models
    Order, OrderParams, OrderResponse, Trade, Trades,
    // Market data models
    Quote, QuoteData, QuoteOHLC, QuoteOHLCData, QuoteLTP, QuoteLTPData, 
    Instrument, MFInstrument, TriggerRangeResponse,
    // Mutual fund models
    MFOrder, MFSIP, MFHolding,
    // Response wrapper
    KiteResponse, KiteErrorResponse,
    // GTT models
    GTT, GTTCondition, GTTOrder,
    // Margin models
    OrderMargins, Charges, GST,
};

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

/// Parse CSV data into Instrument structs using csv-core for WASM compatibility
#[cfg(all(feature = "wasm", target_arch = "wasm32"))]
fn parse_csv_to_instruments(csv_data: &str) -> Result<Vec<Instrument>> {
    let mut reader = Reader::new();
    let mut output = vec![0; 1024];
    let mut field = Vec::new();
    let mut input = csv_data.as_bytes();
    
    let mut records: Vec<Vec<String>> = Vec::new();
    let mut current_record: Vec<String> = Vec::new();
    let mut is_first_row = true;
    
    loop {
        let (result, input_consumed, output_written) = reader.read_field(input, &mut output);
        input = &input[input_consumed..];
        
        match result {
            ReadFieldResult::InputEmpty => {
                if !current_record.is_empty() && !is_first_row {
                    records.push(current_record.clone());
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
                        is_first_row = false;
                    } else {
                        records.push(current_record.clone());
                    }
                    current_record.clear();
                }
            }
        }
    }
    
    // Convert to Instrument structs
    let mut result = Vec::new();
    for record in records {
        if record.len() >= 12 {
            let instrument = Instrument {
                instrument_token: record[0].parse().unwrap_or(0),
                exchange_token: record[1].parse().unwrap_or(0),
                tradingsymbol: record[2].clone(),
                name: record[3].clone(),
                last_price: record[4].parse().unwrap_or(0.0),
                expiry: if record[5].is_empty() { 
                    None 
                } else { 
                    record[5].parse().ok() 
                },
                strike_price: record[6].parse().unwrap_or(0.0),
                tick_size: record[7].parse().unwrap_or(0.0),
                lot_size: record[8].parse().unwrap_or(0.0),
                instrument_type: record[9].clone(),
                segment: record[10].clone(),
                exchange: record[11].clone(),
            };
            result.push(instrument);
        }
    }
    
    Ok(result)
}

/// Parse CSV data into MFInstrument structs using csv-core for WASM compatibility
#[cfg(all(feature = "wasm", target_arch = "wasm32"))]
fn parse_csv_to_mf_instruments(csv_data: &str) -> Result<Vec<MFInstrument>> {
    let mut reader = Reader::new();
    let mut output = vec![0; 1024];
    let mut field = Vec::new();
    let mut input = csv_data.as_bytes();
    
    let mut records: Vec<Vec<String>> = Vec::new();
    let mut current_record: Vec<String> = Vec::new();
    let mut is_first_row = true;
    
    loop {
        let (result, input_consumed, output_written) = reader.read_field(input, &mut output);
        input = &input[input_consumed..];
        
        match result {
            ReadFieldResult::InputEmpty => {
                if !current_record.is_empty() && !is_first_row {
                    records.push(current_record.clone());
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
                        is_first_row = false;
                    } else {
                        records.push(current_record.clone());
                    }
                    current_record.clear();
                }
            }
        }
    }
    
    // Convert to MFInstrument structs
    let mut result = Vec::new();
    for record in records {
        if record.len() >= 16 {
            let mf_instrument = MFInstrument {
                tradingsymbol: record[0].clone(),
                name: record[1].clone(),
                last_price: record[2].parse().unwrap_or(0.0),
                amc: record[3].clone(),
                purchase_allowed: record[4].parse().unwrap_or(false),
                redemption_allowed: record[5].parse().unwrap_or(false),
                minimum_purchase_amount: record[6].parse().unwrap_or(0.0),
                purchase_amount_multiplier: record[7].parse().unwrap_or(0.0),
                minimum_additional_purchase_amount: record[8].parse().unwrap_or(0.0),
                minimum_redemption_quantity: record[9].parse().unwrap_or(0.0),
                redemption_quantity_multiplier: record[10].parse().unwrap_or(0.0),
                dividend_type: record[11].clone(),
                scheme_type: record[12].clone(),
                plan: record[13].clone(),
                settlement_type: record[14].clone(),
                last_price_date: record[15].parse().unwrap_or_else(|_| Utc::now()),
            };
            result.push(mf_instrument);
        }
    }
    
    Ok(result)
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

    /// Helper method to parse typed responses from KiteConnect API
    async fn parse_response<T>(&self, resp: reqwest::Response) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let status = resp.status();
        let response_text = resp.text().await.with_context(|| "Failed to read response text")?;
        
        if status.is_success() {
            // Try to parse as KiteResponse wrapper first
            if let Ok(kite_response) = serde_json::from_str::<KiteResponse<T>>(&response_text) {
                return Ok(kite_response.data);
            }
            
            // If that fails, try parsing the raw response as T directly
            serde_json::from_str::<T>(&response_text)
                .with_context(|| format!("Failed to deserialize response: {}", response_text))
        } else {
            // Try to parse as error response
            if let Ok(error_response) = serde_json::from_str::<KiteErrorResponse>(&response_text) {
                Err(anyhow!("{}: {}", error_response.error_type, error_response.message))
            } else {
                Err(anyhow!("HTTP {} - {}", status, response_text))
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
    /// A `Result<AllMargins>` containing margin data for all segments, or a `Result<Margins>` for a specific segment.
    /// The returned data includes:
    /// - `available` - Available margin for trading with breakdown of cash, collateral, etc.
    /// - `used` - Currently utilized margin with SPAN, exposure, M2M details
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
    /// println!("Equity available balance: {}", all_margins.equity.available.live_balance);
    /// 
    /// // Get margins for specific segment - would need separate method for specific segment
    /// # Ok(())
    /// # }
    /// ```
    pub async fn margins(&self, segment: Option<String>) -> Result<AllMargins> {
        let url: reqwest::Url = if let Some(segment) = segment {
            self.build_url(&format!("/user/margins/{}", segment), None)
        } else {
            self.build_url("/user/margins", None)
        };

        let resp = self.send_request(url, "GET", None).await?;
        self.parse_response(resp).await
    }

    /// Get user profile details
    /// 
    /// Returns basic user profile information including user ID, name, email, broker details,
    /// enabled exchanges, products, and order types.
    /// 
    /// # Returns
    /// 
    /// A `Result<UserProfile>` containing user profile data with fields like:
    /// - `user_id` - Unique user identifier
    /// - `user_name` - User's real name  
    /// - `email` - User's email address
    /// - `broker` - Broker ID
    /// - `exchanges` - List of enabled exchanges
    /// - `products` - List of enabled product types
    /// - `order_types` - List of enabled order types
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
    /// let profile = client.profile().await?;
    /// println!("User: {} ({})", profile.user_name, profile.user_id);
    /// println!("Broker: {}", profile.broker);
    /// println!("Enabled exchanges: {:?}", profile.exchanges);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn profile(&self) -> Result<UserProfile> {
        let url = self.build_url("/user/profile", None);
        let resp = self.send_request(url, "GET", None).await?;
        self.parse_response(resp).await
    }

    /// Retrieves the user's holdings (stocks held in demat account)
    /// 
    /// Holdings represent stocks that are held in the user's demat account.
    /// This includes information about quantity, average price, current market value,
    /// profit/loss, and more.
    /// 
    /// # Returns
    /// 
    /// A `Result<Holdings>` containing a vector of holdings with fields like:
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
    /// // Access holdings data directly
    /// for holding in &holdings {
    ///     println!("Symbol: {}, Quantity: {}, P&L: {}", 
    ///         holding.tradingsymbol, holding.quantity, holding.pnl);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn holdings(&self) -> Result<Holdings> {
        let url = self.build_url("/portfolio/holdings", None);
        let resp = self.send_request(url, "GET", None).await?;
        self.parse_response(resp).await
    }

    /// Retrieves the user's positions (open positions for the day)
    /// 
    /// Positions represent open trading positions for the current trading day.
    /// This includes both intraday and carry-forward positions with details about
    /// profit/loss, margin requirements, and position status.
    /// 
    /// # Returns
    /// 
    /// A `Result<Positions>` containing positions data with fields like:
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
    /// // Check for open day positions
    /// for position in &positions.day {
    ///     if position.quantity != 0 {
    ///         println!("Open position: {} qty {}", 
    ///             position.tradingsymbol, position.quantity);
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn positions(&self) -> Result<Positions> {
        let url = self.build_url("/portfolio/positions", None);
        let resp = self.send_request(url, "GET", None).await?;
        self.parse_response(resp).await
    }

    /// Place an order
    /// 
    /// Places a new order on the exchange with specified parameters.
    /// 
    /// # Arguments
    /// 
    /// * `variety` - Order variety (regular, amo, co, iceberg, etc.)
    /// * `exchange` - Exchange to place the order on (NSE, BSE, etc.)
    /// * `tradingsymbol` - Trading symbol of the instrument
    /// * `transaction_type` - BUY or SELL
    /// * `quantity` - Order quantity
    /// * `product` - Product type (MIS, CNC, NRML)
    /// * `order_type` - Order type (MARKET, LIMIT, SL, SL-M)
    /// * `price` - Order price (required for LIMIT orders)
    /// * `validity` - Order validity (DAY, IOC)
    /// * `disclosed_quantity` - Iceberg quantity
    /// * `trigger_price` - Trigger price for stop-loss orders
    /// * `squareoff` - Squareoff price for bracket orders
    /// * `stoploss` - Stoploss price for bracket orders
    /// * `trailing_stoploss` - Trailing stoploss for bracket orders
    /// * `tag` - Optional tag to identify the order
    /// 
    /// # Returns
    /// 
    /// A `Result<OrderResponse>` containing the order ID of the placed order
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
    /// let order_response = client.place_order(
    ///     "regular",
    ///     "NSE",
    ///     "RELIANCE",
    ///     "BUY", 
    ///     "1",
    ///     Some("CNC"),
    ///     Some("MARKET"),
    ///     None,
    ///     Some("DAY"),
    ///     None,
    ///     None,
    ///     None,
    ///     None,
    ///     None,
    ///     Some("my_order")
    /// ).await?;
    /// 
    /// println!("Order placed with ID: {}", order_response.order_id);
    /// # Ok(())
    /// # }
    /// ```
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
    ) -> Result<OrderResponse> {
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
        self.parse_response(resp).await
    }

    /// Modify an open order
    /// 
    /// Modifies the parameters of an existing open order.
    /// 
    /// # Arguments
    /// 
    /// * `order_id` - The order ID to modify
    /// * `variety` - Order variety
    /// * `quantity` - New order quantity
    /// * `price` - New order price
    /// * `order_type` - New order type
    /// * `validity` - New order validity
    /// * `disclosed_quantity` - New disclosed quantity
    /// * `trigger_price` - New trigger price
    /// * `parent_order_id` - Parent order ID for child orders
    /// 
    /// # Returns
    /// 
    /// A `Result<OrderResponse>` containing the order modification response
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
    /// let response = client.modify_order(
    ///     "123456",          // Order ID
    ///     "regular",         // Variety
    ///     Some("2"),         // New quantity
    ///     Some("2500.0"),    // New price
    ///     Some("LIMIT"),     // Order type
    ///     Some("DAY"),       // Validity
    ///     None,              // Disclosed quantity
    ///     None,              // Trigger price
    ///     None               // Parent order ID
    /// ).await?;
    /// 
    /// println!("Order modified: {}", response.order_id);
    /// # Ok(())
    /// # }
    /// ```
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
    ) -> Result<OrderResponse> {
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
        self.parse_response(resp).await
    }

    /// Cancel an order
    /// 
    /// Cancels an open order.
    /// 
    /// # Arguments
    /// 
    /// * `order_id` - The order ID to cancel
    /// * `variety` - Order variety
    /// * `parent_order_id` - Parent order ID for bracket/cover orders
    /// 
    /// # Returns
    /// 
    /// A `Result<OrderResponse>` containing the cancellation response
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
    /// let response = client.cancel_order(
    ///     "123456",    // Order ID
    ///     "regular",   // Variety
    ///     None         // Parent order ID
    /// ).await?;
    /// 
    /// println!("Order cancelled: {}", response.order_id);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn cancel_order(
        &self,
        order_id: &str,
        variety: &str,
        parent_order_id: Option<&str>,
    ) -> Result<OrderResponse> {
        let mut params = HashMap::new();
        params.insert("order_id", order_id);
        params.insert("variety", variety);
        if let Some(parent_order_id) = parent_order_id {
            params.insert("parent_order_id", parent_order_id);
        }

        let url = self.build_url(&format!("/orders/{}/{}", variety, order_id), None);
        let resp = self.send_request(url, "DELETE", Some(params)).await?;
        self.parse_response(resp).await
    }

    /// Exit a BO/CO order
    /// 
    /// Exits a bracket order or cover order (alias for cancel_order).
    /// 
    /// # Arguments
    /// 
    /// * `order_id` - The order ID to exit
    /// * `variety` - Order variety
    /// * `parent_order_id` - Parent order ID for bracket/cover orders
    /// 
    /// # Returns
    /// 
    /// A `Result<OrderResponse>` containing the exit response
    pub async fn exit_order(
        &self,
        order_id: &str,
        variety: &str,
        parent_order_id: Option<&str>,
    ) -> Result<OrderResponse> {
        self.cancel_order(order_id, variety, parent_order_id).await
    }

    /// Retrieves a list of all orders for the current trading day
    /// 
    /// Returns all orders placed by the user for the current trading day,
    /// including pending, completed, rejected, and cancelled orders.
    /// 
    /// # Returns
    /// 
    /// A `Result<Vec<Order>>` containing orders data with fields like:
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
    /// for order in &orders {
    ///     println!("Order {}: {} - {}", 
    ///         order.order_id, 
    ///         order.tradingsymbol, 
    ///         order.status);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn orders(&self) -> Result<Vec<Order>> {
        let url = self.build_url("/orders", None);
        let resp = self.send_request(url, "GET", None).await?;
        self.parse_response(resp).await
    }

    /// Get the list of order history
    /// 
    /// Retrieves the complete order history/lifecycle for a specific order,
    /// showing all state changes and modifications made to the order.
    /// 
    /// # Arguments
    /// 
    /// * `order_id` - The unique order ID to retrieve history for
    /// 
    /// # Returns
    /// 
    /// A `Result<Vec<Order>>` containing the order history as a list of order states
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
    /// let history = client.order_history("171229000724687").await?;
    /// 
    /// // Access order history directly
    /// for order_state in &history {
    ///     println!("Order status: {}, Time: {:?}", 
    ///         order_state.status, order_state.order_timestamp);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn order_history(&self, order_id: &str) -> Result<Vec<Order>> {
        let params = vec![("order_id", order_id)];
        let url = self.build_url("/orders", Some(params));
        let resp = self.send_request(url, "GET", None).await?;
        self.parse_response(resp).await
    }

    /// Get all trades
    /// 
    /// Retrieves all executed trades for the current trading session.
    /// Trades represent completed transactions with details about execution price,
    /// quantity, and timing.
    /// 
    /// # Returns
    /// 
    /// A `Result<Trades>` containing the trades data
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
    /// let trades = client.trades().await?;
    /// println!("Trades: {:?}", trades);
    /// 
    /// // Calculate total volume
    /// let total_volume: i32 = trades.iter().map(|t| t.quantity).sum();
    /// println!("Total volume: {}", total_volume);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn trades(&self) -> Result<Trades> {
        let url = self.build_url("/trades", None);
        let resp = self.send_request(url, "GET", None).await?;
        self.parse_response(resp).await
    }

    /// Get all trades for a specific order
    /// 
    /// Retrieves all executed trades/fills for a specific order. This is useful
    /// for orders that might be partially filled or executed in multiple lots.
    /// 
    /// # Arguments
    /// 
    /// * `order_id` - The unique order ID to retrieve trades for
    /// 
    /// # Returns
    /// 
    /// A `Result<Vec<Trade>>` containing the list of trades for the order
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
    /// let trades = client.order_trades("171229000724687").await?;
    /// 
    /// // Access trade details directly
    /// for trade in &trades {
    ///     println!("Trade: {} shares at {}", trade.quantity, trade.price);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn order_trades(&self, order_id: &str) -> Result<Vec<Trade>> {
        let url = self.build_url(&format!("/orders/{}/trades", order_id), None);
        let resp = self.send_request(url, "GET", None).await?;
        self.parse_response(resp).await
    }

    /// Modify an open position product type
    /// 
    /// Converts the product type of an existing position (e.g., from MIS to CNC).
    /// This allows changing the position's product without exiting and re-entering.
    /// 
    /// # Arguments
    /// 
    /// * `exchange` - Exchange where the position exists (NSE, BSE, etc.)
    /// * `tradingsymbol` - Trading symbol of the instrument
    /// * `transaction_type` - BUY or SELL 
    /// * `position_type` - "day" or "overnight"
    /// * `quantity` - Quantity of position to convert
    /// * `old_product` - Current product type (MIS, CNC, NRML)
    /// * `new_product` - Target product type (MIS, CNC, NRML)
    /// 
    /// # Returns
    /// 
    /// A `Result<OrderResponse>` containing conversion confirmation
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
    /// // Convert MIS position to CNC
    /// let result = client.convert_position(
    ///     "NSE",        // Exchange
    ///     "RELIANCE",   // Symbol
    ///     "BUY",        // Transaction type
    ///     "day",        // Position type
    ///     "10",         // Quantity
    ///     "MIS",        // Old product
    ///     "CNC"         // New product
    /// ).await?;
    /// 
    /// println!("Position converted: {}", result.order_id);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn convert_position(
        &self,
        exchange: &str,
        tradingsymbol: &str,
        transaction_type: &str,
        position_type: &str,
        quantity: &str,
        old_product: &str,
        new_product: &str,
    ) -> Result<OrderResponse> {
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
        self.parse_response(resp).await
    }

    /// Get all mutual fund orders or individual order info
    /// 
    /// Retrieves mutual fund orders with details about order status, settlement, 
    /// fund information, and execution details.
    /// 
    /// # Arguments
    /// 
    /// * `order_id` - Optional order ID. If None, returns all orders; if Some, returns specific order
    /// 
    /// # Returns
    /// 
    /// A `Result<Vec<MFOrder>>` containing mutual fund order data with fields like:
    /// - `order_id` - Unique order identifier
    /// - `tradingsymbol` - Trading symbol of the mutual fund
    /// - `fund` - Fund name
    /// - `status` - Order status (COMPLETE, CANCELLED, etc.)
    /// - `transaction_type` - BUY or SELL
    /// - `quantity` - Number of units
    /// - `amount` - Order amount
    /// - `average_price` - Execution price per unit
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
    /// // Get all MF orders
    /// let all_orders = client.mf_orders(None).await?;
    /// for order in &all_orders {
    ///     println!("Order {}: {} {} units of {}", 
    ///         order.order_id, order.transaction_type, order.quantity, order.fund);
    /// }
    /// 
    /// // Get specific order
    /// let specific_order = client.mf_orders(Some("123456")).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn mf_orders(&self, order_id: Option<&str>) -> Result<Vec<MFOrder>> {
        let url: reqwest::Url = if let Some(order_id) = order_id {
            self.build_url(&format!("/mf/orders/{}", order_id), None)
        } else {
            self.build_url("/mf/orders", None)
        };

        let resp = self.send_request(url, "GET", None).await?;
        self.parse_response(resp).await
    }

    /// Get the trigger range for a list of instruments
    /// 
    /// Retrieves the allowed price range for placing stoploss orders on given instruments.
    /// This is useful for determining valid trigger prices for bracket and cover orders.
    /// 
    /// # Arguments
    /// 
    /// * `transaction_type` - Transaction type ("BUY" or "SELL")
    /// * `instruments` - List of instrument identifiers (exchange:tradingsymbol or instrument_token)
    /// 
    /// # Returns
    /// 
    /// A `Result<TriggerRangeResponse>` containing a HashMap mapping instrument symbols to their trigger range data with fields like:
    /// - `instrument_token` - Token of the instrument
    /// - `lower` - Lower trigger price limit
    /// - `upper` - Upper trigger price limit
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
    /// let trigger_ranges = client.trigger_range("BUY", instruments).await?;
    /// 
    /// // Access trigger range data directly
    /// for (symbol, range_data) in &trigger_ranges {
    ///     println!("Symbol: {}, Lower: {}, Upper: {}", 
    ///         symbol, range_data.lower, range_data.upper);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn trigger_range(
        &self,
        transaction_type: &str,
        instruments: Vec<&str>,
    ) -> Result<TriggerRangeResponse> {
        let mut params: Vec<(&str, &str)> = Vec::new();
        params.push(("transaction_type", transaction_type));
        
        for instrument in instruments {
            params.push(("instruments", instrument));
        }

        let url = self.build_url("/instruments/trigger_range", Some(params));
        let resp = self.send_request(url, "GET", None).await?;
        self.parse_response(resp).await
    }

    /// Get instruments list
    /// 
    /// Retrieves the list of trading instruments available on the platform.
    /// Returns detailed information about each instrument including symbols, tokens,
    /// expiry dates, strike prices, and other instrument-specific details.
    /// 
    /// # Arguments
    /// 
    /// * `exchange` - Optional exchange name to filter instruments (e.g., "NSE", "BSE", "NFO")
    /// 
    /// # Returns
    /// 
    /// A `Result<Vec<Instrument>>` containing instrument data with fields like:
    /// - `instrument_token` - Unique instrument identifier
    /// - `tradingsymbol` - Trading symbol
    /// - `name` - Instrument name
    /// - `exchange` - Exchange on which the instrument is listed
    /// - `segment` - Market segment
    /// - `instrument_type` - Type of instrument (EQ, FUT, OPT, etc.)
    /// - `expiry` - Expiry date for derivatives (None for equity)
    /// - `strike_price` - Strike price for options
    /// - `lot_size` - Minimum trading lot size
    /// 
    /// # Errors
    /// 
    /// Returns an error if the API request fails or CSV parsing fails
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
    /// // Get all instruments
    /// let all_instruments = client.instruments(None).await?;
    /// println!("Total instruments: {}", all_instruments.len());
    /// 
    /// // Get NSE instruments only
    /// let nse_instruments = client.instruments(Some("NSE")).await?;
    /// for instrument in nse_instruments.iter().take(5) {
    ///     println!("Instrument: {} ({})", instrument.tradingsymbol, instrument.name);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(all(feature = "native", not(target_arch = "wasm32")))]
    pub async fn instruments(&self, exchange: Option<&str>) -> Result<Vec<Instrument>> {
        let url: reqwest::Url = if let Some(exchange) = exchange {
            self.build_url(&format!("/instruments/{}", exchange), None)
        } else {
            self.build_url("/instruments", None)
        };

        let resp = self.send_request(url, "GET", None).await?;
        let body = resp.text().await?;
        
        // Parse CSV response into typed Instrument structs
        let mut rdr = ReaderBuilder::new().from_reader(body.as_bytes());
        let mut result = Vec::new();
        
        for record in rdr.records() {
            let record = record?;
            if record.len() >= 12 {
                let instrument = Instrument {
                    instrument_token: record[0].parse().unwrap_or(0),
                    exchange_token: record[1].parse().unwrap_or(0),
                    tradingsymbol: record[2].to_string(),
                    name: record[3].to_string(),
                    last_price: record[4].parse().unwrap_or(0.0),
                    expiry: if record[5].is_empty() { 
                        None 
                    } else { 
                        record[5].parse().ok() 
                    },
                    strike_price: record[6].parse().unwrap_or(0.0),
                    tick_size: record[7].parse().unwrap_or(0.0),
                    lot_size: record[8].parse().unwrap_or(0.0),
                    instrument_type: record[9].to_string(),
                    segment: record[10].to_string(),
                    exchange: record[11].to_string(),
                };
                result.push(instrument);
            }
        }
        
        Ok(result)
    }

    /// Get instruments list (WASM version - now parses CSV using csv-core)
    #[cfg(all(feature = "wasm", target_arch = "wasm32"))]
    pub async fn instruments(&self, exchange: Option<&str>) -> Result<Vec<Instrument>> {
        let url: reqwest::Url = if let Some(exchange) = exchange {
            self.build_url(&format!("/instruments/{}", exchange), None)
        } else {
            self.build_url("/instruments", None)
        };

        let resp = self.send_request(url, "GET", None).await?;
        let body = resp.text().await?;
        
        // Parse CSV using csv-core for WASM compatibility and convert to Instrument structs
        parse_csv_to_instruments(&body)
    }

    /// Get mutual fund instruments list
    #[cfg(all(feature = "native", not(target_arch = "wasm32")))]
    pub async fn mf_instruments(&self) -> Result<Vec<MFInstrument>> {
        let url = self.build_url("/mf/instruments", None);
        let resp = self.send_request(url, "GET", None).await?;
        let body = resp.text().await?;
        
        // Parse CSV response to MFInstrument structs
        let mut rdr = ReaderBuilder::new().from_reader(body.as_bytes());
        let mut result = Vec::new();
        
        for record_result in rdr.records() {
            let record = record_result?;
            if record.len() >= 16 {
                let mf_instrument = MFInstrument {
                    tradingsymbol: record[0].to_string(),
                    name: record[1].to_string(),
                    last_price: record[2].parse().unwrap_or(0.0),
                    amc: record[3].to_string(),
                    purchase_allowed: record[4].parse().unwrap_or(false),
                    redemption_allowed: record[5].parse().unwrap_or(false),
                    minimum_purchase_amount: record[6].parse().unwrap_or(0.0),
                    purchase_amount_multiplier: record[7].parse().unwrap_or(0.0),
                    minimum_additional_purchase_amount: record[8].parse().unwrap_or(0.0),
                    minimum_redemption_quantity: record[9].parse().unwrap_or(0.0),
                    redemption_quantity_multiplier: record[10].parse().unwrap_or(0.0),
                    dividend_type: record[11].to_string(),
                    scheme_type: record[12].to_string(),
                    plan: record[13].to_string(),
                    settlement_type: record[14].to_string(),
                    last_price_date: record[15].parse().unwrap_or_else(|_| Utc::now()),
                };
                result.push(mf_instrument);
            }
        }
        
        Ok(result)
    }

    /// Get mutual fund instruments list (WASM version - now parses CSV using csv-core)
    #[cfg(all(feature = "wasm", target_arch = "wasm32"))]
    pub async fn mf_instruments(&self) -> Result<Vec<MFInstrument>> {
        let url = self.build_url("/mf/instruments", None);
        let resp = self.send_request(url, "GET", None).await?;
        let body = resp.text().await?;
        
        // Parse CSV using csv-core for WASM compatibility and convert to MFInstrument structs
        parse_csv_to_mf_instruments(&body)
    }

    /// Get instruments list (fallback when no platform features are enabled)
    #[cfg(not(any(
        all(feature = "native", not(target_arch = "wasm32")),
        all(feature = "wasm", target_arch = "wasm32")
    )))]
    pub async fn instruments(&self, _exchange: Option<&str>) -> Result<Vec<Instrument>> {
        Err(anyhow!("Instruments functionality requires either 'native' or 'wasm' feature to be enabled"))
    }

    /// Get mutual fund instruments list (fallback when no platform features are enabled)
    #[cfg(not(any(
        all(feature = "native", not(target_arch = "wasm32")),
        all(feature = "wasm", target_arch = "wasm32")
    )))]
    pub async fn mf_instruments(&self) -> Result<Vec<MFInstrument>> {
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
    /// Place a mutual fund order
    /// 
    /// Places a mutual fund buy or sell order.
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
    /// A `Result<OrderResponse>` containing order confirmation with order_id
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
    /// println!("Order placed with ID: {}", buy_order.order_id);
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
    ) -> Result<OrderResponse> {
        let mut params = HashMap::new();
        params.insert("tradingsymbol", tradingsymbol);
        params.insert("transaction_type", transaction_type);
        
        if let Some(quantity) = quantity { params.insert("quantity", quantity); }
        if let Some(amount) = amount { params.insert("amount", amount); }
        if let Some(tag) = tag { params.insert("tag", tag); }

        let url = self.build_url("/mf/orders", None);
        let resp = self.send_request(url, "POST", Some(params)).await?;
        self.parse_response(resp).await
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
    /// A `Result<OrderResponse>` containing cancellation confirmation
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
    /// println!("Order cancelled: {}", result.order_id);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn cancel_mf_order(&self, order_id: &str) -> Result<OrderResponse> {
        let url = self.build_url(&format!("/mf/orders/{}", order_id), None);
        let resp = self.send_request(url, "DELETE", None).await?;
        self.parse_response(resp).await
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
    /// A `Result<Vec<MFSIP>>` containing SIP information with details like:
    /// - `sip_id` - Unique SIP identifier
    /// - `tradingsymbol` - Fund trading symbol
    /// - `amount` - SIP installment amount
    /// - `frequency` - SIP frequency (monthly, weekly, etc.)
    /// - `status` - SIP status (ACTIVE, PAUSED, CANCELLED)
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
    /// // Access SIP details directly
    /// for sip in &all_sips {
    ///     println!("SIP: {} - {} ({})", sip.sip_id, sip.tradingsymbol, sip.status);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn mf_sips(&self, sip_id: Option<&str>) -> Result<Vec<MFSIP>> {
        let url: reqwest::Url = if let Some(sip_id) = sip_id {
            self.build_url(&format!("/mf/sips/{}", sip_id), None)
        } else {
            self.build_url("/mf/sips", None)
        };

        let resp = self.send_request(url, "GET", None).await?;
        self.parse_response(resp).await
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
    /// A `Result<OrderResponse>` containing SIP creation confirmation with SIP ID
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
    /// 
    /// println!("SIP created with ID: {}", sip.order_id);
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
    ) -> Result<OrderResponse> {
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
        self.parse_response(resp).await
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
    /// A `Result<OrderResponse>` containing modification confirmation
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
    /// 
    /// println!("SIP modified with ID: {}", result.order_id);
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
    ) -> Result<OrderResponse> {
        let mut params = HashMap::new();
        params.insert("amount", amount);
        params.insert("status", status);
        params.insert("instalments", instalments);
        params.insert("frequency", frequency);
        
        if let Some(instalment_day) = instalment_day { params.insert("instalment_day", instalment_day); }

        let url = self.build_url(&format!("/mf/sips/{}", sip_id), None);
        let resp = self.send_request(url, "PUT", Some(params)).await?;
        self.parse_response(resp).await
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
    /// A `Result<OrderResponse>` containing cancellation confirmation
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
    /// println!("SIP cancelled with ID: {}", result.order_id);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn cancel_mf_sip(&self, sip_id: &str) -> Result<OrderResponse> {
        let url = self.build_url(&format!("/mf/sips/{}", sip_id), None);
        let resp = self.send_request(url, "DELETE", None).await?;
        self.parse_response(resp).await
    }

    /// Get mutual fund holdings
    /// 
    /// Retrieves the user's mutual fund holdings with current values and returns.
    /// 
    /// # Returns
    /// 
    /// A `Result<Vec<MFHolding>>` containing mutual fund holdings data with fields like:
    /// - `tradingsymbol` - Trading symbol of the mutual fund
    /// - `folio` - Folio number 
    /// - `fund` - Fund name
    /// - `quantity` - Number of units held
    /// - `average_price` - Average purchase price per unit
    /// - `last_price` - Current NAV/price per unit
    /// - `pnl` - Profit/loss amount
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
    /// // Access holding details directly
    /// for holding in &holdings {
    ///     println!("Fund: {}, Units: {}, Current Value: {}", 
    ///         holding.fund, holding.quantity, holding.last_price);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn mf_holdings(&self) -> Result<Vec<MFHolding>> {
        let url = self.build_url("/mf/holdings", None);
        let resp = self.send_request(url, "GET", None).await?;
        self.parse_response(resp).await
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
    /// A `Result<Quote>` containing a HashMap mapping instrument symbols to quote data with fields like:
    /// - `last_price` - Last traded price
    /// - `ohlc` - Open, High, Low, Close data  
    /// - `depth` - Market depth with bid/ask prices and quantities
    /// - `volume` - Trading volume
    /// - `net_change` - Price change from previous close
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
    /// // Access quote data directly
    /// for (symbol, quote_data) in &quotes {
    ///     println!("Symbol: {}, LTP: {}, Change: {}", 
    ///         symbol, quote_data.last_price, quote_data.net_change);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn quote(&self, instruments: Vec<&str>) -> Result<Quote> {
        let params: Vec<_> = instruments.into_iter().map(|i| ("i", i)).collect();
        let url = self.build_url("/quote", Some(params));
        let resp = self.send_request(url, "GET", None).await?;
        self.parse_response(resp).await
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
    /// A `Result<QuoteOHLC>` containing a HashMap mapping instrument symbols to OHLC data with fields:
    /// - `last_price` - Last traded price
    /// - `ohlc` - Open, High, Low, Close values for the day
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
    /// // Access OHLC data directly
    /// for (symbol, data) in &ohlc_data {
    ///     println!("Symbol: {}, LTP: {}, Open: {}, High: {}, Low: {}, Close: {}", 
    ///         symbol, data.last_price, data.ohlc.open, data.ohlc.high, 
    ///         data.ohlc.low, data.ohlc.close);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn ohlc(&self, instruments: Vec<&str>) -> Result<QuoteOHLC> {
        let params: Vec<_> = instruments.into_iter().map(|i| ("i", i)).collect();
        let url = self.build_url("/quote/ohlc", Some(params));
        let resp = self.send_request(url, "GET", None).await?;
        self.parse_response(resp).await
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
    /// A `Result<QuoteLTP>` containing a HashMap mapping instrument symbols to LTP data with fields:
    /// - `last_price` - Last traded price
    /// - `instrument_token` - Instrument token
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
    /// // Access LTP data directly  
    /// for (symbol, data) in &ltp_data {
    ///     println!("Symbol: {}, LTP: {}", symbol, data.last_price);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn ltp(&self, instruments: Vec<&str>) -> Result<QuoteLTP> {
        let params: Vec<_> = instruments.into_iter().map(|i| ("i", i)).collect();
        let url = self.build_url("/quote/ltp", Some(params));
        let resp = self.send_request(url, "GET", None).await?;
        self.parse_response(resp).await
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
    /// A `Result<Margins>` containing segment-specific margin data with fields like:
    /// - `enabled` - Whether the segment is enabled for the user
    /// - `net` - Net cash balance available for trading
    /// - `available` - Available cash and margin breakdown
    /// - `used` - Utilised margin details
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
    /// println!("Equity margins available: {}", equity_margins.available.live_balance);
    /// println!("Equity segment enabled: {}", equity_margins.enabled);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn instruments_margins(&self, segment: &str) -> Result<Margins> {
        let url = self.build_url(&format!("/margins/{}", segment), None);
        let resp = self.send_request(url, "GET", None).await?;
        self.parse_response(resp).await
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
