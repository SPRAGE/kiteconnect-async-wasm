//! # Endpoint Management Module
//!
//! This module provides centralized endpoint definitions and rate limiting
//! configuration for all KiteConnect API endpoints. It ensures consistent
//! URL construction, proper HTTP methods, and compliance with API rate limits.
//!
//! ## Architecture
//!
//! The endpoint system is built around three core components:
//! 1. **`KiteEndpoint`** - Enum defining all available API endpoints
//! 2. **`RateLimitCategory`** - Categorizes endpoints by their rate limits  
//! 3. **`HttpMethod`** - Defines HTTP methods used by endpoints
//!
//! ## Rate Limiting
//!
//! KiteConnect API enforces different rate limits based on endpoint functionality:
//! - **Quote data**: 1 request/second (most restrictive)
//! - **Historical data**: 3 requests/second
//! - **Order operations**: 10 requests/second  
//! - **Standard operations**: 10 requests/second
//!
//! ## URL Construction
//!
//! Each endpoint knows how to construct its URL path and determine its HTTP method:
//!
//! ```rust
//! use kiteconnect_async_wasm::connect::endpoints::{KiteEndpoint, HttpMethod};
//!
//! let endpoint = KiteEndpoint::Quote;
//! assert_eq!(endpoint.path(), "/quote");
//! assert_eq!(endpoint.method(), HttpMethod::GET);
//! ```
//!
//! ## Thread Safety
//!
//! All types in this module are `Send + Sync` and can be safely used across
//! multiple threads without synchronization.

use std::time::Duration;

/// HTTP method types for API requests
///
/// Represents the standard HTTP methods used by KiteConnect API endpoints.
/// Each endpoint uses a specific HTTP method based on the operation type:
/// - **GET**: Data retrieval (quotes, holdings, orders)
/// - **POST**: Resource creation (place orders, create GTT)
/// - **PUT**: Resource updates (modify orders, update GTT)
/// - **DELETE**: Resource deletion (cancel orders, delete GTT)
///
/// # Example
///
/// ```rust
/// use kiteconnect_async_wasm::connect::endpoints::HttpMethod;
///
/// let method = HttpMethod::GET;
/// assert_eq!(method.as_str(), "GET");
///
/// // Check method type
/// assert!(matches!(method, HttpMethod::GET));
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HttpMethod {
    /// HTTP GET method for data retrieval
    GET,
    /// HTTP POST method for resource creation  
    POST,
    /// HTTP PUT method for resource updates
    PUT,
    /// HTTP DELETE method for resource deletion
    DELETE,
}

impl HttpMethod {
    /// Convert HTTP method to string for use with reqwest
    ///
    /// # Returns
    ///
    /// A static string slice representing the HTTP method
    ///
    /// # Example
    ///
    /// ```rust
    /// use kiteconnect_async_wasm::connect::endpoints::HttpMethod;
    ///
    /// assert_eq!(HttpMethod::GET.as_str(), "GET");
    /// assert_eq!(HttpMethod::POST.as_str(), "POST");
    /// ```
    pub fn as_str(&self) -> &'static str {
        match self {
            HttpMethod::GET => "GET",
            HttpMethod::POST => "POST",
            HttpMethod::PUT => "PUT",
            HttpMethod::DELETE => "DELETE",
        }
    }
}

