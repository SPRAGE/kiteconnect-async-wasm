//! # User Models
//! 
//! This module contains user-related data models for the KiteConnect API.
//! 
//! These models handle:
//! - Authentication and token exchange
//! - User profile information  
//! - Funds and margin data
//! - Session management
//!
//! Based on the official Go library implementation at:
//! https://github.com/zerodha/gokiteconnect/blob/master/user.go

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Custom deserializer for login_time field
mod login_time_format {
    use serde::{self, Deserialize, Deserializer};
    use chrono::{DateTime, Utc, NaiveDateTime};

    const FORMAT: &str = "%Y-%m-%d %H:%M:%S";

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        NaiveDateTime::parse_from_str(&s, FORMAT)
            .map_err(serde::de::Error::custom)
            .map(|dt| dt.and_utc())
    }
}

/// User session response after successful authentication
/// 
/// This is returned after successful token exchange and contains both
/// user profile information and authentication tokens.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSession {
    /// The unique, permanent user ID registered with the broker and exchanges
    pub user_id: String,
    /// User's real name
    pub user_name: String,
    /// Shortened version of the user's real name
    pub user_shortname: String,
    /// Full URL to the user's avatar (PNG image) if available
    pub avatar_url: Option<String>,
    /// User's registered role at the broker (e.g., "individual" for retail users)
    pub user_type: String,
    /// User's email address
    pub email: String,
    /// The broker ID (e.g., "ZERODHA")
    pub broker: String,
    /// Additional user metadata
    pub meta: UserMeta,
    /// Margin product types enabled for the user
    pub products: Vec<String>,
    /// Order types enabled for the user
    pub order_types: Vec<String>,
    /// Exchanges enabled for trading on the user's account
    pub exchanges: Vec<String>,
    /// The API key for which the authentication was performed
    pub api_key: String,
    /// The authentication token used with every subsequent request
    /// Expires at 6 AM on the next day unless invalidated
    pub access_token: String,
    /// A token for public session validation where requests may be exposed to the public
    pub public_token: String,
    /// A token for getting long standing read permissions (available to certain approved platforms)
    pub refresh_token: Option<String>,
    /// User's last login time as a parsed UTC datetime
    #[serde(deserialize_with = "login_time_format::deserialize")]
    pub login_time: DateTime<Utc>,
}

/// User session tokens for token renewal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSessionTokens {
    /// User ID
    pub user_id: String,
    /// New access token
    pub access_token: String,
    /// Refresh token for future renewals
    pub refresh_token: String,
}

/// User profile information without authentication tokens
/// 
/// Contains user details without authentication tokens.
/// This is returned by the /user/profile endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    /// The unique, permanent user ID registered with the broker and exchanges
    pub user_id: String,
    /// User's real name
    pub user_name: String,
    /// Shortened version of the user's real name
    pub user_shortname: String,
    /// Full URL to the user's avatar (PNG image) if available
    pub avatar_url: Option<String>,
    /// User's registered role at the broker
    pub user_type: String,
    /// User's email address
    pub email: String,
    /// The broker ID
    pub broker: String,
    /// Additional user metadata
    pub meta: UserMeta,
    /// Margin product types enabled for the user
    pub products: Vec<String>,
    /// Order types enabled for the user
    pub order_types: Vec<String>,
    /// Exchanges enabled for trading on the user's account
    pub exchanges: Vec<String>,
}

/// Full user profile with additional information
/// 
/// Extended profile information including bank details, phone, PAN, etc.
/// This is returned by the /user/profile/full endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FullUserProfile {
    /// The unique, permanent user ID registered with the broker and exchanges
    pub user_id: String,
    /// User's real name
    pub user_name: String,
    /// Full URL to the user's avatar (PNG image) if available
    pub avatar_url: Option<String>,
    /// User's registered role at the broker
    pub user_type: String,
    /// User's email address
    pub email: String,
    /// User's phone number
    pub phone: String,
    /// The broker ID
    pub broker: String,
    /// Two-factor authentication type
    #[serde(rename = "twofa_type")]
    pub two_fa_type: String,
    /// List of linked bank accounts
    #[serde(rename = "bank_accounts")]
    pub banks: Vec<Bank>,
    /// Depository participant IDs
    #[serde(rename = "dp_ids")]
    pub dp_ids: Vec<String>,
    /// Margin product types enabled for the user
    pub products: Vec<String>,
    /// Order types enabled for the user
    pub order_types: Vec<String>,
    /// Exchanges enabled for trading on the user's account
    pub exchanges: Vec<String>,
    /// PAN card number
    pub pan: String,
    /// Shortened version of the user's real name
    pub user_shortname: String,
    /// User tags
    pub tags: Vec<String>,
    /// Password last changed timestamp
    pub password_timestamp: String,
    /// Two-factor authentication setup timestamp
    #[serde(rename = "twofa_timestamp")]
    pub two_fa_timestamp: String,
    /// Extended user metadata
    pub meta: FullUserMeta,
}

