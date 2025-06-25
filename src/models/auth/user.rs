/*!
User profile and account information data structures.

Handles user details, account types, and user preferences.
*/

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// User profile information from the `profile` API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    /// User ID
    pub user_id: String,

    /// User name/display name
    pub user_name: String,

    /// User short name
    pub user_shortname: String,

    /// User type ("individual", "corporate", etc.)
    pub user_type: String,

    /// Email address
    pub email: String,

    /// Avatar URL
    #[serde(default)]
    pub avatar_url: Option<String>,

    /// Broker identifier  
    pub broker: String,

    /// List of enabled exchanges
    pub exchanges: Vec<String>,

    /// List of enabled products
    pub products: Vec<String>,

    /// List of enabled order types
    pub order_types: Vec<String>,

    /// User metadata
    #[serde(default)]
    pub meta: Option<UserMeta>,
}

/// Additional user metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserMeta {
    /// Demat consent status
    #[serde(default)]
    pub demat_consent: String,
}

impl UserProfile {
    /// Check if user has access to a specific exchange
    pub fn has_exchange(&self, exchange: &str) -> bool {
        self.exchanges.iter().any(|e| e == exchange)
    }

    /// Check if user has access to a specific product
    pub fn has_product(&self, product: &str) -> bool {
        self.products.iter().any(|p| p == product)
    }

    /// Check if user has access to a specific order type
    pub fn has_order_type(&self, order_type: &str) -> bool {
        self.order_types.iter().any(|o| o == order_type)
    }

    /// Get display name (prefer user_name, fallback to user_shortname)
    pub fn display_name(&self) -> &str {
        if !self.user_name.is_empty() {
            &self.user_name
        } else {
            &self.user_shortname
        }
    }

    /// Check if profile has essential information
    pub fn is_complete(&self) -> bool {
        !self.user_id.is_empty() && !self.email.is_empty() && !self.exchanges.is_empty()
    }
}

/// User type enumeration for type-safe handling
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum UserType {
    Individual,
    Corporate,
    Partnership,
    Huf, // Hindu Undivided Family
}

impl UserType {
    /// Check if user type supports specific features
    pub fn supports_family_account(&self) -> bool {
        matches!(self, UserType::Huf)
    }

    /// Check if user type is individual
    pub fn is_individual(&self) -> bool {
        matches!(self, UserType::Individual)
    }

    /// Check if user type is business-related
    pub fn is_business(&self) -> bool {
        matches!(self, UserType::Corporate | UserType::Partnership)
    }
}

impl std::fmt::Display for UserType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserType::Individual => write!(f, "individual"),
            UserType::Corporate => write!(f, "corporate"),
            UserType::Partnership => write!(f, "partnership"),
            UserType::Huf => write!(f, "huf"),
        }
    }
}

impl From<String> for UserType {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "individual" => UserType::Individual,
            "corporate" => UserType::Corporate,
            "partnership" => UserType::Partnership,
            "huf" => UserType::Huf,
            _ => UserType::Individual, // Default fallback
        }
    }
}

impl From<&str> for UserType {
    fn from(s: &str) -> Self {
        Self::from(s.to_string())
    }
}

/// Account status information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountStatus {
    /// Account is active
    pub active: bool,

    /// Trading is enabled
    pub trading_enabled: bool,

    /// Account creation date
    #[serde(default)]
    pub created_at: Option<DateTime<Utc>>,

    /// Last login timestamp
    #[serde(default)]
    pub last_login: Option<DateTime<Utc>>,

    /// Account restrictions (if any)
    #[serde(default)]
    pub restrictions: Vec<String>,

    /// KYC status
    #[serde(default)]
    pub kyc_status: Option<String>,
}

impl AccountStatus {
    /// Check if account can place trades
    pub fn can_trade(&self) -> bool {
        self.active && self.trading_enabled && self.restrictions.is_empty()
    }

    /// Check if account has any restrictions
    pub fn has_restrictions(&self) -> bool {
        !self.restrictions.is_empty()
    }

    /// Check if KYC is complete
    pub fn is_kyc_complete(&self) -> bool {
        self.kyc_status
            .as_ref()
            .map(|status| status.to_lowercase() == "complete")
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_profile() {
        let profile = UserProfile {
            user_id: "TEST123".to_string(),
            user_name: "Test User".to_string(),
            user_shortname: "testuser".to_string(),
            user_type: "individual".to_string(),
            email: "test@example.com".to_string(),
            avatar_url: None,
            broker: "ZERODHA".to_string(),
            exchanges: vec!["NSE".to_string(), "BSE".to_string()],
            products: vec!["CNC".to_string(), "MIS".to_string()],
            order_types: vec!["MARKET".to_string(), "LIMIT".to_string()],
            meta: None,
        };

        assert!(profile.is_complete());
        assert!(profile.has_exchange("NSE"));
        assert!(!profile.has_exchange("MCX"));
        assert_eq!(profile.display_name(), "Test User");
    }

    #[test]
    fn test_user_type() {
        let individual = UserType::Individual;
        assert!(individual.is_individual());
        assert!(!individual.is_business());
        assert!(!individual.supports_family_account());

        let corporate = UserType::Corporate;
        assert!(!corporate.is_individual());
        assert!(corporate.is_business());

        let huf = UserType::Huf;
        assert!(huf.supports_family_account());

        // Test string conversion
        assert_eq!(UserType::from("individual"), UserType::Individual);
        assert_eq!(UserType::from("corporate"), UserType::Corporate);
        assert_eq!(UserType::from("unknown"), UserType::Individual); // Default fallback
    }

    #[test]
    fn test_account_status() {
        let mut status = AccountStatus {
            active: true,
            trading_enabled: true,
            created_at: None,
            last_login: None,
            restrictions: vec![],
            kyc_status: Some("complete".to_string()),
        };

        assert!(status.can_trade());
        assert!(!status.has_restrictions());
        assert!(status.is_kyc_complete());

        // Add restriction
        status.restrictions.push("day_trading_disabled".to_string());
        assert!(!status.can_trade());
        assert!(status.has_restrictions());

        // Test KYC status
        status.kyc_status = Some("pending".to_string());
        assert!(!status.is_kyc_complete());
    }
}