/// Rate limit categories based on official KiteConnect API documentation
///
/// KiteConnect API enforces different rate limits for different types of operations
/// to ensure fair usage and system stability. Understanding these categories is
/// crucial for building responsive applications that don't hit rate limits.
///
/// ## Category Details
///
/// - **Quote**: Real-time market data (most restrictive at 1 req/sec)
/// - **Historical**: Historical market data (3 req/sec)  
/// - **Orders**: Trading operations (10 req/sec)
/// - **Standard**: General operations (10 req/sec)
///
/// ## Rate Limit Enforcement
///
/// Rate limits are enforced using a token bucket algorithm where:
/// 1. Each category has a bucket with a specific capacity
/// 2. Tokens are consumed for each request  
/// 3. Tokens are refilled at the category's rate
/// 4. Requests wait when no tokens are available
///
/// # Example
///
/// ```rust
/// use kiteconnect_async_wasm::connect::endpoints::RateLimitCategory;
///
/// let category = RateLimitCategory::Quote;
/// assert_eq!(category.requests_per_second(), 1);
///
/// let category = RateLimitCategory::Orders;
/// assert_eq!(category.requests_per_second(), 10);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RateLimitCategory {
    /// Quote endpoints: 1 request/second
    ///
    /// Includes real-time market data endpoints like quotes, LTP, and OHLC.
    /// These have the most restrictive limits due to the high-frequency nature
    /// of market data and server load considerations.
    Quote,

    /// Historical candle endpoints: 3 requests/second
    ///
    /// Includes historical OHLC data endpoints. These limits balance the need
    /// for historical analysis with server resource management.
    Historical,

    /// Order placement endpoints: 10 requests/second
    ///
    /// Includes order placement, modification, and cancellation endpoints.
    /// Higher limits support active trading while preventing system abuse.
    Orders,

    /// All other endpoints: 10 requests/second
    ///
    /// Default category for portfolio, profile, and other general operations.
    /// Provides good throughput for typical application usage patterns.
    Standard,
}

impl RateLimitCategory {
    /// Get the rate limit for this category (requests per second)
    ///
    /// # Returns
    ///
    /// The maximum number of requests allowed per second for this category
    ///
    /// # Example
    ///
    /// ```rust
    /// use kiteconnect_async_wasm::connect::endpoints::RateLimitCategory;
    ///
    /// assert_eq!(RateLimitCategory::Quote.requests_per_second(), 1);
    /// assert_eq!(RateLimitCategory::Historical.requests_per_second(), 3);  
    /// assert_eq!(RateLimitCategory::Orders.requests_per_second(), 10);
    /// assert_eq!(RateLimitCategory::Standard.requests_per_second(), 10);
    /// ```
    pub fn requests_per_second(&self) -> u32 {
        match self {
            RateLimitCategory::Quote => 1,
            RateLimitCategory::Historical => 3,
            RateLimitCategory::Orders => 10,
            RateLimitCategory::Standard => 10,
        }
    }

    /// Get the minimum delay between requests for this category
    pub fn min_delay(&self) -> Duration {
        Duration::from_millis(1000 / self.requests_per_second() as u64)
    }
}

/// Endpoint configuration containing method, path, and rate limit info
#[derive(Debug, Clone)]
pub struct Endpoint {
    /// HTTP method for this endpoint
    pub method: HttpMethod,
    /// URL path for this endpoint (without parameters)
    pub path: &'static str,
    /// Rate limit category for this endpoint
    pub rate_limit_category: RateLimitCategory,
    /// Whether this endpoint requires authentication
    pub requires_auth: bool,
}

impl Endpoint {
    /// Create a new endpoint configuration
    pub const fn new(
        method: HttpMethod,
        path: &'static str,
        rate_limit_category: RateLimitCategory,
        requires_auth: bool,
    ) -> Self {
        Self {
            method,
            path,
            rate_limit_category,
            requires_auth,
        }
    }
}

