/*!
Session management data structures for KiteConnect authentication.

Handles login responses, access tokens, and session validation.
*/

use serde::{Deserialize, Serialize};

/// Response from the `generate_session` API call
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionData {
    /// The unique, permanent user id registered with the broker and the exchanges
    pub user_id: String,

    /// User's real name
    pub user_name: String,

    /// Shortened version of the user's real name
    pub user_shortname: String,

    /// User's email
    pub email: String,

    /// User's registered role at the broker. This will be individual for all retail users
    pub user_type: String,

    /// The broker ID
    pub broker: String,

    /// Exchanges enabled for trading on the user's account
    pub exchanges: Vec<String>,

    /// Margin product types enabled for the user
    pub products: Vec<String>,

    /// Order types enabled for the user
    pub order_types: Vec<String>,

    /// The API key for which the authentication was performed
    pub api_key: String,

    /// The authentication token that's used with every subsequent request
    /// Unless this is invalidated using the API, or invalidated by a master-logout
    /// from the Kite Web trading terminal, it'll expire at 6 AM on the next day (regulatory requirement)
    pub access_token: String,

    /// A token for public session validation where requests may be exposed to the public
    #[serde(default)]
    pub public_token: String,

    /// A token for getting long standing read permissions. This is only available to certain approved platforms
    #[serde(default)]
    pub refresh_token: String,

    /// User's last login time
    pub login_time: String,

    /// Session metadata containing demat_consent and other user metadata
    #[serde(default)]
    pub meta: Option<SessionMeta>,

    /// Full URL to the user's avatar (PNG image) if there's one
    #[serde(default)]
    pub avatar_url: Option<String>,
}

/// Additional session metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionMeta {
    /// Demat consent status
    #[serde(default)]
    pub demat_consent: String,
}

impl SessionData {
    /// Check if the session has required authentication data
    pub fn is_valid(&self) -> bool {
        !self.access_token.is_empty() && !self.user_id.is_empty()
    }

    /// Get the access token for API requests
    pub fn token(&self) -> &str {
        &self.access_token
    }

    /// Check if the user has access to a specific exchange
    pub fn has_exchange(&self, exchange: &str) -> bool {
        self.exchanges.iter().any(|e| e == exchange)
    }

    /// Check if the user has access to a specific product
    pub fn has_product(&self, product: &str) -> bool {
        self.products.iter().any(|p| p == product)
    }

    /// Check if the user has access to a specific order type
    pub fn has_order_type(&self, order_type: &str) -> bool {
        self.order_types.iter().any(|o| o == order_type)
    }
}

/// Login URL configuration for OAuth flow
#[derive(Debug, Clone)]
pub struct LoginUrlConfig {
    /// Base login URL
    pub base_url: String,

    /// API key
    pub api_key: String,

    /// Redirect URL after login
    pub redirect_url: Option<String>,

    /// State parameter for CSRF protection
    pub state: Option<String>,
}

impl LoginUrlConfig {
    /// Create new login URL configuration
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            base_url: "https://kite.trade/connect/login".to_string(),
            api_key: api_key.into(),
            redirect_url: None,
            state: None,
        }
    }

    /// Set redirect URL
    pub fn with_redirect_url(mut self, url: impl Into<String>) -> Self {
        self.redirect_url = Some(url.into());
        self
    }

    /// Set state parameter
    pub fn with_state(mut self, state: impl Into<String>) -> Self {
        self.state = Some(state.into());
        self
    }

    /// Generate the complete login URL
    pub fn build_url(&self) -> crate::models::common::KiteResult<String> {
        use url::Url;

        let mut url = Url::parse(&self.base_url)?;

        // Add required parameters
        url.query_pairs_mut()
            .append_pair("api_key", &self.api_key)
            .append_pair("v", "3"); // API version

        // Add optional parameters
        if let Some(ref redirect_url) = self.redirect_url {
            url.query_pairs_mut()
                .append_pair("redirect_url", redirect_url);
        }

        if let Some(ref state) = self.state {
            url.query_pairs_mut().append_pair("state", state);
        }

        Ok(url.to_string())
    }
}

/// Request token from OAuth callback
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestToken {
    /// Request token received from callback
    pub request_token: String,

    /// State parameter (for CSRF validation)
    #[serde(default)]
    pub state: Option<String>,

    /// Action parameter
    #[serde(default)]
    pub action: Option<String>,

    /// Status parameter
    #[serde(default)]
    pub status: Option<String>,
}

impl RequestToken {
    /// Create new request token
    pub fn new(token: impl Into<String>) -> Self {
        Self {
            request_token: token.into(),
            state: None,
            action: None,
            status: None,
        }
    }

    /// Validate request token format
    pub fn is_valid(&self) -> bool {
        !self.request_token.is_empty() && self.request_token.len() >= 10
    }
}

/// Token invalidation response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogoutResponse {
    /// Success status
    pub success: bool,

    /// Response message
    #[serde(default)]
    pub message: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_data_validation() {
        let mut session = SessionData {
            user_id: "test_user".to_string(),
            user_name: "Test User".to_string(),
            user_shortname: "testuser".to_string(),
            email: "test@example.com".to_string(),
            user_type: "individual".to_string(),
            broker: "ZERODHA".to_string(),
            exchanges: vec!["NSE".to_string(), "BSE".to_string()],
            products: vec!["CNC".to_string(), "MIS".to_string()],
            order_types: vec!["MARKET".to_string(), "LIMIT".to_string()],
            api_key: "test_key".to_string(),
            access_token: "test_token".to_string(),
            public_token: String::new(),
            refresh_token: String::new(),
            login_time: "2024-01-01 10:00:00".to_string(),
            meta: None,
            avatar_url: None,
        };

        assert!(session.is_valid());
        assert!(session.has_exchange("NSE"));
        assert!(!session.has_exchange("MCX"));
        assert!(session.has_product("CNC"));
        assert!(session.has_order_type("MARKET"));

        // Test invalid session
        session.access_token.clear();
        assert!(!session.is_valid());
    }

    #[test]
    fn test_login_url_config() {
        let config = LoginUrlConfig::new("test_api_key")
            .with_redirect_url("https://example.com/callback")
            .with_state("random_state");

        let url = config.build_url().expect("Failed to build URL");
        println!("Generated URL: {}", url);

        assert!(url.contains("api_key=test_api_key"));
        assert!(url.contains("v=3"));
        // The URL encoding might be different, let's check for the unencoded version or partial match
        assert!(url.contains("redirect_url=") && url.contains("example.com"));
        assert!(url.contains("state=random_state"));
    }

    #[test]
    fn test_request_token_validation() {
        let valid_token = RequestToken::new("abcdef1234567890");
        assert!(valid_token.is_valid());

        let invalid_token = RequestToken::new("short");
        assert!(!invalid_token.is_valid());

        let empty_token = RequestToken::new("");
        assert!(!empty_token.is_valid());
    }
}