/// Bank account details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bank {
    /// Bank name
    pub name: String,
    /// Branch name
    pub branch: String,
    /// Account number
    pub account: String,
}

/// Basic user metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserMeta {
    /// Demat consent status: "empty", "consent", or "physical"
    pub demat_consent: String,
}

/// Extended user metadata for full profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FullUserMeta {
    /// Power of Attorney status (demat consent)
    #[serde(rename = "poa")]
    pub demat_consent: String,
    /// Silo information
    pub silo: String,
    /// Account blocks/restrictions
    pub account_blocks: Vec<String>,
}

/// All margins containing both equity and commodity segments
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllMargins {
    /// Equity segment margins
    pub equity: Margins,
    /// Commodity segment margins
    pub commodity: Margins,
}

/// Margins for a specific segment (equity or commodity)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Margins {
    /// Indicates whether the segment is enabled for the user
    pub enabled: bool,
    /// Net cash balance available for trading
    /// (intraday_payin + adhoc_margin + collateral)
    pub net: f64,
    /// Available cash and margin details
    pub available: AvailableMargins,
    /// Utilised margin details
    #[serde(rename = "utilised")]
    pub used: UsedMargins,
}

/// Available cash and margin breakdown
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvailableMargins {
    /// Additional margin provided by the broker
    pub adhoc_margin: f64,
    /// Raw cash balance in the account available for trading
    pub cash: f64,
    /// Margin derived from pledged stocks
    pub collateral: f64,
    /// Amount that was deposited during the day
    pub intraday_payin: f64,
    /// Current available balance
    pub live_balance: f64,
    /// Opening balance at the day start
    pub opening_balance: f64,
}

/// Utilised margin breakdown
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsedMargins {
    /// Sum of all utilised margins
    pub debits: f64,
    /// Exposure margin blocked for all open F&O positions
    pub exposure: f64,
    /// Booked intraday profits and losses
    pub m2m_realised: f64,
    /// Un-booked (open) intraday profits and losses
    pub m2m_unrealised: f64,
    /// Value of options premium received by shorting
    pub option_premium: f64,
    /// Funds paid out or withdrawn to bank account during the day
    pub payout: f64,
    /// SPAN margin blocked for all open F&O positions
    pub span: f64,
    /// Value of holdings sold during the day
    pub holding_sales: f64,
    /// Utilised portion of the maximum turnover limit
    pub turnover: f64,
    /// Margin utilised against pledged liquidbees ETFs and liquid mutual funds
    pub liquid_collateral: f64,
    /// Margin utilised against pledged stocks/ETFs
    pub stock_collateral: f64,
    /// Margin blocked when you sell securities from demat or T1 holdings
    pub delivery: f64,
}

/// Authentication request for token exchange
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenRequest {
    /// The public API key
    pub api_key: String,
    /// The one-time token obtained after the login flow
    pub request_token: String,
    /// SHA-256 hash of (api_key + request_token + api_secret)
    pub checksum: String,
}

/// Logout request parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogoutRequest {
    /// The API key
    pub api_key: String,
    /// The access token to invalidate
    pub access_token: String,
}

/// Refresh token request parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshTokenRequest {
    /// The API key
    pub api_key: String,
    /// The refresh token
    pub refresh_token: String,
    /// SHA-256 hash of (api_key + refresh_token + api_secret)
    pub checksum: String,
}

// Implementation methods for convenience and API compatibility

impl UserSession {
    /// Get formatted authorization header value
    pub fn authorization_header(&self) -> String {
        format!("token {}:{}", self.api_key, self.access_token)
    }

    /// Check if the user has access to a specific exchange
    pub fn has_exchange(&self, exchange: &str) -> bool {
        self.exchanges.contains(&exchange.to_string())
    }

    /// Check if the user has access to a specific product type
    pub fn has_product(&self, product: &str) -> bool {
        self.products.contains(&product.to_string())
    }

    /// Check if the user can place a specific order type
    pub fn has_order_type(&self, order_type: &str) -> bool {
        self.order_types.contains(&order_type.to_string())
    }

    /// Get the login time formatted as a string
    pub fn login_time_formatted(&self) -> String {
        self.login_time.format("%Y-%m-%d %H:%M:%S UTC").to_string()
    }

    /// Get the duration since the last login
    pub fn time_since_login(&self) -> chrono::Duration {
        Utc::now().signed_duration_since(self.login_time)
    }

    /// Check if the login was within the last specified number of hours
    pub fn logged_in_within_hours(&self, hours: i64) -> bool {
        let duration = self.time_since_login();
        duration.num_hours() <= hours
    }

    /// Check if the login was today
    pub fn logged_in_today(&self) -> bool {
        let today = Utc::now().date_naive();
        let login_date = self.login_time.date_naive();
        today == login_date
    }
}