/// Comprehensive enum of all KiteConnect API endpoints
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum KiteEndpoint {
    // === Authentication Endpoints ===
    /// Generate login URL
    LoginUrl,
    /// Generate session from request token
    GenerateSession,
    /// Invalidate session
    InvalidateSession,
    /// Renew access token
    RenewAccessToken,
    /// Invalidate refresh token
    InvalidateRefreshToken,

    // === User Profile Endpoints ===
    /// Get user profile
    Profile,
    /// Get user margins
    Margins,
    /// Get segment-specific margins
    MarginsSegment,

    // === Portfolio Endpoints ===
    /// Get holdings
    Holdings,
    /// Get positions
    Positions,
    /// Convert position
    ConvertPosition,

    // === Order Management Endpoints ===
    /// Place order
    PlaceOrder,
    /// Modify order
    ModifyOrder,
    /// Cancel order
    CancelOrder,
    /// Get all orders
    Orders,
    /// Get order history
    OrderHistory,
    /// Get trades
    Trades,
    /// Get order trades
    OrderTrades,

    // === Market Data Endpoints (Quote Category) ===
    /// Get real-time quotes
    Quote,
    /// Get OHLC data
    OHLC,
    /// Get Last Traded Price
    LTP,

    // === Market Data Endpoints (Historical Category) ===
    /// Get historical data
    HistoricalData,

    // === Market Data Endpoints (Standard Category) ===
    /// Get instruments list
    Instruments,
    /// Get MF instruments
    MFInstruments,
    /// Get trigger range
    TriggerRange,
    /// Get market margins
    MarketMargins,

    // === Mutual Fund Endpoints ===
    /// Place MF order
    PlaceMFOrder,
    /// Cancel MF order
    CancelMFOrder,
    /// Get MF orders
    MFOrders,
    /// Get MF order info
    MFOrderInfo,
    /// Get MF holdings
    MFHoldings,
    /// Place SIP
    PlaceSIP,
    /// Modify SIP
    ModifySIP,
    /// Cancel SIP
    CancelSIP,
    /// Get SIPs
    SIPs,
    /// Get SIP info
    SIPInfo,

    // === GTT Endpoints ===
    /// Place GTT
    PlaceGTT,
    /// Modify GTT
    ModifyGTT,
    /// Cancel GTT
    CancelGTT,
    /// Get GTTs
    GTTs,
    /// Get GTT info
    GTTInfo,
}

