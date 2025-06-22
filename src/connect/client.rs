use serde_json::Value as JsonValue;
use anyhow::{anyhow, Context, Result};
use std::collections::HashMap;
use reqwest::header::{HeaderMap, AUTHORIZATION, USER_AGENT};

// Import model types for typed responses
use crate::model::{
    KiteResponse, KiteErrorResponse,
};

use super::request::RequestHandler;

// Native platform imports
#[cfg(all(feature = "native", not(target_arch = "wasm32")))]
use sha2::{Sha256, Digest};

// WASM platform imports  
#[cfg(all(feature = "wasm", target_arch = "wasm32"))]
use web_sys::window;

#[cfg(all(feature = "wasm", target_arch = "wasm32"))]
use js_sys::Uint8Array;

#[cfg(all(feature = "wasm", target_arch = "wasm32"))]
use wasm_bindgen_futures::JsFuture;

#[cfg(not(test))]
const URL: &str = "https://api.kite.trade";

#[cfg(test)]
const URL: &str = "http://127.0.0.1:1234";

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
    pub(super) api_key: String,
    /// Access token for authenticated requests
    pub(super) access_token: String,
    /// Optional callback for session expiry handling
    pub(super) session_expiry_hook: Option<fn() -> ()>,
    /// HTTP client for making requests (shared and reusable)
    pub(super) client: reqwest::Client,
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
    pub(super) fn build_url(&self, path: &str, param: Option<Vec<(&str, &str)>>) -> reqwest::Url {
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
    pub(super) async fn raise_or_return_json(&self, resp: reqwest::Response) -> Result<JsonValue> {
        if resp.status().is_success() {
            let jsn: JsonValue = resp.json().await.with_context(|| "Serialization failed")?;
            Ok(jsn)
        } else {
            let error_text = resp.text().await?;
            Err(anyhow!(error_text))
        }
    }

    /// Helper method to parse typed responses from KiteConnect API
    pub(super) async fn parse_response<T>(&self, resp: reqwest::Response) -> Result<T>
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
        refresh_token: &str,
        api_secret: &str,
    ) -> Result<JsonValue> {
        let input = format!("{}{}{}", self.api_key, refresh_token, api_secret);
        let checksum = self.compute_checksum(&input).await?;

        let api_key: &str = &self.api_key.clone();
        let mut data = HashMap::new();
        data.insert("api_key", api_key);
        data.insert("refresh_token", refresh_token);
        data.insert("checksum", checksum.as_str());

        let url = self.build_url("/session/refresh_token", None);
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

    /// Invalidates the refresh token
    pub async fn invalidate_refresh_token(&self, refresh_token: &str) -> Result<reqwest::Response> {
        let url = self.build_url("/session/refresh_token", None);
        let mut data = HashMap::new();
        data.insert("refresh_token", refresh_token);

        self.send_request(url, "DELETE", Some(data)).await
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
            let mut headers = HeaderMap::new();
            headers.insert("XKiteVersion", "3".parse().unwrap());
            headers.insert(
                AUTHORIZATION,
                format!("token {}:{}", self.api_key, self.access_token)
                    .parse()
                    .unwrap(),
            );
            headers.insert(USER_AGENT, "Rust".parse().unwrap());

            #[cfg(feature = "debug")]
            log::debug!("Making {} request to: {}", method, url);

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
