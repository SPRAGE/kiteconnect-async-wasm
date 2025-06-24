//! # Authentication Module
//! 
//! This module contains authentication-related methods for the KiteConnect API.

use serde_json::Value as JsonValue;
use anyhow::{anyhow, Result};
use std::collections::HashMap;
use crate::connect::endpoints::KiteEndpoint;

// Import typed models for dual API support
use crate::models::common::KiteResult;
use crate::models::auth::{SessionData, UserProfile};

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

use crate::connect::KiteConnect;

impl KiteConnect {
    // === LEGACY API METHODS (JSON responses) ===
    
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
    /// 3. User completes login and is redirected with `request_token` parameter
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

        let resp = self.send_request_with_rate_limiting_and_retry(
            KiteEndpoint::GenerateSession, 
            &[],
            None,
            Some(data)
        ).await.map_err(|e| anyhow!("Generate session failed: {:?}", e))?;

        if resp.status().is_success() {
            let jsn: JsonValue = resp.json().await?;
            self.set_access_token(jsn["data"]["access_token"].as_str().unwrap());
            Ok(jsn)
        } else {
            let error_text: String = resp.text().await?;
            Err(anyhow!(error_text))
        }
    }

    /// Invalidates the access token
    pub async fn invalidate_access_token(&self, access_token: &str) -> Result<reqwest::Response> {
        let mut data = HashMap::new();
        data.insert("access_token", access_token);

        self.send_request_with_rate_limiting_and_retry(
            KiteEndpoint::InvalidateSession, 
            &[],
            None,
            Some(data)
        ).await.map_err(|e| anyhow!("Invalidate access token failed: {:?}", e))
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

        let resp = self.send_request_with_rate_limiting_and_retry(
            KiteEndpoint::RenewAccessToken, 
            &[],
            None,
            Some(data)
        ).await.map_err(|e| anyhow!("Renew access token failed: {:?}", e))?;

        if resp.status().is_success() {
            let jsn: JsonValue = resp.json().await?;
            self.set_access_token(jsn["access_token"].as_str().unwrap());
            Ok(jsn)
        } else {
            let error_text: String = resp.text().await?;
            Err(anyhow!(error_text))
        }
    }

    /// Invalidates the refresh token
    pub async fn invalidate_refresh_token(&self, refresh_token: &str) -> Result<reqwest::Response> {
        let mut data = HashMap::new();
        data.insert("refresh_token", refresh_token);

        self.send_request_with_rate_limiting_and_retry(
            KiteEndpoint::InvalidateRefreshToken, 
            &[],
            None,
            Some(data)
        ).await.map_err(|e| anyhow!("Invalidate refresh token failed: {:?}", e))
    }

    // === TYPED API METHODS (v1.0.0) ===
    
    /// Generates session with typed response
    /// 
    /// Returns strongly typed session data instead of JsonValue.
    /// This is the preferred method for new applications.
    /// 
    /// # Arguments
    /// 
    /// * `request_token` - The request token received after user login
    /// * `api_secret` - Your KiteConnect API secret
    /// 
    /// # Returns
    /// 
    /// A `KiteResult<SessionData>` containing typed session information
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
    /// let session = client.generate_session_typed("request_token", "api_secret").await?;
    /// println!("Access token: {}", session.access_token);
    /// println!("User ID: {}", session.user_id);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn generate_session_typed(
        &mut self,
        request_token: &str,
        api_secret: &str,
    ) -> KiteResult<SessionData> {
        let json_response = self.generate_session(request_token, api_secret).await
            .map_err(|e| crate::models::common::KiteError::Legacy(e))?;
        
        // Extract the data field from response
        let data = json_response["data"].clone();
        self.parse_response(data)
    }

    /// Get user profile with typed response
    /// 
    /// Returns strongly typed user profile data instead of JsonValue.
    /// 
    /// # Returns
    /// 
    /// A `KiteResult<UserProfile>` containing typed user profile information
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
    /// let profile = client.profile_typed().await?;
    /// println!("User: {} ({})", profile.user_name, profile.email);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn profile_typed(&self) -> KiteResult<UserProfile> {
        let resp = self.send_request_with_rate_limiting_and_retry(
            KiteEndpoint::Profile, 
            &[],
            None,
            None
        ).await?;
        let json_response = self.raise_or_return_json_typed(resp).await?;
        
        // Extract the data field from response
        let data = json_response["data"].clone();
        self.parse_response(data)
    }
}