impl KiteEndpoint {
    /// Get endpoint configuration for this endpoint
    pub fn config(&self) -> Endpoint {
        match self {
            // === Authentication Endpoints ===
            KiteEndpoint::LoginUrl => Endpoint::new(
                HttpMethod::GET,
                "/connect/login",
                RateLimitCategory::Standard,
                false,
            ),
            KiteEndpoint::GenerateSession => Endpoint::new(
                HttpMethod::POST,
                "/session/token",
                RateLimitCategory::Standard,
                false,
            ),
            KiteEndpoint::InvalidateSession => Endpoint::new(
                HttpMethod::DELETE,
                "/session/token",
                RateLimitCategory::Standard,
                true,
            ),
            KiteEndpoint::RenewAccessToken => Endpoint::new(
                HttpMethod::POST,
                "/session/refresh_token",
                RateLimitCategory::Standard,
                true,
            ),
            KiteEndpoint::InvalidateRefreshToken => Endpoint::new(
                HttpMethod::DELETE,
                "/session/refresh_token",
                RateLimitCategory::Standard,
                true,
            ),

            // === User Profile Endpoints ===
            KiteEndpoint::Profile => Endpoint::new(
                HttpMethod::GET,
                "/user/profile",
                RateLimitCategory::Standard,
                true,
            ),
            KiteEndpoint::Margins => Endpoint::new(
                HttpMethod::GET,
                "/user/margins",
                RateLimitCategory::Standard,
                true,
            ),
            KiteEndpoint::MarginsSegment => Endpoint::new(
                HttpMethod::GET,
                "/user/margins",
                RateLimitCategory::Standard,
                true,
            ),

            // === Portfolio Endpoints ===
            KiteEndpoint::Holdings => Endpoint::new(
                HttpMethod::GET,
                "/portfolio/holdings",
                RateLimitCategory::Standard,
                true,
            ),
            KiteEndpoint::Positions => Endpoint::new(
                HttpMethod::GET,
                "/portfolio/positions",
                RateLimitCategory::Standard,
                true,
            ),
            KiteEndpoint::ConvertPosition => Endpoint::new(
                HttpMethod::PUT,
                "/portfolio/positions",
                RateLimitCategory::Standard,
                true,
            ),

            // === Order Management Endpoints ===
            KiteEndpoint::PlaceOrder => {
                Endpoint::new(HttpMethod::POST, "/orders", RateLimitCategory::Orders, true)
            }
            KiteEndpoint::ModifyOrder => {
                Endpoint::new(HttpMethod::PUT, "/orders", RateLimitCategory::Orders, true)
            }
            KiteEndpoint::CancelOrder => Endpoint::new(
                HttpMethod::DELETE,
                "/orders",
                RateLimitCategory::Orders,
                true,
            ),
            KiteEndpoint::Orders => Endpoint::new(
                HttpMethod::GET,
                "/orders",
                RateLimitCategory::Standard,
                true,
            ),
            KiteEndpoint::OrderHistory => Endpoint::new(
                HttpMethod::GET,
                "/orders",
                RateLimitCategory::Standard,
                true,
            ),
            KiteEndpoint::Trades => Endpoint::new(
                HttpMethod::GET,
                "/trades",
                RateLimitCategory::Standard,
                true,
            ),
            KiteEndpoint::OrderTrades => Endpoint::new(
                HttpMethod::GET,
                "/orders",
                RateLimitCategory::Standard,
                true,
            ),

            // === Market Data Endpoints (Quote Category) ===
            KiteEndpoint::Quote => {
                Endpoint::new(HttpMethod::GET, "/quote", RateLimitCategory::Quote, true)
            }
            KiteEndpoint::OHLC => Endpoint::new(
                HttpMethod::GET,
                "/quote/ohlc",
                RateLimitCategory::Quote,
                true,
            ),
            KiteEndpoint::LTP => Endpoint::new(
                HttpMethod::GET,
                "/quote/ltp",
                RateLimitCategory::Quote,
                true,
            ),

            // === Market Data Endpoints (Historical Category) ===
            KiteEndpoint::HistoricalData => Endpoint::new(
                HttpMethod::GET,
                "/instruments/historical",
                RateLimitCategory::Historical,
                true,
            ),

            // === Market Data Endpoints (Standard Category) ===
            KiteEndpoint::Instruments => Endpoint::new(
                HttpMethod::GET,
                "/instruments",
                RateLimitCategory::Standard,
                true,
            ),
            KiteEndpoint::MFInstruments => Endpoint::new(
                HttpMethod::GET,
                "/mf/instruments",
                RateLimitCategory::Standard,
                true,
            ),
            KiteEndpoint::TriggerRange => Endpoint::new(
                HttpMethod::GET,
                "/instruments/trigger_range",
                RateLimitCategory::Standard,
                true,
            ),
            KiteEndpoint::MarketMargins => Endpoint::new(
                HttpMethod::GET,
                "/margins",
                RateLimitCategory::Standard,
                true,
            ),

            // === Mutual Fund Endpoints ===
            KiteEndpoint::PlaceMFOrder => Endpoint::new(
                HttpMethod::POST,
                "/mf/orders",
                RateLimitCategory::Orders,
                true,
            ),
            KiteEndpoint::CancelMFOrder => Endpoint::new(
                HttpMethod::DELETE,
                "/mf/orders",
                RateLimitCategory::Orders,
                true,
            ),
            KiteEndpoint::MFOrders => Endpoint::new(
                HttpMethod::GET,
                "/mf/orders",
                RateLimitCategory::Standard,
                true,
            ),
            KiteEndpoint::MFOrderInfo => Endpoint::new(
                HttpMethod::GET,
                "/mf/orders",
                RateLimitCategory::Standard,
                true,
            ),
            KiteEndpoint::MFHoldings => Endpoint::new(
                HttpMethod::GET,
                "/mf/holdings",
                RateLimitCategory::Standard,
                true,
            ),
            KiteEndpoint::PlaceSIP => Endpoint::new(
                HttpMethod::POST,
                "/mf/sips",
                RateLimitCategory::Orders,
                true,
            ),
            KiteEndpoint::ModifySIP => {
                Endpoint::new(HttpMethod::PUT, "/mf/sips", RateLimitCategory::Orders, true)
            }
            KiteEndpoint::CancelSIP => Endpoint::new(
                HttpMethod::DELETE,
                "/mf/sips",
                RateLimitCategory::Orders,
                true,
            ),
            KiteEndpoint::SIPs => Endpoint::new(
                HttpMethod::GET,
                "/mf/sips",
                RateLimitCategory::Standard,
                true,
            ),
            KiteEndpoint::SIPInfo => Endpoint::new(
                HttpMethod::GET,
                "/mf/sips",
                RateLimitCategory::Standard,
                true,
            ),

            // === GTT Endpoints ===
            KiteEndpoint::PlaceGTT => Endpoint::new(
                HttpMethod::POST,
                "/gtt/triggers",
                RateLimitCategory::Orders,
                true,
            ),
            KiteEndpoint::ModifyGTT => Endpoint::new(
                HttpMethod::PUT,
                "/gtt/triggers",
                RateLimitCategory::Orders,
                true,
            ),
            KiteEndpoint::CancelGTT => Endpoint::new(
                HttpMethod::DELETE,
                "/gtt/triggers",
                RateLimitCategory::Orders,
                true,
            ),
            KiteEndpoint::GTTs => Endpoint::new(
                HttpMethod::GET,
                "/gtt/triggers",
                RateLimitCategory::Standard,
                true,
            ),
            KiteEndpoint::GTTInfo => Endpoint::new(
                HttpMethod::GET,
                "/gtt/triggers",
                RateLimitCategory::Standard,
                true,
            ),
        }
    }