impl UserProfile {
    /// Check if the user has access to a specific exchange
    pub fn has_exchange(&self, exchange: &str) -> bool {
        self.exchanges.contains(&exchange.to_string())
    }

    /// Check if the user has access to a specific product type
    pub fn has_product(&self, product: &str) -> bool {
        self.products.contains(&product.to_string())
    }

    /// Check if the user can place a specific order type
    pub fn has_order_type(&self, order_type: &str) -> bool {
        self.order_types.contains(&order_type.to_string())
    }
}

impl FullUserProfile {
    /// Check if the user has access to a specific exchange
    pub fn has_exchange(&self, exchange: &str) -> bool {
        self.exchanges.contains(&exchange.to_string())
    }

    /// Check if the user has access to a specific product type
    pub fn has_product(&self, product: &str) -> bool {
        self.products.contains(&product.to_string())
    }

    /// Check if the user can place a specific order type
    pub fn has_order_type(&self, order_type: &str) -> bool {
        self.order_types.contains(&order_type.to_string())
    }
}

impl Margins {
    /// Calculate total available funds (net + collateral)
    pub fn total_available(&self) -> f64 {
        self.net + self.available.collateral
    }

    /// Calculate utilisation ratio (used / available)
    pub fn utilisation_ratio(&self) -> f64 {
        if self.net > 0.0 {
            self.used.debits / self.net
        } else {
            0.0
        }
    }

    /// Check if the segment has sufficient funds for a given amount
    pub fn has_sufficient_funds(&self, required_amount: f64) -> bool {
        self.net >= required_amount
    }

    /// Get the margin available for new positions
    pub fn available_for_new_positions(&self) -> f64 {
        (self.net - self.used.span - self.used.exposure).max(0.0)
    }
}

impl AllMargins {
    /// Get total net funds across both segments
    pub fn total_net_funds(&self) -> f64 {
        self.equity.net + self.commodity.net
    }

    /// Check if user has sufficient funds in any segment
    pub fn has_sufficient_funds_any_segment(&self, required_amount: f64) -> bool {
        self.equity.has_sufficient_funds(required_amount) ||
        self.commodity.has_sufficient_funds(required_amount)
    }

    /// Get margin information for equity segment
    pub fn equity_segment(&self) -> &Margins {
        &self.equity
    }

    /// Get margin information for commodity segment
    pub fn commodity_segment(&self) -> &Margins {
        &self.commodity
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Utc, Datelike, Timelike};

    #[test]
    fn test_login_time_deserialization() {
        // Test that we can deserialize a string datetime into DateTime<Utc>
        let json = r#"{
            "user_id": "AB1234",
            "user_name": "John Doe",
            "user_shortname": "John",
            "avatar_url": null,
            "user_type": "individual",
            "email": "john@example.com",
            "broker": "ZERODHA",
            "meta": {"demat_consent": "consent"},
            "products": ["CNC", "MIS"],
            "order_types": ["MARKET", "LIMIT"],
            "exchanges": ["NSE", "BSE"],
            "api_key": "test_api_key",
            "access_token": "test_access_token",
            "public_token": "test_public_token",
            "refresh_token": "test_refresh_token",
            "login_time": "2024-01-15 09:15:00"
        }"#;

        let session: UserSession = serde_json::from_str(json).unwrap();
        
        // Verify the datetime was parsed correctly
        assert_eq!(session.user_id, "AB1234");
        assert_eq!(session.login_time.year(), 2024);
        assert_eq!(session.login_time.month(), 1);
        assert_eq!(session.login_time.day(), 15);
        assert_eq!(session.login_time.hour(), 9);
        assert_eq!(session.login_time.minute(), 15);
        assert_eq!(session.login_time.second(), 0);
    }

    #[test]
    fn test_user_session_datetime_methods() {
        let session = UserSession {
            user_id: "AB1234".to_string(),
            user_name: "John Doe".to_string(),
            user_shortname: "John".to_string(),
            avatar_url: None,
            user_type: "individual".to_string(),
            email: "john@example.com".to_string(),
            broker: "ZERODHA".to_string(),
            meta: UserMeta { demat_consent: "consent".to_string() },
            products: vec!["CNC".to_string()],
            order_types: vec!["MARKET".to_string()],
            exchanges: vec!["NSE".to_string()],
            api_key: "test_api_key".to_string(),
            access_token: "test_access_token".to_string(),
            public_token: "test_public_token".to_string(),
            refresh_token: Some("test_refresh_token".to_string()),
            login_time: Utc::now() - chrono::Duration::hours(2), // 2 hours ago
        };

        // Test the helper methods
        assert!(session.login_time_formatted().contains("UTC"));
        assert!(session.time_since_login().num_hours() >= 1);
        assert!(session.logged_in_within_hours(3));
        assert!(!session.logged_in_within_hours(1));
        assert!(session.logged_in_today());
    }
}
