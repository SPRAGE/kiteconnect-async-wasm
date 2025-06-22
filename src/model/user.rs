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
    /// User's last login time in format "yyyy-mm-dd hh:mm:ss"
    pub login_time: String,
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