    /// Get the HTTP method for this endpoint
    pub fn method(&self) -> HttpMethod {
        self.config().method
    }

    /// Get the base path for this endpoint
    pub fn path(&self) -> &'static str {
        self.config().path
    }

    /// Get the rate limit category for this endpoint
    pub fn rate_limit_category(&self) -> RateLimitCategory {
        self.config().rate_limit_category
    }

    /// Check if this endpoint requires authentication
    pub fn requires_auth(&self) -> bool {
        self.config().requires_auth
    }

    /// Build the full URL path with dynamic segments
    ///
    /// # Arguments
    ///
    /// * `segments` - Dynamic path segments to append
    ///
    /// # Example
    ///
    /// ```rust
    /// use kiteconnect_async_wasm::connect::endpoints::KiteEndpoint;
    ///
    /// let endpoint = KiteEndpoint::OrderHistory;
    /// let path = endpoint.build_path(&["order_id_123"]);
    /// assert_eq!(path, "/orders/order_id_123");
    /// ```
    pub fn build_path(&self, segments: &[&str]) -> String {
        let base_path = self.path();
        if segments.is_empty() {
            base_path.to_string()
        } else {
            format!("{}/{}", base_path, segments.join("/"))
        }
    }

    /// Get all endpoints in a specific rate limit category
    pub fn by_rate_limit_category(category: RateLimitCategory) -> Vec<KiteEndpoint> {
        use KiteEndpoint::*;

        let all_endpoints = vec![
            LoginUrl,
            GenerateSession,
            InvalidateSession,
            RenewAccessToken,
            Profile,
            Margins,
            MarginsSegment,
            Holdings,
            Positions,
            ConvertPosition,
            PlaceOrder,
            ModifyOrder,
            CancelOrder,
            Orders,
            OrderHistory,
            Trades,
            OrderTrades,
            Quote,
            OHLC,
            LTP,
            HistoricalData,
            Instruments,
            MFInstruments,
            TriggerRange,
            MarketMargins,
            PlaceMFOrder,
            CancelMFOrder,
            MFOrders,
            MFOrderInfo,
            MFHoldings,
            PlaceSIP,
            ModifySIP,
            CancelSIP,
            SIPs,
            SIPInfo,
            PlaceGTT,
            ModifyGTT,
            CancelGTT,
            GTTs,
            GTTInfo,
        ];

        all_endpoints
            .into_iter()
            .filter(|endpoint| endpoint.rate_limit_category() == category)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rate_limit_categories() {
        assert_eq!(RateLimitCategory::Quote.requests_per_second(), 1);
        assert_eq!(RateLimitCategory::Historical.requests_per_second(), 3);
        assert_eq!(RateLimitCategory::Orders.requests_per_second(), 10);
        assert_eq!(RateLimitCategory::Standard.requests_per_second(), 10);
    }

    #[test]
    fn test_endpoint_configuration() {
        let quote_endpoint = KiteEndpoint::Quote;
        let config = quote_endpoint.config();

        assert_eq!(config.method, HttpMethod::GET);
        assert_eq!(config.path, "/quote");
        assert_eq!(config.rate_limit_category, RateLimitCategory::Quote);
        assert!(config.requires_auth);
    }

    #[test]
    fn test_build_path() {
        let endpoint = KiteEndpoint::OrderHistory;
        assert_eq!(endpoint.build_path(&[]), "/orders");
        assert_eq!(endpoint.build_path(&["order_123"]), "/orders/order_123");
        assert_eq!(
            endpoint.build_path(&["order_123", "trades"]),
            "/orders/order_123/trades"
        );
    }

    #[test]
    fn test_endpoint_methods() {
        assert_eq!(KiteEndpoint::Quote.method(), HttpMethod::GET);
        assert_eq!(KiteEndpoint::PlaceOrder.method(), HttpMethod::POST);
        assert_eq!(KiteEndpoint::ModifyOrder.method(), HttpMethod::PUT);
        assert_eq!(KiteEndpoint::CancelOrder.method(), HttpMethod::DELETE);
    }

    #[test]
    fn test_rate_limit_grouping() {
        let quote_endpoints = KiteEndpoint::by_rate_limit_category(RateLimitCategory::Quote);
        assert!(quote_endpoints.contains(&KiteEndpoint::Quote));
        assert!(quote_endpoints.contains(&KiteEndpoint::OHLC));
        assert!(quote_endpoints.contains(&KiteEndpoint::LTP));

        let historical_endpoints =
            KiteEndpoint::by_rate_limit_category(RateLimitCategory::Historical);
        assert!(historical_endpoints.contains(&KiteEndpoint::HistoricalData));

        let order_endpoints = KiteEndpoint::by_rate_limit_category(RateLimitCategory::Orders);
        assert!(order_endpoints.contains(&KiteEndpoint::PlaceOrder));
        assert!(order_endpoints.contains(&KiteEndpoint::ModifyOrder));
        assert!(order_endpoints.contains(&KiteEndpoint::CancelOrder));
    }

    #[test]
    fn test_authentication_requirements() {
        assert!(!KiteEndpoint::LoginUrl.requires_auth());
        assert!(!KiteEndpoint::GenerateSession.requires_auth());
        assert!(KiteEndpoint::Profile.requires_auth());
        assert!(KiteEndpoint::Holdings.requires_auth());
        assert!(KiteEndpoint::PlaceOrder.requires_auth());
    }

    #[test]
    fn test_min_delay_calculation() {
        assert_eq!(
            RateLimitCategory::Quote.min_delay(),
            Duration::from_millis(1000)
        );
        assert_eq!(
            RateLimitCategory::Historical.min_delay(),
            Duration::from_millis(333)
        );
        assert_eq!(
            RateLimitCategory::Orders.min_delay(),
            Duration::from_millis(100)
        );
        assert_eq!(
            RateLimitCategory::Standard.min_delay(),
            Duration::from_millis(100)
        );
    }
}
